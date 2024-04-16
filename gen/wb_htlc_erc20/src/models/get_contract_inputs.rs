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
pub struct GetContractInputs {
    /// bytes32
    #[serde(rename = "_contractId", skip_serializing_if = "Option::is_none")]
    pub _contract_id: Option<String>,
}

impl GetContractInputs {
    pub fn new() -> GetContractInputs {
        GetContractInputs { _contract_id: None }
    }
}
