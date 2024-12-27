use unicode_segmentation::UnicodeSegmentation;

const FORBIDDEN_CHARACTERS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

pub struct NewSubscrber {
    pub email: String,
    pub name: SubscriberName,
}

#[allow(dead_code)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let contains_forbidden_characters = s.chars().any(|g| FORBIDDEN_CHARACTERS.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{} is not a valid subscriber name.", s)
        } else {
            Self(s)
        }
    }
}
