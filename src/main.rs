use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::{ PgPool };

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let configuration = get_configuration().expect("Failed to read configuration");
	let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect postgres");

	let address = format!("127.0.0.1:{}", configuration.application_port);
	let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener, connection_pool)?.await
}