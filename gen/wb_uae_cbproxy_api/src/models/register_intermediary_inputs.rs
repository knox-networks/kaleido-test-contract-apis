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
pub struct RegisterIntermediaryInputs {
    /// address
    #[serde(
        rename = "intermediaryAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub intermediary_address: Option<String>,
    /// string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl RegisterIntermediaryInputs {
    pub fn new() -> RegisterIntermediaryInputs {
        RegisterIntermediaryInputs {
            intermediary_address: None,
            name: None,
        }
    }
}
