use axum::{http::StatusCode, routing::get, serve::Serve, Router};
use tokio::net::TcpListener;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run(listener: TcpListener) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let app = Router::new().route("/health_check", get(health_check));

    Ok(axum::serve(listener, app))
}