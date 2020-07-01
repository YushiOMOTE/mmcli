# \StatusApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_status_ids_post**](StatusApi.md#users_status_ids_post) | **post** /users/status/ids | Get user statuses by id
[**users_user_id_status_get**](StatusApi.md#users_user_id_status_get) | **get** /users/{user_id}/status | Get user status
[**users_user_id_status_put**](StatusApi.md#users_user_id_status_put) | **put** /users/{user_id}/status | Update user status



## users_status_ids_post

> Vec<crate::models::Status> users_status_ids_post(request_body)
Get user statuses by id

Get a list of user statuses by id from the server. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | List of user ids to fetch | [required] |

### Return type

[**Vec<crate::models::Status>**](Status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_status_get

> crate::models::Status users_user_id_status_get(user_id)
Get user status

Get user status by id from the server. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_status_put

> crate::models::Status users_user_id_status_put(user_id, inline_object25)
Update user status

Manually set a user's status. When setting a user's status, the status will remain that value until set \"online\" again, which will return the status to being automatically updated based on user activity. ##### Permissions Must have `edit_other_users` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**inline_object25** | [**InlineObject25**](InlineObject25.md) |  | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

