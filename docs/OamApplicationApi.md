# \OamApplicationApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_application**](OamApplicationApi.md#create_or_update_application) | **POST** /v1/namespaces/{namespace}/applications/{appname} | create or update oam application in the specified namespace
[**delete_oam_application**](OamApplicationApi.md#delete_oam_application) | **DELETE** /v1/namespaces/{namespace}/applications/{appname} | delete oam application in the specified namespace
[**get_application**](OamApplicationApi.md#get_application) | **GET** /v1/namespaces/{namespace}/applications/{appname} | get the specified oam application in the specified namespace



## create_or_update_application

> create_or_update_application(namespace, appname, body, dry_run, publish_version)
create or update oam application in the specified namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | identifier of the namespace | [required] |
**appname** | **String** | identifier of the oam application | [required] |
**body** | [**V1PeriodApplicationRequest**](V1PeriodApplicationRequest.md) |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**publish_version** | Option<**String**> | The publish version for deploying application.if no specified, {application name}-v{the number of times it was published} (e.g.: demo-app-v13) will be used. the workflow record will use this value as the record identifier, so please ensure that this value is not duplicated if you specify it, otherwise the request will report an error |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_oam_application

> delete_oam_application(namespace, appname)
delete oam application in the specified namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | identifier of the namespace | [required] |
**appname** | **String** | identifier of the oam application | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application

> crate::models::V1PeriodApplicationResponse get_application(namespace, appname)
get the specified oam application in the specified namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | identifier of the namespace | [required] |
**appname** | **String** | identifier of the oam application | [required] |

### Return type

[**crate::models::V1PeriodApplicationResponse**](v1.ApplicationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

