use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT};

use crate::{base::yat::Yat, cli::CreateAddress, settings::Settings};

use super::{
    edit_request::EditRequest,
    error::ClientError,
    payment_address::{ApiError, FriendlyPaymentAddress, PaymentAddressResponse},
};

/// Remote API interface
pub struct ApiClient<'a> {
    settings: &'a Settings,
}

impl<'a> ApiClient<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }

    fn full_uri(&self, path: &str) -> String {
        format!("{}{}", self.settings.api_url, path)
    }

    fn headers(&self) -> Result<HeaderMap, ClientError> {
        let api_key = HeaderValue::from_str(&self.settings.yat_api_key)
            .map_err(|_| ClientError::InvalidApiKey)?;

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(HeaderName::from_static("x-api-key"), api_key);
        Ok(headers)
    }

    /// Fetch payment addresses
    pub async fn fetch_payment_addresses(
        &self,
        yat: &Yat,
    ) -> Result<Vec<FriendlyPaymentAddress>, ClientError> {
        let uri = self.full_uri(&format!("/emoji_id/{}/payment", yat.as_ref()));
        let response = reqwest::Client::new()
            .get(uri)
            .headers(self.headers()?)
            .send()
            .await?
            .assert_success()?
            .json::<PaymentAddressResponse>()
            .await?;
        if let Some(ApiError { code, reason }) = response.error {
            return Err(ClientError::ApiResponseError { code, reason });
        }

        response
            .result
            .unwrap_or_default()
            .into_values()
            .map(|a| FriendlyPaymentAddress::try_from(a).map_err(|err| err.into()))
            .collect()
    }

    /// Create a payment address
    pub async fn create_payment_address(
        &self,
        yat: &Yat,
        create_address: CreateAddress,
    ) -> Result<(), ClientError> {
        let uri = self.full_uri(&format!("/emoji_id/{}", yat.as_ref()));

        // We care only for status code
        let _ = reqwest::Client::new()
            .patch(uri)
            .json::<EditRequest>(&create_address.into())
            .headers(self.headers()?)
            .send()
            .await?
            .assert_success()?;

        Ok(())
    }
}

trait ResponseExt {
    fn assert_success(self) -> Result<reqwest::Response, ClientError>;
}

impl ResponseExt for reqwest::Response {
    /// Extracts 404 error codes
    fn assert_success(self) -> Result<reqwest::Response, ClientError> {
        let status = self.status();
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(ClientError::YatNotFound);
        }
        Ok(self.error_for_status()?)
    }
}
