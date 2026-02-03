use tokio::net::TcpListener;
use zero2prod_axum_v2::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").await.expect("Failed to bind port 8000");
    run(listener).await?.await?;
    Ok(())
}
