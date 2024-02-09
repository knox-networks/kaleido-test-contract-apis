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
pub struct DecreaseAllowanceInputs {
    /// address
    #[serde(rename = "spender", skip_serializing_if = "Option::is_none")]
    pub spender: Option<String>,
    /// uint256
    #[serde(rename = "subtractedValue", skip_serializing_if = "Option::is_none")]
    pub subtracted_value: Option<String>,
}

impl DecreaseAllowanceInputs {
    pub fn new() -> DecreaseAllowanceInputs {
        DecreaseAllowanceInputs {
            spender: None,
            subtracted_value: None,
        }
    }
}