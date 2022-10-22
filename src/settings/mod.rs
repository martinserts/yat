use std::ops::RangeInclusive;

pub mod error;
pub mod loader;
pub mod settings_reader;

#[derive(Debug, Eq, PartialEq)]
pub struct Settings {
    // Corresponds to API_URL env variable
    pub api_url: String,
    // Corresponds to YAT_API_KEY env variable
    pub yat_api_key: String,
    // Corresponds to MIN_YAT_LENGTH and MAX_YAT_LENGTH env variables
    pub required_yat_length: RangeInclusive<u32>,
}
