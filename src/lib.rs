use actix_web::{get, post, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;


#[derive(serde::Deserialize)]
struct FormData {
	email: String,
	name: String
}

#[get("/health_check")]
async fn health_check() ->  HttpResponse {
	HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
	HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        	.service(health_check)
        	.service(subscribe)
    })
    .listen(listener)?
	.run();
    
   	Ok(server)
}