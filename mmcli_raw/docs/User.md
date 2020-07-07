# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**create_at** | Option<**i64**> | The time in milliseconds a user was created | [optional]
**update_at** | Option<**i64**> | The time in milliseconds a user was last updated | [optional]
**delete_at** | Option<**i64**> | The time in milliseconds a user was deleted | [optional]
**username** | Option<**String**> |  | [optional]
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**nickname** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**email_verified** | Option<**bool**> |  | [optional]
**auth_service** | Option<**String**> |  | [optional]
**roles** | Option<**String**> |  | [optional]
**locale** | Option<**String**> |  | [optional]
**notify_props** | Option<[**crate::models::UserNotifyProps**](UserNotifyProps.md)> |  | [optional]
**props** | Option<[**serde_json::Value**](.md)> |  | [optional]
**last_password_update** | Option<**i64**> |  | [optional]
**last_picture_update** | Option<**i64**> |  | [optional]
**failed_attempts** | Option<**i64**> |  | [optional]
**mfa_active** | Option<**bool**> |  | [optional]
**timezone** | Option<[**crate::models::Timezone**](Timezone.md)> |  | [optional]
**terms_of_service_id** | Option<**String**> | ID of accepted terms of service, if any. This field is not present if empty. | [optional]
**terms_of_service_create_at** | Option<**i64**> | The time in milliseconds the user accepted the terms of service | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


