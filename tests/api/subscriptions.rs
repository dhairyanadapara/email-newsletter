use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_ok_test() {
    let app = spawn_app().await;
    let body = "name=dhairya%20nadapara&email=dhairyanadapara98%40gmail.com";
    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "dhairyanadapara98@gmail.com");
    assert_eq!(saved.name, "dhairya nadapara");
}

#[tokio::test]
async fn subscribe_bad_request_test() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("email=not-an-email", "not-an-email"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
