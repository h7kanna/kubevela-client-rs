# \SystemInfoApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_info**](SystemInfoApi.md#get_system_info) | **GET** /api/v1/system_info | 
[**update_system_info**](SystemInfoApi.md#update_system_info) | **PUT** /api/v1/system_info | 



## get_system_info

> crate::models::V1PeriodSystemInfoResponse get_system_info()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodSystemInfoResponse**](v1.SystemInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_system_info

> crate::models::V1PeriodSystemInfoResponse update_system_info(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodSystemInfoRequest**](V1PeriodSystemInfoRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodSystemInfoResponse**](v1.SystemInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

