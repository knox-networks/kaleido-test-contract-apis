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
pub struct GetErc20LiquidityOutputs {
    /// uint256
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

impl GetErc20LiquidityOutputs {
    pub fn new() -> GetErc20LiquidityOutputs {
        GetErc20LiquidityOutputs { output: None }
    }
}
