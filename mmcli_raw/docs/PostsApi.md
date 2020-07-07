# \PostsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channels_channel_id_posts_get**](PostsApi.md#channels_channel_id_posts_get) | **get** /channels/{channel_id}/posts | Get posts for a channel
[**posts_ephemeral_post**](PostsApi.md#posts_ephemeral_post) | **post** /posts/ephemeral | Create a ephemeral post
[**posts_post**](PostsApi.md#posts_post) | **post** /posts | Create a post
[**posts_post_id_actions_action_id_post**](PostsApi.md#posts_post_id_actions_action_id_post) | **post** /posts/{post_id}/actions/{action_id} | Perform a post action
[**posts_post_id_delete**](PostsApi.md#posts_post_id_delete) | **delete** /posts/{post_id} | Delete a post
[**posts_post_id_files_info_get**](PostsApi.md#posts_post_id_files_info_get) | **get** /posts/{post_id}/files/info | Get file info for post
[**posts_post_id_get**](PostsApi.md#posts_post_id_get) | **get** /posts/{post_id} | Get a post
[**posts_post_id_patch_put**](PostsApi.md#posts_post_id_patch_put) | **put** /posts/{post_id}/patch | Patch a post
[**posts_post_id_pin_post**](PostsApi.md#posts_post_id_pin_post) | **post** /posts/{post_id}/pin | Pin a post to the channel
[**posts_post_id_put**](PostsApi.md#posts_post_id_put) | **put** /posts/{post_id} | Update a post
[**posts_post_id_thread_get**](PostsApi.md#posts_post_id_thread_get) | **get** /posts/{post_id}/thread | Get a thread
[**posts_post_id_unpin_post**](PostsApi.md#posts_post_id_unpin_post) | **post** /posts/{post_id}/unpin | Unpin a post to the channel
[**teams_team_id_posts_search_post**](PostsApi.md#teams_team_id_posts_search_post) | **post** /teams/{team_id}/posts/search | Search for team posts
[**users_user_id_channels_channel_id_posts_unread_get**](PostsApi.md#users_user_id_channels_channel_id_posts_unread_get) | **get** /users/{user_id}/channels/{channel_id}/posts/unread | Get posts around last unread
[**users_user_id_posts_flagged_get**](PostsApi.md#users_user_id_posts_flagged_get) | **get** /users/{user_id}/posts/flagged | Get a list of flagged posts
[**users_user_id_posts_post_id_set_unread_post**](PostsApi.md#users_user_id_posts_post_id_set_unread_post) | **post** /users/{user_id}/posts/{post_id}/set_unread | Mark as unread from a post.



## channels_channel_id_posts_get

> crate::models::PostList channels_channel_id_posts_get(channel_id, page, per_page, since, before, after)
Get posts for a channel

Get a page of posts in a channel. Use the query parameters to modify the behaviour of this endpoint. The parameter `since` must not be used with any of `before`, `after`, `page`, and `per_page` parameters. If `since` is used, it will always return all posts since that time limited till 1000. ##### Permissions Must have `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The channel ID to get the posts for | [required] |
**page** | Option<**i64**> | The page to select |  |[default to 0]
**per_page** | Option<**i64**> | The number of posts per page |  |[default to 60]
**since** | Option<**i64**> | Provide a non-zero value in Unix time milliseconds to select posts created after that time |  |
**before** | Option<**String**> | A post id to select the posts that came before this one |  |
**after** | Option<**String**> | A post id to select the posts that came after this one |  |

### Return type

[**crate::models::PostList**](PostList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_ephemeral_post

> crate::models::Post posts_ephemeral_post(inline_object53)
Create a ephemeral post

Create a new ephemeral post in a channel. ##### Permissions Must have `create_post_ephemeral` permission (currently only given to system admin) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object53** | [**InlineObject53**](InlineObject53.md) |  | [required] |

### Return type

[**crate::models::Post**](Post.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post

> crate::models::Post posts_post(inline_object52, set_online)
Create a post

Create a new post in a channel. To create the post as a comment on another post, provide `root_id`. ##### Permissions Must have `create_post` permission for the channel the post is being created in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object52** | [**InlineObject52**](InlineObject52.md) |  | [required] |
**set_online** | Option<**bool**> | Whether to set the user status as online or not. |  |

### Return type

[**crate::models::Post**](Post.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_actions_action_id_post

> crate::models::StatusOk posts_post_id_actions_action_id_post(post_id, action_id)
Perform a post action

Perform a post action, which allows users to interact with integrations through posts. ##### Permissions Must be authenticated and have the `read_channel` permission to the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | Post GUID | [required] |
**action_id** | **String** | Action GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_delete

> crate::models::StatusOk posts_post_id_delete(post_id)
Delete a post

Soft deletes a post, by marking the post as deleted in the database. Soft deleted posts will not be returned in post queries. ##### Permissions Must be logged in as the user or have `delete_others_posts` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | ID of the post to delete | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_files_info_get

> Vec<crate::models::FileInfo> posts_post_id_files_info_get(post_id)
Get file info for post

Gets a list of file information objects for the files attached to a post. ##### Permissions Must have `read_channel` permission for the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | ID of the post | [required] |

### Return type

[**Vec<crate::models::FileInfo>**](FileInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_get

> crate::models::Post posts_post_id_get(post_id)
Get a post

Get a single post. ##### Permissions Must have `read_channel` permission for the channel the post is in or if the channel is public, have the `read_public_channels` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | ID of the post to get | [required] |

### Return type

[**crate::models::Post**](Post.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_patch_put

> crate::models::Post posts_post_id_patch_put(post_id, inline_object55)
Patch a post

Partially update a post by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions Must have the `edit_post` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | Post GUID | [required] |
**inline_object55** | [**InlineObject55**](InlineObject55.md) |  | [required] |

### Return type

[**crate::models::Post**](Post.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_pin_post

> crate::models::StatusOk posts_post_id_pin_post(post_id)
Pin a post to the channel

Pin a post to a channel it is in based from the provided post id string. ##### Permissions Must be authenticated and have the `read_channel` permission to the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | Post GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_put

> crate::models::Post posts_post_id_put(post_id, inline_object54)
Update a post

Update a post. Only the fields listed below are updatable, omitted fields will be treated as blank. ##### Permissions Must have `edit_post` permission for the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | ID of the post to update | [required] |
**inline_object54** | [**InlineObject54**](InlineObject54.md) |  | [required] |

### Return type

[**crate::models::Post**](Post.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_thread_get

> crate::models::PostList posts_post_id_thread_get(post_id)
Get a thread

Get a post and the rest of the posts in the same thread. ##### Permissions Must have `read_channel` permission for the channel the post is in or if the channel is public, have the `read_public_channels` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | ID of a post in the thread | [required] |

### Return type

[**crate::models::PostList**](PostList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post_id_unpin_post

> crate::models::StatusOk posts_post_id_unpin_post(post_id)
Unpin a post to the channel

Unpin a post to a channel it is in based from the provided post id string. ##### Permissions Must be authenticated and have the `read_channel` permission to the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | Post GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_posts_search_post

> crate::models::PostListWithSearchMatches teams_team_id_posts_search_post(team_id, inline_object56)
Search for team posts

Search posts in the team and from the provided terms string. ##### Permissions Must be authenticated and have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object56** | [**InlineObject56**](InlineObject56.md) |  | [required] |

### Return type

[**crate::models::PostListWithSearchMatches**](PostListWithSearchMatches.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_channels_channel_id_posts_unread_get

> crate::models::PostList users_user_id_channels_channel_id_posts_unread_get(user_id, channel_id, limit_before, limit_after)
Get posts around last unread

Get the oldest unread post in the channel for the given user as well as the posts around it. ##### Permissions Must be logged in as the user or have `edit_other_users` permission, and must have `read_channel` permission for the channel. __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**channel_id** | **String** | The channel ID to get the posts for | [required] |
**limit_before** | Option<**i64**> | Number of posts before the last unread posts. Maximum is 200 posts if limit is set greater than that. |  |[default to 60]
**limit_after** | Option<**i64**> | Number of posts after and including the last unread post. Maximum is 200 posts if limit is set greater than that. |  |[default to 60]

### Return type

[**crate::models::PostList**](PostList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_posts_flagged_get

> Vec<crate::models::PostList> users_user_id_posts_flagged_get(user_id, team_id, channel_id, page, per_page)
Get a list of flagged posts

Get a page of flagged posts of a user provided user id string. Selects from a channel, team or all flagged posts by a user. ##### Permissions Must be user or have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**team_id** | Option<**String**> | Team ID |  |
**channel_id** | Option<**String**> | Channel ID |  |
**page** | Option<**i64**> | The page to select |  |[default to 0]
**per_page** | Option<**i64**> | The number of posts per page |  |[default to 60]

### Return type

[**Vec<crate::models::PostList>**](PostList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_posts_post_id_set_unread_post

> crate::models::ChannelUnreadAt users_user_id_posts_post_id_set_unread_post(user_id, post_id)
Mark as unread from a post.

Mark a channel as being unread from a given post. ##### Permissions Must have `read_channel` permission for the channel the post is in or if the channel is public, have the `read_public_channels` permission for the team. Must have `edit_other_users` permission if the user is not the one marking the post for himself.  __Minimum server version__: 5.18 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**post_id** | **String** | Post GUID | [required] |

### Return type

[**crate::models::ChannelUnreadAt**](ChannelUnreadAt.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

