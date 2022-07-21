use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<SubscriberName, String> {
        let is_empty_or_white_space = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 60;
        let forbidden_chars = ['/', '\\', '(', ')', '"', '<', '>', '{', '}'];
        let contains_forbidden_chars = name.chars().any(|c| forbidden_chars.contains(&c));

        if is_empty_or_white_space || is_too_long || contains_forbidden_chars {
            Err(format!("{} is not valid subscriber name", name))
        } else {
            Ok(Self(name))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_60_grapheme_long_name_is_valid() {
        let name = "a".repeat(60);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_60_grapheme_long_name_is_valid() {
        let name = "a".repeat(61);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_name_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_name_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_chars_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "John Doe".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
