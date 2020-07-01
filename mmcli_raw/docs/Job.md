# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique id of the job | [optional]
**_type** | Option<**String**> | The type of job | [optional]
**create_at** | Option<**i64**> | The time at which the job was created | [optional]
**start_at** | Option<**i64**> | The time at which the job was started | [optional]
**last_activity_at** | Option<**i64**> | The last time at which the job had activity | [optional]
**status** | Option<**String**> | The status of the job | [optional]
**progress** | Option<**i32**> | The progress (as a percentage) of the job | [optional]
**data** | Option<[**serde_json::Value**](.md)> | A freeform data field containing additional information about the job | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


