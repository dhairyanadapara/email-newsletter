use crate::domain::SubscriberEmail;
use reqwest::header;
use reqwest::Client;
use secrecy::ExposeSecret;
use secrecy::Secret;

#[derive(Clone, Debug, serde::Serialize)]
struct SenderEmail<'a> {
    email: &'a str,
}

impl<'a> SenderEmail<'a> {
    fn new(email: &'a str) -> SenderEmail {
        SenderEmail { email }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
struct Personalizations<'a> {
    to: Vec<RecipientsEmail<'a>>,
}

impl<'a> Personalizations<'a> {
    fn new(email: &'a str) -> Personalizations {
        Personalizations {
            to: vec![RecipientsEmail::new(email)],
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
struct RecipientsEmail<'a> {
    email: &'a str,
}

impl<'a> RecipientsEmail<'a> {
    fn new(email: &'a str) -> RecipientsEmail {
        RecipientsEmail { email }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
struct Content<'a> {
    r#type: &'a str,
    value: &'a str,
}

impl<'a> Content<'a> {
    fn new(r#type: &'a str, value: &'a str) -> Content<'a> {
        Content { r#type, value }
    }
}

#[derive(serde::Serialize, Debug)]
struct SendEmailRequest<'a> {
    from: SenderEmail<'a>,
    personalizations: Vec<Personalizations<'a>>,
    subject: &'a str,
    content: Vec<Content<'a>>,
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
        text: &str,
        html: &str,
    ) -> Result<(), reqwest::Error> {
        let request_body = SendEmailRequest {
            from: SenderEmail::new(self.sender.as_ref()),
            personalizations: vec![Personalizations::new(recipient.as_ref())],
            subject,
            content: vec![Content::new("text/html", html)],
        };

        tracing::info!("request body {:?}", request_body);
        let url = format!("{}/send", self.base_url);
        let _builder = self
            .http_client
            .post(&url)
            .header(header::AUTHORIZATION, self.auth_token.expose_secret())
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
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("Text").is_some()
                    && body.get("Html").is_some()
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

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_server_returns_500() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));

        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        assert_err!(outcome);
    }
}
