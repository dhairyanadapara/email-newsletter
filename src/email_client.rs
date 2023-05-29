use crate::domain::SubscriberEmail;
use reqwest::header;
use reqwest::Client;
use secrecy::ExposeSecret;
use secrecy::Secret;

#[derive(Clone, Debug, serde::Serialize)]
struct SenderEmail<'a> {
    name: &'a str,
    email: &'a str,
}

impl<'a> SenderEmail<'a> {
    fn new(email: &'a str, name: &'a str) -> SenderEmail<'a> {
        SenderEmail { email, name }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
struct RecipientsEmail<'a> {
    email: &'a str,
    name: &'a str,
}

impl<'a> RecipientsEmail<'a> {
    fn new(email: &'a str, name: &'a str) -> RecipientsEmail<'a> {
        RecipientsEmail { email, name }
    }
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SendEmailRequest<'a> {
    sender: SenderEmail<'a>,
    to: Vec<RecipientsEmail<'a>>,
    subject: &'a str,
    html_content: &'a str,
}

#[derive(Clone, Debug)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    auth_token: Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        auth_token: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        Self {
            http_client: Client::builder().timeout(timeout).build().unwrap(),
            base_url,
            sender,
            auth_token,
        }
    }

    #[tracing::instrument(name = "Sending a confirmation email")]
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html: &str,
    ) -> Result<(), reqwest::Error> {
        let request_body = SendEmailRequest {
            sender: SenderEmail::new(self.sender.as_ref(), "Xplorare"),
            to: vec![RecipientsEmail::new(recipient.as_ref(), "User")],
            subject,
            html_content: html,
        };

        tracing::info!("request body {:?}", request_body);
        let url = format!("{}/email", self.base_url);
        let _builder = self
            .http_client
            .post(&url)
            .header("api-key", self.auth_token.expose_secret())
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use secrecy::Secret;
    use wiremock::matchers::*;
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("sender").is_some()
                    && body.get("to").is_some()
                    && body.get("subject").is_some()
                    && body.get("htmlContent").is_some()
            } else {
                false
            }
        }
    }

    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..10).fake()
    }

    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url,
            email(),
            Secret::new(Faker.fake()),
            std::time::Duration::from_millis(200),
        )
    }

    #[tokio::test]
    async fn send_email_succeeds_if_the_server_returns_200() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(path("/email"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_server_returns_500() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(path("/email"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        println!("{:?}", outcome);

        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));

        Mock::given(path("/email"))
            .and(method("POST"))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        assert_err!(outcome);
    }
}
