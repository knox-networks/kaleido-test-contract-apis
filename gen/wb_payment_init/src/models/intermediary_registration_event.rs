/*
 * u0bpii6ir3
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntermediaryRegistrationEvent {
    /// address
    #[serde(rename = "addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl IntermediaryRegistrationEvent {
    pub fn new() -> IntermediaryRegistrationEvent {
        IntermediaryRegistrationEvent {
            addr: None,
            name: None,
        }
    }
}
