use crate::settings::Settings;

use super::emoji::{drop_combining_marks, is_valid_emoji};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum YatError {
    #[error("Yat contains invalid emoji characters")]
    InvalidEmojis,
    #[error("Yat has invalid length")]
    InvalidLength,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Yat {
    signature: String,
}

impl Yat {
    pub fn new(settings: &Settings, signature: &str) -> Result<Self, YatError> {
        let s: Vec<char> = drop_combining_marks(signature.chars()).collect();
        if !settings.required_yat_length.contains(&(s.len() as u32)) {
            return Err(YatError::InvalidLength);
        }
        if !s.into_iter().all(is_valid_emoji) {
            return Err(YatError::InvalidEmojis);
        }
        Ok(Self {
            signature: signature.to_owned(),
        })
    }
}

impl AsRef<str> for Yat {
    fn as_ref(&self) -> &str {
        &self.signature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> Settings {
        Settings {
            api_url: "https://".to_owned(),
            yat_api_key: "KEY".to_owned(),
            required_yat_length: 2..=3,
        }
    }

    #[test]
    fn it_fails_if_yat_length_is_less_than_required() {
        assert_eq!(Yat::new(&settings(), "💪"), Err(YatError::InvalidLength));
    }

    #[test]
    fn it_fails_if_yat_length_is_greater_than_required() {
        assert_eq!(
            Yat::new(&settings(), "💪💪💪💪"),
            Err(YatError::InvalidLength)
        );
    }

    #[test]
    fn it_fails_if_yat_contains_invalid_emojis() {
        assert_eq!(Yat::new(&settings(), "XX"), Err(YatError::InvalidEmojis));
    }

    #[test]
    fn it_creates_yat_if_length_is_adequate() {
        let yat = Yat::new(&settings(), "💪💪");
        match yat {
            Ok(yat) => {
                assert_eq!(yat.as_ref(), "💪💪")
            }
            Err(_) => panic!(),
        }
    }
}
