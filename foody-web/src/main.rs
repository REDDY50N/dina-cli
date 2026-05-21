use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(hello));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🍽️ foody-web läuft auf http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "🍽️ foody-web - Hello World!"
}