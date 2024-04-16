# \DefaultApi

All URIs are relative to *https://u0s5jrp8nx-u0jo91fxhc-connect.us0-aws.kaleido.io/gateways/u0ff6ziwba*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calculate_hash_get**](DefaultApi.md#calculate_hash_get) | **GET** /{address}/calculateHash | calculateHash(bytes32) [read only]
[**calculate_hash_post**](DefaultApi.md#calculate_hash_post) | **POST** /{address}/calculateHash | calculateHash(bytes32) [read only]
[**constructor_post**](DefaultApi.md#constructor_post) | **POST** / | constructor(address)
[**get_contract_get**](DefaultApi.md#get_contract_get) | **GET** /{address}/getContract | getContract(bytes32) [read only]
[**get_contract_post**](DefaultApi.md#get_contract_post) | **POST** /{address}/getContract | getContract(bytes32) [read only]
[**get_cross_border_contracts_get**](DefaultApi.md#get_cross_border_contracts_get) | **GET** /{address}/getCrossBorderContracts | getCrossBorderContracts(bytes32) [read only]
[**get_cross_border_contracts_post**](DefaultApi.md#get_cross_border_contracts_post) | **POST** /{address}/getCrossBorderContracts | getCrossBorderContracts(bytes32) [read only]
[**get_payment_notification_contract_get**](DefaultApi.md#get_payment_notification_contract_get) | **GET** /{address}/getPaymentNotificationContract | getPaymentNotificationContract() [read only]
[**get_payment_notification_contract_post**](DefaultApi.md#get_payment_notification_contract_post) | **POST** /{address}/getPaymentNotificationContract | getPaymentNotificationContract() [read only]
[**h_tlcerc20_crossborder_unlock_subscribe**](DefaultApi.md#h_tlcerc20_crossborder_unlock_subscribe) | **POST** /HTLCERC20CrossborderUnlock/subscribe | HTLCERC20CrossborderUnlock(bytes32,bytes32) [event]
[**h_tlcerc20_crossborder_unlock_subscribe_all**](DefaultApi.md#h_tlcerc20_crossborder_unlock_subscribe_all) | **POST** /{address}/HTLCERC20CrossborderUnlock/subscribe | HTLCERC20CrossborderUnlock(bytes32,bytes32) [event]
[**h_tlcerc20_new_ammto_payee_subscribe**](DefaultApi.md#h_tlcerc20_new_ammto_payee_subscribe) | **POST** /HTLCERC20NewAMMToPayee/subscribe | HTLCERC20NewAMMToPayee(bytes32,(bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)) [event]
[**h_tlcerc20_new_ammto_payee_subscribe_all**](DefaultApi.md#h_tlcerc20_new_ammto_payee_subscribe_all) | **POST** /{address}/HTLCERC20NewAMMToPayee/subscribe | HTLCERC20NewAMMToPayee(bytes32,(bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)) [event]
[**h_tlcerc20_new_payer_to_amm_subscribe**](DefaultApi.md#h_tlcerc20_new_payer_to_amm_subscribe) | **POST** /HTLCERC20NewPayerToAMM/subscribe | HTLCERC20NewPayerToAMM(bytes32,(bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)) [event]
[**h_tlcerc20_new_payer_to_amm_subscribe_all**](DefaultApi.md#h_tlcerc20_new_payer_to_amm_subscribe_all) | **POST** /{address}/HTLCERC20NewPayerToAMM/subscribe | HTLCERC20NewPayerToAMM(bytes32,(bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)) [event]
[**h_tlcerc20_refund_subscribe**](DefaultApi.md#h_tlcerc20_refund_subscribe) | **POST** /HTLCERC20Refund/subscribe | HTLCERC20Refund(bytes32) [event]
[**h_tlcerc20_refund_subscribe_all**](DefaultApi.md#h_tlcerc20_refund_subscribe_all) | **POST** /{address}/HTLCERC20Refund/subscribe | HTLCERC20Refund(bytes32) [event]
[**h_tlcerc20_withdraw_subscribe**](DefaultApi.md#h_tlcerc20_withdraw_subscribe) | **POST** /HTLCERC20Withdraw/subscribe | HTLCERC20Withdraw(bytes32) [event]
[**h_tlcerc20_withdraw_subscribe_all**](DefaultApi.md#h_tlcerc20_withdraw_subscribe_all) | **POST** /{address}/HTLCERC20Withdraw/subscribe | HTLCERC20Withdraw(bytes32) [event]
[**new_contract_get**](DefaultApi.md#new_contract_get) | **GET** /{address}/newContract | newContract(address,bytes32,uint256,address,uint256,bytes32)
[**new_contract_post**](DefaultApi.md#new_contract_post) | **POST** /{address}/newContract | newContract(address,bytes32,uint256,address,uint256,bytes32)
[**new_cross_border_contract_get**](DefaultApi.md#new_cross_border_contract_get) | **GET** /{address}/newCrossBorderContract | newCrossBorderContract(address,uint256,address,bytes32)
[**new_cross_border_contract_post**](DefaultApi.md#new_cross_border_contract_post) | **POST** /{address}/newCrossBorderContract | newCrossBorderContract(address,uint256,address,bytes32)
[**owner_get**](DefaultApi.md#owner_get) | **GET** /{address}/_owner | _owner() [read only]
[**owner_post**](DefaultApi.md#owner_post) | **POST** /{address}/_owner | _owner() [read only]
[**payment_notification_contract_get**](DefaultApi.md#payment_notification_contract_get) | **GET** /{address}/_paymentNotificationContract | _paymentNotificationContract() [read only]
[**payment_notification_contract_post**](DefaultApi.md#payment_notification_contract_post) | **POST** /{address}/_paymentNotificationContract | _paymentNotificationContract() [read only]
[**refund_get**](DefaultApi.md#refund_get) | **GET** /{address}/refund | refund(bytes32)
[**refund_post**](DefaultApi.md#refund_post) | **POST** /{address}/refund | refund(bytes32)
[**set_payment_notification_contract_get**](DefaultApi.md#set_payment_notification_contract_get) | **GET** /{address}/setPaymentNotificationContract | setPaymentNotificationContract(address)
[**set_payment_notification_contract_post**](DefaultApi.md#set_payment_notification_contract_post) | **POST** /{address}/setPaymentNotificationContract | setPaymentNotificationContract(address)
[**unlock_cross_border_payments_get**](DefaultApi.md#unlock_cross_border_payments_get) | **GET** /{address}/unlockCrossBorderPayments | unlockCrossBorderPayments(bytes32,bytes32)
[**unlock_cross_border_payments_post**](DefaultApi.md#unlock_cross_border_payments_post) | **POST** /{address}/unlockCrossBorderPayments | unlockCrossBorderPayments(bytes32,bytes32)
[**withdraw_get**](DefaultApi.md#withdraw_get) | **GET** /{address}/withdraw | withdraw(bytes32,bytes32)
[**withdraw_post**](DefaultApi.md#withdraw_post) | **POST** /{address}/withdraw | withdraw(bytes32,bytes32)



## calculate_hash_get

> models::CalculateHashOutputs calculate_hash_get(address, input, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
calculateHash(bytes32) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**input** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::CalculateHashOutputs**](calculateHash_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calculate_hash_post

> models::CalculateHashOutputs calculate_hash_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
calculateHash(bytes32) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**CalculateHashInputs**](CalculateHashInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::CalculateHashOutputs**](calculateHash_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## constructor_post

> serde_json::Value constructor_post(body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_register)
constructor(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConstructorInputs**](ConstructorInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |
**kld_register** | Option<**String**> | Register the installed contract on a friendly path (overwrites existing) (header: x-kaleido-register) |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_get

> models::GetContractOutputs get_contract_get(address, _contract_id, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getContract(bytes32) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**_contract_id** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::GetContractOutputs**](getContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_post

> models::GetContractOutputs get_contract_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
getContract(bytes32) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetContractInputs**](GetContractInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::GetContractOutputs**](getContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cross_border_contracts_get

> models::GetCrossBorderContractsOutputs get_cross_border_contracts_get(address, transfer_key, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getCrossBorderContracts(bytes32) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**transfer_key** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::GetCrossBorderContractsOutputs**](getCrossBorderContracts_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cross_border_contracts_post

> models::GetCrossBorderContractsOutputs get_cross_border_contracts_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
getCrossBorderContracts(bytes32) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetCrossBorderContractsInputs**](GetCrossBorderContractsInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::GetCrossBorderContractsOutputs**](getCrossBorderContracts_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_notification_contract_get

> models::GetPaymentNotificationContractOutputs get_payment_notification_contract_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getPaymentNotificationContract() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::GetPaymentNotificationContractOutputs**](getPaymentNotificationContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_notification_contract_post

> models::GetPaymentNotificationContractOutputs get_payment_notification_contract_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
getPaymentNotificationContract() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | **serde_json::Value** |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::GetPaymentNotificationContractOutputs**](getPaymentNotificationContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_crossborder_unlock_subscribe

> models::Htlcerc20CrossborderUnlockEvent h_tlcerc20_crossborder_unlock_subscribe(body)
HTLCERC20CrossborderUnlock(bytes32,bytes32) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20CrossborderUnlockEvent**](HTLCERC20CrossborderUnlock_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_crossborder_unlock_subscribe_all

> models::Htlcerc20CrossborderUnlockEvent h_tlcerc20_crossborder_unlock_subscribe_all(address, body)
HTLCERC20CrossborderUnlock(bytes32,bytes32) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20CrossborderUnlockEvent**](HTLCERC20CrossborderUnlock_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_new_ammto_payee_subscribe

> models::Htlcerc20NewAmmtoPayeeEvent h_tlcerc20_new_ammto_payee_subscribe(body)
HTLCERC20NewAMMToPayee(bytes32,(bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20NewAmmtoPayeeEvent**](HTLCERC20NewAMMToPayee_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_new_ammto_payee_subscribe_all

> models::Htlcerc20NewAmmtoPayeeEvent h_tlcerc20_new_ammto_payee_subscribe_all(address, body)
HTLCERC20NewAMMToPayee(bytes32,(bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20NewAmmtoPayeeEvent**](HTLCERC20NewAMMToPayee_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_new_payer_to_amm_subscribe

> models::Htlcerc20NewPayerToAmmEvent h_tlcerc20_new_payer_to_amm_subscribe(body)
HTLCERC20NewPayerToAMM(bytes32,(bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20NewPayerToAmmEvent**](HTLCERC20NewPayerToAMM_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_new_payer_to_amm_subscribe_all

> models::Htlcerc20NewPayerToAmmEvent h_tlcerc20_new_payer_to_amm_subscribe_all(address, body)
HTLCERC20NewPayerToAMM(bytes32,(bytes32,address,address,address,uint256,bytes32,uint256,bool,bool,bytes32,bytes32)) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20NewPayerToAmmEvent**](HTLCERC20NewPayerToAMM_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_refund_subscribe

> models::Htlcerc20RefundEvent h_tlcerc20_refund_subscribe(body)
HTLCERC20Refund(bytes32) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20RefundEvent**](HTLCERC20Refund_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_refund_subscribe_all

> models::Htlcerc20RefundEvent h_tlcerc20_refund_subscribe_all(address, body)
HTLCERC20Refund(bytes32) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20RefundEvent**](HTLCERC20Refund_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_withdraw_subscribe

> models::Htlcerc20WithdrawEvent h_tlcerc20_withdraw_subscribe(body)
HTLCERC20Withdraw(bytes32) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20WithdrawEvent**](HTLCERC20Withdraw_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_withdraw_subscribe_all

> models::Htlcerc20WithdrawEvent h_tlcerc20_withdraw_subscribe_all(address, body)
HTLCERC20Withdraw(bytes32) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**Htlcerc20CrossborderUnlockSubscribeRequest**](Htlcerc20CrossborderUnlockSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20WithdrawEvent**](HTLCERC20Withdraw_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_contract_get

> models::NewContractOutputs new_contract_get(address, _receiver, _hashlock, _timelock, _token_contract, _amount, _transfer_key, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
newContract(address,bytes32,uint256,address,uint256,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**_receiver** | **String** | address | [required] |
**_hashlock** | **String** | bytes32 | [required] |
**_timelock** | **String** | uint256 | [required] |
**_token_contract** | **String** | address | [required] |
**_amount** | **String** | uint256 | [required] |
**_transfer_key** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::NewContractOutputs**](newContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_contract_post

> models::NewContractOutputs new_contract_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
newContract(address,bytes32,uint256,address,uint256,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**NewContractInputs**](NewContractInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::NewContractOutputs**](newContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_cross_border_contract_get

> models::NewCrossBorderContractOutputs new_cross_border_contract_get(address, _receiver, _timelock, _token_contract, _transfer_key, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
newCrossBorderContract(address,uint256,address,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**_receiver** | **String** | address | [required] |
**_timelock** | **String** | uint256 | [required] |
**_token_contract** | **String** | address | [required] |
**_transfer_key** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::NewCrossBorderContractOutputs**](newCrossBorderContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_cross_border_contract_post

> models::NewCrossBorderContractOutputs new_cross_border_contract_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
newCrossBorderContract(address,uint256,address,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**NewCrossBorderContractInputs**](NewCrossBorderContractInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::NewCrossBorderContractOutputs**](newCrossBorderContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## owner_get

> models::OwnerOutputs owner_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
_owner() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::OwnerOutputs**](_owner_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## owner_post

> models::OwnerOutputs owner_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
_owner() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | **serde_json::Value** |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::OwnerOutputs**](_owner_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_notification_contract_get

> models::PaymentNotificationContractOutputs payment_notification_contract_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
_paymentNotificationContract() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::PaymentNotificationContractOutputs**](_paymentNotificationContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_notification_contract_post

> models::PaymentNotificationContractOutputs payment_notification_contract_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
_paymentNotificationContract() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | **serde_json::Value** |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::PaymentNotificationContractOutputs**](_paymentNotificationContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refund_get

> models::RefundOutputs refund_get(address, _contract_id, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
refund(bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**_contract_id** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::RefundOutputs**](refund_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refund_post

> models::RefundOutputs refund_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
refund(bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**RefundInputs**](RefundInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::RefundOutputs**](refund_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_payment_notification_contract_get

> serde_json::Value set_payment_notification_contract_get(address, payment_notification_contract, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
setPaymentNotificationContract(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**payment_notification_contract** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_payment_notification_contract_post

> serde_json::Value set_payment_notification_contract_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
setPaymentNotificationContract(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**SetPaymentNotificationContractInputs**](SetPaymentNotificationContractInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_cross_border_payments_get

> serde_json::Value unlock_cross_border_payments_get(address, transfer_key, secret, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
unlockCrossBorderPayments(bytes32,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**transfer_key** | **String** | bytes32 | [required] |
**secret** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_cross_border_payments_post

> serde_json::Value unlock_cross_border_payments_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
unlockCrossBorderPayments(bytes32,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**UnlockCrossBorderPaymentsInputs**](UnlockCrossBorderPaymentsInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw_get

> models::WithdrawOutputs withdraw_get(address, _contract_id, _preimage, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
withdraw(bytes32,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**_contract_id** | **String** | bytes32 | [required] |
**_preimage** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::WithdrawOutputs**](withdraw_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw_post

> models::WithdrawOutputs withdraw_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
withdraw(bytes32,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**WithdrawInputs**](WithdrawInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |

### Return type

[**models::WithdrawOutputs**](withdraw_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

