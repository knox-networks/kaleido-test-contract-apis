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
pub struct GetIntermediaryBalanceInputs {
    /// address
    #[serde(
        rename = "intermediaryAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub intermediary_address: Option<String>,
}

impl GetIntermediaryBalanceInputs {
    pub fn new() -> GetIntermediaryBalanceInputs {
        GetIntermediaryBalanceInputs {
            intermediary_address: None,
        }
    }
}
