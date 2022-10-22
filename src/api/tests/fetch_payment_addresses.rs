use httpmock::prelude::*;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_json::json;

use crate::{
    api::{client::ApiClient, error::ClientError, payment_address::FriendlyPaymentAddress},
    base::{currency::Currency, yat::Yat},
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

#[tokio::test]
async fn it_fails_on_404_status_code() {
    let server = MockServer::start();
    let settings = build_settings(&server);
    let yat = yat(&settings);

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path_matches(Regex::new(r"^/emoji_id/.+/payment").unwrap());
        then.status(404);
    });

    let api = ApiClient::new(&settings);
    let result = api.fetch_payment_addresses(&yat).await;

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
        when.method(GET)
            .path_matches(Regex::new(r"^/emoji_id/.+/payment").unwrap());
        then.status(500);
    });

    let api = ApiClient::new(&settings);
    let result = api.fetch_payment_addresses(&yat).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::ReqwestError(_)));

    mock.assert();
}

#[tokio::test]
async fn it_fails_on_invalid_json() {
    let server = MockServer::start();
    let settings = build_settings(&server);
    let yat = yat(&settings);

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path_matches(Regex::new(r"^/emoji_id/.+/payment").unwrap());
        then.status(200).body("INVALID");
    });

    let api = ApiClient::new(&settings);
    let result = api.fetch_payment_addresses(&yat).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::ReqwestError(_)));

    mock.assert();
}

#[tokio::test]
async fn it_fails_with_api_response_error_if_api_has_it() {
    let server = MockServer::start();
    let settings = build_settings(&server);
    let yat = yat(&settings);

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path_matches(Regex::new(r"^/emoji_id/.+/payment").unwrap());
        then.status(200).json_body(json!({
          "error": { "code": "XXX", "reason": "FAILURE" },
          "status": false
        }));
    });

    let api = ApiClient::new(&settings);
    let result = api.fetch_payment_addresses(&yat).await;

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ClientError::ApiResponseError { code, reason }
          if code == "XXX" && reason == "FAILURE"
    ));

    mock.assert();
}

#[tokio::test]
async fn it_fails_on_unknown_categories() {
    let server = MockServer::start();
    let settings = build_settings(&server);
    let yat = yat(&settings);

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path_matches(Regex::new(r"^/emoji_id/.+/payment").unwrap());
        then.status(200).json_body(json!({
          "result": {
            "UNKNOWN": {
              "address": "Address",
              "category": "UNKNOWN",
              "default": false
            },
          },
          "status": true
        }));
    });

    let api = ApiClient::new(&settings);
    let result = api.fetch_payment_addresses(&yat).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ClientError::CurrencyError(_)));

    mock.assert();
}

#[tokio::test]
async fn it_succeeds_on_proper_response() {
    let server = MockServer::start();
    let settings = build_settings(&server);
    let yat = yat(&settings);
    let path = format!(
        "/emoji_id/{}/payment",
        utf8_percent_encode(yat.as_ref(), NON_ALPHANUMERIC)
    );

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(path)
            .header("accept", "application/json")
            .header("x-api-key", "KEY");
        then.status(200).json_body(json!({
          "result": {
            "0x1012": {
              "address": "Address1",
              "category": "0x1012",
              "default": false,
              "description": "Description1"
            },
            "0x1015": {
              "address": "Address2",
              "category": "0x1015",
              "default": false
            },
          },
          "status": true
        }));
    });

    let api = ApiClient::new(&settings);
    let result = api.fetch_payment_addresses(&yat).await;

    assert_eq!(
        result.unwrap(),
        vec![
            FriendlyPaymentAddress {
                currency: Currency::try_from(0x1012).unwrap(),
                address: "Address1".to_owned(),
                description: Some("Description1".to_owned()),
            },
            FriendlyPaymentAddress {
                currency: Currency::try_from(0x1015).unwrap(),
                address: "Address2".to_owned(),
                description: None,
            }
        ]
    );

    mock.assert();
}
