use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<SubscriberEmail, String> {
        if validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(format!("{} is not a valid email.", email))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::{assert_err, assert_ok};

    #[test]
    fn email_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_string_is_whitespace() {
        let email = " ".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_string_missing_symbol() {
        let email = "johndoe.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_string_id_missing() {
        let email = "@gmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_string_valid_address() {
        let email = "johndoe@example.com".to_string();
        assert_ok!(SubscriberEmail::parse(email));
    }
}
