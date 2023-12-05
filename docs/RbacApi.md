# \RbacApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_platform_permission**](RbacApi.md#create_platform_permission) | **POST** /api/v1/permissions | create the platform perm policy
[**create_platform_role**](RbacApi.md#create_platform_role) | **POST** /api/v1/roles | create platform level role
[**delete_platform_permission**](RbacApi.md#delete_platform_permission) | **DELETE** /api/v1/permissions/{permissionName} | delete a platform perm policy
[**delete_platform_role**](RbacApi.md#delete_platform_role) | **DELETE** /api/v1/roles/{roleName} | update platform level role
[**list_platform_permissions**](RbacApi.md#list_platform_permissions) | **GET** /api/v1/permissions | list all platform level perm policies
[**list_platform_roles**](RbacApi.md#list_platform_roles) | **GET** /api/v1/roles | list all platform level roles
[**update_platform_role**](RbacApi.md#update_platform_role) | **PUT** /api/v1/roles/{roleName} | update platform level role



## create_platform_permission

> crate::models::V1PeriodPermissionBase create_platform_permission(body)
create the platform perm policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreatePermissionRequest**](V1PeriodCreatePermissionRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodPermissionBase**](v1.PermissionBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_platform_role

> crate::models::V1PeriodRoleBase create_platform_role(body)
create platform level role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateRoleRequest**](V1PeriodCreateRoleRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodRoleBase**](v1.RoleBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_platform_permission

> serde_json::Value delete_platform_permission(permission_name)
delete a platform perm policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_name** | **String** | identifier of the permission | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_platform_role

> serde_json::Value delete_platform_role(role_name)
update platform level role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** | identifier of the role | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_platform_permissions

> Vec<crate::models::V1PeriodPermissionBase> list_platform_permissions()
list all platform level perm policies

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::V1PeriodPermissionBase>**](v1.PermissionBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_platform_roles

> crate::models::V1PeriodListRolesResponse list_platform_roles()
list all platform level roles

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodListRolesResponse**](v1.ListRolesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_platform_role

> crate::models::V1PeriodRoleBase update_platform_role(role_name, body)
update platform level role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** | identifier of the role | [required] |
**body** | [**V1PeriodUpdateRoleRequest**](V1PeriodUpdateRoleRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodRoleBase**](v1.RoleBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

