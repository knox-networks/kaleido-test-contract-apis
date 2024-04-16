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
pub struct SetPaymentNotificationContractInputs {
    /// address
    #[serde(
        rename = "paymentNotificationContract",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_notification_contract: Option<String>,
}

impl SetPaymentNotificationContractInputs {
    pub fn new() -> SetPaymentNotificationContractInputs {
        SetPaymentNotificationContractInputs {
            payment_notification_contract: None,
        }
    }
}
