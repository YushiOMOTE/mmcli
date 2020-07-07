# \SchemesApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**schemes_get**](SchemesApi.md#schemes_get) | **get** /schemes | Get the schemes.
[**schemes_post**](SchemesApi.md#schemes_post) | **post** /schemes | Create a scheme
[**schemes_scheme_id_channels_get**](SchemesApi.md#schemes_scheme_id_channels_get) | **get** /schemes/{scheme_id}/channels | Get a page of channels which use this scheme.
[**schemes_scheme_id_delete**](SchemesApi.md#schemes_scheme_id_delete) | **delete** /schemes/{scheme_id} | Delete a scheme
[**schemes_scheme_id_get**](SchemesApi.md#schemes_scheme_id_get) | **get** /schemes/{scheme_id} | Get a scheme
[**schemes_scheme_id_patch_put**](SchemesApi.md#schemes_scheme_id_patch_put) | **put** /schemes/{scheme_id}/patch | Patch a scheme
[**schemes_scheme_id_teams_get**](SchemesApi.md#schemes_scheme_id_teams_get) | **get** /schemes/{scheme_id}/teams | Get a page of teams which use this scheme.



## schemes_get

> Vec<crate::models::Scheme> schemes_get(scope, page, per_page)
Get the schemes.

Get a page of schemes. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | Option<**String**> | Limit the results returned to the provided scope, either `team` or `channel`. |  |[default to ]
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of schemes per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Scheme>**](Scheme.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemes_post

> crate::models::Scheme schemes_post(inline_object78)
Create a scheme

Create a new scheme.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object78** | [**InlineObject78**](InlineObject78.md) |  | [required] |

### Return type

[**crate::models::Scheme**](Scheme.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemes_scheme_id_channels_get

> Vec<crate::models::Channel> schemes_scheme_id_channels_get(scheme_id, page, per_page)
Get a page of channels which use this scheme.

Get a page of channels which use this scheme. The provided Scheme ID should be for a Channel-scoped Scheme. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | Scheme GUID | [required] |
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of channels per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemes_scheme_id_delete

> crate::models::StatusOk schemes_scheme_id_delete(scheme_id)
Delete a scheme

Soft deletes a scheme, by marking the scheme as deleted in the database.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | ID of the scheme to delete | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemes_scheme_id_get

> crate::models::Scheme schemes_scheme_id_get(scheme_id)
Get a scheme

Get a scheme from the provided scheme id.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | Scheme GUID | [required] |

### Return type

[**crate::models::Scheme**](Scheme.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemes_scheme_id_patch_put

> crate::models::Scheme schemes_scheme_id_patch_put(scheme_id, inline_object79)
Patch a scheme

Partially update a scheme by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | Scheme GUID | [required] |
**inline_object79** | [**InlineObject79**](InlineObject79.md) |  | [required] |

### Return type

[**crate::models::Scheme**](Scheme.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemes_scheme_id_teams_get

> Vec<crate::models::Team> schemes_scheme_id_teams_get(scheme_id, page, per_page)
Get a page of teams which use this scheme.

Get a page of teams which use this scheme. The provided Scheme ID should be for a Team-scoped Scheme. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | Scheme GUID | [required] |
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of teams per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

