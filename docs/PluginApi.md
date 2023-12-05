# \PluginApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**detail_plugin**](PluginApi.md#detail_plugin) | **GET** /api/v1/manage/plugins/{pluginId} | Detail an installed plugin
[**detail_plugin_0**](PluginApi.md#detail_plugin_0) | **GET** /api/v1/plugins/{pluginId} | Detail an installed plugin
[**disable_plugin**](PluginApi.md#disable_plugin) | **POST** /api/v1/manage/plugins/{pluginId}/disable | Disable an installed plugin
[**enable_plugin**](PluginApi.md#enable_plugin) | **POST** /api/v1/manage/plugins/{pluginId}/enable | Enable an installed plugin
[**install_plugin**](PluginApi.md#install_plugin) | **POST** /api/v1/manage/plugins/{pluginId}/install | Install one specific plugin
[**list_enabled_plugins**](PluginApi.md#list_enabled_plugins) | **GET** /api/v1/plugins | List the enabled plugins
[**list_installed_plugins**](PluginApi.md#list_installed_plugins) | **GET** /api/v1/manage/plugins | List the installed plugins
[**plugin_setting**](PluginApi.md#plugin_setting) | **POST** /api/v1/manage/plugins/{pluginId}/setting | Set an installed plugin
[**uninstall_plugin**](PluginApi.md#uninstall_plugin) | **POST** /api/v1/manage/plugins/{pluginId}/uninstall | Uninstall one specific plugin



## detail_plugin

> crate::models::V1PeriodManagedPluginDto detail_plugin(plugin_id)
Detail an installed plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | identifier of the plugin. | [required] |

### Return type

[**crate::models::V1PeriodManagedPluginDto**](v1.ManagedPluginDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_plugin_0

> crate::models::V1PeriodPluginDto detail_plugin_0(plugin_id)
Detail an installed plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | identifier of the plugin. | [required] |

### Return type

[**crate::models::V1PeriodPluginDto**](v1.PluginDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_plugin

> crate::models::V1PeriodManagedPluginDto disable_plugin(plugin_id)
Disable an installed plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | identifier of the plugin. | [required] |

### Return type

[**crate::models::V1PeriodManagedPluginDto**](v1.ManagedPluginDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_plugin

> crate::models::V1PeriodManagedPluginDto enable_plugin(plugin_id, body)
Enable an installed plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | identifier of the plugin. | [required] |
**body** | [**V1PeriodPluginEnableRequest**](V1PeriodPluginEnableRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodManagedPluginDto**](v1.ManagedPluginDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_plugin

> crate::models::V1PeriodManagedPluginDto install_plugin(plugin_id, body)
Install one specific plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | identifier of the plugin. | [required] |
**body** | [**V1PeriodInstallPluginRequest**](V1PeriodInstallPluginRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodManagedPluginDto**](v1.ManagedPluginDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_enabled_plugins

> crate::models::V1PeriodListPluginResponse list_enabled_plugins()
List the enabled plugins

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodListPluginResponse**](v1.ListPluginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_installed_plugins

> crate::models::V1PeriodListPluginResponse list_installed_plugins()
List the installed plugins

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodListPluginResponse**](v1.ListPluginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_setting

> crate::models::V1PeriodManagedPluginDto plugin_setting(plugin_id)
Set an installed plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | identifier of the plugin. | [required] |

### Return type

[**crate::models::V1PeriodManagedPluginDto**](v1.ManagedPluginDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_plugin

> serde_json::Value uninstall_plugin(plugin_id)
Uninstall one specific plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | identifier of the plugin. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

