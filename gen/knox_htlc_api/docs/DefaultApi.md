# \DefaultApi

All URIs are relative to *https://u0yt7psxje-u0w0qcw9cy-connect.us0-aws.kaleido.io/gateways/u0sek73vdz*

Method | HTTP request | Description
------------- | ------------- | -------------
[**constructor_post**](DefaultApi.md#constructor_post) | **POST** / | constructor()
[**get_contract_get**](DefaultApi.md#get_contract_get) | **GET** /{address}/getContract | getContract(bytes32) [read only]
[**get_contract_post**](DefaultApi.md#get_contract_post) | **POST** /{address}/getContract | getContract(bytes32) [read only]
[**h_tlcerc20_new_subscribe**](DefaultApi.md#h_tlcerc20_new_subscribe) | **POST** /HTLCERC20New/subscribe | HTLCERC20New(bytes32,address,address,address,uint256,bytes32,uint256) [event]
[**h_tlcerc20_new_subscribe_all**](DefaultApi.md#h_tlcerc20_new_subscribe_all) | **POST** /{address}/HTLCERC20New/subscribe | HTLCERC20New(bytes32,address,address,address,uint256,bytes32,uint256) [event]
[**h_tlcerc20_refund_subscribe**](DefaultApi.md#h_tlcerc20_refund_subscribe) | **POST** /HTLCERC20Refund/subscribe | HTLCERC20Refund(bytes32) [event]
[**h_tlcerc20_refund_subscribe_all**](DefaultApi.md#h_tlcerc20_refund_subscribe_all) | **POST** /{address}/HTLCERC20Refund/subscribe | HTLCERC20Refund(bytes32) [event]
[**h_tlcerc20_withdraw_subscribe**](DefaultApi.md#h_tlcerc20_withdraw_subscribe) | **POST** /HTLCERC20Withdraw/subscribe | HTLCERC20Withdraw(bytes32) [event]
[**h_tlcerc20_withdraw_subscribe_all**](DefaultApi.md#h_tlcerc20_withdraw_subscribe_all) | **POST** /{address}/HTLCERC20Withdraw/subscribe | HTLCERC20Withdraw(bytes32) [event]
[**new_contract_get**](DefaultApi.md#new_contract_get) | **GET** /{address}/newContract | newContract(address,bytes32,uint256,address,uint256)
[**new_contract_post**](DefaultApi.md#new_contract_post) | **POST** /{address}/newContract | newContract(address,bytes32,uint256,address,uint256)
[**refund_get**](DefaultApi.md#refund_get) | **GET** /{address}/refund | refund(bytes32)
[**refund_post**](DefaultApi.md#refund_post) | **POST** /{address}/refund | refund(bytes32)
[**withdraw_get**](DefaultApi.md#withdraw_get) | **GET** /{address}/withdraw | withdraw(bytes32,bytes32)
[**withdraw_post**](DefaultApi.md#withdraw_post) | **POST** /{address}/withdraw | withdraw(bytes32,bytes32)



## constructor_post

> serde_json::Value constructor_post(body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid, kld_register)
constructor()

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |
**kld_privacygroupid** | Option<**String**> | Private transaction group ID (header: x-kaleido-privacyGroupId) |  |
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
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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

> models::GetContractOutputs get_contract_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
getContract(bytes32) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetContractInputs**](GetContractInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |
**kld_privacygroupid** | Option<**String**> | Private transaction group ID (header: x-kaleido-privacyGroupId) |  |

### Return type

[**models::GetContractOutputs**](getContract_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_new_subscribe

> models::Htlcerc20NewEvent h_tlcerc20_new_subscribe(body)
HTLCERC20New(bytes32,address,address,address,uint256,bytes32,uint256) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Htlcerc20NewSubscribeRequest**](Htlcerc20NewSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20NewEvent**](HTLCERC20New_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_tlcerc20_new_subscribe_all

> models::Htlcerc20NewEvent h_tlcerc20_new_subscribe_all(address, body)
HTLCERC20New(bytes32,address,address,address,uint256,bytes32,uint256) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**Htlcerc20NewSubscribeRequest**](Htlcerc20NewSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20NewEvent**](HTLCERC20New_event.md)

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
**body** | [**Htlcerc20NewSubscribeRequest**](Htlcerc20NewSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

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
**body** | [**Htlcerc20NewSubscribeRequest**](Htlcerc20NewSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

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
**body** | [**Htlcerc20NewSubscribeRequest**](Htlcerc20NewSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

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
**body** | [**Htlcerc20NewSubscribeRequest**](Htlcerc20NewSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::Htlcerc20WithdrawEvent**](HTLCERC20Withdraw_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_contract_get

> models::NewContractOutputs new_contract_get(address, _receiver, _hashlock, _timelock, _token_contract, _amount, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
newContract(address,bytes32,uint256,address,uint256)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**_receiver** | **String** | address | [required] |
**_hashlock** | **String** | bytes32 | [required] |
**_timelock** | **String** | uint256 | [required] |
**_token_contract** | **String** | address | [required] |
**_amount** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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

> models::NewContractOutputs new_contract_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
newContract(address,bytes32,uint256,address,uint256)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**NewContractInputs**](NewContractInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |
**kld_privacygroupid** | Option<**String**> | Private transaction group ID (header: x-kaleido-privacyGroupId) |  |

### Return type

[**models::NewContractOutputs**](newContract_outputs.md)

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
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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

> models::RefundOutputs refund_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
refund(bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**RefundInputs**](RefundInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |
**kld_privacygroupid** | Option<**String**> | Private transaction group ID (header: x-kaleido-privacyGroupId) |  |

### Return type

[**models::RefundOutputs**](refund_outputs.md)

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
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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

> models::WithdrawOutputs withdraw_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
withdraw(bytes32,bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**WithdrawInputs**](WithdrawInputs.md) |  | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_sync** | Option<**bool**> | Block the HTTP request until the tx is mined (does not store the receipt) (header: x-kaleido-sync) |  |[default to true]
**kld_call** | Option<**bool**> | Perform a read-only call with the same parameters that would be used to invoke, and return result (header: x-kaleido-call) |  |
**kld_privatefrom** | Option<**String**> | Private transaction sender (header: x-kaleido-privatefrom) |  |
**kld_privatefor** | Option<**String**> | Private transaction recipients (comma separated or multiple params) (header: x-kaleido-privatefor) |  |
**kld_blocknumber** | Option<**String**> | The target block number for eth_call requests. One of 'earliest/latest/pending', a number or a hex string (header: x-kaleido-blocknumber) |  |
**kld_acktype** | Option<**String**> | Set to 'receipt' to store a receipt before acknowledging an async request (header: x-kaleido-acktype) |  |
**kld_privacygroupid** | Option<**String**> | Private transaction group ID (header: x-kaleido-privacyGroupId) |  |

### Return type

[**models::WithdrawOutputs**](withdraw_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

