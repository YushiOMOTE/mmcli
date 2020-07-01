# \CommandsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**commands_command_id_delete**](CommandsApi.md#commands_command_id_delete) | **delete** /commands/{command_id} | Delete a command
[**commands_command_id_get**](CommandsApi.md#commands_command_id_get) | **get** /commands/{command_id} | Get a command
[**commands_command_id_move_put**](CommandsApi.md#commands_command_id_move_put) | **put** /commands/{command_id}/move | Move a command
[**commands_command_id_put**](CommandsApi.md#commands_command_id_put) | **put** /commands/{command_id} | Update a command
[**commands_command_id_regen_token_put**](CommandsApi.md#commands_command_id_regen_token_put) | **put** /commands/{command_id}/regen_token | Generate a new token
[**commands_execute_post**](CommandsApi.md#commands_execute_post) | **post** /commands/execute | Execute a command
[**commands_get**](CommandsApi.md#commands_get) | **get** /commands | List commands for a team
[**commands_post**](CommandsApi.md#commands_post) | **post** /commands | Create a command
[**teams_team_id_commands_autocomplete_get**](CommandsApi.md#teams_team_id_commands_autocomplete_get) | **get** /teams/{team_id}/commands/autocomplete | List autocomplete commands
[**teams_team_id_commands_autocomplete_suggestions_get**](CommandsApi.md#teams_team_id_commands_autocomplete_suggestions_get) | **get** /teams/{team_id}/commands/autocomplete_suggestions | List commands' autocomplete data



## commands_command_id_delete

> crate::models::StatusOk commands_command_id_delete(command_id)
Delete a command

Delete a command based on command id string. ##### Permissions Must have `manage_slash_commands` permission for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to delete | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commands_command_id_get

> crate::models::Command commands_command_id_get(command_id)
Get a command

Get a command definition based on command id string. ##### Permissions Must have `manage_slash_commands` permission for the team the command is in.  __Minimum server version__: 5.22 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to get | [required] |

### Return type

[**crate::models::Command**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commands_command_id_move_put

> crate::models::StatusOk commands_command_id_move_put(command_id, inline_object73)
Move a command

Move a command to a different team based on command id string. ##### Permissions Must have `manage_slash_commands` permission for the team the command is currently in and the destination team.  __Minimum server version__: 5.22 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to move | [required] |
**inline_object73** | [**InlineObject73**](InlineObject73.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commands_command_id_put

> crate::models::Command commands_command_id_put(command_id, command)
Update a command

Update a single command based on command id string and Command struct. ##### Permissions Must have `manage_slash_commands` permission for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to update | [required] |
**command** | [**Command**](Command.md) |  | [required] |

### Return type

[**crate::models::Command**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commands_command_id_regen_token_put

> crate::models::InlineResponse20010 commands_command_id_regen_token_put(command_id)
Generate a new token

Generate a new token for the command based on command id string. ##### Permissions Must have `manage_slash_commands` permission for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to generate the new token | [required] |

### Return type

[**crate::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commands_execute_post

> crate::models::CommandResponse commands_execute_post(inline_object74)
Execute a command

Execute a command on a team. ##### Permissions Must have `use_slash_commands` permission for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object74** | [**InlineObject74**](InlineObject74.md) |  | [required] |

### Return type

[**crate::models::CommandResponse**](CommandResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commands_get

> Vec<crate::models::Command> commands_get(team_id, custom_only)
List commands for a team

List commands for a team. ##### Permissions `manage_slash_commands` if need list custom commands. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<**String**> | The team id. |  |
**custom_only** | Option<**bool**> | To get only the custom commands. If set to false will get the custom if the user have access plus the system commands, otherwise just the system commands.  |  |[default to false]

### Return type

[**Vec<crate::models::Command>**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commands_post

> crate::models::Command commands_post(inline_object72)
Create a command

Create a command for a team. ##### Permissions `manage_slash_commands` for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object72** | [**InlineObject72**](InlineObject72.md) |  | [required] |

### Return type

[**crate::models::Command**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_commands_autocomplete_get

> Vec<crate::models::Command> teams_team_id_commands_autocomplete_get(team_id)
List autocomplete commands

List autocomplete commands in the team. ##### Permissions `view_team` for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |

### Return type

[**Vec<crate::models::Command>**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_commands_autocomplete_suggestions_get

> Vec<crate::models::AutocompleteSuggestion> teams_team_id_commands_autocomplete_suggestions_get(team_id, user_input)
List commands' autocomplete data

List commands' autocomplete data for the team. ##### Permissions `view_team` for the team. __Minimum server version__: 5.24 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_input** | **String** | String inputted by the user. | [required] |

### Return type

[**Vec<crate::models::AutocompleteSuggestion>**](AutocompleteSuggestion.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

