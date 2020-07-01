# Post

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**create_at** | Option<**i64**> | The time in milliseconds a post was created | [optional]
**update_at** | Option<**i64**> | The time in milliseconds a post was last updated | [optional]
**delete_at** | Option<**i64**> | The time in milliseconds a post was deleted | [optional]
**edit_at** | Option<**i64**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**channel_id** | Option<**String**> |  | [optional]
**root_id** | Option<**String**> |  | [optional]
**parent_id** | Option<**String**> |  | [optional]
**original_id** | Option<**String**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**props** | Option<[**serde_json::Value**](.md)> |  | [optional]
**hashtag** | Option<**String**> |  | [optional]
**filenames** | Option<**Vec<String>**> | This field will only appear on some posts created before Mattermost 3.5 and has since been deprecated. | [optional]
**file_ids** | Option<**Vec<String>**> |  | [optional]
**pending_post_id** | Option<**String**> |  | [optional]
**metadata** | Option<[**crate::models::PostMetadata**](PostMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


