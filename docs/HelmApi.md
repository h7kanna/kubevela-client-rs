# \HelmApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chart_values**](HelmApi.md#chart_values) | **GET** /api/v1/repository/chart/values | get chart value
[**get_chart_values**](HelmApi.md#get_chart_values) | **GET** /api/v1/repository/charts/{chart}/versions/{version}/values | get chart value
[**get_image_info**](HelmApi.md#get_image_info) | **GET** /api/v1/repository/image/info | get the oci repos
[**get_image_repos**](HelmApi.md#get_image_repos) | **GET** /api/v1/repository/image/repos | get the oci repos
[**list_chart_versions**](HelmApi.md#list_chart_versions) | **GET** /api/v1/repository/charts/{chart}/versions | list versions
[**list_charts**](HelmApi.md#list_charts) | **GET** /api/v1/repository/charts | list charts
[**list_repo**](HelmApi.md#list_repo) | **GET** /api/v1/repository/chart_repos | list chart repo
[**list_versions_from_query**](HelmApi.md#list_versions_from_query) | **GET** /api/v1/repository/chart/versions | list versions



## chart_values

> String chart_values(project, chart, version, repo_url, repo_type, secret_name)
get chart value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | the config project | [required] |
**chart** | **String** | helm chart | [required] |
**version** | **String** | helm chart version | [required] |
**repo_url** | **String** | helm repository url | [required] |
**repo_type** | **String** | helm repository type | [required] |
**secret_name** | Option<**String**> | secret of the repo |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chart_values

> serde_json::Value get_chart_values(project, chart, version, repo_url, secret_name)
get chart value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | the config project | [required] |
**chart** | **String** | identifier of the helm chart | [required] |
**version** | **String** | version of the helm chart | [required] |
**repo_url** | Option<**String**> | helm repository url |  |
**secret_name** | Option<**String**> | secret of the repo |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_info

> crate::models::V1PeriodImageInfo get_image_info(project, name, secret_name)
get the oci repos

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | the config project | [required] |
**name** | **String** | the image name | [required] |
**secret_name** | Option<**String**> | the secret name of the image repository |  |

### Return type

[**crate::models::V1PeriodImageInfo**](v1.ImageInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_repos

> crate::models::V1PeriodListImageRegistryResponse get_image_repos(project)
get the oci repos

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | the config project | [required] |

### Return type

[**crate::models::V1PeriodListImageRegistryResponse**](v1.ListImageRegistryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_chart_versions

> crate::models::V1PeriodChartVersionListResponse list_chart_versions(project, chart, repo_url, secret_name)
list versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | the config project | [required] |
**chart** | **String** | identifier of the helm chart | [required] |
**repo_url** | Option<**String**> | helm repository url |  |
**secret_name** | Option<**String**> | secret of the repo |  |

### Return type

[**crate::models::V1PeriodChartVersionListResponse**](v1.ChartVersionListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_charts

> Vec<String> list_charts(project, repo_url, secret_name)
list charts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | the config project | [required] |
**repo_url** | Option<**String**> | helm repository url |  |
**secret_name** | Option<**String**> | secret of the repo |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_repo

> Vec<String> list_repo(project)
list chart repo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | the config project | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_versions_from_query

> crate::models::V1PeriodChartVersionListResponse list_versions_from_query(project, chart, repo_url, secret_name)
list versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | the config project | [required] |
**chart** | **String** | helm chart | [required] |
**repo_url** | **String** | helm repository url | [required] |
**secret_name** | Option<**String**> | secret of the repo |  |

### Return type

[**crate::models::V1PeriodChartVersionListResponse**](v1.ChartVersionListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

