# Session

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_at** | Option<**i64**> | The time in milliseconds a session was created | [optional]
**device_id** | Option<**String**> |  | [optional]
**expires_at** | Option<**i64**> | The time in milliseconds a session will expire | [optional]
**id** | Option<**String**> |  | [optional]
**is_oauth** | Option<**bool**> |  | [optional]
**last_activity_at** | Option<**i64**> | The time in milliseconds of the last activity of a session | [optional]
**props** | Option<[**serde_json::Value**](.md)> |  | [optional]
**roles** | Option<**String**> |  | [optional]
**team_members** | Option<[**Vec<crate::models::TeamMember>**](TeamMember.md)> |  | [optional]
**token** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


