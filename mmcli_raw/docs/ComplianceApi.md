# \ComplianceApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**compliance_reports_get**](ComplianceApi.md#compliance_reports_get) | **get** /compliance/reports | Get reports
[**compliance_reports_post**](ComplianceApi.md#compliance_reports_post) | **post** /compliance/reports | Create report
[**compliance_reports_report_id_download_get**](ComplianceApi.md#compliance_reports_report_id_download_get) | **get** /compliance/reports/{report_id}/download | Download a report
[**compliance_reports_report_id_get**](ComplianceApi.md#compliance_reports_report_id_get) | **get** /compliance/reports/{report_id} | Get a report



## compliance_reports_get

> Vec<crate::models::Compliance> compliance_reports_get(page, per_page)
Get reports

Get a list of compliance reports previously created by page, selected with `page` and `per_page` query parameters. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of reports per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Compliance>**](Compliance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compliance_reports_post

> crate::models::Compliance compliance_reports_post()
Create report

Create and save a compliance report. ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Compliance**](Compliance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compliance_reports_report_id_download_get

> compliance_reports_report_id_download_get(report_id)
Download a report

Download the full contents of a report as a file. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | Compliance report GUID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compliance_reports_report_id_get

> crate::models::Compliance compliance_reports_report_id_get(report_id)
Get a report

Get a compliance reports previously created. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | Compliance report GUID | [required] |

### Return type

[**crate::models::Compliance**](Compliance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

