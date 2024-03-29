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
pub struct ConstructorInputs {
    /// uint8
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<String>,
    /// uint256
    #[serde(rename = "initialSupply", skip_serializing_if = "Option::is_none")]
    pub initial_supply: Option<String>,
    /// string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// string
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl ConstructorInputs {
    pub fn new() -> ConstructorInputs {
        ConstructorInputs {
            decimals: None,
            initial_supply: None,
            name: None,
            symbol: None,
        }
    }
}
