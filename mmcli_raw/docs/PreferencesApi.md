# \PreferencesApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_user_id_preferences_category_get**](PreferencesApi.md#users_user_id_preferences_category_get) | **get** /users/{user_id}/preferences/{category} | List a user's preferences by category
[**users_user_id_preferences_category_name_preference_name_get**](PreferencesApi.md#users_user_id_preferences_category_name_preference_name_get) | **get** /users/{user_id}/preferences/{category}/name/{preference_name} | Get a specific user preference
[**users_user_id_preferences_delete_post**](PreferencesApi.md#users_user_id_preferences_delete_post) | **post** /users/{user_id}/preferences/delete | Delete user's preferences
[**users_user_id_preferences_get**](PreferencesApi.md#users_user_id_preferences_get) | **get** /users/{user_id}/preferences | Get the user's preferences
[**users_user_id_preferences_put**](PreferencesApi.md#users_user_id_preferences_put) | **put** /users/{user_id}/preferences | Save the user's preferences



## users_user_id_preferences_category_get

> Vec<crate::models::Preference> users_user_id_preferences_category_get(user_id, category)
List a user's preferences by category

Lists the current user's stored preferences in the given category. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**category** | **String** | The category of a group of preferences | [required] |

### Return type

[**Vec<crate::models::Preference>**](Preference.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_preferences_category_name_preference_name_get

> crate::models::Preference users_user_id_preferences_category_name_preference_name_get(user_id, category, preference_name)
Get a specific user preference

Gets a single preference for the current user with the given category and name. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**category** | **String** | The category of a group of preferences | [required] |
**preference_name** | **String** | The name of the preference | [required] |

### Return type

[**crate::models::Preference**](Preference.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_preferences_delete_post

> crate::models::StatusOk users_user_id_preferences_delete_post(user_id, preference)
Delete user's preferences

Delete a list of the user's preferences. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**preference** | [**Vec<crate::models::Preference>**](Preference.md) | List of preference objects | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_preferences_get

> Vec<crate::models::Preference> users_user_id_preferences_get(user_id)
Get the user's preferences

Get a list of the user's preferences. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**Vec<crate::models::Preference>**](Preference.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_preferences_put

> crate::models::StatusOk users_user_id_preferences_put(user_id, preference)
Save the user's preferences

Save a list of the user's preferences. ##### Permissions Must be logged in as the user being updated or have the `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**preference** | [**Vec<crate::models::Preference>**](Preference.md) | List of preference objects | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

