use crate::game::{GamePhase, GameSession, PersistedSession, PlayerId};
use crate::protocol::{ClientMessage, ServerMessage, StateSnapshot};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

struct ManagedGame {
    session: GameSession,
    connections: HashMap<PlayerId, mpsc::UnboundedSender<Message>>,
    host: Option<PlayerId>,
}

/// DEV-ONLY: naive full-state dump used so restarting the process (e.g. via
/// `cargo watch`) doesn't wipe out an in-progress game. Rewritten wholesale
/// after every mutation — no migrations, no locking across processes. If a
/// game type's shape changes, loading old data will just fail and fall back
/// to an empty state; delete this file any time you want a true fresh start.
const DEV_STATE_FILE: &str = "dev_state.json";

#[derive(Serialize, Deserialize)]
struct PersistedGame {
    session: PersistedSession,
    host: Option<PlayerId>,
}

#[derive(Clone)]
pub struct AppState {
    games: Arc<Mutex<HashMap<Uuid, Arc<Mutex<ManagedGame>>>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Loads `DEV_STATE_FILE` if present, otherwise starts empty. See the
    /// file-level comment on `DEV_STATE_FILE` — this is a dev convenience,
    /// not a real persistence layer.
    pub async fn load_from_disk() -> Self {
        let state = AppState::new();
        let Ok(contents) = tokio::fs::read_to_string(DEV_STATE_FILE).await else {
            return state;
        };
        let Ok(persisted) = serde_json::from_str::<HashMap<Uuid, PersistedGame>>(&contents) else {
            tracing::warn!("{DEV_STATE_FILE} exists but failed to parse — ignoring, starting fresh");
            return state;
        };
        let count = persisted.len();
        let mut games = state.games.lock().await;
        for (id, pg) in persisted {
            games.insert(
                id,
                Arc::new(Mutex::new(ManagedGame {
                    session: GameSession::from_persisted(pg.session),
                    connections: HashMap::new(),
                    host: pg.host,
                })),
            );
        }
        drop(games);
        tracing::info!("Loaded dev game state from {DEV_STATE_FILE} ({count} games)");
        state
    }

    /// Dumps every game's state to `DEV_STATE_FILE`. Called after every
    /// mutating client message — see the file-level comment on
    /// `DEV_STATE_FILE`.
    async fn save_to_disk(&self) {
        let games = self.games.lock().await;
        let mut persisted: HashMap<Uuid, PersistedGame> = HashMap::with_capacity(games.len());
        for (&id, game_arc) in games.iter() {
            let game = game_arc.lock().await;
            persisted.insert(
                id,
                PersistedGame {
                    session: game.session.to_persisted(),
                    host: game.host,
                },
            );
        }
        drop(games);
        if let Ok(json) = serde_json::to_string_pretty(&persisted) {
            if let Err(e) = tokio::fs::write(DEV_STATE_FILE, json).await {
                tracing::warn!("failed to write {DEV_STATE_FILE}: {e}");
            }
        }
    }
}

pub fn router(state: AppState) -> Router {
    // Permissive CORS: the Vite dev server (localhost:5173) calls this
    // backend cross-origin during development. The production build is
    // served from this same origin, where CORS is a no-op anyway.
    let cors = tower_http::cors::CorsLayer::permissive();

    Router::new()
        .route("/api/games", post(create_game))
        .route("/ws/:game_id", get(ws_handler))
        .nest_service("/", tower_http::services::ServeDir::new("static"))
        .layer(cors)
        .with_state(state)
}

#[derive(Serialize)]
struct CreateGameResponse {
    game_id: Uuid,
}

async fn create_game(State(state): State<AppState>) -> Json<CreateGameResponse> {
    let id = Uuid::new_v4();
    let session = GameSession::new_lobby(id);
    let managed = ManagedGame {
        session,
        connections: HashMap::new(),
        host: None,
    };
    state.games.lock().await.insert(id, Arc::new(Mutex::new(managed)));
    state.save_to_disk().await;
    Json(CreateGameResponse { game_id: id })
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(game_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, game_id, state))
}

