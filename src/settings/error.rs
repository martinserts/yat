#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum SettingsError {
    #[error(
        "A required environment variable `{0}` was not found. \
             Environment variables can be defined in an .env file \
             located in current or parent directories"
    )]
    VariableMissing(String),
    #[error("Environment variable `{0}` should be a number")]
    NotNumber(String),
    #[error("Environment variable `{0}` should be greater than 0")]
    NotGreaterThanZero(String),
    #[error("Environment variable `{max}` should not be larger than `{min}`")]
    MustNotBeLarger { min: String, max: String },
}
