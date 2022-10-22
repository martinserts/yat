use httpmock::prelude::*;
use httpmock::Method::PATCH;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_json::json;

use crate::{
    api::{client::ApiClient, error::ClientError},
    base::{currency::Currency, yat::Yat},
    cli::CreateAddress,
    settings::Settings,
};

fn build_settings(server: &MockServer) -> Settings {
    Settings {
        api_url: server.base_url(),
        yat_api_key: "KEY".to_owned(),
        required_yat_length: 2..=3,
    }
}

fn yat(settings: &Settings) -> Yat {
    Yat::new(settings, "💪💪💪").unwrap()
}

fn create_address() -> CreateAddress {
    CreateAddress {
        currency: Currency::try_from("Cardano").unwrap(),
        address: "ADDR".to_owned(),
        description: None,
    }
}

#[tokio::test]
async fn it_fails_on_404_status_code() {
    let server = MockServer::start();
    let settings = build_settings(&server);
    let yat = yat(&settings);

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path_matches(Regex::new(r"^/emoji_id/.+").unwrap());
        then.status(404);
    });

    let api = ApiClient::new(&settings);
    let result = api.create_payment_address(&yat, create_address()).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::YatNotFound));

    mock.assert();
}

#[tokio::test]
async fn it_fails_on_other_failed_status_code() {
    let server = MockServer::start();
    let settings = build_settings(&server);
    let yat = yat(&settings);

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path_matches(Regex::new(r"^/emoji_id/.+").unwrap());
        then.status(500);
    });

    let api = ApiClient::new(&settings);
    let result = api.create_payment_address(&yat, create_address()).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::ReqwestError(_)));

    mock.assert();
}

#[tokio::test]
async fn it_succeeds_on_positive_response() {
    let server = MockServer::start();
    let settings = build_settings(&server);
    let yat = yat(&settings);
    let path = format!(
        "/emoji_id/{}",
        utf8_percent_encode(yat.as_ref(), NON_ALPHANUMERIC)
    );

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(path)
            .header("accept", "application/json")
            .header("x-api-key", "KEY")
            .json_body(json!({
              "insert": [
                { "data": "ADDR", "tag": "0x1012" }
              ]
            }));
        then.status(200);
    });

    let api = ApiClient::new(&settings);
    let result = api.create_payment_address(&yat, create_address()).await;

    assert!(result.is_ok());

    mock.assert();
}
