use actix_web::{get, web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct Parameters {
    subscription_token: String,
}
#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
#[get("/subscriptions/confirm")]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
