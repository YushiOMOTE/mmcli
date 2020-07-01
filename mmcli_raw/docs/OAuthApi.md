# \OAuthApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth_apps_app_id_delete**](OAuthApi.md#oauth_apps_app_id_delete) | **delete** /oauth/apps/{app_id} | Delete an OAuth app
[**oauth_apps_app_id_get**](OAuthApi.md#oauth_apps_app_id_get) | **get** /oauth/apps/{app_id} | Get an OAuth app
[**oauth_apps_app_id_info_get**](OAuthApi.md#oauth_apps_app_id_info_get) | **get** /oauth/apps/{app_id}/info | Get info on an OAuth app
[**oauth_apps_app_id_put**](OAuthApi.md#oauth_apps_app_id_put) | **put** /oauth/apps/{app_id} | Update an OAuth app
[**oauth_apps_app_id_regen_secret_post**](OAuthApi.md#oauth_apps_app_id_regen_secret_post) | **post** /oauth/apps/{app_id}/regen_secret | Regenerate OAuth app secret
[**oauth_apps_get**](OAuthApi.md#oauth_apps_get) | **get** /oauth/apps | Get OAuth apps
[**oauth_apps_post**](OAuthApi.md#oauth_apps_post) | **post** /oauth/apps | Register OAuth app
[**users_user_id_oauth_apps_authorized_get**](OAuthApi.md#users_user_id_oauth_apps_authorized_get) | **get** /users/{user_id}/oauth/apps/authorized | Get authorized OAuth apps



## oauth_apps_app_id_delete

> crate::models::StatusOk oauth_apps_app_id_delete(app_id)
Delete an OAuth app

Delete and unregister an OAuth 2.0 client application  ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_apps_app_id_get

> crate::models::OAuthApp oauth_apps_app_id_get(app_id)
Get an OAuth app

Get an OAuth 2.0 client application registered with Mattermost. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_apps_app_id_info_get

> crate::models::OAuthApp oauth_apps_app_id_info_get(app_id)
Get info on an OAuth app

Get public information about an OAuth 2.0 client application registered with Mattermost. The application's client secret will be blanked out. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_apps_app_id_put

> crate::models::OAuthApp oauth_apps_app_id_put(app_id, inline_object76)
Update an OAuth app

Update an OAuth 2.0 client application based on OAuth struct. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |
**inline_object76** | [**InlineObject76**](InlineObject76.md) |  | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_apps_app_id_regen_secret_post

> crate::models::OAuthApp oauth_apps_app_id_regen_secret_post(app_id)
Regenerate OAuth app secret

Regenerate the client secret for an OAuth 2.0 client application registered with Mattermost. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_apps_get

> Vec<crate::models::OAuthApp> oauth_apps_get(page, per_page)
Get OAuth apps

Get a page of OAuth 2.0 client applications registered with Mattermost. ##### Permissions With `manage_oauth` permission, the apps registered by the logged in user are returned. With `manage_system_wide_oauth` permission, all apps regardless of creator are returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of apps per page. |  |[default to 60]

### Return type

[**Vec<crate::models::OAuthApp>**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_apps_post

> crate::models::OAuthApp oauth_apps_post(inline_object75)
Register OAuth app

Register an OAuth 2.0 client application with Mattermost as the service provider. ##### Permissions Must have `manage_oauth` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object75** | [**InlineObject75**](InlineObject75.md) |  | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_oauth_apps_authorized_get

> Vec<crate::models::OAuthApp> users_user_id_oauth_apps_authorized_get(user_id, page, per_page)
Get authorized OAuth apps

Get a page of OAuth 2.0 client applications authorized to access a user's account. ##### Permissions Must be authenticated as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of apps per page. |  |[default to 60]

### Return type

[**Vec<crate::models::OAuthApp>**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

