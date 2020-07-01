# \ChannelsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channels_channel_id_convert_post**](ChannelsApi.md#channels_channel_id_convert_post) | **post** /channels/{channel_id}/convert | Convert a channel from public to private
[**channels_channel_id_delete**](ChannelsApi.md#channels_channel_id_delete) | **delete** /channels/{channel_id} | Delete a channel
[**channels_channel_id_get**](ChannelsApi.md#channels_channel_id_get) | **get** /channels/{channel_id} | Get a channel
[**channels_channel_id_member_counts_by_group_get**](ChannelsApi.md#channels_channel_id_member_counts_by_group_get) | **get** /channels/{channel_id}/member_counts_by_group | Channel members counts for each group that has atleast one member in the channel
[**channels_channel_id_members_get**](ChannelsApi.md#channels_channel_id_members_get) | **get** /channels/{channel_id}/members | Get channel members
[**channels_channel_id_members_ids_post**](ChannelsApi.md#channels_channel_id_members_ids_post) | **post** /channels/{channel_id}/members/ids | Get channel members by ids
[**channels_channel_id_members_minus_group_members_get**](ChannelsApi.md#channels_channel_id_members_minus_group_members_get) | **get** /channels/{channel_id}/members_minus_group_members | Channel members minus group members.
[**channels_channel_id_members_post**](ChannelsApi.md#channels_channel_id_members_post) | **post** /channels/{channel_id}/members | Add user to channel
[**channels_channel_id_members_user_id_delete**](ChannelsApi.md#channels_channel_id_members_user_id_delete) | **delete** /channels/{channel_id}/members/{user_id} | Remove user from channel
[**channels_channel_id_members_user_id_get**](ChannelsApi.md#channels_channel_id_members_user_id_get) | **get** /channels/{channel_id}/members/{user_id} | Get channel member
[**channels_channel_id_members_user_id_notify_props_put**](ChannelsApi.md#channels_channel_id_members_user_id_notify_props_put) | **put** /channels/{channel_id}/members/{user_id}/notify_props | Update channel notifications
[**channels_channel_id_members_user_id_roles_put**](ChannelsApi.md#channels_channel_id_members_user_id_roles_put) | **put** /channels/{channel_id}/members/{user_id}/roles | Update channel roles
[**channels_channel_id_members_user_id_scheme_roles_put**](ChannelsApi.md#channels_channel_id_members_user_id_scheme_roles_put) | **put** /channels/{channel_id}/members/{user_id}/schemeRoles | Update the scheme-derived roles of a channel member.
[**channels_channel_id_moderations_get**](ChannelsApi.md#channels_channel_id_moderations_get) | **get** /channels/{channel_id}/moderations | Get information about channel's moderation.
[**channels_channel_id_moderations_patch_put**](ChannelsApi.md#channels_channel_id_moderations_patch_put) | **put** /channels/{channel_id}/moderations/patch | Get information about channel's moderation.
[**channels_channel_id_move_post**](ChannelsApi.md#channels_channel_id_move_post) | **post** /channels/{channel_id}/move | Move a channel
[**channels_channel_id_patch_put**](ChannelsApi.md#channels_channel_id_patch_put) | **put** /channels/{channel_id}/patch | Patch a channel
[**channels_channel_id_pinned_get**](ChannelsApi.md#channels_channel_id_pinned_get) | **get** /channels/{channel_id}/pinned | Get a channel's pinned posts
[**channels_channel_id_privacy_put**](ChannelsApi.md#channels_channel_id_privacy_put) | **put** /channels/{channel_id}/privacy | Update channel's privacy
[**channels_channel_id_put**](ChannelsApi.md#channels_channel_id_put) | **put** /channels/{channel_id} | Update a channel
[**channels_channel_id_restore_post**](ChannelsApi.md#channels_channel_id_restore_post) | **post** /channels/{channel_id}/restore | Restore a channel
[**channels_channel_id_scheme_put**](ChannelsApi.md#channels_channel_id_scheme_put) | **put** /channels/{channel_id}/scheme | Set a channel's scheme
[**channels_channel_id_stats_get**](ChannelsApi.md#channels_channel_id_stats_get) | **get** /channels/{channel_id}/stats | Get channel statistics
[**channels_channel_id_timezones_get**](ChannelsApi.md#channels_channel_id_timezones_get) | **get** /channels/{channel_id}/timezones | Get timezones in a channel
[**channels_direct_post**](ChannelsApi.md#channels_direct_post) | **post** /channels/direct | Create a direct message channel
[**channels_get**](ChannelsApi.md#channels_get) | **get** /channels | Get a list of all channels
[**channels_group_post**](ChannelsApi.md#channels_group_post) | **post** /channels/group | Create a group message channel
[**channels_group_search_post**](ChannelsApi.md#channels_group_search_post) | **post** /channels/group/search | Search Group Channels
[**channels_members_user_id_view_post**](ChannelsApi.md#channels_members_user_id_view_post) | **post** /channels/members/{user_id}/view | View channel
[**channels_post**](ChannelsApi.md#channels_post) | **post** /channels | Create a channel
[**channels_search_post**](ChannelsApi.md#channels_search_post) | **post** /channels/search | Search all private and open type channels across all teams
[**teams_name_team_name_channels_name_channel_name_get**](ChannelsApi.md#teams_name_team_name_channels_name_channel_name_get) | **get** /teams/name/{team_name}/channels/name/{channel_name} | Get a channel by name and team name
[**teams_team_id_channels_autocomplete_get**](ChannelsApi.md#teams_team_id_channels_autocomplete_get) | **get** /teams/{team_id}/channels/autocomplete | Autocomplete channels
[**teams_team_id_channels_deleted_get**](ChannelsApi.md#teams_team_id_channels_deleted_get) | **get** /teams/{team_id}/channels/deleted | Get deleted channels
[**teams_team_id_channels_get**](ChannelsApi.md#teams_team_id_channels_get) | **get** /teams/{team_id}/channels | Get public channels
[**teams_team_id_channels_ids_post**](ChannelsApi.md#teams_team_id_channels_ids_post) | **post** /teams/{team_id}/channels/ids | Get a list of channels by ids
[**teams_team_id_channels_name_channel_name_get**](ChannelsApi.md#teams_team_id_channels_name_channel_name_get) | **get** /teams/{team_id}/channels/name/{channel_name} | Get a channel by name
[**teams_team_id_channels_search_archived_post**](ChannelsApi.md#teams_team_id_channels_search_archived_post) | **post** /teams/{team_id}/channels/search_archived | Search archived channels
[**teams_team_id_channels_search_autocomplete_get**](ChannelsApi.md#teams_team_id_channels_search_autocomplete_get) | **get** /teams/{team_id}/channels/search_autocomplete | Autocomplete channels for search
[**teams_team_id_channels_search_post**](ChannelsApi.md#teams_team_id_channels_search_post) | **post** /teams/{team_id}/channels/search | Search channels
[**users_user_id_channels_channel_id_unread_get**](ChannelsApi.md#users_user_id_channels_channel_id_unread_get) | **get** /users/{user_id}/channels/{channel_id}/unread | Get unread messages
[**users_user_id_teams_team_id_channels_get**](ChannelsApi.md#users_user_id_teams_team_id_channels_get) | **get** /users/{user_id}/teams/{team_id}/channels | Get channels for user
[**users_user_id_teams_team_id_channels_members_get**](ChannelsApi.md#users_user_id_teams_team_id_channels_members_get) | **get** /users/{user_id}/teams/{team_id}/channels/members | Get channel memberships and roles for a user



## channels_channel_id_convert_post

> crate::models::Channel channels_channel_id_convert_post(channel_id)
Convert a channel from public to private

Will be deprecated in 6.0  Convert into private channel from the provided channel id string.  __Minimum server version__: 4.10  ##### Permissions `manage_team` permission for the team of the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_delete

> crate::models::StatusOk channels_channel_id_delete(channel_id)
Delete a channel

Soft deletes a channel, by marking the channel as deleted in the database. Soft deleted channels will not be accessible in the user interface. Direct and group message channels cannot be deleted. ##### Permissions `delete_public_channel` permission if the channel is public, `delete_private_channel` permission if the channel is private, or have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_get

> crate::models::Channel channels_channel_id_get(channel_id)
Get a channel

Get channel from the provided channel id string. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_member_counts_by_group_get

> channels_channel_id_member_counts_by_group_get(channel_id, include_timezones)
Channel members counts for each group that has atleast one member in the channel

Returns a set of ChannelMemberCountByGroup objects which contain a `group_id`, `channel_member_count` and a `channel_member_timezones_count`. ##### Permissions Must have `read_channel` permission for the given channel. __Minimum server version__: 5.24 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**include_timezones** | Option<**bool**> | Defines if member timezone counts should be returned or not |  |[default to false]

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_get

> Vec<crate::models::ChannelMember> channels_channel_id_members_get(channel_id, page, per_page)
Get channel members

Get a page of members for a channel. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of members per page. |  |[default to 60]

### Return type

[**Vec<crate::models::ChannelMember>**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_ids_post

> Vec<crate::models::ChannelMember> channels_channel_id_members_ids_post(channel_id, request_body)
Get channel members by ids

Get a list of channel members based on the provided user ids. ##### Permissions Must have the `read_channel` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**request_body** | [**Vec<String>**](String.md) | List of user ids | [required] |

### Return type

[**Vec<crate::models::ChannelMember>**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_minus_group_members_get

> channels_channel_id_members_minus_group_members_get(channel_id, group_ids, page, per_page)
Channel members minus group members.

Get the set of users who are members of the channel minus the set of users who are members of the given groups. Each user object contains an array of group objects representing the group memberships for that user. Each user object contains the boolean fields `scheme_guest`, `scheme_user`, and `scheme_admin` representing the roles that user has for the given channel.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**group_ids** | **String** | A comma-separated list of group ids. | [required] |[default to ]
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of users per page. |  |[default to 0]

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_post

> crate::models::ChannelMember channels_channel_id_members_post(channel_id, inline_object47)
Add user to channel

Add a user to a channel by creating a channel member object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The channel ID | [required] |
**inline_object47** | [**InlineObject47**](InlineObject47.md) |  | [required] |

### Return type

[**crate::models::ChannelMember**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_user_id_delete

> crate::models::StatusOk channels_channel_id_members_user_id_delete(channel_id, user_id)
Remove user from channel

Delete a channel member, effectively removing them from a channel.  In server version 5.3 and later, channel members can only be deleted from public or private channels. ##### Permissions `manage_public_channel_members` permission if the channel is public. `manage_private_channel_members` permission if the channel is private. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_user_id_get

> crate::models::ChannelMember channels_channel_id_members_user_id_get(channel_id, user_id)
Get channel member

Get a channel member. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::ChannelMember**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_user_id_notify_props_put

> crate::models::StatusOk channels_channel_id_members_user_id_notify_props_put(channel_id, user_id, channel_notify_props)
Update channel notifications

Update a user's notification properties for a channel. Only the provided fields are updated. ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**channel_notify_props** | [**ChannelNotifyProps**](ChannelNotifyProps.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_user_id_roles_put

> crate::models::StatusOk channels_channel_id_members_user_id_roles_put(channel_id, user_id, inline_object48)
Update channel roles

Update a user's roles for a channel. ##### Permissions Must have `manage_channel_roles` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**inline_object48** | [**InlineObject48**](InlineObject48.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_members_user_id_scheme_roles_put

> crate::models::StatusOk channels_channel_id_members_user_id_scheme_roles_put(channel_id, user_id, inline_object49)
Update the scheme-derived roles of a channel member.

Update a channel member's scheme_admin/scheme_user properties. Typically this should either be `scheme_admin=false, scheme_user=true` for ordinary channel member, or `scheme_admin=true, scheme_user=true` for a channel admin. __Minimum server version__: 5.0 ##### Permissions Must be authenticated and have the `manage_channel_roles` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**inline_object49** | [**InlineObject49**](InlineObject49.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_moderations_get

> Vec<crate::models::ChannelModeration> channels_channel_id_moderations_get(channel_id)
Get information about channel's moderation.

##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**Vec<crate::models::ChannelModeration>**](ChannelModeration.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_moderations_patch_put

> Vec<crate::models::ChannelModeration> channels_channel_id_moderations_patch_put(channel_id, channel_moderation_patch)
Get information about channel's moderation.

##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**channel_moderation_patch** | [**ChannelModerationPatch**](ChannelModerationPatch.md) |  | [required] |

### Return type

[**Vec<crate::models::ChannelModeration>**](ChannelModeration.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_move_post

> crate::models::Channel channels_channel_id_move_post(channel_id, inline_object44)
Move a channel

Move a channel to another team.  __Minimum server version__: 5.26  ##### Permissions  Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object44** | [**InlineObject44**](InlineObject44.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_patch_put

> crate::models::Channel channels_channel_id_patch_put(channel_id, inline_object42)
Patch a channel

Partially update a channel by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions If updating a public channel, `manage_public_channel_members` permission is required. If updating a private channel, `manage_private_channel_members` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object42** | [**InlineObject42**](InlineObject42.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_pinned_get

> crate::models::PostList channels_channel_id_pinned_get(channel_id)
Get a channel's pinned posts

Get a list of pinned posts for channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::PostList**](PostList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_privacy_put

> crate::models::Channel channels_channel_id_privacy_put(channel_id, inline_object43)
Update channel's privacy

Updates channel's privacy allowing changing a channel from Public to Private and back.  __Minimum server version__: 5.16  ##### Permissions `manage_team` permission for the team of the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object43** | [**InlineObject43**](InlineObject43.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_put

> crate::models::Channel channels_channel_id_put(channel_id, inline_object41)
Update a channel

Update a channel. The fields that can be updated are listed as parameters. Omitted fields will be treated as blanks. ##### Permissions If updating a public channel, `manage_public_channel_members` permission is required. If updating a private channel, `manage_private_channel_members` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object41** | [**InlineObject41**](InlineObject41.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_restore_post

> crate::models::Channel channels_channel_id_restore_post(channel_id)
Restore a channel

Restore channel from the provided channel id string.  __Minimum server version__: 3.10  ##### Permissions `manage_team` permission for the team of the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_scheme_put

> crate::models::StatusOk channels_channel_id_scheme_put(channel_id, inline_object51)
Set a channel's scheme

Set a channel's scheme, more specifically sets the scheme_id value of a channel record.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 4.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object51** | [**InlineObject51**](InlineObject51.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_stats_get

> crate::models::ChannelStats channels_channel_id_stats_get(channel_id)
Get channel statistics

Get statistics for a channel. ##### Permissions Must have the `read_channel` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::ChannelStats**](ChannelStats.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_channel_id_timezones_get

> Vec<String> channels_channel_id_timezones_get(channel_id)
Get timezones in a channel

Get a list of timezones for the users who are in this channel.  __Minimum server version__: 5.6  ##### Permissions Must have the `read_channel` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_direct_post

> crate::models::Channel channels_direct_post(request_body)
Create a direct message channel

Create a new direct message channel between two users. ##### Permissions Must be one of the two users and have `create_direct_channel` permission. Having the `manage_system` permission voids the previous requirements. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | The two user ids to be in the direct message | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_get

> Vec<crate::models::Channel> channels_get(not_associated_to_group, page, per_page, exclude_default_channels)
Get a list of all channels

##### Permissions `manage_system` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**not_associated_to_group** | Option<**String**> | Group GUID |  |
**page** | Option<**i32**> |  |  |[default to 0]
**per_page** | Option<**i32**> |  |  |[default to 0]
**exclude_default_channels** | Option<**bool**> | Whether to exclude default channels (ex Town Square, Off-Topic) from the results. |  |[default to false]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_group_post

> crate::models::Channel channels_group_post(request_body)
Create a group message channel

Create a new group message channel to group of users. If the logged in user's id is not included in the list, it will be appended to the end. ##### Permissions Must have `create_group_channel` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | User ids to be in the group message channel | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_group_search_post

> Vec<crate::models::Channel> channels_group_search_post(inline_object40)
Search Group Channels

Get a list of group channels for a user which members' usernames match the search term.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object40** | [**InlineObject40**](InlineObject40.md) |  | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_members_user_id_view_post

> crate::models::InlineResponse2008 channels_members_user_id_view_post(user_id, inline_object50)
View channel

Perform all the actions involved in viewing a channel. This includes marking channels as read, clearing push notifications, and updating the active channel. ##### Permissions Must be logged in as user or have `edit_other_users` permission.  __Response only includes `last_viewed_at_times` in Mattermost server 4.3 and newer.__ 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID to perform the view action for | [required] |
**inline_object50** | [**InlineObject50**](InlineObject50.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_post

> crate::models::Channel channels_post(inline_object38)
Create a channel

Create a new channel. ##### Permissions If creating a public channel, `create_public_channel` permission is required. If creating a private channel, `create_private_channel` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object38** | [**InlineObject38**](InlineObject38.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_search_post

> crate::models::InlineResponse2007 channels_search_post(inline_object39)
Search all private and open type channels across all teams

Returns all private and open type channels where 'term' matches on the name, display name, or purpose of the channel.  Configured 'default' channels (ex Town Square and Off-Topic) can be excluded from the results with the `exclude_default_channels` boolean parameter.  Channels that are associated (via GroupChannel records) to a given group can be excluded from the results with the `not_associated_to_group` parameter and a group id string. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object39** | [**InlineObject39**](InlineObject39.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_name_team_name_channels_name_channel_name_get

> crate::models::Channel teams_name_team_name_channels_name_channel_name_get(team_name, channel_name, include_deleted)
Get a channel by name and team name

Gets a channel from the provided team name and channel name strings. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | **String** | Team Name | [required] |
**channel_name** | **String** | Channel Name | [required] |
**include_deleted** | Option<**bool**> | Defines if deleted channels should be returned or not |  |[default to false]

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_channels_autocomplete_get

> Vec<crate::models::Channel> teams_team_id_channels_autocomplete_get(team_id, name)
Autocomplete channels

Autocomplete public channels on a team based on the search term provided in the request URL.  __Minimum server version__: 4.7  ##### Permissions Must have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**name** | **String** | Name or display name | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_channels_deleted_get

> Vec<crate::models::Channel> teams_team_id_channels_deleted_get(team_id, page, per_page)
Get deleted channels

Get a page of deleted channels on a team based on query string parameters - team_id, page and per_page.  __Minimum server version__: 3.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of public channels per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_channels_get

> Vec<crate::models::Channel> teams_team_id_channels_get(team_id, page, per_page)
Get public channels

Get a page of public channels on a team based on query string parameters - page and per_page. ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of public channels per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_channels_ids_post

> Vec<crate::models::Channel> teams_team_id_channels_ids_post(team_id, request_body)
Get a list of channels by ids

Get a list of public channels on a team by id. ##### Permissions `view_team` for the team the channels are on. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**request_body** | [**Vec<String>**](String.md) | List of channel ids | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_channels_name_channel_name_get

> crate::models::Channel teams_team_id_channels_name_channel_name_get(team_id, channel_name, include_deleted)
Get a channel by name

Gets channel from the provided team id and channel name strings. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**channel_name** | **String** | Channel Name | [required] |
**include_deleted** | Option<**bool**> | Defines if deleted channels should be returned or not |  |[default to false]

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_channels_search_archived_post

> Vec<crate::models::Channel> teams_team_id_channels_search_archived_post(team_id, inline_object46)
Search archived channels

Search archived channels on a team based on the search term provided in the request body.  __Minimum server version__: 5.18  ##### Permissions Must have the `list_team_channels` permission.  In server version 5.18 and later, a user without the `list_team_channels` permission will be able to use this endpoint, with the search results limited to the channels that the user is a member of. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object46** | [**InlineObject46**](InlineObject46.md) |  | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_channels_search_autocomplete_get

> Vec<crate::models::Channel> teams_team_id_channels_search_autocomplete_get(team_id, name)
Autocomplete channels for search

Autocomplete your channels on a team based on the search term provided in the request URL.  __Minimum server version__: 5.4  ##### Permissions Must have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**name** | **String** | Name or display name | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_channels_search_post

> Vec<crate::models::Channel> teams_team_id_channels_search_post(team_id, inline_object45)
Search channels

Search public channels on a team based on the search term provided in the request body. ##### Permissions Must have the `list_team_channels` permission.  In server version 5.16 and later, a user without the `list_team_channels` permission will be able to use this endpoint, with the search results limited to the channels that the user is a member of. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object45** | [**InlineObject45**](InlineObject45.md) |  | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_channels_channel_id_unread_get

> crate::models::ChannelUnread users_user_id_channels_channel_id_unread_get(user_id, channel_id)
Get unread messages

Get the total unread messages and mentions for a channel for a user. ##### Permissions Must be logged in as user and have the `read_channel` permission, or have `edit_other_usrs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::ChannelUnread**](ChannelUnread.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_teams_team_id_channels_get

> Vec<crate::models::Channel> users_user_id_teams_team_id_channels_get(user_id, team_id)
Get channels for user

Get all the channels on a team for a user. ##### Permissions Logged in as the user, or have `edit_other_users` permission, and `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_teams_team_id_channels_members_get

> Vec<crate::models::ChannelMember> users_user_id_teams_team_id_channels_members_get(user_id, team_id)
Get channel memberships and roles for a user

Get all channel memberships and associated membership roles (i.e. `channel_user`, `channel_admin`) for a user on a specific team. ##### Permissions Logged in as the user and `view_team` permission for the team. Having `manage_system` permission voids the previous requirements. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**Vec<crate::models::ChannelMember>**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

