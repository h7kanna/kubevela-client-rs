# \CloudshellApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**destroy_cloud_shell**](CloudshellApi.md#destroy_cloud_shell) | **DELETE** /api/v1/cloudshell | destroy the user's cloud shell environment
[**prepare_cloud_shell**](CloudshellApi.md#prepare_cloud_shell) | **POST** /api/v1/cloudshell | prepare the user's cloud shell environment
[**proxy**](CloudshellApi.md#proxy) | **GET** /view/cloudshell | prepare the user's cloud shell environment
[**proxy_path**](CloudshellApi.md#proxy_path) | **GET** /view/cloudshell/{subpath} | prepare the user's cloud shell environment



## destroy_cloud_shell

> crate::models::V1PeriodSimpleResponse destroy_cloud_shell()
destroy the user's cloud shell environment

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_cloud_shell

> crate::models::V1PeriodCloudShellPrepareResponse prepare_cloud_shell()
prepare the user's cloud shell environment

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodCloudShellPrepareResponse**](v1.CloudShellPrepareResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy

> crate::models::V1PeriodSimpleResponse proxy()
prepare the user's cloud shell environment

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_path

> crate::models::V1PeriodSimpleResponse proxy_path(subpath)
prepare the user's cloud shell environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subpath** | **String** | subpath | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