fn send_err(tx: &mpsc::UnboundedSender<Message>, message: impl Into<String>) {
    let msg = ServerMessage::Error { message: message.into() };
    let _ = tx.send(Message::Text(serde_json::to_string(&msg).unwrap()));
}

async fn broadcast_state(game: &ManagedGame) {
    for (&player_id, tx) in game.connections.iter() {
        let snapshot = StateSnapshot::for_player(&game.session, player_id);
        let msg = ServerMessage::State { snapshot };
        let _ = tx.send(Message::Text(serde_json::to_string(&msg).unwrap()));
    }
    if game.session.phase == GamePhase::Ended {
        if let Some(result) = game.session.result.clone() {
            for tx in game.connections.values() {
                let msg = ServerMessage::Result { result: result.clone() };
                let _ = tx.send(Message::Text(serde_json::to_string(&msg).unwrap()));
            }
        }
    }
}

async fn handle_socket(socket: WebSocket, game_id: Uuid, state: AppState) {
    let Some(game_arc) = state.games.lock().await.get(&game_id).cloned() else {
        return;
    };

    let (mut ws_tx, mut ws_rx) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    let mut player_id: Option<PlayerId> = None;

    while let Some(Ok(msg)) = ws_rx.next().await {
        let Message::Text(text) = msg else { continue };
        let parsed: Result<ClientMessage, _> = serde_json::from_str(&text);
        let Ok(client_msg) = parsed else {
            send_err(&tx, "could not parse message");
            continue;
        };

        let mut game = game_arc.lock().await;

        match client_msg {
            ClientMessage::Spectate => {
                if player_id.is_some() {
                    send_err(&tx, "already joined");
                    continue;
                }
                let id = Uuid::new_v4();
                player_id = Some(id);
                game.connections.insert(id, tx.clone());
                broadcast_state(&game).await;
            }
            ClientMessage::Join { name } => {
                if player_id.is_some() {
                    send_err(&tx, "already joined");
                    continue;
                }
                match game.session.add_player(name) {
                    Ok(id) => {
                        player_id = Some(id);
                        if game.host.is_none() {
                            game.host = Some(id);
                        }
                        game.connections.insert(id, tx.clone());
                        let _ = tx.send(Message::Text(
                            serde_json::to_string(&ServerMessage::Joined { player_id: id }).unwrap(),
                        ));
                        broadcast_state(&game).await;
                    }
                    Err(e) => send_err(&tx, e),
                }
            }
            ClientMessage::Start => {
                let Some(pid) = player_id else {
                    send_err(&tx, "join before starting");
                    continue;
                };
                if game.host != Some(pid) {
                    send_err(&tx, "only the host can start the game");
                    continue;
                }
                match game.session.start() {
                    Ok(()) => broadcast_state(&game).await,
                    Err(e) => send_err(&tx, e),
                }
            }
            ClientMessage::Select { option_id, placement } => {
                let Some(pid) = player_id else {
                    send_err(&tx, "join first");
                    continue;
                };
                match game.session.select_option(pid, option_id, placement) {
                    Ok(()) => broadcast_state(&game).await,
                    Err(e) => send_err(&tx, e),
                }
            }
            ClientMessage::Shuffle => {
                let Some(pid) = player_id else {
                    send_err(&tx, "join first");
                    continue;
                };
                match game.session.shuffle_market(pid) {
                    Ok(()) => broadcast_state(&game).await,
                    Err(e) => send_err(&tx, e),
                }
            }
        }
        drop(game);
        state.save_to_disk().await;
    }

    if let Some(pid) = player_id {
        let mut game = game_arc.lock().await;
        game.connections.remove(&pid);
    }
    send_task.abort();
}
