/*
 * u0vtfbfnvz
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstructorInputs {
    /// address
    #[serde(
        rename = "xCentralBankTokenAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub x_central_bank_token_address: Option<String>,
}

impl ConstructorInputs {
    pub fn new() -> ConstructorInputs {
        ConstructorInputs {
            x_central_bank_token_address: None,
        }
    }
}
