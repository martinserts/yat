use crate::base::currency::CurrencyError;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("API error `{0}`")]
    ReqwestError(#[from] reqwest::Error),
    #[error("API response error `{code}`: {reason}")]
    ApiResponseError { code: String, reason: String },
    #[error("Currency error `{0}`")]
    CurrencyError(#[from] CurrencyError),
    #[error("Yat was not found")]
    YatNotFound,
}
