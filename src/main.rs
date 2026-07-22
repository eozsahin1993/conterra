use conterra::server::{router, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Dev-only convenience: restores whatever was in dev_state.json, if
    // present, so restarting this process doesn't wipe out an in-progress
    // game. Delete that file for a true fresh start.
    let state = AppState::load_from_disk().await;
    let app = router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4173").await.unwrap();
    tracing::info!("Conterra listening on http://127.0.0.1:4173");
    axum::serve(listener, app).await.unwrap();
}
