# \DefinitionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**detail_definition**](DefinitionApi.md#detail_definition) | **GET** /api/v1/definitions/{definitionName} | Detail a definition
[**list_definitions**](DefinitionApi.md#list_definitions) | **GET** /api/v1/definitions | list all definitions
[**update_definition_status**](DefinitionApi.md#update_definition_status) | **PUT** /api/v1/definitions/{definitionName}/status | Update the status for a definition
[**update_ui_schema**](DefinitionApi.md#update_ui_schema) | **PUT** /api/v1/definitions/{definitionName}/uischema | Update the UI schema for a definition



## detail_definition

> crate::models::V1PeriodSimpleResponse detail_definition(definition_name, r#type)
Detail a definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**definition_name** | **String** | identifier of the definition | [required] |
**r#type** | Option<**String**> | query the definition type |  |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_definitions

> crate::models::V1PeriodSimpleResponse list_definitions(r#type, query_all, applied_workload, owner_addon, scope)
list all definitions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | query the definition type | [required] |
**query_all** | Option<**bool**> | query all definitions include hidden in UI |  |[default to false]
**applied_workload** | Option<**String**> | if specified, query the trait definition applied to the workload |  |
**owner_addon** | Option<**String**> | query by which addon created the definition |  |
**scope** | Option<**String**> | query by the specified scope like WorkflowRun or Application |  |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_definition_status

> crate::models::V1PeriodSimpleResponse update_definition_status(definition_name, body)
Update the status for a definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**definition_name** | **String** | identifier of the definition | [required] |
**body** | [**V1PeriodUpdateDefinitionStatusRequest**](V1PeriodUpdateDefinitionStatusRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ui_schema

> crate::models::V1PeriodSimpleResponse update_ui_schema(definition_name, body)
Update the UI schema for a definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**definition_name** | **String** | identifier of the definition | [required] |
**body** | [**V1PeriodUpdateUiSchemaRequest**](V1PeriodUpdateUiSchemaRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

