# \DataretentionApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**data_retention_policy_get**](DataretentionApi.md#data_retention_policy_get) | **get** /data_retention/policy | Get the data retention policy details.



## data_retention_policy_get

> crate::models::DataRetentionPolicy data_retention_policy_get()
Get the data retention policy details.

Gets the current data retention policy details from the server, including what data should be purged and the cutoff times for each data type that should be purged. __Minimum server version__: 4.3 ##### Permissions Requires an active session but no other permissions. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataRetentionPolicy**](DataRetentionPolicy.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

