# \DefaultApi

All URIs are relative to *https://u0yt7psxje-u0w0qcw9cy-connect.us0-aws.kaleido.io/gateways/kaleidoerc20mb*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allowance_get**](DefaultApi.md#allowance_get) | **GET** /{address}/allowance | allowance(address,address) [read only]
[**allowance_post**](DefaultApi.md#allowance_post) | **POST** /{address}/allowance | allowance(address,address) [read only]
[**approval_subscribe**](DefaultApi.md#approval_subscribe) | **POST** /Approval/subscribe | Approval(address,address,uint256) [event]
[**approval_subscribe_all**](DefaultApi.md#approval_subscribe_all) | **POST** /{address}/Approval/subscribe | Approval(address,address,uint256) [event]
[**approve_get**](DefaultApi.md#approve_get) | **GET** /{address}/approve | approve(address,uint256)
[**approve_post**](DefaultApi.md#approve_post) | **POST** /{address}/approve | approve(address,uint256)
[**balance_of_get**](DefaultApi.md#balance_of_get) | **GET** /{address}/balanceOf | balanceOf(address) [read only]
[**balance_of_post**](DefaultApi.md#balance_of_post) | **POST** /{address}/balanceOf | balanceOf(address) [read only]
[**burn_from_get**](DefaultApi.md#burn_from_get) | **GET** /{address}/burnFrom | burnFrom(address,uint256)
[**burn_from_post**](DefaultApi.md#burn_from_post) | **POST** /{address}/burnFrom | burnFrom(address,uint256)
[**burn_get**](DefaultApi.md#burn_get) | **GET** /{address}/burn | burn(uint256)
[**burn_post**](DefaultApi.md#burn_post) | **POST** /{address}/burn | burn(uint256)
[**constructor_post**](DefaultApi.md#constructor_post) | **POST** / | constructor(string,string,uint8,uint256)
[**d_efaultadminrole_get**](DefaultApi.md#d_efaultadminrole_get) | **GET** /{address}/DEFAULT_ADMIN_ROLE | DEFAULT_ADMIN_ROLE() [read only]
[**d_efaultadminrole_post**](DefaultApi.md#d_efaultadminrole_post) | **POST** /{address}/DEFAULT_ADMIN_ROLE | DEFAULT_ADMIN_ROLE() [read only]
[**decimals_get**](DefaultApi.md#decimals_get) | **GET** /{address}/decimals | decimals() [read only]
[**decimals_post**](DefaultApi.md#decimals_post) | **POST** /{address}/decimals | decimals() [read only]
[**decrease_allowance_get**](DefaultApi.md#decrease_allowance_get) | **GET** /{address}/decreaseAllowance | decreaseAllowance(address,uint256)
[**decrease_allowance_post**](DefaultApi.md#decrease_allowance_post) | **POST** /{address}/decreaseAllowance | decreaseAllowance(address,uint256)
[**get_role_admin_get**](DefaultApi.md#get_role_admin_get) | **GET** /{address}/getRoleAdmin | getRoleAdmin(bytes32) [read only]
[**get_role_admin_post**](DefaultApi.md#get_role_admin_post) | **POST** /{address}/getRoleAdmin | getRoleAdmin(bytes32) [read only]
[**get_role_member_count_get**](DefaultApi.md#get_role_member_count_get) | **GET** /{address}/getRoleMemberCount | getRoleMemberCount(bytes32) [read only]
[**get_role_member_count_post**](DefaultApi.md#get_role_member_count_post) | **POST** /{address}/getRoleMemberCount | getRoleMemberCount(bytes32) [read only]
[**get_role_member_get**](DefaultApi.md#get_role_member_get) | **GET** /{address}/getRoleMember | getRoleMember(bytes32,uint256) [read only]
[**get_role_member_post**](DefaultApi.md#get_role_member_post) | **POST** /{address}/getRoleMember | getRoleMember(bytes32,uint256) [read only]
[**grant_role_get**](DefaultApi.md#grant_role_get) | **GET** /{address}/grantRole | grantRole(bytes32,address)
[**grant_role_post**](DefaultApi.md#grant_role_post) | **POST** /{address}/grantRole | grantRole(bytes32,address)
[**has_role_get**](DefaultApi.md#has_role_get) | **GET** /{address}/hasRole | hasRole(bytes32,address) [read only]
[**has_role_post**](DefaultApi.md#has_role_post) | **POST** /{address}/hasRole | hasRole(bytes32,address) [read only]
[**increase_allowance_get**](DefaultApi.md#increase_allowance_get) | **GET** /{address}/increaseAllowance | increaseAllowance(address,uint256)
[**increase_allowance_post**](DefaultApi.md#increase_allowance_post) | **POST** /{address}/increaseAllowance | increaseAllowance(address,uint256)
[**m_interrole_get**](DefaultApi.md#m_interrole_get) | **GET** /{address}/MINTER_ROLE | MINTER_ROLE() [read only]
[**m_interrole_post**](DefaultApi.md#m_interrole_post) | **POST** /{address}/MINTER_ROLE | MINTER_ROLE() [read only]
[**mint_get**](DefaultApi.md#mint_get) | **GET** /{address}/mint | mint(address,uint256)
[**mint_post**](DefaultApi.md#mint_post) | **POST** /{address}/mint | mint(address,uint256)
[**name_get**](DefaultApi.md#name_get) | **GET** /{address}/name | name() [read only]
[**name_post**](DefaultApi.md#name_post) | **POST** /{address}/name | name() [read only]
[**renounce_role_get**](DefaultApi.md#renounce_role_get) | **GET** /{address}/renounceRole | renounceRole(bytes32,address)
[**renounce_role_post**](DefaultApi.md#renounce_role_post) | **POST** /{address}/renounceRole | renounceRole(bytes32,address)
[**revoke_role_get**](DefaultApi.md#revoke_role_get) | **GET** /{address}/revokeRole | revokeRole(bytes32,address)
[**revoke_role_post**](DefaultApi.md#revoke_role_post) | **POST** /{address}/revokeRole | revokeRole(bytes32,address)
[**role_granted_subscribe**](DefaultApi.md#role_granted_subscribe) | **POST** /RoleGranted/subscribe | RoleGranted(bytes32,address,address) [event]
[**role_granted_subscribe_all**](DefaultApi.md#role_granted_subscribe_all) | **POST** /{address}/RoleGranted/subscribe | RoleGranted(bytes32,address,address) [event]
[**role_revoked_subscribe**](DefaultApi.md#role_revoked_subscribe) | **POST** /RoleRevoked/subscribe | RoleRevoked(bytes32,address,address) [event]
[**role_revoked_subscribe_all**](DefaultApi.md#role_revoked_subscribe_all) | **POST** /{address}/RoleRevoked/subscribe | RoleRevoked(bytes32,address,address) [event]
[**symbol_get**](DefaultApi.md#symbol_get) | **GET** /{address}/symbol | symbol() [read only]
[**symbol_post**](DefaultApi.md#symbol_post) | **POST** /{address}/symbol | symbol() [read only]
[**total_supply_get**](DefaultApi.md#total_supply_get) | **GET** /{address}/totalSupply | totalSupply() [read only]
[**total_supply_post**](DefaultApi.md#total_supply_post) | **POST** /{address}/totalSupply | totalSupply() [read only]
[**transfer_from_get**](DefaultApi.md#transfer_from_get) | **GET** /{address}/transferFrom | transferFrom(address,address,uint256)
[**transfer_from_post**](DefaultApi.md#transfer_from_post) | **POST** /{address}/transferFrom | transferFrom(address,address,uint256)
[**transfer_get**](DefaultApi.md#transfer_get) | **GET** /{address}/transfer | transfer(address,uint256)
[**transfer_post**](DefaultApi.md#transfer_post) | **POST** /{address}/transfer | transfer(address,uint256)
[**transfer_subscribe**](DefaultApi.md#transfer_subscribe) | **POST** /Transfer/subscribe | Transfer(address,address,uint256) [event]
[**transfer_subscribe_all**](DefaultApi.md#transfer_subscribe_all) | **POST** /{address}/Transfer/subscribe | Transfer(address,address,uint256) [event]



## allowance_get

> crate::models::AllowanceOutputs allowance_get(address, owner, spender, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
allowance(address,address) [read only]

See {IERC20-allowance}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**owner** | **String** | address | [required] |
**spender** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::AllowanceOutputs**](allowance_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allowance_post

> crate::models::AllowanceOutputs allowance_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
allowance(address,address) [read only]

See {IERC20-allowance}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**AllowanceInputs**](AllowanceInputs.md) |  | [required] |
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

[**crate::models::AllowanceOutputs**](allowance_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approval_subscribe

> crate::models::ApprovalEvent approval_subscribe(body)
Approval(address,address,uint256) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApprovalSubscribeRequest**](ApprovalSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**crate::models::ApprovalEvent**](Approval_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approval_subscribe_all

> crate::models::ApprovalEvent approval_subscribe_all(address, body)
Approval(address,address,uint256) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**ApprovalSubscribeRequest**](ApprovalSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**crate::models::ApprovalEvent**](Approval_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_get

> crate::models::ApproveOutputs approve_get(address, spender, amount, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
approve(address,uint256)

See {IERC20-approve}. Requirements: - `spender` cannot be the zero address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**spender** | **String** | address | [required] |
**amount** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::ApproveOutputs**](approve_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_post

> crate::models::ApproveOutputs approve_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
approve(address,uint256)

See {IERC20-approve}. Requirements: - `spender` cannot be the zero address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**ApproveInputs**](ApproveInputs.md) |  | [required] |
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

[**crate::models::ApproveOutputs**](approve_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance_of_get

> crate::models::BalanceOfOutputs balance_of_get(address, account, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
balanceOf(address) [read only]

See {IERC20-balanceOf}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**account** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::BalanceOfOutputs**](balanceOf_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance_of_post

> crate::models::BalanceOfOutputs balance_of_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
balanceOf(address) [read only]

See {IERC20-balanceOf}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**BalanceOfInputs**](BalanceOfInputs.md) |  | [required] |
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

[**crate::models::BalanceOfOutputs**](balanceOf_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## burn_from_get

> serde_json::Value burn_from_get(address, account, amount, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
burnFrom(address,uint256)

Destroys `amount` tokens from `account`, deducting from the caller's allowance. See {ERC20-_burn} and {ERC20-allowance}. Requirements: - the caller must have allowance for ``accounts``'s tokens of at least `amount`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**account** | **String** | address | [required] |
**amount** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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


## burn_from_post

> serde_json::Value burn_from_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
burnFrom(address,uint256)

Destroys `amount` tokens from `account`, deducting from the caller's allowance. See {ERC20-_burn} and {ERC20-allowance}. Requirements: - the caller must have allowance for ``accounts``'s tokens of at least `amount`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**BurnFromInputs**](BurnFromInputs.md) |  | [required] |
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

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## burn_get

> serde_json::Value burn_get(address, amount, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
burn(uint256)

Destroys `amount` tokens from the caller. See {ERC20-_burn}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**amount** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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


## burn_post

> serde_json::Value burn_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
burn(uint256)

Destroys `amount` tokens from the caller. See {ERC20-_burn}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**BurnInputs**](BurnInputs.md) |  | [required] |
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

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## constructor_post

> serde_json::Value constructor_post(body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid, kld_register)
constructor(string,string,uint8,uint256)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConstructorInputs**](ConstructorInputs.md) |  | [required] |
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


## d_efaultadminrole_get

> crate::models::DefaultAdminRoleOutputs d_efaultadminrole_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
DEFAULT_ADMIN_ROLE() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::DefaultAdminRoleOutputs**](DEFAULT_ADMIN_ROLE_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## d_efaultadminrole_post

> crate::models::DefaultAdminRoleOutputs d_efaultadminrole_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
DEFAULT_ADMIN_ROLE() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
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

### Return type

[**crate::models::DefaultAdminRoleOutputs**](DEFAULT_ADMIN_ROLE_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decimals_get

> crate::models::DecimalsOutputs decimals_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
decimals() [read only]

Returns the number of decimals used to get its user representation. For example, if `decimals` equals `2`, a balance of `505` tokens should be displayed to a user as `5,05` (`505 / 10 ** 2`). Tokens usually opt for a value of 18, imitating the relationship between Ether and Wei. This is the value {ERC20} uses, unless {_setupDecimals} is called. NOTE: This information is only used for _display_ purposes: it in no way affects any of the arithmetic of the contract, including {IERC20-balanceOf} and {IERC20-transfer}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::DecimalsOutputs**](decimals_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decimals_post

> crate::models::DecimalsOutputs decimals_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
decimals() [read only]

Returns the number of decimals used to get its user representation. For example, if `decimals` equals `2`, a balance of `505` tokens should be displayed to a user as `5,05` (`505 / 10 ** 2`). Tokens usually opt for a value of 18, imitating the relationship between Ether and Wei. This is the value {ERC20} uses, unless {_setupDecimals} is called. NOTE: This information is only used for _display_ purposes: it in no way affects any of the arithmetic of the contract, including {IERC20-balanceOf} and {IERC20-transfer}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
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

### Return type

[**crate::models::DecimalsOutputs**](decimals_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decrease_allowance_get

> crate::models::DecreaseAllowanceOutputs decrease_allowance_get(address, spender, subtracted_value, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
decreaseAllowance(address,uint256)

Atomically decreases the allowance granted to `spender` by the caller. This is an alternative to {approve} that can be used as a mitigation for problems described in {IERC20-approve}. Emits an {Approval} event indicating the updated allowance. Requirements: - `spender` cannot be the zero address. - `spender` must have allowance for the caller of at least `subtractedValue`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**spender** | **String** | address | [required] |
**subtracted_value** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::DecreaseAllowanceOutputs**](decreaseAllowance_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decrease_allowance_post

> crate::models::DecreaseAllowanceOutputs decrease_allowance_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
decreaseAllowance(address,uint256)

Atomically decreases the allowance granted to `spender` by the caller. This is an alternative to {approve} that can be used as a mitigation for problems described in {IERC20-approve}. Emits an {Approval} event indicating the updated allowance. Requirements: - `spender` cannot be the zero address. - `spender` must have allowance for the caller of at least `subtractedValue`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**DecreaseAllowanceInputs**](DecreaseAllowanceInputs.md) |  | [required] |
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

[**crate::models::DecreaseAllowanceOutputs**](decreaseAllowance_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_admin_get

> crate::models::GetRoleAdminOutputs get_role_admin_get(address, role, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getRoleAdmin(bytes32) [read only]

Returns the admin role that controls `role`. See {grantRole} and {revokeRole}. To change a role's admin, use {_setRoleAdmin}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**role** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::GetRoleAdminOutputs**](getRoleAdmin_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_admin_post

> crate::models::GetRoleAdminOutputs get_role_admin_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
getRoleAdmin(bytes32) [read only]

Returns the admin role that controls `role`. See {grantRole} and {revokeRole}. To change a role's admin, use {_setRoleAdmin}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetRoleAdminInputs**](GetRoleAdminInputs.md) |  | [required] |
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

[**crate::models::GetRoleAdminOutputs**](getRoleAdmin_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_member_count_get

> crate::models::GetRoleMemberCountOutputs get_role_member_count_get(address, role, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getRoleMemberCount(bytes32) [read only]

Returns the number of accounts that have `role`. Can be used together with {getRoleMember} to enumerate all bearers of a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**role** | **String** | bytes32 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::GetRoleMemberCountOutputs**](getRoleMemberCount_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_member_count_post

> crate::models::GetRoleMemberCountOutputs get_role_member_count_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
getRoleMemberCount(bytes32) [read only]

Returns the number of accounts that have `role`. Can be used together with {getRoleMember} to enumerate all bearers of a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetRoleMemberCountInputs**](GetRoleMemberCountInputs.md) |  | [required] |
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

[**crate::models::GetRoleMemberCountOutputs**](getRoleMemberCount_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_member_get

> crate::models::GetRoleMemberOutputs get_role_member_get(address, role, index, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
getRoleMember(bytes32,uint256) [read only]

Returns one of the accounts that have `role`. `index` must be a value between 0 and {getRoleMemberCount}, non-inclusive. Role bearers are not sorted in any particular way, and their ordering may change at any point. WARNING: When using {getRoleMember} and {getRoleMemberCount}, make sure you perform all queries on the same block. See the following https://forum.openzeppelin.com/t/iterating-over-elements-on-enumerableset-in-openzeppelin-contracts/2296[forum post] for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**role** | **String** | bytes32 | [required] |
**index** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::GetRoleMemberOutputs**](getRoleMember_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_member_post

> crate::models::GetRoleMemberOutputs get_role_member_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
getRoleMember(bytes32,uint256) [read only]

Returns one of the accounts that have `role`. `index` must be a value between 0 and {getRoleMemberCount}, non-inclusive. Role bearers are not sorted in any particular way, and their ordering may change at any point. WARNING: When using {getRoleMember} and {getRoleMemberCount}, make sure you perform all queries on the same block. See the following https://forum.openzeppelin.com/t/iterating-over-elements-on-enumerableset-in-openzeppelin-contracts/2296[forum post] for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GetRoleMemberInputs**](GetRoleMemberInputs.md) |  | [required] |
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

[**crate::models::GetRoleMemberOutputs**](getRoleMember_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_role_get

> serde_json::Value grant_role_get(address, role, account, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
grantRole(bytes32,address)

Grants `role` to `account`. If `account` had not been already granted `role`, emits a {RoleGranted} event. Requirements: - the caller must have ``role``'s admin role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**role** | **String** | bytes32 | [required] |
**account** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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


## grant_role_post

> serde_json::Value grant_role_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
grantRole(bytes32,address)

Grants `role` to `account`. If `account` had not been already granted `role`, emits a {RoleGranted} event. Requirements: - the caller must have ``role``'s admin role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**GrantRoleInputs**](GrantRoleInputs.md) |  | [required] |
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

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## has_role_get

> crate::models::HasRoleOutputs has_role_get(address, role, account, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
hasRole(bytes32,address) [read only]

Returns `true` if `account` has been granted `role`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**role** | **String** | bytes32 | [required] |
**account** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::HasRoleOutputs**](hasRole_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## has_role_post

> crate::models::HasRoleOutputs has_role_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
hasRole(bytes32,address) [read only]

Returns `true` if `account` has been granted `role`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**HasRoleInputs**](HasRoleInputs.md) |  | [required] |
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

[**crate::models::HasRoleOutputs**](hasRole_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## increase_allowance_get

> crate::models::IncreaseAllowanceOutputs increase_allowance_get(address, spender, added_value, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
increaseAllowance(address,uint256)

Atomically increases the allowance granted to `spender` by the caller. This is an alternative to {approve} that can be used as a mitigation for problems described in {IERC20-approve}. Emits an {Approval} event indicating the updated allowance. Requirements: - `spender` cannot be the zero address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**spender** | **String** | address | [required] |
**added_value** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::IncreaseAllowanceOutputs**](increaseAllowance_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## increase_allowance_post

> crate::models::IncreaseAllowanceOutputs increase_allowance_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
increaseAllowance(address,uint256)

Atomically increases the allowance granted to `spender` by the caller. This is an alternative to {approve} that can be used as a mitigation for problems described in {IERC20-approve}. Emits an {Approval} event indicating the updated allowance. Requirements: - `spender` cannot be the zero address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**IncreaseAllowanceInputs**](IncreaseAllowanceInputs.md) |  | [required] |
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

[**crate::models::IncreaseAllowanceOutputs**](increaseAllowance_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## m_interrole_get

> crate::models::MinterRoleOutputs m_interrole_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
MINTER_ROLE() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::MinterRoleOutputs**](MINTER_ROLE_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## m_interrole_post

> crate::models::MinterRoleOutputs m_interrole_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
MINTER_ROLE() [read only]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
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

### Return type

[**crate::models::MinterRoleOutputs**](MINTER_ROLE_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mint_get

> serde_json::Value mint_get(address, to, amount, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
mint(address,uint256)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**to** | **String** | address | [required] |
**amount** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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


## mint_post

> serde_json::Value mint_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
mint(address,uint256)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**MintInputs**](MintInputs.md) |  | [required] |
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

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## name_get

> crate::models::NameOutputs name_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
name() [read only]

Returns the name of the token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::NameOutputs**](name_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## name_post

> crate::models::NameOutputs name_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
name() [read only]

Returns the name of the token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
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

### Return type

[**crate::models::NameOutputs**](name_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## renounce_role_get

> serde_json::Value renounce_role_get(address, role, account, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
renounceRole(bytes32,address)

Revokes `role` from the calling account. Roles are often managed via {grantRole} and {revokeRole}: this function's purpose is to provide a mechanism for accounts to lose their privileges if they are compromised (such as when a trusted device is misplaced). If the calling account had been granted `role`, emits a {RoleRevoked} event. Requirements: - the caller must be `account`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**role** | **String** | bytes32 | [required] |
**account** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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


## renounce_role_post

> serde_json::Value renounce_role_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
renounceRole(bytes32,address)

Revokes `role` from the calling account. Roles are often managed via {grantRole} and {revokeRole}: this function's purpose is to provide a mechanism for accounts to lose their privileges if they are compromised (such as when a trusted device is misplaced). If the calling account had been granted `role`, emits a {RoleRevoked} event. Requirements: - the caller must be `account`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**RenounceRoleInputs**](RenounceRoleInputs.md) |  | [required] |
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

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_role_get

> serde_json::Value revoke_role_get(address, role, account, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
revokeRole(bytes32,address)

Revokes `role` from `account`. If `account` had been granted `role`, emits a {RoleRevoked} event. Requirements: - the caller must have ``role``'s admin role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**role** | **String** | bytes32 | [required] |
**account** | **String** | address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
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


## revoke_role_post

> serde_json::Value revoke_role_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
revokeRole(bytes32,address)

Revokes `role` from `account`. If `account` had been granted `role`, emits a {RoleRevoked} event. Requirements: - the caller must have ``role``'s admin role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**RevokeRoleInputs**](RevokeRoleInputs.md) |  | [required] |
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

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_granted_subscribe

> crate::models::RoleGrantedEvent role_granted_subscribe(body)
RoleGranted(bytes32,address,address) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApprovalSubscribeRequest**](ApprovalSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**crate::models::RoleGrantedEvent**](RoleGranted_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_granted_subscribe_all

> crate::models::RoleGrantedEvent role_granted_subscribe_all(address, body)
RoleGranted(bytes32,address,address) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**ApprovalSubscribeRequest**](ApprovalSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**crate::models::RoleGrantedEvent**](RoleGranted_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_revoked_subscribe

> crate::models::RoleRevokedEvent role_revoked_subscribe(body)
RoleRevoked(bytes32,address,address) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApprovalSubscribeRequest**](ApprovalSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**crate::models::RoleRevokedEvent**](RoleRevoked_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_revoked_subscribe_all

> crate::models::RoleRevokedEvent role_revoked_subscribe_all(address, body)
RoleRevoked(bytes32,address,address) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**ApprovalSubscribeRequest**](ApprovalSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**crate::models::RoleRevokedEvent**](RoleRevoked_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## symbol_get

> crate::models::SymbolOutputs symbol_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
symbol() [read only]

Returns the symbol of the token, usually a shorter version of the name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::SymbolOutputs**](symbol_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## symbol_post

> crate::models::SymbolOutputs symbol_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
symbol() [read only]

Returns the symbol of the token, usually a shorter version of the name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
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

### Return type

[**crate::models::SymbolOutputs**](symbol_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## total_supply_get

> crate::models::TotalSupplyOutputs total_supply_get(address, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
totalSupply() [read only]

See {IERC20-totalSupply}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::TotalSupplyOutputs**](totalSupply_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## total_supply_post

> crate::models::TotalSupplyOutputs total_supply_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
totalSupply() [read only]

See {IERC20-totalSupply}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
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

### Return type

[**crate::models::TotalSupplyOutputs**](totalSupply_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_from_get

> crate::models::TransferFromOutputs transfer_from_get(address, sender, recipient, amount, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
transferFrom(address,address,uint256)

See {IERC20-transferFrom}. Emits an {Approval} event indicating the updated allowance. This is not required by the EIP. See the note at the beginning of {ERC20}; Requirements: - `sender` and `recipient` cannot be the zero address. - `sender` must have a balance of at least `amount`. - the caller must have allowance for ``sender``'s tokens of at least `amount`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**sender** | **String** | address | [required] |
**recipient** | **String** | address | [required] |
**amount** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::TransferFromOutputs**](transferFrom_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_from_post

> crate::models::TransferFromOutputs transfer_from_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
transferFrom(address,address,uint256)

See {IERC20-transferFrom}. Emits an {Approval} event indicating the updated allowance. This is not required by the EIP. See the note at the beginning of {ERC20}; Requirements: - `sender` and `recipient` cannot be the zero address. - `sender` must have a balance of at least `amount`. - the caller must have allowance for ``sender``'s tokens of at least `amount`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**TransferFromInputs**](TransferFromInputs.md) |  | [required] |
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

[**crate::models::TransferFromOutputs**](transferFrom_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_get

> crate::models::TransferOutputs transfer_get(address, recipient, amount, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_transaction)
transfer(address,uint256)

See {IERC20-transfer}. Requirements: - `recipient` cannot be the zero address. - the caller must have a balance of at least `amount`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**recipient** | **String** | address | [required] |
**amount** | **String** | uint256 | [required] |
**kld_id** | Option<**String**> | Optionally set the ID for this request - must be unique if set (header: x-kaleido-id) |  |
**kld_from** | Option<**String**> | The 'from' address (header: x-kaleido-from) |  |[default to 0x8df941bda2dd8524e8ee870335bdfb62ec8a6384]
**kld_ethvalue** | Option<**i32**> | Ether value to send with the transaction (header: x-kaleido-ethvalue) |  |
**kld_gas** | Option<**i32**> | Gas to send with the transaction (auto-calculated if not set) (header: x-kaleido-gas) |  |
**kld_gasprice** | Option<**i32**> | Gas Price offered (header: x-kaleido-gasprice) |  |
**kld_transaction** | Option<**String**> | Query the details for the provided transaction hash (header: x-kaleido-transaction) |  |

### Return type

[**crate::models::TransferOutputs**](transfer_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_post

> crate::models::TransferOutputs transfer_post(address, body, kld_id, kld_from, kld_ethvalue, kld_gas, kld_gasprice, kld_sync, kld_call, kld_privatefrom, kld_privatefor, kld_blocknumber, kld_acktype, kld_privacygroupid)
transfer(address,uint256)

See {IERC20-transfer}. Requirements: - `recipient` cannot be the zero address. - the caller must have a balance of at least `amount`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**TransferInputs**](TransferInputs.md) |  | [required] |
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

[**crate::models::TransferOutputs**](transfer_outputs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_subscribe

> crate::models::TransferEvent transfer_subscribe(body)
Transfer(address,address,uint256) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApprovalSubscribeRequest**](ApprovalSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**crate::models::TransferEvent**](Transfer_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_subscribe_all

> crate::models::TransferEvent transfer_subscribe_all(address, body)
Transfer(address,address,uint256) [event]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The contract address | [required] |
**body** | [**ApprovalSubscribeRequest**](ApprovalSubscribeRequest.md) | Subscription configuration for the REST Gateway (response schema will be delivered async over the configured stream) | [required] |

### Return type

[**crate::models::TransferEvent**](Transfer_event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

