# \DefaultApi

All URIs are relative to *https://u0s5jrp8nx-u0jo91fxhc-connect.us0-aws.kaleido.io/gateways/u0vtfbfnvz*

Method | HTTP request | Description
------------- | ------------- | -------------
[**constructor_post**](DefaultApi.md#constructor_post) | **POST** / | constructor(address)
[**get_intermediary_balance_get**](DefaultApi.md#get_intermediary_balance_get) | **GET** /{address}/getIntermediaryBalance | getIntermediaryBalance(address) [read only]
[**get_intermediary_balance_post**](DefaultApi.md#get_intermediary_balance_post) | **POST** /{address}/getIntermediaryBalance | getIntermediaryBalance(address) [read only]
[**get_intermediary_get**](DefaultApi.md#get_intermediary_get) | **GET** /{address}/getIntermediary | getIntermediary(address) [read only]
[**get_intermediary_post**](DefaultApi.md#get_intermediary_post) | **POST** /{address}/getIntermediary | getIntermediary(address) [read only]
[**getx_central_bank_token_get**](DefaultApi.md#getx_central_bank_token_get) | **GET** /{address}/getxCentralBankToken | getxCentralBankToken() [read only]
[**getx_central_bank_token_post**](DefaultApi.md#getx_central_bank_token_post) | **POST** /{address}/getxCentralBankToken | getxCentralBankToken() [read only]
[**intermediary_mint_transfer_subscribe**](DefaultApi.md#intermediary_mint_transfer_subscribe) | **POST** /IntermediaryMintTransfer/subscribe | IntermediaryMintTransfer(address,uint256) [event]
[**intermediary_mint_transfer_subscribe_all**](DefaultApi.md#intermediary_mint_transfer_subscribe_all) | **POST** /{address}/IntermediaryMintTransfer/subscribe | IntermediaryMintTransfer(address,uint256) [event]
[**intermediary_registration_subscribe**](DefaultApi.md#intermediary_registration_subscribe) | **POST** /IntermediaryRegistration/subscribe | IntermediaryRegistration(address,string) [event]
[**intermediary_registration_subscribe_all**](DefaultApi.md#intermediary_registration_subscribe_all) | **POST** /{address}/IntermediaryRegistration/subscribe | IntermediaryRegistration(address,string) [event]
[**mint_transfer_intermediary_get**](DefaultApi.md#mint_transfer_intermediary_get) | **GET** /{address}/mintTransferIntermediary | mintTransferIntermediary(address,uint256)
[**mint_transfer_intermediary_post**](DefaultApi.md#mint_transfer_intermediary_post) | **POST** /{address}/mintTransferIntermediary | mintTransferIntermediary(address,uint256)
[**owner_get**](DefaultApi.md#owner_get) | **GET** /{address}/owner | owner() [read only]
[**owner_post**](DefaultApi.md#owner_post) | **POST** /{address}/owner | owner() [read only]
[**register_intermediary_get**](DefaultApi.md#register_intermediary_get) | **GET** /{address}/registerIntermediary | registerIntermediary(address,string)
[**register_intermediary_post**](DefaultApi.md#register_intermediary_post) | **POST** /{address}/registerIntermediary | registerIntermediary(address,string)
[**set_owner_get**](DefaultApi.md#set_owner_get) | **GET** /{address}/setOwner | setOwner(address)
[**set_owner_post**](DefaultApi.md#set_owner_post) | **POST** /{address}/setOwner | setOwner(address)
[**setx_central_bank_token_get**](DefaultApi.md#setx_central_bank_token_get) | **GET** /{address}/setxCentralBankToken | setxCentralBankToken(address)
[**setx_central_bank_token_post**](DefaultApi.md#setx_central_bank_token_post) | **POST** /{address}/setxCentralBankToken | setxCentralBankToken(address)



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


## get_intermediary_balance_get

> models::GetIntermediaryBalanceOutputs get_intermediary_balance_get(address, intermediary_address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getIntermediaryBalance(address) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**intermediary_address** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::GetIntermediaryBalanceOutputs**](getIntermediaryBalance_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intermediary_balance_post

> models::GetIntermediaryBalanceOutputs get_intermediary_balance_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
getIntermediaryBalance(address) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetIntermediaryBalanceInputs**](GetIntermediaryBalanceInputs.md) |  | [required] |
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

[**models::GetIntermediaryBalanceOutputs**](getIntermediaryBalance_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intermediary_get

> models::GetIntermediaryOutputs get_intermediary_get(address, intermediary_address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getIntermediary(address) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**intermediary_address** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::GetIntermediaryOutputs**](getIntermediary_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intermediary_post

> models::GetIntermediaryOutputs get_intermediary_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
getIntermediary(address) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetIntermediaryInputs**](GetIntermediaryInputs.md) |  | [required] |
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

[**models::GetIntermediaryOutputs**](getIntermediary_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getx_central_bank_token_get

> models::GetxCentralBankTokenOutputs getx_central_bank_token_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getxCentralBankToken() [read only]

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

[**models::GetxCentralBankTokenOutputs**](getxCentralBankToken_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getx_central_bank_token_post

> models::GetxCentralBankTokenOutputs getx_central_bank_token_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
getxCentralBankToken() [read only]

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

[**models::GetxCentralBankTokenOutputs**](getxCentralBankToken_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## intermediary_mint_transfer_subscribe

> models::IntermediaryMintTransferEvent intermediary_mint_transfer_subscribe(body)
IntermediaryMintTransfer(address,uint256) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**IntermediaryMintTransferSubscribeRequest**](IntermediaryMintTransferSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::IntermediaryMintTransferEvent**](IntermediaryMintTransfer_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## intermediary_mint_transfer_subscribe_all

> models::IntermediaryMintTransferEvent intermediary_mint_transfer_subscribe_all(address, body)
IntermediaryMintTransfer(address,uint256) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**IntermediaryMintTransferSubscribeRequest**](IntermediaryMintTransferSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::IntermediaryMintTransferEvent**](IntermediaryMintTransfer_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## intermediary_registration_subscribe

> models::IntermediaryRegistrationEvent intermediary_registration_subscribe(body)
IntermediaryRegistration(address,string) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**IntermediaryMintTransferSubscribeRequest**](IntermediaryMintTransferSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::IntermediaryRegistrationEvent**](IntermediaryRegistration_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## intermediary_registration_subscribe_all

> models::IntermediaryRegistrationEvent intermediary_registration_subscribe_all(address, body)
IntermediaryRegistration(address,string) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**IntermediaryMintTransferSubscribeRequest**](IntermediaryMintTransferSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::IntermediaryRegistrationEvent**](IntermediaryRegistration_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mint_transfer_intermediary_get

> serde_json::Value mint_transfer_intermediary_get(address, intermediary, num_tokens, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
mintTransferIntermediary(address,uint256)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**intermediary** | **String** | address | [required] |
**num_tokens** | **String** | uint256 | [required] |
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


## mint_transfer_intermediary_post

> serde_json::Value mint_transfer_intermediary_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
mintTransferIntermediary(address,uint256)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**MintTransferIntermediaryInputs**](MintTransferIntermediaryInputs.md) |  | [required] |
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


## owner_get

> models::OwnerOutputs owner_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
owner() [read only]

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

[**models::OwnerOutputs**](owner_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## owner_post

> models::OwnerOutputs owner_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
owner() [read only]

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

[**models::OwnerOutputs**](owner_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_intermediary_get

> serde_json::Value register_intermediary_get(address, intermediary_address, name, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
registerIntermediary(address,string)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**intermediary_address** | **String** | address | [required] |
**name** | **String** | string | [required] |
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


## register_intermediary_post

> serde_json::Value register_intermediary_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
registerIntermediary(address,string)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**RegisterIntermediaryInputs**](RegisterIntermediaryInputs.md) |  | [required] |
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


## set_owner_get

> serde_json::Value set_owner_get(address, owner_address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
setOwner(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**owner_address** | **String** | address | [required] |
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


## set_owner_post

> serde_json::Value set_owner_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
setOwner(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**SetOwnerInputs**](SetOwnerInputs.md) |  | [required] |
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


## setx_central_bank_token_get

> serde_json::Value setx_central_bank_token_get(address, token_address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
setxCentralBankToken(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**token_address** | **String** | address | [required] |
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


## setx_central_bank_token_post

> serde_json::Value setx_central_bank_token_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
setxCentralBankToken(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**SetxCentralBankTokenInputs**](SetxCentralBankTokenInputs.md) |  | [required] |
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

