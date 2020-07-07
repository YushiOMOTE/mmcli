# \JobsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_get**](JobsApi.md#jobs_get) | **get** /jobs | Get the jobs.
[**jobs_job_id_cancel_post**](JobsApi.md#jobs_job_id_cancel_post) | **post** /jobs/{job_id}/cancel | Cancel a job.
[**jobs_job_id_get**](JobsApi.md#jobs_job_id_get) | **get** /jobs/{job_id} | Get a job.
[**jobs_post**](JobsApi.md#jobs_post) | **post** /jobs | Create a new job.
[**jobs_type_type_get**](JobsApi.md#jobs_type_type_get) | **get** /jobs/type/{type} | Get the jobs of the given type.



## jobs_get

> Vec<crate::models::Job> jobs_get(page, per_page)
Get the jobs.

Get a page of jobs. Use the query parameters to modify the behaviour of this endpoint. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of jobs per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Job>**](Job.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_job_id_cancel_post

> crate::models::StatusOk jobs_job_id_cancel_post(job_id)
Cancel a job.

Cancel a job. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_job_id_get

> crate::models::Job jobs_job_id_get(job_id)
Get a job.

Gets a single job. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job GUID | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_post

> crate::models::Job jobs_post(inline_object57)
Create a new job.

Create a new job. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object57** | [**InlineObject57**](InlineObject57.md) |  | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_type_type_get

> Vec<crate::models::Job> jobs_type_type_get(_type, page, per_page)
Get the jobs of the given type.

Get a page of jobs of the given type. Use the query parameters to modify the behaviour of this endpoint. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | Job type | [required] |
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of jobs per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Job>**](Job.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

