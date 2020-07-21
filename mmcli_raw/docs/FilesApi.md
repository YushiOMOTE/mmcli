# \FilesApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_file_id_get**](FilesApi.md#files_file_id_get) | **get** /files/{file_id} | Get a file
[**files_file_id_info_get**](FilesApi.md#files_file_id_info_get) | **get** /files/{file_id}/info | Get metadata for a file
[**files_file_id_link_get**](FilesApi.md#files_file_id_link_get) | **get** /files/{file_id}/link | Get a public file link
[**files_file_id_preview_get**](FilesApi.md#files_file_id_preview_get) | **get** /files/{file_id}/preview | Get a file's preview
[**files_file_id_public_get**](FilesApi.md#files_file_id_public_get) | **get** /files/{file_id}/public | Get a public file
[**files_file_id_thumbnail_get**](FilesApi.md#files_file_id_thumbnail_get) | **get** /files/{file_id}/thumbnail | Get a file's thumbnail
[**files_post**](FilesApi.md#files_post) | **post** /files | Upload a file



## files_file_id_get

> files_file_id_get(file_id)
Get a file

Gets a file that has been uploaded previously. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_file_id_info_get

> crate::models::FileInfo files_file_id_info_get(file_id)
Get metadata for a file

Gets a file's info. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file info to get | [required] |

### Return type

[**crate::models::FileInfo**](FileInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_file_id_link_get

> crate::models::InlineResponse20011 files_file_id_link_get(file_id)
Get a public file link

Gets a public link for a file that can be accessed without logging into Mattermost. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get a link for | [required] |

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_file_id_preview_get

> files_file_id_preview_get(file_id)
Get a file's preview

Gets a file's preview. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_file_id_public_get

> files_file_id_public_get(file_id, h)
Get a public file

##### Permissions No permissions required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get | [required] |
**h** | **String** | File hash | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_file_id_thumbnail_get

> files_file_id_thumbnail_get(file_id)
Get a file's thumbnail

Gets a file's thumbnail. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_post

> crate::models::InlineResponse201 files_post(files, channel_id, client_ids)
Upload a file

Uploads a file that can later be attached to a post.  This request can either be a multipart/form-data request with a channel_id, files and optional client_ids defined in the FormData, or it can be a request with the channel_id and filename defined as query parameters with the contents of a single file in the body of the request.  Only multipart/form-data requests are supported by server versions up to and including 4.7. Server versions 4.8 and higher support both types of requests.  ##### Permissions Must have `upload_file` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**files** | Option<**std::path::PathBuf**> | A file to be uploaded |  |
**channel_id** | Option<**String**> | The ID of the channel that this file will be uploaded to |  |
**client_ids** | Option<**String**> | A unique identifier for the file that will be returned in the response |  |

### Return type

[**crate::models::InlineResponse201**](inline_response_201.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

