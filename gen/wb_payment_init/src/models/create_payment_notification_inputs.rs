/*
 * u0zvk4trgb
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePaymentNotificationInputs {
    /// (string,string)
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<serde_json::Value>,
    /// bytes
    #[serde(rename = "pacsJson", skip_serializing_if = "Option::is_none")]
    pub pacs_json: Option<String>,
    /// (address,address,uint256,uint256)
    #[serde(rename = "transferDetails", skip_serializing_if = "Option::is_none")]
    pub transfer_details: Option<serde_json::Value>,
    /// string
    #[serde(rename = "uetr", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
}

impl CreatePaymentNotificationInputs {
    pub fn new() -> CreatePaymentNotificationInputs {
        CreatePaymentNotificationInputs {
            aliases: None,
            pacs_json: None,
            transfer_details: None,
            uetr: None,
        }
    }
}
