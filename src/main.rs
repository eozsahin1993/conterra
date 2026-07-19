use conterra::server::{router, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();
    let app = router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4173").await.unwrap();
    tracing::info!("Conterra listening on http://127.0.0.1:4173");
    axum::serve(listener, app).await.unwrap();
}
