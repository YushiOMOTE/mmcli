# \BleveApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bleve_purge_indexes_post**](BleveApi.md#bleve_purge_indexes_post) | **post** /bleve/purge_indexes | Purge all Bleve indexes



## bleve_purge_indexes_post

> crate::models::StatusOk bleve_purge_indexes_post()
Purge all Bleve indexes

Deletes all Bleve indexes and their contents. After calling this endpoint, it is necessary to schedule a new Bleve indexing job to repopulate the indexes. __Minimum server version__: 5.24 ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

