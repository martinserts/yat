use serde::Serialize;

use crate::cli::CreateAddress;

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct EditRequest {
    pub insert: Vec<InsertData>,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct InsertData {
    pub data: String,
    pub tag: String,
}

impl From<CreateAddress> for EditRequest {
    fn from(create_data: CreateAddress) -> Self {
        let mut data_parts = vec![create_data.address];
        if let Some(description) = create_data.description {
            data_parts.push(description);
        }
        let data = data_parts.join("|");
        let tag = create_data.currency.get_hex_code();
        Self {
            insert: vec![InsertData { data, tag }],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::currency::Currency;

    use super::*;

    #[test]
    fn it_converts_from_create_address_without_description() {
        assert_eq!(
            EditRequest::from(CreateAddress {
                currency: Currency::try_from("Cardano").unwrap(),
                address: "ADDR".to_owned(),
                description: None,
            }),
            EditRequest {
                insert: vec![InsertData {
                    data: "ADDR".to_owned(),
                    tag: "0x1012".to_owned(),
                }]
            }
        );
    }

    #[test]
    fn it_converts_from_create_address_with_description() {
        assert_eq!(
            EditRequest::from(CreateAddress {
                currency: Currency::try_from("Cardano").unwrap(),
                address: "ADDR".to_owned(),
                description: Some("Some description".to_owned()),
            }),
            EditRequest {
                insert: vec![InsertData {
                    data: "ADDR|Some description".to_owned(),
                    tag: "0x1012".to_owned(),
                }]
            }
        );
    }
}
