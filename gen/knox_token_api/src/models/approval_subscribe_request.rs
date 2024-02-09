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
pub struct ApprovalSubscribeRequest {
    /// The block number to start the subscription from
    #[serde(rename = "fromBlock", skip_serializing_if = "Option::is_none")]
    pub from_block: Option<String>,
    /// The ID of an event stream already configured in the REST Gateway
    #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

impl ApprovalSubscribeRequest {
    pub fn new() -> ApprovalSubscribeRequest {
        ApprovalSubscribeRequest {
            from_block: None,
            stream: None,
        }
    }
}
