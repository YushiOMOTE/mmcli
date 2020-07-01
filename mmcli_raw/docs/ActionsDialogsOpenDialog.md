# ActionsDialogsOpenDialog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_id** | Option<**String**> | Set an ID that will be included when the dialog is submitted | [optional]
**title** | **String** | Title of the dialog | 
**introduction_text** | Option<**String**> | Markdown formatted introductory paragraph | [optional]
**elements** | [**Vec<serde_json::Value>**](serde_json::Value.md) | Input elements, see https://docs.mattermost.com/developer/interactive-dialogs.html#elements | 
**submit_label** | Option<**String**> | Label on the submit button | [optional]
**notify_on_cancel** | Option<**bool**> | Set true to receive payloads when user cancels a dialog | [optional]
**state** | Option<**String**> | Set some state to be echoed back with the dialog submission | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


