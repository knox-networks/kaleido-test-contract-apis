/*
 * u0ta9zm0up
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetErc20LiquidityInputs {
    /// address
    #[serde(rename = "tokenAddress", skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
}

impl GetErc20LiquidityInputs {
    pub fn new() -> GetErc20LiquidityInputs {
        GetErc20LiquidityInputs {
            token_address: None,
        }
    }
}
