# InlineObject52

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_id** | **String** | The channel ID to post in | 
**message** | **String** | The message contents, can be formatted with Markdown | 
**root_id** | Option<**String**> | The post ID to comment on | [optional]
**file_ids** | Option<**Vec<String>**> | A list of file IDs to associate with the post. Note that posts are limited to 5 files maximum. Please use additional posts for more files. | [optional]
**props** | Option<[**serde_json::Value**](.md)> | A general JSON property bag to attach to the post | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


