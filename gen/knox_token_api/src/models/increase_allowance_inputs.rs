/*
 * kaleidoerc20mb
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncreaseAllowanceInputs {
    /// uint256
    #[serde(rename = "addedValue", skip_serializing_if = "Option::is_none")]
    pub added_value: Option<String>,
    /// address
    #[serde(rename = "spender", skip_serializing_if = "Option::is_none")]
    pub spender: Option<String>,
}

impl IncreaseAllowanceInputs {
    pub fn new() -> IncreaseAllowanceInputs {
        IncreaseAllowanceInputs {
            added_value: None,
            spender: None,
        }
    }
}
