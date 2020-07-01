# InlineObject2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**term** | **String** | The term to match against username, full name, nickname and email | 
**team_id** | Option<**String**> | If provided, only search users on this team | [optional]
**not_in_team_id** | Option<**String**> | If provided, only search users not on this team | [optional]
**in_channel_id** | Option<**String**> | If provided, only search users in this channel | [optional]
**not_in_channel_id** | Option<**String**> | If provided, only search users not in this channel. Must specifiy `team_id` when using this option | [optional]
**in_group_id** | Option<**String**> | If provided, only search users in this group. Must have `manage_system` permission. | [optional]
**group_constrained** | Option<**bool**> | When used with `not_in_channel_id` or `not_in_team_id`, returns only the users that are allowed to join the channel or team based on its group constrains. | [optional]
**allow_inactive** | Option<**bool**> | When `true`, include deactivated users in the results | [optional]
**without_team** | Option<**bool**> | Set this to `true` if you would like to search for users that are not on a team. This option takes precendence over `team_id`, `in_channel_id`, and `not_in_channel_id`. | [optional]
**limit** | Option<**i32**> | The maximum number of users to return in the results  __Available as of server version 5.6. Defaults to `100` if not provided or on an earlier server version.__  | [optional][default to 100]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


