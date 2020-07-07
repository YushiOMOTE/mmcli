# \WebhooksApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hooks_incoming_get**](WebhooksApi.md#hooks_incoming_get) | **get** /hooks/incoming | List incoming webhooks
[**hooks_incoming_hook_id_delete**](WebhooksApi.md#hooks_incoming_hook_id_delete) | **delete** /hooks/incoming/{hook_id} | Delete an incoming webhook
[**hooks_incoming_hook_id_get**](WebhooksApi.md#hooks_incoming_hook_id_get) | **get** /hooks/incoming/{hook_id} | Get an incoming webhook
[**hooks_incoming_hook_id_put**](WebhooksApi.md#hooks_incoming_hook_id_put) | **put** /hooks/incoming/{hook_id} | Update an incoming webhook
[**hooks_incoming_post**](WebhooksApi.md#hooks_incoming_post) | **post** /hooks/incoming | Create an incoming webhook
[**hooks_outgoing_get**](WebhooksApi.md#hooks_outgoing_get) | **get** /hooks/outgoing | List outgoing webhooks
[**hooks_outgoing_hook_id_delete**](WebhooksApi.md#hooks_outgoing_hook_id_delete) | **delete** /hooks/outgoing/{hook_id} | Delete an outgoing webhook
[**hooks_outgoing_hook_id_get**](WebhooksApi.md#hooks_outgoing_hook_id_get) | **get** /hooks/outgoing/{hook_id} | Get an outgoing webhook
[**hooks_outgoing_hook_id_put**](WebhooksApi.md#hooks_outgoing_hook_id_put) | **put** /hooks/outgoing/{hook_id} | Update an outgoing webhook
[**hooks_outgoing_hook_id_regen_token_post**](WebhooksApi.md#hooks_outgoing_hook_id_regen_token_post) | **post** /hooks/outgoing/{hook_id}/regen_token | Regenerate the token for the outgoing webhook.
[**hooks_outgoing_post**](WebhooksApi.md#hooks_outgoing_post) | **post** /hooks/outgoing | Create an outgoing webhook



## hooks_incoming_get

> Vec<crate::models::IncomingWebhook> hooks_incoming_get(page, per_page, team_id)
List incoming webhooks

Get a page of a list of incoming webhooks. Optionally filter for a specific team using query parameters. ##### Permissions `manage_webhooks` for the system or `manage_webhooks` for the specific team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of hooks per page. |  |[default to 60]
**team_id** | Option<**String**> | The ID of the team to get hooks for. |  |

### Return type

[**Vec<crate::models::IncomingWebhook>**](IncomingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_incoming_hook_id_delete

> crate::models::StatusOk hooks_incoming_hook_id_delete(hook_id)
Delete an incoming webhook

Delete an incoming webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Incoming webhook GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_incoming_hook_id_get

> crate::models::IncomingWebhook hooks_incoming_hook_id_get(hook_id)
Get an incoming webhook

Get an incoming webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Incoming Webhook GUID | [required] |

### Return type

[**crate::models::IncomingWebhook**](IncomingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_incoming_hook_id_put

> crate::models::IncomingWebhook hooks_incoming_hook_id_put(hook_id, inline_object65)
Update an incoming webhook

Update an incoming webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Incoming Webhook GUID | [required] |
**inline_object65** | [**InlineObject65**](InlineObject65.md) |  | [required] |

### Return type

[**crate::models::IncomingWebhook**](IncomingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_incoming_post

> crate::models::IncomingWebhook hooks_incoming_post(inline_object64)
Create an incoming webhook

Create an incoming webhook for a channel. ##### Permissions `manage_webhooks` for the channel the webhook is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object64** | [**InlineObject64**](InlineObject64.md) |  | [required] |

### Return type

[**crate::models::IncomingWebhook**](IncomingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_outgoing_get

> Vec<crate::models::OutgoingWebhook> hooks_outgoing_get(page, per_page, team_id, channel_id)
List outgoing webhooks

Get a page of a list of outgoing webhooks. Optionally filter for a specific team or channel using query parameters. ##### Permissions `manage_webhooks` for the system or `manage_webhooks` for the specific team/channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of hooks per page. |  |[default to 60]
**team_id** | Option<**String**> | The ID of the team to get hooks for. |  |
**channel_id** | Option<**String**> | The ID of the channel to get hooks for. |  |

### Return type

[**Vec<crate::models::OutgoingWebhook>**](OutgoingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_outgoing_hook_id_delete

> crate::models::StatusOk hooks_outgoing_hook_id_delete(hook_id)
Delete an outgoing webhook

Delete an outgoing webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Outgoing webhook GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_outgoing_hook_id_get

> crate::models::OutgoingWebhook hooks_outgoing_hook_id_get(hook_id)
Get an outgoing webhook

Get an outgoing webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Outgoing webhook GUID | [required] |

### Return type

[**crate::models::OutgoingWebhook**](OutgoingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_outgoing_hook_id_put

> crate::models::OutgoingWebhook hooks_outgoing_hook_id_put(hook_id, inline_object67)
Update an outgoing webhook

Update an outgoing webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | outgoing Webhook GUID | [required] |
**inline_object67** | [**InlineObject67**](InlineObject67.md) |  | [required] |

### Return type

[**crate::models::OutgoingWebhook**](OutgoingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_outgoing_hook_id_regen_token_post

> crate::models::StatusOk hooks_outgoing_hook_id_regen_token_post(hook_id)
Regenerate the token for the outgoing webhook.

Regenerate the token for the outgoing webhook. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Outgoing webhook GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hooks_outgoing_post

> crate::models::OutgoingWebhook hooks_outgoing_post(inline_object66)
Create an outgoing webhook

Create an outgoing webhook for a team. ##### Permissions `manage_webhooks` for the team the webhook is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object66** | [**InlineObject66**](InlineObject66.md) |  | [required] |

### Return type

[**crate::models::OutgoingWebhook**](OutgoingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

