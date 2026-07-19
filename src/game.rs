//! Pure turn-engine / game-state logic — no networking here (see
//! `server.rs` for the WebSocket layer that wraps this). Kept network-free
//! so the rules can be unit-tested directly.

use crate::balance::TOTAL_TURNS_PER_PLAYER;
use crate::board::Board;
use crate::growth::{run_growth_pass, GrowthReport};
use crate::hex::Hex;
use crate::market::{self, MarketOption};
use crate::objectives::{deal_objective, evaluate_objective, SecretObjective};
use crate::scoring::group_threshold_status;
use crate::species::FoodWebEdge;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type PlayerId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    #[serde(skip)]
    pub objective: Option<SecretObjective>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Lobby,
    InProgress,
    Ended,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum PlacementInput {
    Terrain { origin: Hex, rotation: i32 },
    Animal { hex: Hex },
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerResult {
    pub player_id: PlayerId,
    pub name: String,
    pub objective: SecretObjective,
    pub met: bool,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct GameResult {
    pub group_threshold_met: bool,
    pub total_population: u32,
    pub longest_corridor: usize,
    pub players: Vec<PlayerResult>,
    pub winners: Vec<PlayerId>,
}

pub struct GameSession {
    pub id: Uuid,
    pub board: Board,
    edges: Vec<FoodWebEdge>,
    pub players: Vec<Player>,
    pub turn_order: Vec<PlayerId>,
    pub current_turn_idx: usize,
    pub turns_taken: u32,
    pub total_turns: u32,
    pub market_row: Vec<MarketOption>,
    pub phase: GamePhase,
    pub result: Option<GameResult>,
    pub last_growth: Option<GrowthReport>,
    rng: StdRng,
}

impl GameSession {
    pub fn new_lobby(id: Uuid) -> Self {
        GameSession {
            id,
            board: Board::new(crate::balance::BOARD_RADIUS),
            edges: market::species_edges(),
            players: Vec::new(),
            turn_order: Vec::new(),
            current_turn_idx: 0,
            turns_taken: 0,
            total_turns: 0,
            market_row: Vec::new(),
            phase: GamePhase::Lobby,
            result: None,
            last_growth: None,
            rng: StdRng::from_entropy(),
        }
    }

    pub fn add_player(&mut self, name: String) -> Result<PlayerId, String> {
        if self.phase != GamePhase::Lobby {
            return Err("game already started".into());
        }
        let id = Uuid::new_v4();
        self.players.push(Player {
            id,
            name,
            objective: None,
        });
        Ok(id)
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.phase != GamePhase::Lobby {
            return Err("game already started".into());
        }
        if self.players.is_empty() {
            return Err("need at least one player".into());
        }
        for p in self.players.iter_mut() {
            p.objective = Some(deal_objective(&mut self.rng));
        }
        self.turn_order = self.players.iter().map(|p| p.id).collect();
        self.total_turns = self.players.len() as u32 * TOTAL_TURNS_PER_PLAYER;
        crate::board::seed_starting_terrain(&mut self.board, &mut self.rng);
        self.market_row = market::generate_market_row(&self.board, &self.edges, &mut self.rng);
        self.phase = GamePhase::InProgress;
        Ok(())
    }

    pub fn current_player(&self) -> Option<PlayerId> {
        self.turn_order.get(self.current_turn_idx).copied()
    }

    fn objective_for(&self, player_id: PlayerId) -> Option<SecretObjective> {
        self.players.iter().find(|p| p.id == player_id).and_then(|p| p.objective)
    }

    pub fn select_option(
        &mut self,
        player_id: PlayerId,
        option_id: Uuid,
        placement: PlacementInput,
    ) -> Result<(), String> {
        if self.phase != GamePhase::InProgress {
            return Err("game is not in progress".into());
        }
        if self.current_player() != Some(player_id) {
            return Err("not your turn".into());
        }
        let idx = self
            .market_row
            .iter()
            .position(|o| o.id() == option_id)
            .ok_or("no such market option")?;

        match (&self.market_row[idx], &placement) {
            (MarketOption::TerrainShape { terrain, offsets, .. }, PlacementInput::Terrain { origin, rotation }) => {
                if !market::can_place_shape(&self.board, *origin, offsets, *rotation) {
                    return Err("shape does not fit there".into());
                }
                let placed = market::rotated_translated(offsets, *origin, *rotation);
                self.board.place_terrain_shape(&placed, *terrain);
                Ok(())
            }
            (MarketOption::AnimalPlacement { species, .. }, PlacementInput::Animal { hex }) => {
                self.board.place_animal(*hex, *species)
            }
            _ => Err("placement type does not match the chosen market option".into()),
        }?;

        self.market_row[idx] = market::generate_market_row(&self.board, &self.edges, &mut self.rng)
            .into_iter()
            .next()
            .unwrap();
        self.advance_turn();
        Ok(())
    }

    pub fn shuffle_market(&mut self, player_id: PlayerId) -> Result<(), String> {
        if self.phase != GamePhase::InProgress {
            return Err("game is not in progress".into());
        }
        if self.current_player() != Some(player_id) {
            return Err("not your turn".into());
        }
        self.market_row = market::generate_market_row(&self.board, &self.edges, &mut self.rng);
        self.advance_turn();
        Ok(())
    }

    fn advance_turn(&mut self) {
        self.turns_taken += 1;
        self.current_turn_idx = (self.current_turn_idx + 1) % self.turn_order.len();
        if self.current_turn_idx == 0 {
            let report = run_growth_pass(&mut self.board, &self.edges, &mut self.rng);
            self.last_growth = Some(report);
        }
        if self.turns_taken >= self.total_turns {
            self.end_game();
        }
    }

    fn end_game(&mut self) {
        let status = group_threshold_status(&self.board);
        let mut player_results = Vec::new();
        for p in &self.players {
            let objective = p.objective.expect("objectives dealt at start()");
            let outcome = evaluate_objective(&self.board, objective);
            player_results.push(PlayerResult {
                player_id: p.id,
                name: p.name.clone(),
                objective,
                met: outcome.met,
                score: outcome.score,
            });
        }
        let winners = if status.met {
            let best = player_results.iter().map(|r| r.score).max().unwrap_or(0);
            player_results
                .iter()
                .filter(|r| r.score == best)
                .map(|r| r.player_id)
                .collect()
        } else {
            Vec::new()
        };
        self.result = Some(GameResult {
            group_threshold_met: status.met,
            total_population: status.total_population,
            longest_corridor: status.longest_corridor,
            players: player_results,
            winners,
        });
        self.phase = GamePhase::Ended;
    }

    /// Player-scoped objective the caller is allowed to see (their own only).
    pub fn my_objective(&self, player_id: PlayerId) -> Option<SecretObjective> {
        self.objective_for(player_id)
    }
}
