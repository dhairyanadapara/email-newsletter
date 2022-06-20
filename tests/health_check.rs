// use actix_web::rt::net::TcpListener;
use std::net::TcpListener;
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;

#[tokio::test]
async fn health_check_test() {

	let address = spawn_app();
	let client = reqwest::Client::new();

	let response = client
					.get(format!("{}/health_check", address))
					.send()
					.await
					.expect("Failed to execute request");

	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_ok_test() {
	let address = spawn_app();
	let configuration = get_configuration().expect("Failed to read configuration");
	let connection_string = configuration.database.connection_string();
	

	let mut connection = PgConnection::connect(&connection_string)
						.await
						.expect("Failed to connect postgres");
	let client = reqwest::Client::new();

	let body = "name=dhairya%20nadapara&email=dhairyanadapara98%40gmail.com";
	let response = client
					.post(&format!("{}/subscriptions", address))
					.header("Content-Type", "application/x-www-form-urlencoded")
					.body(body)
					.send()
					.await
					.expect("Failed to execture request");

	assert_eq!(200, response.status().as_u16());


	let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
					.fetch_one(&mut connection)
					.await
					.expect("Failed to fetch saved subscription");

	assert_eq!(saved.email, "dhairyanadapara98@gmail.com");
	assert_eq!(saved.name, "dhairya nadapara");




}


#[tokio::test]
async fn subscribe_bad_request_test() {
	// Arrange
	let address = spawn_app();
	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20guin", "missing the email"),
		("email=ursula_le_guin%40gmail.com", "missing the name"),
		("", "missing both name and email")
	];

	for (invalid_body, error_message) in test_cases {
		let response = client
						.post(&format!("{}/subscriptions", address))
						.header("Content-Type", "application/x-www-form-urlencoded")
						.body(invalid_body)
						.send()
						.await
						.expect("Failed to execture request");

		assert_eq!(400, response.status().as_u16(), "The API did not fail with 400 Bad Request when the payload was {}.", error_message);
	}

}

fn spawn_app() -> String {
	let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

	let port = listener.local_addr().unwrap().port();
	let server = zero2prod::startup::run(listener).expect("Failed to bind address");
	
	let _ = tokio::spawn(server);
	format!("http://127.0.0.1:{:?}", port)
}