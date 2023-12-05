# \TargetApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_target**](TargetApi.md#create_target) | **POST** /api/v1/targets | create Target
[**delete_target**](TargetApi.md#delete_target) | **DELETE** /api/v1/targets/{targetName} | delete Target
[**detail_target**](TargetApi.md#detail_target) | **GET** /api/v1/targets/{targetName} | detail Target
[**list_targets**](TargetApi.md#list_targets) | **GET** /api/v1/targets | list Target
[**update_target**](TargetApi.md#update_target) | **PUT** /api/v1/targets/{targetName} | update application Target config



## create_target

> crate::models::V1PeriodDetailTargetResponse create_target(body)
create Target

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateTargetRequest**](V1PeriodCreateTargetRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodDetailTargetResponse**](v1.DetailTargetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_target

> crate::models::V1PeriodSimpleResponse delete_target(target_name)
delete Target

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_name** | **String** | identifier of the Target | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_target

> crate::models::V1PeriodDetailTargetResponse detail_target(target_name)
detail Target

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_name** | **String** | identifier of the Target. | [required] |

### Return type

[**crate::models::V1PeriodDetailTargetResponse**](v1.DetailTargetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_targets

> crate::models::V1PeriodListTargetResponse list_targets(page, page_size, project)
list Target

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page for paging |  |
**page_size** | Option<**i32**> | PageSize for paging |  |
**project** | Option<**String**> | list targets by project name |  |

### Return type

[**crate::models::V1PeriodListTargetResponse**](v1.ListTargetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_target

> crate::models::V1PeriodDetailTargetResponse update_target(target_name, body)
update application Target config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_name** | **String** | identifier of the Target | [required] |
**body** | [**V1PeriodUpdateTargetRequest**](V1PeriodUpdateTargetRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodDetailTargetResponse**](v1.DetailTargetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

