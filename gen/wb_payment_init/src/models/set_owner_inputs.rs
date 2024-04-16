/*
 * u0zvk4trgb
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetOwnerInputs {
    /// address
    #[serde(rename = "ownerAddress", skip_serializing_if = "Option::is_none")]
    pub owner_address: Option<String>,
}

impl SetOwnerInputs {
    pub fn new() -> SetOwnerInputs {
        SetOwnerInputs {
            owner_address: None,
        }
    }
}
