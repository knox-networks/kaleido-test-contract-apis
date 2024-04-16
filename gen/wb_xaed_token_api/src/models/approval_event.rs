/*
 * u0i7e09use
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalEvent {
    /// address
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// address
    #[serde(rename = "spender", skip_serializing_if = "Option::is_none")]
    pub spender: Option<String>,
    /// uint256
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApprovalEvent {
    pub fn new() -> ApprovalEvent {
        ApprovalEvent {
            owner: None,
            spender: None,
            value: None,
        }
    }
}
