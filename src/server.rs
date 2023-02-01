use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn health() -> &'static str {
    "Success"
}

pub async fn start_server() {
    let port_env: u16 = std::env::var("PORT")
        .map(|v| v.parse().unwrap())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port_env));

    let app = Router::new().route("/health", get(health));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
