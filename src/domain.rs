use unicode_segmentation::UnicodeSegmentation;


pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<SubscriberName, String> {
        let is_empty_or_white_space = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 60;
        let forbidden_chars = ['/', '\\', '(', ')', '"', '<', '>', '{', '}'];
        let contains_forbidden_chars = name.chars().any(|c| forbidden_chars.contains(&c));

        if is_empty_or_white_space || is_too_long || contains_forbidden_chars {
            panic!("{} is not valid subscriber name", name);
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

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
