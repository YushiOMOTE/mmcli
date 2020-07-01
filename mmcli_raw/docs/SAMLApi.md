# \SAMLApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**saml_certificate_idp_delete**](SAMLApi.md#saml_certificate_idp_delete) | **delete** /saml/certificate/idp | Remove IDP certificate
[**saml_certificate_idp_post**](SAMLApi.md#saml_certificate_idp_post) | **post** /saml/certificate/idp | Upload IDP certificate
[**saml_certificate_private_delete**](SAMLApi.md#saml_certificate_private_delete) | **delete** /saml/certificate/private | Remove private key
[**saml_certificate_private_post**](SAMLApi.md#saml_certificate_private_post) | **post** /saml/certificate/private | Upload private key
[**saml_certificate_public_delete**](SAMLApi.md#saml_certificate_public_delete) | **delete** /saml/certificate/public | Remove public certificate
[**saml_certificate_public_post**](SAMLApi.md#saml_certificate_public_post) | **post** /saml/certificate/public | Upload public certificate
[**saml_certificate_status_get**](SAMLApi.md#saml_certificate_status_get) | **get** /saml/certificate/status | Get certificate status
[**saml_metadata_get**](SAMLApi.md#saml_metadata_get) | **get** /saml/metadata | Get metadata
[**saml_metadatafromidp_post**](SAMLApi.md#saml_metadatafromidp_post) | **post** /saml/metadatafromidp | Get metadata from Identity Provider



## saml_certificate_idp_delete

> crate::models::StatusOk saml_certificate_idp_delete()
Remove IDP certificate

Delete the current IDP certificate being used with your SAML configuration. This will also disable SAML on your system as this certificate is required for SAML. ##### Permissions Must have `manage_system` permission. 

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


## saml_certificate_idp_post

> crate::models::StatusOk saml_certificate_idp_post(certificate)
Upload IDP certificate

Upload the IDP certificate to be used with your SAML configuration. The server will pick a hard-coded filename for the IdpCertificateFile setting in your `config.json`. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate** | **std::path::PathBuf** | The IDP certificate file | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## saml_certificate_private_delete

> crate::models::StatusOk saml_certificate_private_delete()
Remove private key

Delete the current private key being used with your SAML configuration. This will also disable encryption for SAML on your system as this key is required for that. ##### Permissions Must have `manage_system` permission. 

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


## saml_certificate_private_post

> crate::models::StatusOk saml_certificate_private_post(certificate)
Upload private key

Upload the private key to be used for encryption with your SAML configuration. The server will pick a hard-coded filename for the PrivateKeyFile setting in your `config.json`. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate** | **std::path::PathBuf** | The private key file | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## saml_certificate_public_delete

> crate::models::StatusOk saml_certificate_public_delete()
Remove public certificate

Delete the current public certificate being used with your SAML configuration. This will also disable encryption for SAML on your system as this certificate is required for that. ##### Permissions Must have `manage_system` permission. 

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


## saml_certificate_public_post

> crate::models::StatusOk saml_certificate_public_post(certificate)
Upload public certificate

Upload the public certificate to be used for encryption with your SAML configuration. The server will pick a hard-coded filename for the PublicCertificateFile setting in your `config.json`. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate** | **std::path::PathBuf** | The public certificate file | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## saml_certificate_status_get

> crate::models::SamlCertificateStatus saml_certificate_status_get()
Get certificate status

Get the status of the uploaded certificates and keys in use by your SAML configuration. ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SamlCertificateStatus**](SamlCertificateStatus.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## saml_metadata_get

> String saml_metadata_get()
Get metadata

Get SAML metadata from the server. SAML must be configured properly. ##### Permissions No permission required. 

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## saml_metadatafromidp_post

> String saml_metadatafromidp_post()
Get metadata from Identity Provider

Get SAML metadata from the Identity Provider. SAML must be configured properly. ##### Permissions No permission required. 

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

