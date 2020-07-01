# Role

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier of the role. | [optional]
**name** | Option<**String**> | The unique name of the role, used when assigning roles to users/groups in contexts. | [optional]
**display_name** | Option<**String**> | The human readable name for the role. | [optional]
**description** | Option<**String**> | A human readable description of the role. | [optional]
**permissions** | Option<**Vec<String>**> | A list of the unique names of the permissions this role grants. | [optional]
**scheme_managed** | Option<**bool**> | indicates if this role is managed by a scheme (true), or is a custom stand-alone role (false). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


