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
pub struct AllowanceInputs {
    /// address
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// address
    #[serde(rename = "spender", skip_serializing_if = "Option::is_none")]
    pub spender: Option<String>,
}

impl AllowanceInputs {
    pub fn new() -> AllowanceInputs {
        AllowanceInputs {
            owner: None,
            spender: None,
        }
    }
}
