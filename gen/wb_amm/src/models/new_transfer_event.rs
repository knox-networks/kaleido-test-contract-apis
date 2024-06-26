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
pub struct NewTransferEvent {
    /// bytes32
    #[serde(rename = "contractId", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    /// (address,address,(address,address),bytes32,string,(string,string),(address,address,uint256,uint256),bytes,(uint8,uint8,uint8),uint256,bytes32)
    #[serde(
        rename = "paymentNotification",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_notification: Option<serde_json::Value>,
}

impl NewTransferEvent {
    pub fn new() -> NewTransferEvent {
        NewTransferEvent {
            contract_id: None,
            payment_notification: None,
        }
    }
}
