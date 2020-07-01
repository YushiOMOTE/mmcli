# \BotsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bots_bot_user_id_assign_user_id_post**](BotsApi.md#bots_bot_user_id_assign_user_id_post) | **post** /bots/{bot_user_id}/assign/{user_id} | Assign a bot to a user
[**bots_bot_user_id_convert_to_user_post**](BotsApi.md#bots_bot_user_id_convert_to_user_post) | **post** /bots/{bot_user_id}/convert_to_user | Convert a bot into a user
[**bots_bot_user_id_disable_post**](BotsApi.md#bots_bot_user_id_disable_post) | **post** /bots/{bot_user_id}/disable | Disable a bot
[**bots_bot_user_id_enable_post**](BotsApi.md#bots_bot_user_id_enable_post) | **post** /bots/{bot_user_id}/enable | Enable a bot
[**bots_bot_user_id_get**](BotsApi.md#bots_bot_user_id_get) | **get** /bots/{bot_user_id} | Get a bot
[**bots_bot_user_id_icon_delete**](BotsApi.md#bots_bot_user_id_icon_delete) | **delete** /bots/{bot_user_id}/icon | Delete bot's LHS icon image
[**bots_bot_user_id_icon_get**](BotsApi.md#bots_bot_user_id_icon_get) | **get** /bots/{bot_user_id}/icon | Get bot's LHS icon
[**bots_bot_user_id_icon_post**](BotsApi.md#bots_bot_user_id_icon_post) | **post** /bots/{bot_user_id}/icon | Set bot's LHS icon image
[**bots_bot_user_id_put**](BotsApi.md#bots_bot_user_id_put) | **put** /bots/{bot_user_id} | Patch a bot
[**bots_get**](BotsApi.md#bots_get) | **get** /bots | Get bots
[**bots_post**](BotsApi.md#bots_post) | **post** /bots | Create a bot
[**users_user_id_convert_to_bot_post**](BotsApi.md#users_user_id_convert_to_bot_post) | **post** /users/{user_id}/convert_to_bot | Convert a user into a bot



## bots_bot_user_id_assign_user_id_post

> crate::models::Bot bots_bot_user_id_assign_user_id_post(bot_user_id, user_id)
Assign a bot to a user

Assign a bot to a specified user. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**user_id** | **String** | The user ID to assign the bot to. | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_bot_user_id_convert_to_user_post

> crate::models::StatusOk bots_bot_user_id_convert_to_user_post(bot_user_id, inline_object86, set_system_admin)
Convert a bot into a user

Convert a bot into a user.  __Minimum server version__: 5.26  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**inline_object86** | [**InlineObject86**](InlineObject86.md) |  | [required] |
**set_system_admin** | Option<**bool**> | Whether to give the user the system admin role. |  |[default to false]

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_bot_user_id_disable_post

> crate::models::Bot bots_bot_user_id_disable_post(bot_user_id)
Disable a bot

Disable a bot. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_bot_user_id_enable_post

> crate::models::Bot bots_bot_user_id_enable_post(bot_user_id)
Enable a bot

Enable a bot. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_bot_user_id_get

> crate::models::Bot bots_bot_user_id_get(bot_user_id, include_deleted)
Get a bot

Get a bot specified by its bot id. ##### Permissions Must have `read_bots` permission for bots you are managing, and `read_others_bots` permission for bots others are managing. __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**include_deleted** | Option<**bool**> | If deleted bots should be returned. |  |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_bot_user_id_icon_delete

> crate::models::StatusOk bots_bot_user_id_icon_delete(bot_user_id)
Delete bot's LHS icon image

Delete bot's LHS icon image based on bot_user_id string parameter. ##### Permissions Must have `manage_bots` permission. __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_bot_user_id_icon_get

> bots_bot_user_id_icon_get(bot_user_id)
Get bot's LHS icon

Get a bot's LHS icon image based on bot_user_id string parameter. ##### Permissions Must be logged in. __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_bot_user_id_icon_post

> crate::models::StatusOk bots_bot_user_id_icon_post(bot_user_id, image)
Set bot's LHS icon image

Set a bot's LHS icon image based on bot_user_id string parameter. Icon image must be SVG format, all other formats are rejected. ##### Permissions Must have `manage_bots` permission. __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**image** | **std::path::PathBuf** | SVG icon image to be uploaded | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_bot_user_id_put

> crate::models::Bot bots_bot_user_id_put(bot_user_id, inline_object84)
Patch a bot

Partially update a bot by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**inline_object84** | [**InlineObject84**](InlineObject84.md) |  | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_get

> Vec<crate::models::Bot> bots_get(page, per_page, include_deleted, only_orphaned)
Get bots

Get a page of a list of bots. ##### Permissions Must have `read_bots` permission for bots you are managing, and `read_others_bots` permission for bots others are managing. __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of users per page. There is a maximum limit of 200 users per page. |  |[default to 60]
**include_deleted** | Option<**bool**> | If deleted bots should be returned. |  |
**only_orphaned** | Option<**bool**> | When true, only orphaned bots will be returned. A bot is consitered orphaned if it's owner has been deactivated. |  |

### Return type

[**Vec<crate::models::Bot>**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bots_post

> crate::models::Bot bots_post(inline_object83)
Create a bot

Create a new bot account on the system. Username is required. ##### Permissions Must have `create_bot` permission. __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object83** | [**InlineObject83**](InlineObject83.md) |  | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_convert_to_bot_post

> crate::models::StatusOk users_user_id_convert_to_bot_post(user_id)
Convert a user into a bot

Convert a user into a bot.  __Minimum server version__: 5.26  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

