# InlineObject82

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The URL to send the submitted dialog payload to | 
**channel_id** | **String** | Channel ID the user submitted the dialog from | 
**team_id** | **String** | Team ID the user submitted the dialog from | 
**submission** | [**serde_json::Value**](.md) | String map where keys are element names and values are the element input values | 
**callback_id** | Option<**String**> | Callback ID sent when the dialog was opened | [optional]
**state** | Option<**String**> | State sent when the dialog was opened | [optional]
**cancelled** | Option<**bool**> | Set to true if the dialog was cancelled | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


