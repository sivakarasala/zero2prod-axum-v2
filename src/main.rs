use sqlx::PgPool;
use tokio::net::TcpListener;
use zero2prod_axum_v2::configuration::get_configuration;
use zero2prod_axum_v2::startup::run;
use zero2prod_axum_v2::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod_axum_v2".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect_lazy_with(configuration.database.connection_options());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).await.expect(&format!(
        "Failed to bind port {}",
        configuration.application.port
    ));
    run(listener, connection_pool).await?.await?;
    Ok(())
}
