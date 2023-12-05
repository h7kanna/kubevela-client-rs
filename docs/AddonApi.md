# \AddonApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**detail_addon**](AddonApi.md#detail_addon) | **GET** /api/v1/addons/{addonName} | show details of an addon
[**disable_addon**](AddonApi.md#disable_addon) | **POST** /api/v1/addons/{addonName}/disable | disable an addon
[**enable_addon**](AddonApi.md#enable_addon) | **POST** /api/v1/addons/{addonName}/enable | enable an addon
[**list**](AddonApi.md#list) | **GET** /api/v1/enabled_addon | list all enabled addons
[**list_addons**](AddonApi.md#list_addons) | **GET** /api/v1/addons | list all addons
[**status_addon**](AddonApi.md#status_addon) | **GET** /api/v1/addons/{addonName}/status | show status of an addon
[**update_addon**](AddonApi.md#update_addon) | **PUT** /api/v1/addons/{addonName}/update | update an addon



## detail_addon

> crate::models::V1PeriodDetailAddonResponse detail_addon(addon_name, version, registry)
show details of an addon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_name** | **String** | addon name to query detail | [required] |
**version** | Option<**String**> | specify addon version to enable |  |
**registry** | Option<**String**> | filter addons from given registry |  |

### Return type

[**crate::models::V1PeriodDetailAddonResponse**](v1.DetailAddonResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_addon

> crate::models::V1PeriodAddonStatusResponse disable_addon(addon_name, force)
disable an addon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_name** | **String** | addon name to enable | [required] |
**force** | Option<**bool**> | force disable an addon |  |

### Return type

[**crate::models::V1PeriodAddonStatusResponse**](v1.AddonStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_addon

> crate::models::V1PeriodAddonStatusResponse enable_addon(addon_name, body)
enable an addon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_name** | **String** | addon name to enable | [required] |
**body** | [**V1PeriodEnableAddonRequest**](V1PeriodEnableAddonRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodAddonStatusResponse**](v1.AddonStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::V1PeriodListEnabledAddonResponse list(registry, query)
list all enabled addons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry** | Option<**String**> | filter addons from given registry |  |
**query** | Option<**String**> | Fuzzy search based on name and description. |  |

### Return type

[**crate::models::V1PeriodListEnabledAddonResponse**](v1.ListEnabledAddonResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_addons

> crate::models::V1PeriodListAddonResponse list_addons(registry, query)
list all addons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry** | Option<**String**> | filter addons from given registry |  |
**query** | Option<**String**> | Fuzzy search based on name and description. |  |

### Return type

[**crate::models::V1PeriodListAddonResponse**](v1.ListAddonResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_addon

> crate::models::V1PeriodAddonStatusResponse status_addon(addon_name)
show status of an addon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_name** | **String** | addon name to query status | [required] |

### Return type

[**crate::models::V1PeriodAddonStatusResponse**](v1.AddonStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_addon

> crate::models::V1PeriodAddonStatusResponse update_addon(addon_name, body)
update an addon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_name** | **String** | addon name to update | [required] |
**body** | [**V1PeriodEnableAddonRequest**](V1PeriodEnableAddonRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodAddonStatusResponse**](v1.AddonStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

