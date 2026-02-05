use sqlx::PgPool;
use tokio::net::TcpListener;
use zero2prod_axum_v2::configuration::get_configuration;
use zero2prod_axum_v2::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).await.expect(&format!(
        "Failed to bind port {}",
        configuration.application_port
    ));
    run(listener, connection_pool).await?.await?;
    Ok(())
}
