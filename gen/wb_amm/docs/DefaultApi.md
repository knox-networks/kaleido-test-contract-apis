# \DefaultApi

All URIs are relative to *https://u0s5jrp8nx-u0jo91fxhc-connect.us0-aws.kaleido.io/gateways/u0ta9zm0up*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calculate_swap_get**](DefaultApi.md#calculate_swap_get) | **GET** /{address}/calculateSwap | calculateSwap(address,address,uint256,uint256) [read only]
[**calculate_swap_post**](DefaultApi.md#calculate_swap_post) | **POST** /{address}/calculateSwap | calculateSwap(address,address,uint256,uint256) [read only]
[**calculate_total_liquidity_get**](DefaultApi.md#calculate_total_liquidity_get) | **GET** /{address}/calculateTotalLiquidity | calculateTotalLiquidity(address,address,uint256) [read only]
[**calculate_total_liquidity_post**](DefaultApi.md#calculate_total_liquidity_post) | **POST** /{address}/calculateTotalLiquidity | calculateTotalLiquidity(address,address,uint256) [read only]
[**constructor_post**](DefaultApi.md#constructor_post) | **POST** / | constructor()
[**get_erc20_liquidity_get**](DefaultApi.md#get_erc20_liquidity_get) | **GET** /{address}/getERC20Liquidity | getERC20Liquidity(address) [read only]
[**get_erc20_liquidity_post**](DefaultApi.md#get_erc20_liquidity_post) | **POST** /{address}/getERC20Liquidity | getERC20Liquidity(address) [read only]
[**get_htlc_contract_address_get**](DefaultApi.md#get_htlc_contract_address_get) | **GET** /{address}/getHTLCContractAddress | getHTLCContractAddress() [read only]
[**get_htlc_contract_address_post**](DefaultApi.md#get_htlc_contract_address_post) | **POST** /{address}/getHTLCContractAddress | getHTLCContractAddress() [read only]
[**get_payment_notification_contract_get**](DefaultApi.md#get_payment_notification_contract_get) | **GET** /{address}/getPaymentNotificationContract | getPaymentNotificationContract() [read only]
[**get_payment_notification_contract_post**](DefaultApi.md#get_payment_notification_contract_post) | **POST** /{address}/getPaymentNotificationContract | getPaymentNotificationContract() [read only]
[**new_transfer_get**](DefaultApi.md#new_transfer_get) | **GET** /{address}/newTransfer | newTransfer(bytes32)
[**new_transfer_post**](DefaultApi.md#new_transfer_post) | **POST** /{address}/newTransfer | newTransfer(bytes32)
[**new_transfer_subscribe**](DefaultApi.md#new_transfer_subscribe) | **POST** /NewTransfer/subscribe | NewTransfer(bytes32,(address,address,(address,address),bytes32,string,(string,string),(address,address,uint256,uint256),bytes,(uint8,uint8,uint8),uint256,bytes32)) [event]
[**new_transfer_subscribe_all**](DefaultApi.md#new_transfer_subscribe_all) | **POST** /{address}/NewTransfer/subscribe | NewTransfer(bytes32,(address,address,(address,address),bytes32,string,(string,string),(address,address,uint256,uint256),bytes,(uint8,uint8,uint8),uint256,bytes32)) [event]
[**set_htlc_contract_address_get**](DefaultApi.md#set_htlc_contract_address_get) | **GET** /{address}/setHTLCContractAddress | setHTLCContractAddress(address)
[**set_htlc_contract_address_post**](DefaultApi.md#set_htlc_contract_address_post) | **POST** /{address}/setHTLCContractAddress | setHTLCContractAddress(address)
[**set_payment_notification_contract_get**](DefaultApi.md#set_payment_notification_contract_get) | **GET** /{address}/setPaymentNotificationContract | setPaymentNotificationContract(address)
[**set_payment_notification_contract_post**](DefaultApi.md#set_payment_notification_contract_post) | **POST** /{address}/setPaymentNotificationContract | setPaymentNotificationContract(address)



## calculate_swap_get

> models::CalculateSwapOutputs calculate_swap_get(address, payer_token_address, payee_token_address, exchange_rate, amount, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
calculateSwap(address,address,uint256,uint256) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**payer_token_address** | **String** | address | [required] |
**payee_token_address** | **String** | address | [required] |
**exchange_rate** | **String** | uint256 | [required] |
**amount** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::CalculateSwapOutputs**](calculateSwap_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calculate_swap_post

> models::CalculateSwapOutputs calculate_swap_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
calculateSwap(address,address,uint256,uint256) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**CalculateSwapInputs**](CalculateSwapInputs.md) |  | [required] |
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

[**models::CalculateSwapOutputs**](calculateSwap_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calculate_total_liquidity_get

> models::CalculateTotalLiquidityOutputs calculate_total_liquidity_get(address, payer_token_address, payee_token_address, exchange_rate, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
calculateTotalLiquidity(address,address,uint256) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**payer_token_address** | **String** | address | [required] |
**payee_token_address** | **String** | address | [required] |
**exchange_rate** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0xe20be4f77d982d7324237dfd3b03bb6552d28129]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**models::CalculateTotalLiquidityOutputs**](calculateTotalLiquidity_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calculate_total_liquidity_post

> models::CalculateTotalLiquidityOutputs calculate_total_liquidity_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
calculateTotalLiquidity(address,address,uint256) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**CalculateTotalLiquidityInputs**](CalculateTotalLiquidityInputs.md) |  | [required] |
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

[**models::CalculateTotalLiquidityOutputs**](calculateTotalLiquidity_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## constructor_post

> serde_json::Value constructor_post(body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_register)
constructor()

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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
**kld_register** | Option<**String**> | Register the installed contract on a friendly path (overwrites existing) (header: x-kaleido-register) |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_erc20_liquidity_get

> models::GetErc20LiquidityOutputs get_erc20_liquidity_get(address, token_address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getERC20Liquidity(address) [read only]

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

[**models::GetErc20LiquidityOutputs**](getERC20Liquidity_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_erc20_liquidity_post

> models::GetErc20LiquidityOutputs get_erc20_liquidity_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
getERC20Liquidity(address) [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetErc20LiquidityInputs**](GetErc20LiquidityInputs.md) |  | [required] |
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

[**models::GetErc20LiquidityOutputs**](getERC20Liquidity_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_htlc_contract_address_get

> models::GetHtlcContractAddressOutputs get_htlc_contract_address_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getHTLCContractAddress() [read only]

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

[**models::GetHtlcContractAddressOutputs**](getHTLCContractAddress_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_htlc_contract_address_post

> models::GetHtlcContractAddressOutputs get_htlc_contract_address_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
getHTLCContractAddress() [read only]

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

[**models::GetHtlcContractAddressOutputs**](getHTLCContractAddress_outputs.md)

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


## new_transfer_get

> serde_json::Value new_transfer_get(address, contract_id, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
newTransfer(bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**contract_id** | **String** | bytes32 | [required] |
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


## new_transfer_post

> serde_json::Value new_transfer_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
newTransfer(bytes32)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**NewTransferInputs**](NewTransferInputs.md) |  | [required] |
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


## new_transfer_subscribe

> models::NewTransferEvent new_transfer_subscribe(body)
NewTransfer(bytes32,(address,address,(address,address),bytes32,string,(string,string),(address,address,uint256,uint256),bytes,(uint8,uint8,uint8),uint256,bytes32)) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**NewTransferSubscribeRequest**](NewTransferSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::NewTransferEvent**](NewTransfer_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_transfer_subscribe_all

> models::NewTransferEvent new_transfer_subscribe_all(address, body)
NewTransfer(bytes32,(address,address,(address,address),bytes32,string,(string,string),(address,address,uint256,uint256),bytes,(uint8,uint8,uint8),uint256,bytes32)) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**NewTransferSubscribeRequest**](NewTransferSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**models::NewTransferEvent**](NewTransfer_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_htlc_contract_address_get

> serde_json::Value set_htlc_contract_address_get(address, htlc_handler_address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
setHTLCContractAddress(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**htlc_handler_address** | **String** | address | [required] |
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


## set_htlc_contract_address_post

> serde_json::Value set_htlc_contract_address_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype)
setHTLCContractAddress(address)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**SetHtlcContractAddressInputs**](SetHtlcContractAddressInputs.md) |  | [required] |
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

