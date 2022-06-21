use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String
}

/** Page 68 **/
#[post("/subscriptions")]
pub async fn subscribe(
	form: web::Form<FormData>,
	pool: web::Data<PgPool>
) -> HttpResponse {
	match sqlx::query!(
		r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4)
		"#,
		Uuid::new_v4(),
		form.email,
		form.name,
		Utc::now()
	)
	.execute(pool.get_ref())
	.await 
	{
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => {
			println!("Failed to execute query: {}", e);
			HttpResponse::InternalServerError().finish()
		} 
	}
}
