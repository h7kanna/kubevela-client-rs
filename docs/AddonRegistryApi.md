# \AddonRegistryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_addon_registry**](AddonRegistryApi.md#create_addon_registry) | **POST** /api/v1/addon_registries | create an addon registry
[**delete_addon_registry**](AddonRegistryApi.md#delete_addon_registry) | **DELETE** /api/v1/addon_registries/{addonRegName} | delete an addon registry
[**list_addon_registry**](AddonRegistryApi.md#list_addon_registry) | **GET** /api/v1/addon_registries | list all addon registry
[**update_addon_registry**](AddonRegistryApi.md#update_addon_registry) | **PUT** /api/v1/addon_registries/{addonRegName} | update an addon registry



## create_addon_registry

> crate::models::V1PeriodAddonRegistry create_addon_registry(body)
create an addon registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateAddonRegistryRequest**](V1PeriodCreateAddonRegistryRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodAddonRegistry**](v1.AddonRegistry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_addon_registry

> crate::models::V1PeriodAddonRegistry delete_addon_registry(addon_reg_name)
delete an addon registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_reg_name** | **String** | identifier of the addon registry | [required] |

### Return type

[**crate::models::V1PeriodAddonRegistry**](v1.AddonRegistry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_addon_registry

> crate::models::V1PeriodListAddonRegistryResponse list_addon_registry()
list all addon registry

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodListAddonRegistryResponse**](v1.ListAddonRegistryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_addon_registry

> crate::models::V1PeriodAddonRegistry update_addon_registry(addon_reg_name, body)
update an addon registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_reg_name** | **String** | identifier of the addon registry | [required] |
**body** | [**V1PeriodUpdateAddonRegistryRequest**](V1PeriodUpdateAddonRegistryRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodAddonRegistry**](v1.AddonRegistry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

