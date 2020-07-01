# InlineObject1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** |  | 
**username** | **String** |  | 
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**nickname** | Option<**String**> |  | [optional]
**auth_data** | Option<**String**> | Service-specific authentication data, such as email address. | [optional]
**auth_service** | Option<**String**> | The authentication service, one of \"email\", \"gitlab\", \"ldap\", \"saml\", \"office365\", \"google\", and \"\". | [optional]
**password** | Option<**String**> | The password used for email authentication. | [optional]
**locale** | Option<**String**> |  | [optional]
**props** | Option<[**serde_json::Value**](.md)> |  | [optional]
**notify_props** | Option<[**crate::models::UserNotifyProps**](UserNotifyProps.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


