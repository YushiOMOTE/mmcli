# \UsersApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bots_bot_user_id_convert_to_user_post**](UsersApi.md#bots_bot_user_id_convert_to_user_post) | **post** /bots/{bot_user_id}/convert_to_user | Convert a bot into a user
[**users_autocomplete_get**](UsersApi.md#users_autocomplete_get) | **get** /users/autocomplete | Autocomplete users
[**users_delete**](UsersApi.md#users_delete) | **delete** /users | Permanent delete all users
[**users_email_email_get**](UsersApi.md#users_email_email_get) | **get** /users/email/{email} | Get a user by email
[**users_email_verify_post**](UsersApi.md#users_email_verify_post) | **post** /users/email/verify | Verify user email
[**users_email_verify_send_post**](UsersApi.md#users_email_verify_send_post) | **post** /users/email/verify/send | Send verification email
[**users_get**](UsersApi.md#users_get) | **get** /users | Get users
[**users_group_channels_post**](UsersApi.md#users_group_channels_post) | **post** /users/group_channels | Get users by group channels ids
[**users_ids_post**](UsersApi.md#users_ids_post) | **post** /users/ids | Get users by ids
[**users_known_get**](UsersApi.md#users_known_get) | **get** /users/known | Get user IDs of known users
[**users_login_post**](UsersApi.md#users_login_post) | **post** /users/login | Login to Mattermost server
[**users_login_switch_post**](UsersApi.md#users_login_switch_post) | **post** /users/login/switch | Switch login method
[**users_logout_post**](UsersApi.md#users_logout_post) | **post** /users/logout | Logout from the Mattermost server
[**users_mfa_post**](UsersApi.md#users_mfa_post) | **post** /users/mfa | Check MFA
[**users_password_reset_post**](UsersApi.md#users_password_reset_post) | **post** /users/password/reset | Reset password
[**users_password_reset_send_post**](UsersApi.md#users_password_reset_send_post) | **post** /users/password/reset/send | Send password reset email
[**users_post**](UsersApi.md#users_post) | **post** /users | Create a user
[**users_search_post**](UsersApi.md#users_search_post) | **post** /users/search | Search users
[**users_sessions_device_put**](UsersApi.md#users_sessions_device_put) | **put** /users/sessions/device | Attach mobile device
[**users_sessions_revoke_all_post**](UsersApi.md#users_sessions_revoke_all_post) | **post** /users/sessions/revoke/all | Revoke all sessions from all users.
[**users_stats_get**](UsersApi.md#users_stats_get) | **get** /users/stats | Get total count of users in the system
[**users_tokens_disable_post**](UsersApi.md#users_tokens_disable_post) | **post** /users/tokens/disable | Disable personal access token
[**users_tokens_enable_post**](UsersApi.md#users_tokens_enable_post) | **post** /users/tokens/enable | Enable personal access token
[**users_tokens_get**](UsersApi.md#users_tokens_get) | **get** /users/tokens | Get user access tokens
[**users_tokens_revoke_post**](UsersApi.md#users_tokens_revoke_post) | **post** /users/tokens/revoke | Revoke a user access token
[**users_tokens_search_post**](UsersApi.md#users_tokens_search_post) | **post** /users/tokens/search | Search tokens
[**users_tokens_token_id_get**](UsersApi.md#users_tokens_token_id_get) | **get** /users/tokens/{token_id} | Get a user access token
[**users_user_id_active_put**](UsersApi.md#users_user_id_active_put) | **put** /users/{user_id}/active | Update user active status
[**users_user_id_audits_get**](UsersApi.md#users_user_id_audits_get) | **get** /users/{user_id}/audits | Get user's audits
[**users_user_id_auth_put**](UsersApi.md#users_user_id_auth_put) | **put** /users/{user_id}/auth | Update a user's authentication method
[**users_user_id_convert_to_bot_post**](UsersApi.md#users_user_id_convert_to_bot_post) | **post** /users/{user_id}/convert_to_bot | Convert a user into a bot
[**users_user_id_delete**](UsersApi.md#users_user_id_delete) | **delete** /users/{user_id} | Deactivate a user account.
[**users_user_id_demote_post**](UsersApi.md#users_user_id_demote_post) | **post** /users/{user_id}/demote | Demote a user to a guest
[**users_user_id_email_verify_member_post**](UsersApi.md#users_user_id_email_verify_member_post) | **post** /users/{user_id}/email/verify/member | Verify user email by ID
[**users_user_id_get**](UsersApi.md#users_user_id_get) | **get** /users/{user_id} | Get a user
[**users_user_id_image_default_get**](UsersApi.md#users_user_id_image_default_get) | **get** /users/{user_id}/image/default | Return user's default (generated) profile image
[**users_user_id_image_delete**](UsersApi.md#users_user_id_image_delete) | **delete** /users/{user_id}/image | Delete user's profile image
[**users_user_id_image_get**](UsersApi.md#users_user_id_image_get) | **get** /users/{user_id}/image | Get user's profile image
[**users_user_id_image_post**](UsersApi.md#users_user_id_image_post) | **post** /users/{user_id}/image | Set user's profile image
[**users_user_id_mfa_generate_post**](UsersApi.md#users_user_id_mfa_generate_post) | **post** /users/{user_id}/mfa/generate | Generate MFA secret
[**users_user_id_mfa_put**](UsersApi.md#users_user_id_mfa_put) | **put** /users/{user_id}/mfa | Update a user's MFA
[**users_user_id_password_put**](UsersApi.md#users_user_id_password_put) | **put** /users/{user_id}/password | Update a user's password
[**users_user_id_patch_put**](UsersApi.md#users_user_id_patch_put) | **put** /users/{user_id}/patch | Patch a user
[**users_user_id_promote_post**](UsersApi.md#users_user_id_promote_post) | **post** /users/{user_id}/promote | Promote a guest to user
[**users_user_id_put**](UsersApi.md#users_user_id_put) | **put** /users/{user_id} | Update a user
[**users_user_id_roles_put**](UsersApi.md#users_user_id_roles_put) | **put** /users/{user_id}/roles | Update a user's roles
[**users_user_id_sessions_get**](UsersApi.md#users_user_id_sessions_get) | **get** /users/{user_id}/sessions | Get user's sessions
[**users_user_id_sessions_revoke_all_post**](UsersApi.md#users_user_id_sessions_revoke_all_post) | **post** /users/{user_id}/sessions/revoke/all | Revoke all active sessions for a user
[**users_user_id_sessions_revoke_post**](UsersApi.md#users_user_id_sessions_revoke_post) | **post** /users/{user_id}/sessions/revoke | Revoke a user session
[**users_user_id_terms_of_service_get**](UsersApi.md#users_user_id_terms_of_service_get) | **get** /users/{user_id}/terms_of_service | Fetches user's latest terms of service action if the latest action was for acceptance.
[**users_user_id_terms_of_service_post**](UsersApi.md#users_user_id_terms_of_service_post) | **post** /users/{user_id}/terms_of_service | Records user action when they accept or decline custom terms of service
[**users_user_id_tokens_get**](UsersApi.md#users_user_id_tokens_get) | **get** /users/{user_id}/tokens | Get user access tokens
[**users_user_id_tokens_post**](UsersApi.md#users_user_id_tokens_post) | **post** /users/{user_id}/tokens | Create a user access token
[**users_user_id_typing_post**](UsersApi.md#users_user_id_typing_post) | **post** /users/{user_id}/typing | Publish a user typing websocket event.
[**users_username_username_get**](UsersApi.md#users_username_username_get) | **get** /users/username/{username} | Get a user by username
[**users_usernames_post**](UsersApi.md#users_usernames_post) | **post** /users/usernames | Get users by usernames



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


## users_autocomplete_get

> crate::models::UserAutocomplete users_autocomplete_get(name, team_id, channel_id, limit)
Autocomplete users

Get a list of users for the purpose of autocompleting based on the provided search term. Specify a combination of `team_id` and `channel_id` to filter results further. ##### Permissions Requires an active session and `view_team` and `read_channel` on any teams or channels used to filter the results further. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Username, nickname first name or last name | [required] |
**team_id** | Option<**String**> | Team ID |  |
**channel_id** | Option<**String**> | Channel ID |  |
**limit** | Option<**i64**> | The maximum number of users to return in each subresult  __Available as of server version 5.6. Defaults to `100` if not provided or on an earlier server version.__  |  |[default to 100]

### Return type

[**crate::models::UserAutocomplete**](UserAutocomplete.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_delete

> users_delete()
Permanent delete all users

Permanently deletes all users and all their related information, including posts.  __Minimum server version__: 5.26.0  __Local mode only__: This endpoint is only available through [local mode](https://docs.mattermost.com/administration/mmctl-cli-tool.html#local-mode). 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_email_email_get

> crate::models::User users_email_email_get(email)
Get a user by email

Get a user object by providing a user email. Sensitive information will be sanitized out. ##### Permissions Requires an active session and for the current session to be able to view another user's email based on the server's privacy settings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | User Email | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_email_verify_post

> crate::models::StatusOk users_email_verify_post(inline_object15)
Verify user email

Verify the email used by a user to sign-up their account with. ##### Permissions No permissions required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object15** | [**InlineObject15**](InlineObject15.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_email_verify_send_post

> crate::models::StatusOk users_email_verify_send_post(inline_object16)
Send verification email

Send an email with a verification link to a user that has an email matching the one in the request body. This endpoint will return success even if the email does not match any users on the system. ##### Permissions No permissions required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object16** | [**InlineObject16**](InlineObject16.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get

> Vec<crate::models::User> users_get(page, per_page, in_team, not_in_team, in_channel, not_in_channel, in_group, group_constrained, without_team, active, inactive, role, sort)
Get users

Get a page of a list of users. Based on query string parameters, select users from a team, channel, or select users not in a specific channel.  Since server version 4.0, some basic sorting is available using the `sort` query parameter. Sorting is currently only supported when selecting users on a team. ##### Permissions Requires an active session and (if specified) membership to the channel or team being selected from. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of users per page. There is a maximum limit of 200 users per page. |  |[default to 60]
**in_team** | Option<**String**> | The ID of the team to get users for. |  |
**not_in_team** | Option<**String**> | The ID of the team to exclude users for. Must not be used with \"in_team\" query parameter. |  |
**in_channel** | Option<**String**> | The ID of the channel to get users for. |  |
**not_in_channel** | Option<**String**> | The ID of the channel to exclude users for. Must be used with \"in_channel\" query parameter. |  |
**in_group** | Option<**String**> | The ID of the group to get users for. Must have `manage_system` permission. |  |
**group_constrained** | Option<**bool**> | When used with `not_in_channel` or `not_in_team`, returns only the users that are allowed to join the channel or team based on its group constrains. |  |
**without_team** | Option<**bool**> | Whether or not to list users that are not on any team. This option takes precendence over `in_team`, `in_channel`, and `not_in_channel`. |  |
**active** | Option<**bool**> | Whether or not to list only users that are active. This option cannot be used along with the `inactive` option. |  |
**inactive** | Option<**bool**> | Whether or not to list only users that are deactivated. This option cannot be used along with the `active` option. |  |
**role** | Option<**String**> | Returns users that have this role. |  |
**sort** | Option<**String**> | Sort is only available in conjunction with certain options below. The paging parameter is also always available.  ##### `in_team` Can be \"\", \"last_activity_at\" or \"create_at\". When left blank, sorting is done by username. __Minimum server version__: 4.0 ##### `in_channel` Can be \"\", \"status\". When left blank, sorting is done by username. `status` will sort by User's current status (Online, Away, DND, Offline), then by Username. __Minimum server version__: 4.7  |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_group_channels_post

> crate::models::InlineResponse200 users_group_channels_post(request_body)
Get users by group channels ids

Get an object containing a key per group channel id in the query and its value as a list of users members of that group channel.  The user must be a member of the group ids in the query, or they will be omitted from the response. ##### Permissions Requires an active session but no other permissions.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | List of group channel ids | [required] |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_ids_post

> Vec<crate::models::User> users_ids_post(request_body, since)
Get users by ids

Get a list of users based on a provided list of user ids. ##### Permissions Requires an active session but no other permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | List of user ids | [required] |
**since** | Option<**i64**> | Only return users that have been modified since the given Unix timestamp (in milliseconds).  __Minimum server version__: 5.14  |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_known_get

> crate::models::UsersStats users_known_get()
Get user IDs of known users

Get the list of user IDs of users with any direct relationship with a user. That means any user sharing any channel, including direct and group channels. ##### Permissions Must be authenticated.  __Minimum server version__: 5.23 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UsersStats**](UsersStats.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_login_post

> crate::models::User users_login_post(inline_object)
Login to Mattermost server

##### Permissions No permission required 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_login_switch_post

> crate::models::InlineResponse2003 users_login_switch_post(inline_object17)
Switch login method

Switch a user's login method from using email to OAuth2/SAML/LDAP or back to email. When switching to OAuth2/SAML, account switching is not complete until the user follows the returned link and completes any steps on the OAuth2/SAML service provider.  To switch from email to OAuth2/SAML, specify `current_service`, `new_service`, `email` and `password`.  To switch from OAuth2/SAML to email, specify `current_service`, `new_service`, `email` and `new_password`.  To switch from email to LDAP/AD, specify `current_service`, `new_service`, `email`, `password`, `ldap_ip` and `new_password` (this is the user's LDAP password).  To switch from LDAP/AD to email, specify `current_service`, `new_service`, `ldap_ip`, `password` (this is the user's LDAP password), `email`  and `new_password`.  Additionally, specify `mfa_code` when trying to switch an account on LDAP/AD or email that has MFA activated.  ##### Permissions No current authentication required except when switching from OAuth2/SAML to email. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object17** | [**InlineObject17**](InlineObject17.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_logout_post

> crate::models::StatusOk users_logout_post()
Logout from the Mattermost server

##### Permissions An active session is required 

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


## users_mfa_post

> crate::models::InlineResponse2002 users_mfa_post(inline_object10)
Check MFA

Check if a user has multi-factor authentication active on their account by providing a login id. Used to check whether an MFA code needs to be provided when logging in. ##### Permissions No permission required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object10** | [**InlineObject10**](InlineObject10.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_password_reset_post

> crate::models::StatusOk users_password_reset_post(inline_object8)
Reset password

Update the password for a user using a one-use, timed recovery code tied to the user's account. Only works for non-SSO users. ##### Permissions No permissions required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object8** | [**InlineObject8**](InlineObject8.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_password_reset_send_post

> crate::models::StatusOk users_password_reset_send_post(inline_object12)
Send password reset email

Send an email containing a link for resetting the user's password. The link will contain a one-use, timed recovery code tied to the user's account. Only works for non-SSO users. ##### Permissions No permissions required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object12** | [**InlineObject12**](InlineObject12.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_post

> crate::models::User users_post(inline_object1, t, iid)
Create a user

Create a new user on the system. Password is required for email login. For other authentication types such as LDAP or SAML, auth_data and auth_service fields are required. ##### Permissions No permission required but user creation can be controlled by server configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object1** | [**InlineObject1**](InlineObject1.md) |  | [required] |
**t** | Option<**String**> | Token id from an email invitation |  |
**iid** | Option<**String**> | Token id from an invitation link |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_search_post

> Vec<crate::models::User> users_search_post(inline_object2)
Search users

Get a list of users based on search criteria provided in the request body. Searches are typically done against username, full name, nickname and email unless otherwise configured by the server. ##### Permissions Requires an active session and `read_channel` and/or `view_team` permissions for any channels or teams specified in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object2** | [**InlineObject2**](InlineObject2.md) |  | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_sessions_device_put

> crate::models::StatusOk users_sessions_device_put(inline_object14)
Attach mobile device

Attach a mobile device id to the currently logged in session. This will enable push notifications for a user, if configured by the server. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object14** | [**InlineObject14**](InlineObject14.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_sessions_revoke_all_post

> users_sessions_revoke_all_post()
Revoke all sessions from all users.

For any session currently on the server (including admin) it will be revoked. Clients will be notified to log out users.  __Minimum server version__: 5.14  ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_stats_get

> crate::models::UsersStats users_stats_get()
Get total count of users in the system

Get a total count of users in the system. ##### Permissions Must be authenticated. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UsersStats**](UsersStats.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_disable_post

> crate::models::StatusOk users_tokens_disable_post(inline_object20)
Disable personal access token

Disable a personal access token and delete any sessions using the token. The token can be re-enabled using `/users/tokens/enable`.  __Minimum server version__: 4.4  ##### Permissions Must have `revoke_user_access_token` permission. For non-self requests, must also have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object20** | [**InlineObject20**](InlineObject20.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_enable_post

> crate::models::StatusOk users_tokens_enable_post(inline_object21)
Enable personal access token

Re-enable a personal access token that has been disabled.  __Minimum server version__: 4.4  ##### Permissions Must have `create_user_access_token` permission. For non-self requests, must also have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object21** | [**InlineObject21**](InlineObject21.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_get

> Vec<crate::models::UserAccessTokenSanitized> users_tokens_get(page, per_page)
Get user access tokens

Get a page of user access tokens for users on the system. Does not include the actual authentication tokens. Use query parameters for paging.  __Minimum server version__: 4.7  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of tokens per page. |  |[default to 60]

### Return type

[**Vec<crate::models::UserAccessTokenSanitized>**](UserAccessTokenSanitized.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_revoke_post

> crate::models::StatusOk users_tokens_revoke_post(inline_object19)
Revoke a user access token

Revoke a user access token and delete any sessions using the token.  __Minimum server version__: 4.1  ##### Permissions Must have `revoke_user_access_token` permission. For non-self requests, must also have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object19** | [**InlineObject19**](InlineObject19.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_search_post

> Vec<crate::models::UserAccessTokenSanitized> users_tokens_search_post(inline_object22)
Search tokens

Get a list of tokens based on search criteria provided in the request body. Searches are done against the token id, user id and username.  __Minimum server version__: 4.7  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object22** | [**InlineObject22**](InlineObject22.md) |  | [required] |

### Return type

[**Vec<crate::models::UserAccessTokenSanitized>**](UserAccessTokenSanitized.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_token_id_get

> crate::models::UserAccessTokenSanitized users_tokens_token_id_get(token_id)
Get a user access token

Get a user access token. Does not include the actual authentication token.  __Minimum server version__: 4.1  ##### Permissions Must have `read_user_access_token` permission. For non-self requests, must also have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | User access token GUID | [required] |

### Return type

[**crate::models::UserAccessTokenSanitized**](UserAccessTokenSanitized.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_active_put

> crate::models::StatusOk users_user_id_active_put(user_id, inline_object6)
Update user active status

Update user active or inactive status.  __Since server version 4.6, users using a SSO provider to login can be activated or deactivated with this endpoint. However, if their activation status in Mattermost does not reflect their status in the SSO provider, the next synchronization or login by that user will reset the activation status to that of their account in the SSO provider. Server versions 4.5 and before do not allow activation or deactivation of SSO users from this endpoint.__ ##### Permissions User can deactivate themselves. User with `manage_system` permission can activate or deactivate a user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object6** | [**InlineObject6**](InlineObject6.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_audits_get

> Vec<crate::models::Audit> users_user_id_audits_get(user_id)
Get user's audits

Get a list of audit by providing the user GUID. ##### Permissions Must be logged in as the user or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**Vec<crate::models::Audit>**](Audit.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_auth_put

> crate::models::UserAuthData users_user_id_auth_put(user_id, user_auth_data)
Update a user's authentication method

Updates a user's authentication method. This can be used to change them to/from LDAP authentication for example.  __Minimum server version__: 4.6 ##### Permissions Must have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**user_auth_data** | [**UserAuthData**](UserAuthData.md) |  | [required] |

### Return type

[**crate::models::UserAuthData**](UserAuthData.md)

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


## users_user_id_delete

> crate::models::StatusOk users_user_id_delete(user_id)
Deactivate a user account.

Deactivates the user and revokes all its sessions by archiving its user object. ##### Permissions Must be logged in as the user being deactivated or have the `edit_other_users` permission. 

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


## users_user_id_demote_post

> crate::models::StatusOk users_user_id_demote_post(user_id)
Demote a user to a guest

Convert a regular user into a guest. This will convert the user into a guest for the whole system while retaining their existing team and channel memberships.  __Minimum server version__: 5.16  ##### Permissions Must be logged in as the user or have the `demote_to_guest` permission. 

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


## users_user_id_email_verify_member_post

> crate::models::User users_user_id_email_verify_member_post(user_id)
Verify user email by ID

Verify the email used by a user without a token.  __Minimum server version__: 5.24  ##### Permissions  Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_get

> crate::models::User users_user_id_get(user_id)
Get a user

Get a user a object. Sensitive information will be sanitized out. ##### Permissions Requires an active session but no other permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_image_default_get

> users_user_id_image_default_get(user_id)
Return user's default (generated) profile image

Returns the default (generated) user profile image based on user_id string parameter. ##### Permissions Must be logged in. __Minimum server version__: 5.5 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_image_delete

> crate::models::StatusOk users_user_id_image_delete(user_id)
Delete user's profile image

Delete user's profile image and reset to default image based on user_id string parameter. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. __Minimum server version__: 5.5 

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


## users_user_id_image_get

> users_user_id_image_get(user_id)
Get user's profile image

Get a user's profile image based on user_id string parameter. ##### Permissions Must be logged in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_image_post

> crate::models::StatusOk users_user_id_image_post(user_id, image)
Set user's profile image

Set a user's profile image based on user_id string parameter. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**image** | **std::path::PathBuf** | The image to be uploaded | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_mfa_generate_post

> crate::models::InlineResponse2001 users_user_id_mfa_generate_post(user_id)
Generate MFA secret

Generates an multi-factor authentication secret for a user and returns it as a string and as base64 encoded QR code image. ##### Permissions Must be logged in as the user or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_mfa_put

> crate::models::StatusOk users_user_id_mfa_put(user_id, inline_object9)
Update a user's MFA

Activates multi-factor authentication for the user if `activate` is true and a valid `code` is provided. If activate is false, then `code` is not required and multi-factor authentication is disabled for the user. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object9** | [**InlineObject9**](InlineObject9.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_password_put

> crate::models::StatusOk users_user_id_password_put(user_id, inline_object11)
Update a user's password

Update a user's password. New password must meet password policy set by server configuration. Current password is required if you're updating your own password. ##### Permissions Must be logged in as the user the password is being changed for or have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object11** | [**InlineObject11**](InlineObject11.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_patch_put

> crate::models::User users_user_id_patch_put(user_id, inline_object4)
Patch a user

Partially update a user by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object4** | [**InlineObject4**](InlineObject4.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_promote_post

> crate::models::StatusOk users_user_id_promote_post(user_id)
Promote a guest to user

Convert a guest into a regular user. This will convert the guest into a user for the whole system while retaining any team and channel memberships and automatically joining them to the default channels.  __Minimum server version__: 5.16  ##### Permissions Must be logged in as the user or have the `promote_guest` permission. 

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


## users_user_id_put

> crate::models::User users_user_id_put(user_id, inline_object3)
Update a user

Update a user by providing the user object. The fields that can be updated are defined in the request body, all other provided fields will be ignored. Any fields not included in the request body will be set to null or reverted to default values. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object3** | [**InlineObject3**](InlineObject3.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_roles_put

> crate::models::StatusOk users_user_id_roles_put(user_id, inline_object5)
Update a user's roles

Update a user's system-level roles. Valid user roles are \"system_user\", \"system_admin\" or both of them. Overwrites any previously assigned system-level roles. ##### Permissions Must have the `manage_roles` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object5** | [**InlineObject5**](InlineObject5.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_sessions_get

> Vec<crate::models::Session> users_user_id_sessions_get(user_id)
Get user's sessions

Get a list of sessions by providing the user GUID. Sensitive information will be sanitized out. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**Vec<crate::models::Session>**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_sessions_revoke_all_post

> crate::models::StatusOk users_user_id_sessions_revoke_all_post(user_id)
Revoke all active sessions for a user

Revokes all user sessions from the provided user id and session id strings. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. __Minimum server version__: 4.4 

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


## users_user_id_sessions_revoke_post

> crate::models::StatusOk users_user_id_sessions_revoke_post(user_id, inline_object13)
Revoke a user session

Revokes a user session from the provided user id and session id strings. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object13** | [**InlineObject13**](InlineObject13.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_terms_of_service_get

> crate::models::UserTermsOfService users_user_id_terms_of_service_get(user_id)
Fetches user's latest terms of service action if the latest action was for acceptance.

Will be deprecated in v6.0 Fetches user's latest terms of service action if the latest action was for acceptance.  __Minimum server version__: 5.6 ##### Permissions Must be logged in as the user being acted on. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::UserTermsOfService**](UserTermsOfService.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_terms_of_service_post

> crate::models::StatusOk users_user_id_terms_of_service_post(user_id, inline_object23)
Records user action when they accept or decline custom terms of service

Records user action when they accept or decline custom terms of service. Records the action in audit table. Updates user's last accepted terms of service ID if they accepted it.  __Minimum server version__: 5.4 ##### Permissions Must be logged in as the user being acted on. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object23** | [**InlineObject23**](InlineObject23.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_tokens_get

> Vec<crate::models::UserAccessTokenSanitized> users_user_id_tokens_get(user_id, page, per_page)
Get user access tokens

Get a list of user access tokens for a user. Does not include the actual authentication tokens. Use query parameters for paging.  __Minimum server version__: 4.1  ##### Permissions Must have `read_user_access_token` permission. For non-self requests, must also have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of tokens per page. |  |[default to 60]

### Return type

[**Vec<crate::models::UserAccessTokenSanitized>**](UserAccessTokenSanitized.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_tokens_post

> crate::models::UserAccessToken users_user_id_tokens_post(user_id, inline_object18)
Create a user access token

Generate a user access token that can be used to authenticate with the Mattermost REST API.  __Minimum server version__: 4.1  ##### Permissions Must have `create_user_access_token` permission. For non-self requests, must also have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object18** | [**InlineObject18**](InlineObject18.md) |  | [required] |

### Return type

[**crate::models::UserAccessToken**](UserAccessToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_typing_post

> users_user_id_typing_post(user_id, inline_object24)
Publish a user typing websocket event.

Notify users in the given channel via websocket that the given user is typing. __Minimum server version__: 5.26 ##### Permissions Must have `manage_system` permission to publish for any user other than oneself. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object24** | Option<[**InlineObject24**](InlineObject24.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_username_username_get

> crate::models::User users_username_username_get(username)
Get a user by username

Get a user object by providing a username. Sensitive information will be sanitized out. ##### Permissions Requires an active session but no other permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Username | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_usernames_post

> Vec<crate::models::User> users_usernames_post(request_body)
Get users by usernames

Get a list of users based on a provided list of usernames. ##### Permissions Requires an active session but no other permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | List of usernames | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

