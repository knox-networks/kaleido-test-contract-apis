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
pub struct NewCrossBorderContractOutputs {
    /// bytes32
    #[serde(rename = "contractId", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
}

impl NewCrossBorderContractOutputs {
    pub fn new() -> NewCrossBorderContractOutputs {
        NewCrossBorderContractOutputs { contract_id: None }
    }
}
