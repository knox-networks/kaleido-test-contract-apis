/*
 * u0ff6ziwba
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculateHashInputs {
    /// bytes32
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

impl CalculateHashInputs {
    pub fn new() -> CalculateHashInputs {
        CalculateHashInputs { input: None }
    }
}