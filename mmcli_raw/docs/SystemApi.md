# \SystemApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_old_get**](SystemApi.md#analytics_old_get) | **get** /analytics/old | Get analytics
[**audits_get**](SystemApi.md#audits_get) | **get** /audits | Get audits
[**caches_invalidate_post**](SystemApi.md#caches_invalidate_post) | **post** /caches/invalidate | Invalidate all the caches
[**config_client_get**](SystemApi.md#config_client_get) | **get** /config/client | Get client configuration
[**config_environment_get**](SystemApi.md#config_environment_get) | **get** /config/environment | Get configuration made through environment variables
[**config_get**](SystemApi.md#config_get) | **get** /config | Get configuration
[**config_patch_put**](SystemApi.md#config_patch_put) | **put** /config/patch | Patch configuration
[**config_put**](SystemApi.md#config_put) | **put** /config | Update configuration
[**config_reload_post**](SystemApi.md#config_reload_post) | **post** /config/reload | Reload configuration
[**database_recycle_post**](SystemApi.md#database_recycle_post) | **post** /database/recycle | Recycle database connections
[**email_test_post**](SystemApi.md#email_test_post) | **post** /email/test | Send a test email
[**file_s3_test_post**](SystemApi.md#file_s3_test_post) | **post** /file/s3_test | Test AWS S3 connection
[**image_get**](SystemApi.md#image_get) | **get** /image | Get an image by url
[**license_client_get**](SystemApi.md#license_client_get) | **get** /license/client | Get client license
[**license_delete**](SystemApi.md#license_delete) | **delete** /license | Remove license file
[**license_post**](SystemApi.md#license_post) | **post** /license | Upload license file
[**logs_get**](SystemApi.md#logs_get) | **get** /logs | Get logs
[**logs_post**](SystemApi.md#logs_post) | **post** /logs | Add log message
[**redirect_location_get**](SystemApi.md#redirect_location_get) | **get** /redirect_location | Get redirect location
[**server_busy_delete**](SystemApi.md#server_busy_delete) | **delete** /server_busy | Clears the server busy (high load) flag
[**server_busy_get**](SystemApi.md#server_busy_get) | **get** /server_busy | Get server busy expiry time.
[**server_busy_post**](SystemApi.md#server_busy_post) | **post** /server_busy | Set the server busy (high load) flag
[**site_url_test_post**](SystemApi.md#site_url_test_post) | **post** /site_url/test | Checks the validity of a Site URL
[**system_ping_get**](SystemApi.md#system_ping_get) | **get** /system/ping | Check system health
[**system_timezones_get**](SystemApi.md#system_timezones_get) | **get** /system/timezones | Retrieve a list of supported timezones
[**trial_license_post**](SystemApi.md#trial_license_post) | **post** /trial-license | Request and install a trial license for your server



## analytics_old_get

> analytics_old_get(name, team_id)
Get analytics

Get some analytics data about the system. This endpoint uses the old format, the `/analytics` route is reserved for the new format when it gets implemented.  The returned JSON changes based on the `name` query parameter but is always key/value pairs.  __Minimum server version__: 4.0  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Possible values are \"standard\", \"post_counts_day\", \"user_counts_with_posts_day\" or \"extra_counts\" |  |[default to standard]
**team_id** | Option<**String**> | The team ID to filter the data by |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## audits_get

> Vec<crate::models::Audit> audits_get(page, per_page)
Get audits

Get a page of audits for all users on the system, selected with `page` and `per_page` query parameters. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of audits per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Audit>**](Audit.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## caches_invalidate_post

> crate::models::StatusOk caches_invalidate_post()
Invalidate all the caches

Purge all the in-memory caches for the Mattermost server. This can have a temporary negative effect on performance while the caches are re-populated. ##### Permissions Must have `manage_system` permission. 

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


## config_client_get

> config_client_get(format)
Get client configuration

Get a subset of the server configuration needed by the client. ##### Permissions No permission required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** | Must be `old`, other formats not implemented yet | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_environment_get

> crate::models::EnvironmentConfig config_environment_get()
Get configuration made through environment variables

Retrieve a json object mirroring the server configuration where fields are set to true if the corresponding config setting is set through an environment variable. Settings that haven't been set through environment variables will be missing from the object.  __Minimum server version__: 4.10  ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EnvironmentConfig**](EnvironmentConfig.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_get

> crate::models::Config config_get()
Get configuration

Retrieve the current server configuration ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Config**](Config.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_patch_put

> crate::models::Config config_patch_put(config)
Patch configuration

Submit configuration to patch. As of server version 4.8, the `PluginSettings.EnableUploads` setting cannot be modified by this endpoint. ##### Permissions Must have `manage_system` permission. __Minimum server version__: 5.20 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config** | [**Config**](Config.md) | Mattermost configuration | [required] |

### Return type

[**crate::models::Config**](Config.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_put

> crate::models::Config config_put(config)
Update configuration

Submit a new configuration for the server to use. As of server version 4.8, the `PluginSettings.EnableUploads` setting cannot be modified by this endpoint. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config** | [**Config**](Config.md) | Mattermost configuration | [required] |

### Return type

[**crate::models::Config**](Config.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_reload_post

> crate::models::StatusOk config_reload_post()
Reload configuration

Reload the configuration file to pick up on any changes made to it. ##### Permissions Must have `manage_system` permission. 

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


## database_recycle_post

> crate::models::StatusOk database_recycle_post()
Recycle database connections

Recycle database connections by closing and reconnecting all connections to master and read replica databases. ##### Permissions Must have `manage_system` permission. 

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


## email_test_post

> crate::models::StatusOk email_test_post(config)
Send a test email

Send a test email to make sure you have your email settings configured correctly. Optionally provide a configuration in the request body to test. If no valid configuration is present in the request body the current server configuration will be tested. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config** | [**Config**](Config.md) | Mattermost configuration | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_s3_test_post

> crate::models::StatusOk file_s3_test_post(config)
Test AWS S3 connection

Send a test to validate if can connect to AWS S3. Optionally provide a configuration in the request body to test. If no valid configuration is present in the request body the current server configuration will be tested. ##### Permissions Must have `manage_system` permission. __Minimum server version__: 4.8 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config** | [**Config**](Config.md) | Mattermost configuration | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_get

> std::path::PathBuf image_get()
Get an image by url

Fetches an image via Mattermost image proxy. __Minimum server version__: 3.10 ##### Permissions Must be logged in. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/_*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_client_get

> license_client_get(format)
Get client license

Get a subset of the server license needed by the client. ##### Permissions No permission required but having the `manage_system` permission returns more information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** | Must be `old`, other formats not implemented yet | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_delete

> license_delete()
Remove license file

Remove the license file from the server. This will disable all enterprise features.  __Minimum server version__: 4.0  ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_post

> crate::models::StatusOk license_post(license)
Upload license file

Upload a license to enable enterprise features.  __Minimum server version__: 4.0  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license** | **std::path::PathBuf** | The license to be uploaded | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logs_get

> Vec<String> logs_get(page, logs_per_page)
Get logs

Get a page of server logs, selected with `page` and `logs_per_page` query parameters. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**logs_per_page** | Option<**String**> | The number of logs per page. There is a maximum limit of 10000 logs per page. |  |[default to 10000]

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logs_post

> Vec<String> logs_post(inline_object61)
Add log message

Add log messages to the server logs. ##### Permissions Users with `manage_system` permission can log ERROR or DEBUG messages. Logged in users can log ERROR or DEBUG messages when `ServiceSettings.EnableDeveloper` is `true` or just DEBUG messages when `false`. Non-logged in users can log ERROR or DEBUG messages when `ServiceSettings.EnableDeveloper` is `true` and cannot log when `false`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object61** | [**InlineObject61**](InlineObject61.md) |  | [required] |

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redirect_location_get

> crate::models::InlineResponse2009 redirect_location_get(url)
Get redirect location

__Minimum server version__: 3.10 ##### Permissions Must be logged in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Url to check | [required] |

### Return type

[**crate::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/_*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_busy_delete

> crate::models::StatusOk server_busy_delete()
Clears the server busy (high load) flag

Marks the server as not having high load which re-enables non-critical services such as search, statuses and typing notifications.  __Minimum server version__: 5.20  ##### Permissions Must have `manage_system` permission. 

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


## server_busy_get

> crate::models::ServerBusy server_busy_get()
Get server busy expiry time.

Gets the timestamp corresponding to when the server busy flag will be automatically cleared.   __Minimum server version__: 5.20  ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServerBusy**](Server_Busy.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_busy_post

> crate::models::StatusOk server_busy_post(seconds)
Set the server busy (high load) flag

Marks the server as currently having high load which disables non-critical services such as search, statuses and typing notifications.  __Minimum server version__: 5.20  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seconds** | Option<**String**> | Number of seconds until server is automatically marked as not busy. |  |[default to 3600]

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_url_test_post

> crate::models::StatusOk site_url_test_post(inline_object58)
Checks the validity of a Site URL

Sends a Ping request to the mattermost server using the specified Site URL.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.16 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object58** | [**InlineObject58**](InlineObject58.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_ping_get

> crate::models::StatusOk system_ping_get()
Check system health

Check if the server is up and healthy based on the configuration setting `GoRoutineHealthThreshold`. If `GoRoutineHealthThreshold` and the number of goroutines on the server exceeds that threshold the server is considered unhealthy. If `GoRoutineHealthThreshold` is not set or the number of goroutines is below the threshold the server is considered healthy. __Minimum server version__: 3.10 ##### Permissions Must be logged in. 

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


## system_timezones_get

> Vec<String> system_timezones_get()
Retrieve a list of supported timezones

__Minimum server version__: 3.10 ##### Permissions Must be logged in. 

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trial_license_post

> trial_license_post(inline_object60)
Request and install a trial license for your server

Request and install a trial license for your server __Minimum server version__: 5.25 ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object60** | [**InlineObject60**](InlineObject60.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

