# \ClusterApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connect_cloud_cluster**](ClusterApi.md#connect_cloud_cluster) | **POST** /api/v1/clusters/cloud_clusters/{provider}/connect | create cluster from cloud cluster
[**create_cloud_cluster**](ClusterApi.md#create_cloud_cluster) | **POST** /api/v1/clusters/cloud_clusters/{provider}/create | create cloud cluster
[**create_kube_cluster**](ClusterApi.md#create_kube_cluster) | **POST** /api/v1/clusters | create cluster
[**create_namespace**](ClusterApi.md#create_namespace) | **POST** /api/v1/clusters/{clusterName}/namespaces | create namespace in cluster
[**delete_cloud_cluster_creation**](ClusterApi.md#delete_cloud_cluster_creation) | **DELETE** /api/v1/clusters/cloud_clusters/{provider}/creation/{cloudClusterName} | delete cloud cluster creation
[**delete_kube_cluster**](ClusterApi.md#delete_kube_cluster) | **DELETE** /api/v1/clusters/{clusterName} | delete cluster
[**get_cloud_cluster_creation_status**](ClusterApi.md#get_cloud_cluster_creation_status) | **GET** /api/v1/clusters/cloud_clusters/{provider}/creation/{cloudClusterName} | check cloud cluster create status
[**get_kube_cluster**](ClusterApi.md#get_kube_cluster) | **GET** /api/v1/clusters/{clusterName} | detail cluster info
[**list_cloud_cluster_creation**](ClusterApi.md#list_cloud_cluster_creation) | **GET** /api/v1/clusters/cloud_clusters/{provider}/creation | list cloud cluster creation
[**list_cloud_clusters**](ClusterApi.md#list_cloud_clusters) | **POST** /api/v1/clusters/cloud_clusters/{provider} | list cloud clusters
[**list_kube_clusters**](ClusterApi.md#list_kube_clusters) | **GET** /api/v1/clusters | list all clusters
[**modify_kube_cluster**](ClusterApi.md#modify_kube_cluster) | **PUT** /api/v1/clusters/{clusterName} | modify cluster



## connect_cloud_cluster

> crate::models::V1PeriodClusterBase connect_cloud_cluster(provider, body)
create cluster from cloud cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | identifier of the cloud provider | [required] |
**body** | [**V1PeriodConnectCloudClusterRequest**](V1PeriodConnectCloudClusterRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodClusterBase**](v1.ClusterBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cloud_cluster

> crate::models::V1PeriodCreateCloudClusterResponse create_cloud_cluster(provider, body)
create cloud cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | identifier of the cloud provider | [required] |
**body** | [**V1PeriodCreateCloudClusterRequest**](V1PeriodCreateCloudClusterRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodCreateCloudClusterResponse**](v1.CreateCloudClusterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_kube_cluster

> crate::models::V1PeriodClusterBase create_kube_cluster(body)
create cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateClusterRequest**](V1PeriodCreateClusterRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodClusterBase**](v1.ClusterBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespace

> crate::models::V1PeriodCreateClusterNamespaceResponse create_namespace(cluster_name, body)
create namespace in cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | name of the target cluster | [required] |
**body** | [**V1PeriodCreateClusterNamespaceRequest**](V1PeriodCreateClusterNamespaceRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodCreateClusterNamespaceResponse**](v1.CreateClusterNamespaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cloud_cluster_creation

> crate::models::V1PeriodCreateCloudClusterResponse delete_cloud_cluster_creation(provider, cloud_cluster_name)
delete cloud cluster creation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | identifier of the cloud provider | [required] |
**cloud_cluster_name** | **String** | identifier for cloud cluster which is creating | [required] |

### Return type

[**crate::models::V1PeriodCreateCloudClusterResponse**](v1.CreateCloudClusterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_kube_cluster

> crate::models::V1PeriodClusterBase delete_kube_cluster(cluster_name)
delete cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | identifier of the cluster | [required] |

### Return type

[**crate::models::V1PeriodClusterBase**](v1.ClusterBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_cluster_creation_status

> crate::models::V1PeriodCreateCloudClusterResponse get_cloud_cluster_creation_status(provider, cloud_cluster_name)
check cloud cluster create status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | identifier of the cloud provider | [required] |
**cloud_cluster_name** | **String** | identifier for cloud cluster which is creating | [required] |

### Return type

[**crate::models::V1PeriodCreateCloudClusterResponse**](v1.CreateCloudClusterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kube_cluster

> crate::models::V1PeriodDetailClusterResponse get_kube_cluster(cluster_name)
detail cluster info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | identifier of the cluster | [required] |

### Return type

[**crate::models::V1PeriodDetailClusterResponse**](v1.DetailClusterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cloud_cluster_creation

> crate::models::V1PeriodListCloudClusterCreationResponse list_cloud_cluster_creation(provider)
list cloud cluster creation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | identifier of the cloud provider | [required] |

### Return type

[**crate::models::V1PeriodListCloudClusterCreationResponse**](v1.ListCloudClusterCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cloud_clusters

> crate::models::V1PeriodListCloudClusterResponse list_cloud_clusters(provider, body, page, page_size)
list cloud clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | identifier of the cloud provider | [required] |
**body** | [**V1PeriodAccessKeyRequest**](V1PeriodAccessKeyRequest.md) |  | [required] |
**page** | Option<**i32**> | Page for paging |  |[default to 0]
**page_size** | Option<**i32**> | PageSize for paging |  |[default to 20]

### Return type

[**crate::models::V1PeriodListCloudClusterResponse**](v1.ListCloudClusterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_kube_clusters

> crate::models::V1PeriodListClusterResponse list_kube_clusters(query, page, page_size)
list all clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Fuzzy search based on name or description |  |
**page** | Option<**i32**> | Page for paging |  |[default to 0]
**page_size** | Option<**i32**> | PageSize for paging |  |[default to 20]

### Return type

[**crate::models::V1PeriodListClusterResponse**](v1.ListClusterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_kube_cluster

> crate::models::V1PeriodClusterBase modify_kube_cluster(cluster_name, body)
modify cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | identifier of the cluster | [required] |
**body** | [**V1PeriodCreateClusterRequest**](V1PeriodCreateClusterRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodClusterBase**](v1.ClusterBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

