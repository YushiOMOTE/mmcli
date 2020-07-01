# \EmojiApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**emoji_autocomplete_get**](EmojiApi.md#emoji_autocomplete_get) | **get** /emoji/autocomplete | Autocomplete custom emoji
[**emoji_emoji_id_delete**](EmojiApi.md#emoji_emoji_id_delete) | **delete** /emoji/{emoji_id} | Delete a custom emoji
[**emoji_emoji_id_get**](EmojiApi.md#emoji_emoji_id_get) | **get** /emoji/{emoji_id} | Get a custom emoji
[**emoji_emoji_id_image_get**](EmojiApi.md#emoji_emoji_id_image_get) | **get** /emoji/{emoji_id}/image | Get custom emoji image
[**emoji_get**](EmojiApi.md#emoji_get) | **get** /emoji | Get a list of custom emoji
[**emoji_name_emoji_name_get**](EmojiApi.md#emoji_name_emoji_name_get) | **get** /emoji/name/{emoji_name} | Get a custom emoji by name
[**emoji_post**](EmojiApi.md#emoji_post) | **post** /emoji | Create a custom emoji
[**emoji_search_post**](EmojiApi.md#emoji_search_post) | **post** /emoji/search | Search custom emoji



## emoji_autocomplete_get

> crate::models::Emoji emoji_autocomplete_get(name)
Autocomplete custom emoji

Get a list of custom emoji with names starting with or matching the provided name. Returns a maximum of 100 results. ##### Permissions Must be authenticated.  __Minimum server version__: 4.7 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The emoji name to search. | [required] |

### Return type

[**crate::models::Emoji**](Emoji.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_emoji_id_delete

> crate::models::Emoji emoji_emoji_id_delete(emoji_id)
Delete a custom emoji

Delete a custom emoji. ##### Permissions Must have the `manage_team` or `manage_system` permissions or be the user who created the emoji. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emoji_id** | **String** | Emoji GUID | [required] |

### Return type

[**crate::models::Emoji**](Emoji.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_emoji_id_get

> crate::models::Emoji emoji_emoji_id_get(emoji_id)
Get a custom emoji

Get some metadata for a custom emoji. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emoji_id** | **String** | Emoji GUID | [required] |

### Return type

[**crate::models::Emoji**](Emoji.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_emoji_id_image_get

> emoji_emoji_id_image_get(emoji_id)
Get custom emoji image

Get the image for a custom emoji. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emoji_id** | **String** | Emoji GUID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_get

> crate::models::Emoji emoji_get(page, per_page, sort)
Get a list of custom emoji

Get a page of metadata for custom emoji on the system. Since server version 4.7, sort using the `sort` query parameter. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of users per page. |  |[default to 60]
**sort** | Option<**String**> | Either blank for no sorting or \"name\" to sort by emoji names. Minimum server version for sorting is 4.7. |  |[default to ]

### Return type

[**crate::models::Emoji**](Emoji.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_name_emoji_name_get

> crate::models::Emoji emoji_name_emoji_name_get(emoji_name)
Get a custom emoji by name

Get some metadata for a custom emoji using its name. ##### Permissions Must be authenticated.  __Minimum server version__: 4.7 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emoji_name** | **String** | Emoji name | [required] |

### Return type

[**crate::models::Emoji**](Emoji.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_post

> crate::models::Emoji emoji_post(image, emoji)
Create a custom emoji

Create a custom emoji for the team. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | **std::path::PathBuf** | A file to be uploaded | [required] |
**emoji** | **String** | A JSON object containing a `name` field with the name of the emoji and a `creator_id` field with the id of the authenticated user. | [required] |

### Return type

[**crate::models::Emoji**](Emoji.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emoji_search_post

> Vec<crate::models::Emoji> emoji_search_post(inline_object63)
Search custom emoji

Search for custom emoji by name based on search criteria provided in the request body. A maximum of 200 results are returned. ##### Permissions Must be authenticated.  __Minimum server version__: 4.7 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object63** | [**InlineObject63**](InlineObject63.md) |  | [required] |

### Return type

[**Vec<crate::models::Emoji>**](Emoji.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

