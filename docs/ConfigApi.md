# \ConfigApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_config**](ConfigApi.md#create_config) | **POST** /api/v1/configs | create or update a config
[**delete_config**](ConfigApi.md#delete_config) | **DELETE** /api/v1/configs/{configName} | delete a config
[**get_config**](ConfigApi.md#get_config) | **GET** /api/v1/configs/{configName} | detail a config
[**get_config_template**](ConfigApi.md#get_config_template) | **GET** /api/v1/config_templates/{templateName} | Detail a template
[**get_configs**](ConfigApi.md#get_configs) | **GET** /api/v1/configs | list all configs that belong to the system scope
[**list_config_templates**](ConfigApi.md#list_config_templates) | **GET** /api/v1/config_templates | List all config templates from the system namespace
[**update_config**](ConfigApi.md#update_config) | **PUT** /api/v1/configs/{configName} | update a config



## create_config

> crate::models::V1PeriodConfig create_config(body)
create or update a config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateConfigRequest**](V1PeriodCreateConfigRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodConfig**](v1.Config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_config

> serde_json::Value delete_config(config_name)
delete a config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** | identifier of the config | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config

> Vec<serde_json::Value> get_config(config_name)
detail a config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** | identifier of the config | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config_template

> crate::models::V1PeriodConfigTemplateDetail get_config_template(template_name, namespace)
Detail a template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_name** | **String** | identifier of the config template | [required] |
**namespace** | Option<**String**> | the name of the namespace |  |

### Return type

[**crate::models::V1PeriodConfigTemplateDetail**](v1.ConfigTemplateDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configs

> crate::models::V1PeriodListConfigResponse get_configs(template)
list all configs that belong to the system scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template** | Option<**String**> | the name of the template |  |

### Return type

[**crate::models::V1PeriodListConfigResponse**](v1.ListConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_config_templates

> crate::models::V1PeriodListConfigTemplateResponse list_config_templates()
List all config templates from the system namespace

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodListConfigTemplateResponse**](v1.ListConfigTemplateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config

> Vec<serde_json::Value> update_config(config_name)
update a config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** | identifier of the config | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

