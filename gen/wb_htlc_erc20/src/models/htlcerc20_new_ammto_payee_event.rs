/*
 * u0ff6ziwba
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Htlcerc20NewAmmtoPayeeEvent {
    /// bytes32
    #[serde(rename = "contractId", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    /// (bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)
    #[serde(rename = "lockContract", skip_serializing_if = "Option::is_none")]
    pub lock_contract: Option<serde_json::Value>,
}

impl Htlcerc20NewAmmtoPayeeEvent {
    pub fn new() -> Htlcerc20NewAmmtoPayeeEvent {
        Htlcerc20NewAmmtoPayeeEvent {
            contract_id: None,
            lock_contract: None,
        }
    }
}
