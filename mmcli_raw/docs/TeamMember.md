# TeamMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_id** | Option<**String**> | The ID of the team this member belongs to. | [optional]
**user_id** | Option<**String**> | The ID of the user this member relates to. | [optional]
**roles** | Option<**String**> | The complete list of roles assigned to this team member, as a space-separated list of role names, including any roles granted implicitly through permissions schemes. | [optional]
**delete_at** | Option<**i32**> | The time in milliseconds that this team member was deleted. | [optional]
**scheme_user** | Option<**bool**> | Whether this team member holds the default user role defined by the team's permissions scheme. | [optional]
**scheme_admin** | Option<**bool**> | Whether this team member holds the default admin role defined by the team's permissions scheme. | [optional]
**explicit_roles** | Option<**String**> | The list of roles explicitly assigned to this team member, as a space separated list of role names. This list does *not* include any roles granted implicitly through permissions schemes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


