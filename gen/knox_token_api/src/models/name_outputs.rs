/*
 * kaleidoerc20mb
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameOutputs {
    /// string
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

impl NameOutputs {
    pub fn new() -> NameOutputs {
        NameOutputs { output: None }
    }
}
