use actix_web::{ web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::routes::*;
use sqlx::PgPool;


pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    
    let db_pool =  web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
        	.service(health_check)
        	.service(subscribe)
        	.app_data(db_pool.clone())
    })
    .listen(listener)?
	.run();
    
   	Ok(server)
}