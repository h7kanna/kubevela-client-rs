# \EnvApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**envcreate**](EnvApi.md#envcreate) | **POST** /api/v1/envs | create an env
[**envdelete**](EnvApi.md#envdelete) | **DELETE** /api/v1/envs/{envName} | delete one env
[**envlist**](EnvApi.md#envlist) | **GET** /api/v1/envs | list all envs
[**envupdate**](EnvApi.md#envupdate) | **PUT** /api/v1/envs/{envName} | update an env



## envcreate

> crate::models::V1PeriodEnv envcreate(body)
create an env

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateEnvRequest**](V1PeriodCreateEnvRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodEnv**](v1.Env.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## envdelete

> serde_json::Value envdelete(env_name)
delete one env

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**env_name** | **String** | identifier of the environment | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## envlist

> crate::models::V1PeriodListEnvResponse envlist()
list all envs

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodListEnvResponse**](v1.ListEnvResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## envupdate

> crate::models::V1PeriodEnv envupdate(env_name, body)
update an env

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**env_name** | **String** | identifier of the environment | [required] |
**body** | [**V1PeriodCreateEnvRequest**](V1PeriodCreateEnvRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodEnv**](v1.Env.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

