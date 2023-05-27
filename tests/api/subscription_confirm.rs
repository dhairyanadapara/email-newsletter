use std::assert_eq;

use crate::helpers::spawn_app;
use reqwest::Response;
use wiremock::matchers::{method, path};
use wiremock::{Mock, Request, ResponseTemplate};

#[tokio::test]
async fn confirmation_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let response: Response = reqwest::get(format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(400, response.status().as_u16())
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = spawn_app().await;

    // send request email request
    // read confirmation
    // get request
    // open link
    // listen on request
    // validate the token
    // update the status
    // check the status

    let body = "name=dhairya%20nadapara&email=dhairya%40zuru.tech";

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(body.into()).await;
    assert_eq!(200, response.status().as_u16());

    let email_request: &Request = &app.email_server.received_requests().await.unwrap()[0];

    let confirmation_link = app.get_subscription_link(email_request);

    let response = reqwest::get(confirmation_link).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // subscribe to newsletter
    // check mail
    // get confirmation list
    // open link
    // check confirmation status

    let app = spawn_app().await;
    let body = "name=dhairya%20nadapara&email=dhairya%40zuru.tech";

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(body.to_string()).await;

    assert_eq!(200, response.status().as_u16());

    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let inter_data = sqlx::query!("SELECT email, name, status from subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscriptions");

    assert_eq!(inter_data.name, "dhairya nadapara");
    assert_eq!(inter_data.email, "dhairya@zuru.tech");
    assert_eq!(inter_data.status, "pending_confirmation");

    let confirmation_link = app.get_subscription_link(email_request);

    let conf_response = reqwest::get(confirmation_link).await.unwrap();

    assert_eq!(200, conf_response.status().as_u16());

    let data = sqlx::query!("SELECT email, name, status from subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscriptions");

    assert_eq!(data.name, "dhairya nadapara");
    assert_eq!(data.email, "dhairya@zuru.tech");
    assert_eq!(data.status, "confirmed");
}
