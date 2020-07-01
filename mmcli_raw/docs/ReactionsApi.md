# \ReactionsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**posts_ids_reactions_post**](ReactionsApi.md#posts_ids_reactions_post) | **post** /posts/ids/reactions | Bulk get the reaction for posts
[**posts_post_id_reactions_get**](ReactionsApi.md#posts_post_id_reactions_get) | **get** /posts/{post_id}/reactions | Get a list of reactions to a post
[**reactions_post**](ReactionsApi.md#reactions_post) | **post** /reactions | Create a reaction
[**users_user_id_posts_post_id_reactions_emoji_name_delete**](ReactionsApi.md#users_user_id_posts_post_id_reactions_emoji_name_delete) | **delete** /users/{user_id}/posts/{post_id}/reactions/{emoji_name} | Remove a reaction from a post



## posts_ids_reactions_post

> ::std::collections::HashMap<String, Vec<crate::models::Reaction>> posts_ids_reactions_post(request_body)
Bulk get the reaction for posts

Get a list of reactions made by all users to a given post. ##### Permissions Must have `read_channel` permission for the channel the post is in.  __Minimum server version__: 5.8 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | Array of post IDs | [required] |

### Return type

[**::std::collections::HashMap<String, Vec<crate::models::Reaction>>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_reactions_get

> Vec<crate::models::Reaction> posts_post_id_reactions_get(post_id)
Get a list of reactions to a post

Get a list of reactions made by all users to a given post. ##### Permissions Must have `read_channel` permission for the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | ID of a post | [required] |

### Return type

[**Vec<crate::models::Reaction>**](Reaction.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_post

> crate::models::Reaction reactions_post(reaction)
Create a reaction

Create a reaction. ##### Permissions Must have `read_channel` permission for the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reaction** | [**Reaction**](Reaction.md) | The user's reaction with its post_id, user_id, and emoji_name fields set | [required] |

### Return type

[**crate::models::Reaction**](Reaction.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_posts_post_id_reactions_emoji_name_delete

> crate::models::StatusOk users_user_id_posts_post_id_reactions_emoji_name_delete(user_id, post_id, emoji_name)
Remove a reaction from a post

Deletes a reaction made by a user from the given post. ##### Permissions Must be user or have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**post_id** | **String** | ID of the post | [required] |
**emoji_name** | **String** | emoji name | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

