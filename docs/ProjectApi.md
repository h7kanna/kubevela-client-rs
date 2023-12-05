# \ProjectApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_distribution**](ProjectApi.md#apply_distribution) | **POST** /api/v1/projects/{projectName}/distributions | apply the distribution job of the config
[**create_project**](ProjectApi.md#create_project) | **POST** /api/v1/projects | create a project
[**create_project_config**](ProjectApi.md#create_project_config) | **POST** /api/v1/projects/{projectName}/configs | create a config in a project
[**create_project_permission**](ProjectApi.md#create_project_permission) | **POST** /api/v1/projects/{projectName}/permissions | create a project level perm policy
[**create_project_role**](ProjectApi.md#create_project_role) | **POST** /api/v1/projects/{projectName}/roles | create project level role
[**create_project_user**](ProjectApi.md#create_project_user) | **POST** /api/v1/projects/{projectName}/users | add a user to a project
[**delete_distribution**](ProjectApi.md#delete_distribution) | **DELETE** /api/v1/projects/{projectName}/distributions/{distributionName} | delete a distribution job of the config
[**delete_project**](ProjectApi.md#delete_project) | **DELETE** /api/v1/projects/{projectName} | delete a project
[**delete_project_config**](ProjectApi.md#delete_project_config) | **DELETE** /api/v1/projects/{projectName}/configs/{configName} | delete a config from a project
[**delete_project_permission**](ProjectApi.md#delete_project_permission) | **DELETE** /api/v1/projects/{projectName}/permissions/{permissionName} | delete a project level perm policy
[**delete_project_role**](ProjectApi.md#delete_project_role) | **DELETE** /api/v1/projects/{projectName}/roles/{roleName} | delete project level role
[**delete_project_user**](ProjectApi.md#delete_project_user) | **DELETE** /api/v1/projects/{projectName}/users/{userName} | delete a user from a project
[**detail_config**](ProjectApi.md#detail_config) | **GET** /api/v1/projects/{projectName}/configs/{configName} | detail a config in a project
[**detail_project**](ProjectApi.md#detail_project) | **GET** /api/v1/projects/{projectName} | detail a project
[**get_config_template_by_template_name**](ProjectApi.md#get_config_template_by_template_name) | **GET** /api/v1/projects/{projectName}/config_templates/{templateName} | Detail a template
[**get_config_templates**](ProjectApi.md#get_config_templates) | **GET** /api/v1/projects/{projectName}/config_templates | get the templates which are in a project
[**get_project_configs**](ProjectApi.md#get_project_configs) | **GET** /api/v1/projects/{projectName}/configs | get configs which are in a project
[**get_providers**](ProjectApi.md#get_providers) | **GET** /api/v1/projects/{projectName}/providers | get providers which are in a project
[**list_distributions**](ProjectApi.md#list_distributions) | **GET** /api/v1/projects/{projectName}/distributions | list the distribution jobs of the config
[**list_project_permissions**](ProjectApi.md#list_project_permissions) | **GET** /api/v1/projects/{projectName}/permissions | list all project level perm policies
[**list_project_roles**](ProjectApi.md#list_project_roles) | **GET** /api/v1/projects/{projectName}/roles | list all project level roles
[**list_project_targets**](ProjectApi.md#list_project_targets) | **GET** /api/v1/projects/{projectName}/targets | get targets list belong to a project
[**list_project_user**](ProjectApi.md#list_project_user) | **GET** /api/v1/projects/{projectName}/users | list all users belong to a project
[**list_projects**](ProjectApi.md#list_projects) | **GET** /api/v1/projects | list all projects
[**update_project**](ProjectApi.md#update_project) | **PUT** /api/v1/projects/{projectName} | update a project
[**update_project_config**](ProjectApi.md#update_project_config) | **PUT** /api/v1/projects/{projectName}/configs/{configName} | update a config in a project
[**update_project_role**](ProjectApi.md#update_project_role) | **PUT** /api/v1/projects/{projectName}/roles/{roleName} | update project level role
[**update_project_user**](ProjectApi.md#update_project_user) | **PUT** /api/v1/projects/{projectName}/users/{userName} | update a user from a project



## apply_distribution

> serde_json::Value apply_distribution(project_name, body)
apply the distribution job of the config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**body** | [**V1PeriodCreateConfigDistributionRequest**](V1PeriodCreateConfigDistributionRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project

> crate::models::V1PeriodProjectBase create_project(body)
create a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateProjectRequest**](V1PeriodCreateProjectRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodProjectBase**](v1.ProjectBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_config

> crate::models::V1PeriodConfig create_project_config(project_name, body)
create a config in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**body** | [**V1PeriodCreateConfigRequest**](V1PeriodCreateConfigRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodConfig**](v1.Config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_permission

> Vec<crate::models::V1PeriodPermissionBase> create_project_permission(project_name)
create a project level perm policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**Vec<crate::models::V1PeriodPermissionBase>**](v1.PermissionBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_role

> crate::models::V1PeriodRoleBase create_project_role(project_name, body)
create project level role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**body** | [**V1PeriodCreateRoleRequest**](V1PeriodCreateRoleRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodRoleBase**](v1.RoleBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_user

> crate::models::V1PeriodProjectUserBase create_project_user(project_name, body)
add a user to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**body** | [**V1PeriodAddProjectUserRequest**](V1PeriodAddProjectUserRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodProjectUserBase**](v1.ProjectUserBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_distribution

> serde_json::Value delete_distribution(project_name, distribution_name)
delete a distribution job of the config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**distribution_name** | **String** | identifier of the distribution | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> serde_json::Value delete_project(project_name)
delete a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_config

> serde_json::Value delete_project_config(project_name, config_name)
delete a config from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**config_name** | **String** | identifier of the config | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_permission

> Vec<crate::models::V1PeriodPermissionBase> delete_project_permission(project_name, permission_name)
delete a project level perm policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**permission_name** | **String** | identifier of the permission | [required] |

### Return type

[**Vec<crate::models::V1PeriodPermissionBase>**](v1.PermissionBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_role

> serde_json::Value delete_project_role(project_name, role_name)
delete project level role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**role_name** | **String** | identifier of the project role | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_user

> serde_json::Value delete_project_user(project_name, user_name, body)
delete a user from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**user_name** | **String** | identifier of the project user | [required] |
**body** | [**V1PeriodUpdateProjectUserRequest**](V1PeriodUpdateProjectUserRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_config

> crate::models::V1PeriodConfig detail_config(project_name, config_name, body)
detail a config in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**config_name** | **String** | identifier of the config | [required] |
**body** | [**V1PeriodUpdateConfigRequest**](V1PeriodUpdateConfigRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodConfig**](v1.Config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_project

> crate::models::V1PeriodProjectBase detail_project(project_name)
detail a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**crate::models::V1PeriodProjectBase**](v1.ProjectBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config_template_by_template_name

> crate::models::V1PeriodConfigTemplateDetail get_config_template_by_template_name(project_name, template_name, namespace)
Detail a template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
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


## get_config_templates

> crate::models::V1PeriodListConfigTemplateResponse get_config_templates(project_name, namespace)
get the templates which are in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**namespace** | **String** | the namespace of the template | [required] |

### Return type

[**crate::models::V1PeriodListConfigTemplateResponse**](v1.ListConfigTemplateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_configs

> crate::models::V1PeriodListConfigResponse get_project_configs(project_name, template)
get configs which are in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**template** | Option<**String**> | the template name |  |

### Return type

[**crate::models::V1PeriodListConfigResponse**](v1.ListConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_providers

> crate::models::V1PeriodListTerraformProviderResponse get_providers(project_name)
get providers which are in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**crate::models::V1PeriodListTerraformProviderResponse**](v1.ListTerraformProviderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_distributions

> crate::models::V1PeriodListConfigDistributionResponse list_distributions(project_name)
list the distribution jobs of the config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**crate::models::V1PeriodListConfigDistributionResponse**](v1.ListConfigDistributionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_permissions

> Vec<crate::models::V1PeriodPermissionBase> list_project_permissions(project_name)
list all project level perm policies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**Vec<crate::models::V1PeriodPermissionBase>**](v1.PermissionBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_roles

> crate::models::V1PeriodListRolesResponse list_project_roles(project_name)
list all project level roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**crate::models::V1PeriodListRolesResponse**](v1.ListRolesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_targets

> serde_json::Value list_project_targets(project_name)
get targets list belong to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_user

> crate::models::V1PeriodListProjectUsersResponse list_project_user(project_name)
list all users belong to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |

### Return type

[**crate::models::V1PeriodListProjectUsersResponse**](v1.ListProjectUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_projects

> crate::models::V1PeriodListProjectResponse list_projects()
list all projects

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodListProjectResponse**](v1.ListProjectResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> crate::models::V1PeriodProjectBase update_project(project_name, body)
update a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**body** | [**V1PeriodUpdateProjectRequest**](V1PeriodUpdateProjectRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodProjectBase**](v1.ProjectBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_config

> crate::models::V1PeriodConfig update_project_config(project_name, config_name, body)
update a config in a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**config_name** | **String** | identifier of the config | [required] |
**body** | [**V1PeriodUpdateConfigRequest**](V1PeriodUpdateConfigRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodConfig**](v1.Config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_role

> crate::models::V1PeriodRoleBase update_project_role(project_name, role_name, body)
update project level role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**role_name** | **String** | identifier of the project role | [required] |
**body** | [**V1PeriodUpdateRoleRequest**](V1PeriodUpdateRoleRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodRoleBase**](v1.RoleBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_user

> crate::models::V1PeriodProjectUserBase update_project_user(project_name, user_name, body)
update a user from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | identifier of the project | [required] |
**user_name** | **String** | identifier of the project user | [required] |
**body** | [**V1PeriodUpdateProjectUserRequest**](V1PeriodUpdateProjectUserRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodProjectUserBase**](v1.ProjectUserBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

