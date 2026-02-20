use crate::routes::{health_check, subscribe};
use axum::{
    Router,
    routing::{get, post},
    serve::Serve,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(db_pool);
    Ok(axum::serve(listener, app))
}
