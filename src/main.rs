use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {
	let configuration = get_configuration().expect("Failed to read configuration");
	let address = format!("127.0.0.1:{}", configuration.application_port);
	let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener)?.await
}