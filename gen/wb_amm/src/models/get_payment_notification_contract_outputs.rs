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
pub struct GetPaymentNotificationContractOutputs {
    /// address
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl GetPaymentNotificationContractOutputs {
    pub fn new() -> GetPaymentNotificationContractOutputs {
        GetPaymentNotificationContractOutputs { result: None }
    }
}
