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
pub struct IntermediaryMintTransferEvent {
    /// address
    #[serde(rename = "addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// uint256
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl IntermediaryMintTransferEvent {
    pub fn new() -> IntermediaryMintTransferEvent {
        IntermediaryMintTransferEvent {
            addr: None,
            amount: None,
        }
    }
}
