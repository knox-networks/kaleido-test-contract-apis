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
pub struct GetRoleMemberCountInputs {
    /// bytes32
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl GetRoleMemberCountInputs {
    pub fn new() -> GetRoleMemberCountInputs {
        GetRoleMemberCountInputs { role: None }
    }
}
