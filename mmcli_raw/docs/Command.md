# Command

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the slash command | [optional]
**token** | Option<**String**> | The token which is used to verify the source of the payload | [optional]
**create_at** | Option<**i64**> | The time in milliseconds the command was created | [optional]
**update_at** | Option<**i64**> | The time in milliseconds the command was last updated | [optional]
**deleted_at** | Option<**i64**> | The time in milliseconds the command was deleted, 0 if never deleted | [optional]
**creator_id** | Option<**String**> | The user id for the commands creator | [optional]
**team_id** | Option<**String**> | The team id for which this command is configured | [optional]
**trigger** | Option<**String**> | The string that triggers this command | [optional]
**method** | Option<**String**> | Is the trigger done with HTTP Get ('G') or HTTP Post ('P') | [optional]
**username** | Option<**String**> | What is the username for the response post | [optional]
**icon_url** | Option<**String**> | The url to find the icon for this users avatar | [optional]
**auto_complete** | Option<**bool**> | Use auto complete for this command | [optional]
**auto_complete_desc** | Option<**String**> | The description for this command shown when selecting the command | [optional]
**auto_complete_hint** | Option<**String**> | The hint for this command | [optional]
**display_name** | Option<**String**> | Display name for the command | [optional]
**description** | Option<**String**> | Description for this command | [optional]
**url** | Option<**String**> | The URL that is triggered | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


