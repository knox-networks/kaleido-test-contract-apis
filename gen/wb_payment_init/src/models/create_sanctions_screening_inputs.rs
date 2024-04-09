/*
 * u0hkte1rqe
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSanctionsScreeningInputs {
    /// bytes
    #[serde(rename = "pacsJson", skip_serializing_if = "Option::is_none")]
    pub pacs_json: Option<String>,
    /// string
    #[serde(rename = "payeePHone", skip_serializing_if = "Option::is_none")]
    pub payee_p_hone: Option<String>,
    /// string
    #[serde(rename = "payerPhone", skip_serializing_if = "Option::is_none")]
    pub payer_phone: Option<String>,
    /// string
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl CreateSanctionsScreeningInputs {
    pub fn new() -> CreateSanctionsScreeningInputs {
        CreateSanctionsScreeningInputs {
            pacs_json: None,
            payee_p_hone: None,
            payer_phone: None,
            uuid: None,
        }
    }
}