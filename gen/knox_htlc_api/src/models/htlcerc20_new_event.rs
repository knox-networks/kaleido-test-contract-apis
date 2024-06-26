/*
 * u0sek73vdz
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Htlcerc20NewEvent {
    /// uint256
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// bytes32
    #[serde(rename = "contractId", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    /// bytes32
    #[serde(rename = "hashlock", skip_serializing_if = "Option::is_none")]
    pub hashlock: Option<String>,
    /// address
    #[serde(rename = "receiver", skip_serializing_if = "Option::is_none")]
    pub receiver: Option<String>,
    /// address
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// uint256
    #[serde(rename = "timelock", skip_serializing_if = "Option::is_none")]
    pub timelock: Option<String>,
    /// address
    #[serde(rename = "tokenContract", skip_serializing_if = "Option::is_none")]
    pub token_contract: Option<String>,
}

impl Htlcerc20NewEvent {
    pub fn new() -> Htlcerc20NewEvent {
        Htlcerc20NewEvent {
            amount: None,
            contract_id: None,
            hashlock: None,
            receiver: None,
            sender: None,
            timelock: None,
            token_contract: None,
        }
    }
}
