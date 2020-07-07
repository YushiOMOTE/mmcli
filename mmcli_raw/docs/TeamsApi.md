# \TeamsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**teams_get**](TeamsApi.md#teams_get) | **get** /teams | Get teams
[**teams_invite_invite_id_get**](TeamsApi.md#teams_invite_invite_id_get) | **get** /teams/invite/{invite_id} | Get invite info for a team
[**teams_invites_email_delete**](TeamsApi.md#teams_invites_email_delete) | **delete** /teams/invites/email | Invalidate active email invitations
[**teams_members_invite_post**](TeamsApi.md#teams_members_invite_post) | **post** /teams/members/invite | Add user to team from invite
[**teams_name_name_exists_get**](TeamsApi.md#teams_name_name_exists_get) | **get** /teams/name/{name}/exists | Check if team exists
[**teams_name_name_get**](TeamsApi.md#teams_name_name_get) | **get** /teams/name/{name} | Get a team by name
[**teams_post**](TeamsApi.md#teams_post) | **post** /teams | Create a team
[**teams_search_post**](TeamsApi.md#teams_search_post) | **post** /teams/search | Search teams
[**teams_team_id_delete**](TeamsApi.md#teams_team_id_delete) | **delete** /teams/{team_id} | Delete a team
[**teams_team_id_get**](TeamsApi.md#teams_team_id_get) | **get** /teams/{team_id} | Get a team
[**teams_team_id_image_delete**](TeamsApi.md#teams_team_id_image_delete) | **delete** /teams/{team_id}/image | Remove the team icon
[**teams_team_id_image_get**](TeamsApi.md#teams_team_id_image_get) | **get** /teams/{team_id}/image | Get the team icon
[**teams_team_id_image_post**](TeamsApi.md#teams_team_id_image_post) | **post** /teams/{team_id}/image | Sets the team icon
[**teams_team_id_import_post**](TeamsApi.md#teams_team_id_import_post) | **post** /teams/{team_id}/import | Import a Team from other application
[**teams_team_id_invite_email_post**](TeamsApi.md#teams_team_id_invite_email_post) | **post** /teams/{team_id}/invite/email | Invite users to the team by email
[**teams_team_id_invite_guests_email_post**](TeamsApi.md#teams_team_id_invite_guests_email_post) | **post** /teams/{team_id}/invite-guests/email | Invite guests to the team by email
[**teams_team_id_members_batch_post**](TeamsApi.md#teams_team_id_members_batch_post) | **post** /teams/{team_id}/members/batch | Add multiple users to team
[**teams_team_id_members_get**](TeamsApi.md#teams_team_id_members_get) | **get** /teams/{team_id}/members | Get team members
[**teams_team_id_members_ids_post**](TeamsApi.md#teams_team_id_members_ids_post) | **post** /teams/{team_id}/members/ids | Get team members by ids
[**teams_team_id_members_minus_group_members_get**](TeamsApi.md#teams_team_id_members_minus_group_members_get) | **get** /teams/{team_id}/members_minus_group_members | Team members minus group members.
[**teams_team_id_members_post**](TeamsApi.md#teams_team_id_members_post) | **post** /teams/{team_id}/members | Add user to team
[**teams_team_id_members_user_id_delete**](TeamsApi.md#teams_team_id_members_user_id_delete) | **delete** /teams/{team_id}/members/{user_id} | Remove user from team
[**teams_team_id_members_user_id_get**](TeamsApi.md#teams_team_id_members_user_id_get) | **get** /teams/{team_id}/members/{user_id} | Get a team member
[**teams_team_id_members_user_id_roles_put**](TeamsApi.md#teams_team_id_members_user_id_roles_put) | **put** /teams/{team_id}/members/{user_id}/roles | Update a team member roles
[**teams_team_id_members_user_id_scheme_roles_put**](TeamsApi.md#teams_team_id_members_user_id_scheme_roles_put) | **put** /teams/{team_id}/members/{user_id}/schemeRoles | Update the scheme-derived roles of a team member.
[**teams_team_id_patch_put**](TeamsApi.md#teams_team_id_patch_put) | **put** /teams/{team_id}/patch | Patch a team
[**teams_team_id_privacy_put**](TeamsApi.md#teams_team_id_privacy_put) | **put** /teams/{team_id}/privacy | Update teams's privacy
[**teams_team_id_put**](TeamsApi.md#teams_team_id_put) | **put** /teams/{team_id} | Update a team
[**teams_team_id_regenerate_invite_id_post**](TeamsApi.md#teams_team_id_regenerate_invite_id_post) | **post** /teams/{team_id}/regenerate_invite_id | Regenerate the Invite ID from a Team
[**teams_team_id_restore_post**](TeamsApi.md#teams_team_id_restore_post) | **post** /teams/{team_id}/restore | Restore a team
[**teams_team_id_scheme_put**](TeamsApi.md#teams_team_id_scheme_put) | **put** /teams/{team_id}/scheme | Set a team's scheme
[**teams_team_id_stats_get**](TeamsApi.md#teams_team_id_stats_get) | **get** /teams/{team_id}/stats | Get a team stats
[**users_user_id_teams_get**](TeamsApi.md#users_user_id_teams_get) | **get** /users/{user_id}/teams | Get a user's teams
[**users_user_id_teams_members_get**](TeamsApi.md#users_user_id_teams_members_get) | **get** /users/{user_id}/teams/members | Get team members for a user
[**users_user_id_teams_team_id_unread_get**](TeamsApi.md#users_user_id_teams_team_id_unread_get) | **get** /users/{user_id}/teams/{team_id}/unread | Get unreads for a team
[**users_user_id_teams_unread_get**](TeamsApi.md#users_user_id_teams_unread_get) | **get** /users/{user_id}/teams/unread | Get team unreads for a user



## teams_get

> Vec<crate::models::Team> teams_get(page, per_page, include_total_count)
Get teams

For regular users only returns open teams. Users with the \"manage_system\" permission will return teams regardless of type. The result is based on query string parameters - page and per_page. ##### Permissions Must be authenticated. \"manage_system\" permission is required to show all teams. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of teams per page. |  |[default to 60]
**include_total_count** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_invite_invite_id_get

> crate::models::InlineResponse2006 teams_invite_invite_id_get(invite_id)
Get invite info for a team

Get the `name`, `display_name`, `description` and `id` for a team from the invite id.  __Minimum server version__: 4.0  ##### Permissions No authentication required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_id** | **String** | Invite id for a team | [required] |

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_invites_email_delete

> crate::models::StatusOk teams_invites_email_delete()
Invalidate active email invitations

Invalidate active email invitations that have not been accepted by the user. ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_members_invite_post

> crate::models::TeamMember teams_members_invite_post(token)
Add user to team from invite

Using either an invite id or hash/data pair from an email invite link, add a user to a team. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Token id from the invitation | [required] |

### Return type

[**crate::models::TeamMember**](TeamMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_name_name_exists_get

> crate::models::TeamExists teams_name_name_exists_get(name)
Check if team exists

Check if the team exists based on a team name. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Team Name | [required] |

### Return type

[**crate::models::TeamExists**](TeamExists.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_name_name_get

> crate::models::Team teams_name_name_get(name)
Get a team by name

Get a team based on provided name string ##### Permissions Must be authenticated, team type is open and have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Team Name | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_post

> crate::models::Team teams_post(inline_object26)
Create a team

Create a new team on the system. ##### Permissions Must be authenticated and have the `create_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object26** | [**InlineObject26**](InlineObject26.md) |  | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_search_post

> crate::models::InlineResponse2004 teams_search_post(inline_object30)
Search teams

Search teams based on search term provided in the request body. ##### Permissions Logged in user only shows open teams Logged in user with \"manage_system\" permission shows all teams 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object30** | [**InlineObject30**](InlineObject30.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_delete

> crate::models::StatusOk teams_team_id_delete(team_id, permanent)
Delete a team

Soft deletes a team, by marking the team as deleted in the database. Soft deleted teams will not be accessible in the user interface.  Optionally use the permanent query parameter to hard delete the team for compliance reasons. As of server version 5.0, to use this feature `ServiceSettings.EnableAPITeamDeletion` must be set to `true` in the server's configuration. ##### Permissions Must have the `manage_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**permanent** | Option<**bool**> | Permanently delete the team, to be used for compliance reasons only. As of server version 5.0, `ServiceSettings.EnableAPITeamDeletion` must be set to `true` in the server's configuration. |  |[default to false]

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_get

> crate::models::Team teams_team_id_get(team_id)
Get a team

Get a team on the system. ##### Permissions Must be authenticated and have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_image_delete

> crate::models::StatusOk teams_team_id_image_delete(team_id)
Remove the team icon

Remove the team icon for the team.  __Minimum server version__: 4.10  ##### Permissions Must be authenticated and have the `manage_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_image_get

> teams_team_id_image_get(team_id)
Get the team icon

Get the team icon of the team.  __Minimum server version__: 4.9  ##### Permissions User must be authenticated. In addition, team must be open or the user must have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_image_post

> crate::models::StatusOk teams_team_id_image_post(team_id, image)
Sets the team icon

Sets the team icon for the team.  __Minimum server version__: 4.9  ##### Permissions Must be authenticated and have the `manage_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**image** | **std::path::PathBuf** | The image to be uploaded | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_import_post

> crate::models::InlineResponse2005 teams_team_id_import_post(team_id, file, filesize, import_from)
Import a Team from other application

Import a team into a existing team. Import users, channels, posts, hooks. ##### Permissions Must have `permission_import_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**file** | **std::path::PathBuf** | A file to be uploaded in zip format. | [required] |
**filesize** | **i64** | The size of the zip file to be imported. | [required] |
**import_from** | **String** | String that defines from which application the team was exported to be imported into Mattermost. | [required] |

### Return type

[**crate::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_invite_email_post

> crate::models::StatusOk teams_team_id_invite_email_post(team_id, request_body)
Invite users to the team by email

Invite users to the existing team usign the user's email. ##### Permissions Must have `invite_to_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**request_body** | [**Vec<String>**](String.md) | List of user's email | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_invite_guests_email_post

> crate::models::StatusOk teams_team_id_invite_guests_email_post(team_id, inline_object35)
Invite guests to the team by email

Invite guests to existing team channels usign the user's email.  __Minimum server version__: 5.16  ##### Permissions Must have `invite_guest` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object35** | [**InlineObject35**](InlineObject35.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_batch_post

> Vec<crate::models::TeamMember> teams_team_id_members_batch_post(team_id, team_member, graceful)
Add multiple users to team

Add a number of users to the team by user_id. ##### Permissions Must be authenticated. Authenticated user must have the `add_user_to_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**team_member** | [**Vec<crate::models::TeamMember>**](TeamMember.md) |  | [required] |
**graceful** | Option<**bool**> | Instead of aborting the operation if a user cannot be added, return an arrray that will contain both the success and added members and the ones with error, in form of `[{\"member\": {...}, \"user_id\", \"...\", \"error\": {...}}]` |  |

### Return type

[**Vec<crate::models::TeamMember>**](TeamMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_get

> Vec<crate::models::TeamMember> teams_team_id_members_get(team_id, page, per_page)
Get team members

Get a page team members list based on query string parameters - team id, page and per page. ##### Permissions Must be authenticated and have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of users per page. |  |[default to 60]

### Return type

[**Vec<crate::models::TeamMember>**](TeamMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_ids_post

> Vec<crate::models::TeamMember> teams_team_id_members_ids_post(team_id, request_body)
Get team members by ids

Get a list of team members based on a provided array of user ids. ##### Permissions Must have `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**request_body** | [**Vec<String>**](String.md) | List of user ids | [required] |

### Return type

[**Vec<crate::models::TeamMember>**](TeamMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_minus_group_members_get

> teams_team_id_members_minus_group_members_get(team_id, group_ids, page, per_page)
Team members minus group members.

Get the set of users who are members of the team minus the set of users who are members of the given groups. Each user object contains an array of group objects representing the group memberships for that user. Each user object contains the boolean fields `scheme_guest`, `scheme_user`, and `scheme_admin` representing the roles that user has for the given team.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**group_ids** | **String** | A comma-separated list of group ids. | [required] |[default to ]
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of users per page. |  |[default to 0]

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_post

> crate::models::TeamMember teams_team_id_members_post(team_id, inline_object31)
Add user to team

Add user to the team by user_id. ##### Permissions Must be authenticated and team be open to add self. For adding another user, authenticated user must have the `add_user_to_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object31** | [**InlineObject31**](InlineObject31.md) |  | [required] |

### Return type

[**crate::models::TeamMember**](TeamMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_user_id_delete

> crate::models::StatusOk teams_team_id_members_user_id_delete(team_id, user_id)
Remove user from team

Delete the team member object for a user, effectively removing them from a team. ##### Permissions Must be logged in as the user or have the `remove_user_from_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_user_id_get

> crate::models::TeamMember teams_team_id_members_user_id_get(team_id, user_id)
Get a team member

Get a team member on the system. ##### Permissions Must be authenticated and have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::TeamMember**](TeamMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_user_id_roles_put

> crate::models::StatusOk teams_team_id_members_user_id_roles_put(team_id, user_id, inline_object33)
Update a team member roles

Update a team member roles. Valid team roles are \"team_user\", \"team_admin\" or both of them. Overwrites any previously assigned team roles. ##### Permissions Must be authenticated and have the `manage_team_roles` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**inline_object33** | [**InlineObject33**](InlineObject33.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_members_user_id_scheme_roles_put

> crate::models::StatusOk teams_team_id_members_user_id_scheme_roles_put(team_id, user_id, inline_object34)
Update the scheme-derived roles of a team member.

Update a team member's scheme_admin/scheme_user properties. Typically this should either be `scheme_admin=false, scheme_user=true` for ordinary team member, or `scheme_admin=true, scheme_user=true` for a team admin.  __Minimum server version__: 5.0  ##### Permissions Must be authenticated and have the `manage_team_roles` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**inline_object34** | [**InlineObject34**](InlineObject34.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_patch_put

> crate::models::Team teams_team_id_patch_put(team_id, inline_object28)
Patch a team

Partially update a team by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions Must have the `manage_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object28** | [**InlineObject28**](InlineObject28.md) |  | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_privacy_put

> crate::models::Team teams_team_id_privacy_put(team_id, inline_object29)
Update teams's privacy

Updates team's privacy allowing changing a team from Public (open) to Private (invitation only) and back.  __Minimum server version__: 5.24  ##### Permissions `manage_team` permission for the team of the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object29** | [**InlineObject29**](InlineObject29.md) |  | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_put

> crate::models::Team teams_team_id_put(team_id, inline_object27)
Update a team

Update a team by providing the team object. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions Must have the `manage_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object27** | [**InlineObject27**](InlineObject27.md) |  | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_regenerate_invite_id_post

> crate::models::Team teams_team_id_regenerate_invite_id_post(team_id)
Regenerate the Invite ID from a Team

Regenerates the invite ID used in invite links of a team ##### Permissions Must be authenticated and have the `manage_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_restore_post

> crate::models::Team teams_team_id_restore_post(team_id)
Restore a team

Restore a team that was previously soft deleted.  __Minimum server version__: 5.24  ##### Permissions Must have the `manage_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_scheme_put

> crate::models::StatusOk teams_team_id_scheme_put(team_id, inline_object37)
Set a team's scheme

Set a team's scheme, more specifically sets the scheme_id value of a team record.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object37** | [**InlineObject37**](InlineObject37.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_stats_get

> crate::models::TeamStats teams_team_id_stats_get(team_id)
Get a team stats

Get a team stats on the system. ##### Permissions Must be authenticated and have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::TeamStats**](TeamStats.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_teams_get

> Vec<crate::models::Team> users_user_id_teams_get(user_id)
Get a user's teams

Get a list of teams that a user is on. ##### Permissions Must be authenticated as the user or have the `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_teams_members_get

> Vec<crate::models::TeamMember> users_user_id_teams_members_get(user_id)
Get team members for a user

Get a list of team members for a user. Useful for getting the ids of teams the user is on and the roles they have in those teams. ##### Permissions Must be logged in as the user or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**Vec<crate::models::TeamMember>**](TeamMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_teams_team_id_unread_get

> crate::models::TeamUnread users_user_id_teams_team_id_unread_get(user_id, team_id)
Get unreads for a team

Get the unread mention and message counts for a team for the specified user. ##### Permissions Must be the user or have `edit_other_users` permission and have `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::TeamUnread**](TeamUnread.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_teams_unread_get

> Vec<crate::models::TeamUnread> users_user_id_teams_unread_get(user_id, exclude_team)
Get team unreads for a user

Get the count for unread messages and mentions in the teams the user is a member of. ##### Permissions Must be logged in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**exclude_team** | **String** | Optional team id to be excluded from the results | [required] |

### Return type

[**Vec<crate::models::TeamUnread>**](TeamUnread.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

