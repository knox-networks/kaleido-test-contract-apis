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
pub struct GetIntermediaryOutputs {
    /// (string,address)
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}

impl GetIntermediaryOutputs {
    pub fn new() -> GetIntermediaryOutputs {
        GetIntermediaryOutputs { output: None }
    }
}
