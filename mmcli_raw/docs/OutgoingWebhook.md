# OutgoingWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this outgoing webhook | [optional]
**create_at** | Option<**i64**> | The time in milliseconds a outgoing webhook was created | [optional]
**update_at** | Option<**i64**> | The time in milliseconds a outgoing webhook was last updated | [optional]
**delete_at** | Option<**i64**> | The time in milliseconds a outgoing webhook was deleted | [optional]
**creator_id** | Option<**String**> | The Id of the user who created the webhook | [optional]
**team_id** | Option<**String**> | The ID of the team that the webhook watchs | [optional]
**channel_id** | Option<**String**> | The ID of a public channel that the webhook watchs | [optional]
**description** | Option<**String**> | The description for this outgoing webhook | [optional]
**display_name** | Option<**String**> | The display name for this outgoing webhook | [optional]
**trigger_words** | Option<**Vec<String>**> | List of words for the webhook to trigger on | [optional]
**trigger_when** | Option<**i32**> | When to trigger the webhook, `0` when a trigger word is present at all and `1` if the message starts with a trigger word | [optional]
**callback_urls** | Option<**Vec<String>**> | The URLs to POST the payloads to when the webhook is triggered | [optional]
**content_type** | Option<**String**> | The format to POST the data in, either `application/json` or `application/x-www-form-urlencoded` | [optional][default to application/x-www-form-urlencoded]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


