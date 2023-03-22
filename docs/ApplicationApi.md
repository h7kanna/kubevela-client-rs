# \ApplicationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_application_trait**](ApplicationApi.md#add_application_trait) | **POST** /api/v1/applications/{appName}/components/{compName}/traits | add trait for a component
[**application_statistics**](ApplicationApi.md#application_statistics) | **GET** /api/v1/applications/{appName}/statistics | detail one application 
[**compare_app**](ApplicationApi.md#compare_app) | **POST** /api/v1/applications/{appName}/compare | compare application
[**create_application**](ApplicationApi.md#create_application) | **POST** /api/v1/applications | create one application 
[**create_application_env**](ApplicationApi.md#create_application_env) | **POST** /api/v1/applications/{appName}/envs | creating an application environment 
[**create_application_policy**](ApplicationApi.md#create_application_policy) | **POST** /api/v1/applications/{appName}/policies | create policy for application
[**create_application_trigger**](ApplicationApi.md#create_application_trigger) | **POST** /api/v1/applications/{appName}/triggers | Create an application trigger
[**create_component**](ApplicationApi.md#create_component) | **POST** /api/v1/applications/{appName}/components | create component  for application 
[**create_or_update_application_workflow**](ApplicationApi.md#create_or_update_application_workflow) | **POST** /api/v1/applications/{appName}/workflows | create application workflow
[**delete_application**](ApplicationApi.md#delete_application) | **DELETE** /api/v1/applications/{appName} | delete one application
[**delete_application_env**](ApplicationApi.md#delete_application_env) | **DELETE** /api/v1/applications/{appName}/envs/{envName} | delete an application environment 
[**delete_application_policy**](ApplicationApi.md#delete_application_policy) | **DELETE** /api/v1/applications/{appName}/policies/{policyName} | detail policy for application
[**delete_application_trait**](ApplicationApi.md#delete_application_trait) | **DELETE** /api/v1/applications/{appName}/components/{compName}/traits/{traitType} | delete trait from a component
[**delete_application_trigger**](ApplicationApi.md#delete_application_trigger) | **DELETE** /api/v1/applications/{appName}/triggers/{token} | Delete an application trigger
[**delete_component**](ApplicationApi.md#delete_component) | **DELETE** /api/v1/applications/{appName}/components/{compName} | delete a component
[**delete_workflow**](ApplicationApi.md#delete_workflow) | **DELETE** /api/v1/applications/{appName}/workflows/{workflowName} | deletet workflow
[**deploy_application**](ApplicationApi.md#deploy_application) | **POST** /api/v1/applications/{appName}/deploy | deploy or upgrade the application
[**detail_application**](ApplicationApi.md#detail_application) | **GET** /api/v1/applications/{appName} | detail one application 
[**detail_application_policy**](ApplicationApi.md#detail_application_policy) | **GET** /api/v1/applications/{appName}/policies/{policyName} | detail policy for application
[**detail_application_revision**](ApplicationApi.md#detail_application_revision) | **GET** /api/v1/applications/{appName}/revisions/{revision} | detail revision for application
[**detail_component**](ApplicationApi.md#detail_component) | **GET** /api/v1/applications/{appName}/components/{compName} | detail component for application 
[**detail_workflow**](ApplicationApi.md#detail_workflow) | **GET** /api/v1/applications/{appName}/workflows/{workflowName} | detail application workflow
[**detail_workflow_record**](ApplicationApi.md#detail_workflow_record) | **GET** /api/v1/applications/{appName}/workflows/{workflowName}/records/{record} | query application workflow execution record detail
[**dry_run_app_or_revision**](ApplicationApi.md#dry_run_app_or_revision) | **POST** /api/v1/applications/{appName}/dry-run | dry-run application to latest revision
[**get_application_status**](ApplicationApi.md#get_application_status) | **GET** /api/v1/applications/{appName}/envs/{envName}/status | get application status
[**get_workflow_record_inputs**](ApplicationApi.md#get_workflow_record_inputs) | **GET** /api/v1/applications/{appName}/workflows/{workflowName}/records/{record}/inputs | get the workflow step inputs
[**get_workflow_record_logs**](ApplicationApi.md#get_workflow_record_logs) | **GET** /api/v1/applications/{appName}/workflows/{workflowName}/records/{record}/logs | get the workflow step logs
[**get_workflow_record_outputs**](ApplicationApi.md#get_workflow_record_outputs) | **GET** /api/v1/applications/{appName}/workflows/{workflowName}/records/{record}/outputs | get the workflow step inputs
[**list_application_components**](ApplicationApi.md#list_application_components) | **GET** /api/v1/applications/{appName}/components | gets the list of application components
[**list_application_envs**](ApplicationApi.md#list_application_envs) | **GET** /api/v1/applications/{appName}/envs | list policy for application
[**list_application_policies**](ApplicationApi.md#list_application_policies) | **GET** /api/v1/applications/{appName}/policies | list policy for application
[**list_application_records**](ApplicationApi.md#list_application_records) | **GET** /api/v1/applications/{appName}/records | list application records
[**list_application_revisions**](ApplicationApi.md#list_application_revisions) | **GET** /api/v1/applications/{appName}/revisions | list revisions for application
[**list_application_triggers**](ApplicationApi.md#list_application_triggers) | **GET** /api/v1/applications/{appName}/triggers | List the application triggers
[**list_application_workflows**](ApplicationApi.md#list_application_workflows) | **GET** /api/v1/applications/{appName}/workflows | list application workflow
[**list_applications**](ApplicationApi.md#list_applications) | **GET** /api/v1/applications | list all applications
[**list_workflow_records**](ApplicationApi.md#list_workflow_records) | **GET** /api/v1/applications/{appName}/workflows/{workflowName}/records | query application workflow execution record
[**publish_application_template**](ApplicationApi.md#publish_application_template) | **POST** /api/v1/applications/{appName}/template | create one application template
[**recycle_application_env**](ApplicationApi.md#recycle_application_env) | **POST** /api/v1/applications/{appName}/envs/{envName}/recycle | recycle application env
[**reset_app_to_latest_revision**](ApplicationApi.md#reset_app_to_latest_revision) | **POST** /api/v1/applications/{appName}/reset | reset application to latest revision
[**resume_workflow_record**](ApplicationApi.md#resume_workflow_record) | **GET** /api/v1/applications/{appName}/workflows/{workflowName}/records/{record}/resume | resume suspend workflow record
[**rollback_application_with_revision**](ApplicationApi.md#rollback_application_with_revision) | **POST** /api/v1/applications/{appName}/revisions/{revision}/rollback | detail revision for application
[**rollback_workflow_record**](ApplicationApi.md#rollback_workflow_record) | **GET** /api/v1/applications/{appName}/workflows/{workflowName}/records/{record}/rollback | rollback suspend application record
[**terminate_workflow_record**](ApplicationApi.md#terminate_workflow_record) | **GET** /api/v1/applications/{appName}/workflows/{workflowName}/records/{record}/terminate | terminate suspend workflow record
[**update_application**](ApplicationApi.md#update_application) | **PUT** /api/v1/applications/{appName} | update one application 
[**update_application_env**](ApplicationApi.md#update_application_env) | **PUT** /api/v1/applications/{appName}/envs/{envName} | set application  differences in the specified environment
[**update_application_policy**](ApplicationApi.md#update_application_policy) | **PUT** /api/v1/applications/{appName}/policies/{policyName} | update policy for application
[**update_application_trait**](ApplicationApi.md#update_application_trait) | **PUT** /api/v1/applications/{appName}/components/{compName}/traits/{traitType} | update trait from a component
[**update_application_trigger**](ApplicationApi.md#update_application_trigger) | **PUT** /api/v1/applications/{appName}/triggers/{token} | Update an application trigger
[**update_component**](ApplicationApi.md#update_component) | **PUT** /api/v1/applications/{appName}/components/{compName} | update component config
[**update_workflow**](ApplicationApi.md#update_workflow) | **PUT** /api/v1/applications/{appName}/workflows/{workflowName} | update application workflow config



## add_application_trait

> serde_json::Value add_application_trait(app_name, comp_name, body)
add trait for a component

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**comp_name** | **String** | identifier of the component | [required] |
**body** | [**V1PeriodCreateApplicationTraitRequest**](V1PeriodCreateApplicationTraitRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_statistics

> crate::models::V1PeriodApplicationStatisticsResponse application_statistics(app_name)
detail one application 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |

### Return type

[**crate::models::V1PeriodApplicationStatisticsResponse**](v1.ApplicationStatisticsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compare_app

> crate::models::V1PeriodAppCompareResponse compare_app(app_name, body)
compare application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**body** | [**V1PeriodAppCompareReq**](V1PeriodAppCompareReq.md) |  | [required] |

### Return type

[**crate::models::V1PeriodAppCompareResponse**](v1.AppCompareResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application

> crate::models::V1PeriodApplicationBase create_application(body)
create one application 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateApplicationRequest**](V1PeriodCreateApplicationRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodApplicationBase**](v1.ApplicationBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_env

> crate::models::V1PeriodEnvBinding create_application_env(app_name, body)
creating an application environment 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**body** | [**V1PeriodCreateApplicationEnvbindingRequest**](V1PeriodCreateApplicationEnvbindingRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodEnvBinding**](v1.EnvBinding.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_policy

> crate::models::V1PeriodPolicyBase create_application_policy(app_name, body)
create policy for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**body** | [**V1PeriodCreatePolicyRequest**](V1PeriodCreatePolicyRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodPolicyBase**](v1.PolicyBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_trigger

> crate::models::V1PeriodApplicationTriggerBase create_application_trigger(app_name, body)
Create an application trigger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**body** | [**V1PeriodCreateApplicationTriggerRequest**](V1PeriodCreateApplicationTriggerRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodApplicationTriggerBase**](v1.ApplicationTriggerBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_component

> crate::models::V1PeriodComponentBase create_component(app_name, body)
create component  for application 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**body** | [**V1PeriodCreateComponentRequest**](V1PeriodCreateComponentRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodComponentBase**](v1.ComponentBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_application_workflow

> crate::models::V1PeriodSimpleResponse create_or_update_application_workflow(app_name, body)
create application workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**body** | [**V1PeriodCreateWorkflowRequest**](V1PeriodCreateWorkflowRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application

> serde_json::Value delete_application(app_name)
delete one application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_env

> serde_json::Value delete_application_env(app_name, env_name)
delete an application environment 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**env_name** | **String** | identifier of the envBinding  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_policy

> serde_json::Value delete_application_policy(app_name, policy_name, force)
detail policy for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**policy_name** | **String** | identifier of the application policy | [required] |
**force** | Option<**bool**> | Force delete the policy and all references |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_trait

> crate::models::V1PeriodApplicationTrait delete_application_trait(app_name, comp_name, trait_type)
delete trait from a component

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**comp_name** | **String** | identifier of the component | [required] |
**trait_type** | **String** | identifier of the type of trait | [required] |

### Return type

[**crate::models::V1PeriodApplicationTrait**](v1.ApplicationTrait.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_trigger

> serde_json::Value delete_application_trigger(app_name, token)
Delete an application trigger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**token** | **String** | identifier of the trigger | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_component

> serde_json::Value delete_component(app_name, comp_name)
delete a component

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**comp_name** | **String** | identifier of the component | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow

> crate::models::V1PeriodSimpleResponse delete_workflow(app_name, workflow_name)
deletet workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deploy_application

> crate::models::V1PeriodApplicationDeployResponse deploy_application(app_name, body)
deploy or upgrade the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**body** | [**V1PeriodApplicationDeployRequest**](V1PeriodApplicationDeployRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodApplicationDeployResponse**](v1.ApplicationDeployResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_application

> crate::models::V1PeriodDetailApplicationResponse detail_application(app_name)
detail one application 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |

### Return type

[**crate::models::V1PeriodDetailApplicationResponse**](v1.DetailApplicationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_application_policy

> crate::models::V1PeriodDetailPolicyResponse detail_application_policy(app_name, policy_name)
detail policy for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**policy_name** | **String** | identifier of the application policy | [required] |

### Return type

[**crate::models::V1PeriodDetailPolicyResponse**](v1.DetailPolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_application_revision

> crate::models::V1PeriodDetailRevisionResponse detail_application_revision(app_name, revision)
detail revision for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**revision** | **String** | identifier of the application revision | [required] |

### Return type

[**crate::models::V1PeriodDetailRevisionResponse**](v1.DetailRevisionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_component

> crate::models::V1PeriodDetailComponentResponse detail_component(app_name, comp_name)
detail component for application 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**comp_name** | **String** | identifier of the component | [required] |

### Return type

[**crate::models::V1PeriodDetailComponentResponse**](v1.DetailComponentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_workflow

> crate::models::V1PeriodSimpleResponse detail_workflow(app_name, workflow_name)
detail application workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workfloc. | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_workflow_record

> crate::models::V1PeriodSimpleResponse detail_workflow_record(app_name, workflow_name, record)
query application workflow execution record detail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**record** | **String** | identifier of the workflow record | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dry_run_app_or_revision

> crate::models::V1PeriodAppDryRunResponse dry_run_app_or_revision(app_name, body)
dry-run application to latest revision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**body** | [**V1PeriodAppDryRunReq**](V1PeriodAppDryRunReq.md) |  | [required] |

### Return type

[**crate::models::V1PeriodAppDryRunResponse**](v1.AppDryRunResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_status

> crate::models::V1PeriodApplicationStatusResponse get_application_status(app_name, env_name)
get application status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**env_name** | **String** | identifier of the application envbinding | [required] |

### Return type

[**crate::models::V1PeriodApplicationStatusResponse**](v1.ApplicationStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_record_inputs

> crate::models::V1PeriodGetPipelineRunInputResponse get_workflow_record_inputs(app_name, workflow_name, record, step)
get the workflow step inputs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**record** | **String** | identifier of the workflow record | [required] |
**step** | **String** | Specified the step filter | [required] |

### Return type

[**crate::models::V1PeriodGetPipelineRunInputResponse**](v1.GetPipelineRunInputResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_record_logs

> get_workflow_record_logs(app_name, workflow_name, record, step)
get the workflow step logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**record** | **String** | identifier of the workflow record | [required] |
**step** | **String** | Specified the step filter | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_record_outputs

> crate::models::V1PeriodGetPipelineRunOutputResponse get_workflow_record_outputs(app_name, workflow_name, record, step)
get the workflow step inputs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**record** | **String** | identifier of the workflow record | [required] |
**step** | **String** | Specified the step filter | [required] |

### Return type

[**crate::models::V1PeriodGetPipelineRunOutputResponse**](v1.GetPipelineRunOutputResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_components

> crate::models::V1PeriodComponentListResponse list_application_components(app_name, env_name)
gets the list of application components

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**env_name** | Option<**String**> | list components that deployed in define env |  |

### Return type

[**crate::models::V1PeriodComponentListResponse**](v1.ComponentListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_envs

> crate::models::V1PeriodListApplicationEnvBinding list_application_envs(app_name)
list policy for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |

### Return type

[**crate::models::V1PeriodListApplicationEnvBinding**](v1.ListApplicationEnvBinding.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_policies

> crate::models::V1PeriodListApplicationPolicy list_application_policies(app_name)
list policy for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |

### Return type

[**crate::models::V1PeriodListApplicationPolicy**](v1.ListApplicationPolicy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_records

> list_application_records(app_name)
list application records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_revisions

> crate::models::V1PeriodListRevisionsResponse list_application_revisions(app_name, env_name, status, page, page_size)
list revisions for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**env_name** | Option<**String**> | query identifier of the env |  |
**status** | Option<**String**> | query identifier of the status |  |
**page** | Option<**i32**> | query the page number |  |
**page_size** | Option<**i32**> | query the page size number |  |

### Return type

[**crate::models::V1PeriodListRevisionsResponse**](v1.ListRevisionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_triggers

> crate::models::V1PeriodListApplicationTriggerResponse list_application_triggers(app_name)
List the application triggers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |

### Return type

[**crate::models::V1PeriodListApplicationTriggerResponse**](v1.ListApplicationTriggerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_workflows

> crate::models::V1PeriodSimpleResponse list_application_workflows(app_name)
list application workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_applications

> crate::models::V1PeriodListApplicationResponse list_applications(query, project, env, target_name)
list all applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Fuzzy search based on name or description |  |
**project** | Option<**String**> | search base on project name |  |
**env** | Option<**String**> | search base on env name |  |
**target_name** | Option<**String**> | Name of the application delivery target |  |

### Return type

[**crate::models::V1PeriodListApplicationResponse**](v1.ListApplicationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflow_records

> crate::models::V1PeriodSimpleResponse list_workflow_records(app_name, workflow_name, page, page_size)
query application workflow execution record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**page** | Option<**i32**> | query the page number |  |
**page_size** | Option<**i32**> | query the page size number |  |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_application_template

> crate::models::V1PeriodApplicationTemplateBase publish_application_template(app_name, body)
create one application template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**body** | [**V1PeriodCreateApplicationTemplateRequest**](V1PeriodCreateApplicationTemplateRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodApplicationTemplateBase**](v1.ApplicationTemplateBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recycle_application_env

> serde_json::Value recycle_application_env(app_name, env_name)
recycle application env

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**env_name** | **String** | identifier of the application envbinding | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_app_to_latest_revision

> crate::models::V1PeriodAppResetResponse reset_app_to_latest_revision(app_name)
reset application to latest revision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |

### Return type

[**crate::models::V1PeriodAppResetResponse**](v1.AppResetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_workflow_record

> serde_json::Value resume_workflow_record(app_name, workflow_name, record, step)
resume suspend workflow record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**record** | **String** | identifier of the  workflow record | [required] |
**step** | Option<**String**> | resume the workflow with specific step |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rollback_application_with_revision

> crate::models::V1PeriodApplicationRollbackResponse rollback_application_with_revision(app_name, revision)
detail revision for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**revision** | **String** | identifier of the application revision | [required] |

### Return type

[**crate::models::V1PeriodApplicationRollbackResponse**](v1.ApplicationRollbackResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rollback_workflow_record

> crate::models::V1PeriodWorkflowRecordBase rollback_workflow_record(app_name, workflow_name, record, rollback_version)
rollback suspend application record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**record** | **String** | identifier of the workflow record | [required] |
**rollback_version** | Option<**String**> | identifier of the rollback revision |  |

### Return type

[**crate::models::V1PeriodWorkflowRecordBase**](v1.WorkflowRecordBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_workflow_record

> serde_json::Value terminate_workflow_record(app_name, workflow_name, record)
terminate suspend workflow record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**record** | **String** | identifier of the workflow record | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application

> crate::models::V1PeriodApplicationBase update_application(app_name, body)
update one application 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**body** | [**V1PeriodUpdateApplicationRequest**](V1PeriodUpdateApplicationRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodApplicationBase**](v1.ApplicationBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_env

> crate::models::V1PeriodEnvBinding update_application_env(app_name, env_name, body)
set application  differences in the specified environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**env_name** | **String** | identifier of the envBinding  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::V1PeriodEnvBinding**](v1.EnvBinding.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_policy

> crate::models::V1PeriodDetailPolicyResponse update_application_policy(app_name, policy_name, body)
update policy for application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**policy_name** | **String** | identifier of the application policy | [required] |
**body** | [**V1PeriodUpdatePolicyRequest**](V1PeriodUpdatePolicyRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodDetailPolicyResponse**](v1.DetailPolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_trait

> crate::models::V1PeriodApplicationTrait update_application_trait(app_name, comp_name, trait_type, body)
update trait from a component

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**comp_name** | **String** | identifier of the component | [required] |
**trait_type** | **String** | identifier of the type of trait | [required] |
**body** | [**V1PeriodUpdateApplicationTraitRequest**](V1PeriodUpdateApplicationTraitRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodApplicationTrait**](v1.ApplicationTrait.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_trigger

> crate::models::V1PeriodApplicationTriggerBase update_application_trigger(app_name, token)
Update an application trigger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application  | [required] |
**token** | **String** | identifier of the trigger | [required] |

### Return type

[**crate::models::V1PeriodApplicationTriggerBase**](v1.ApplicationTriggerBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_component

> crate::models::V1PeriodComponentBase update_component(app_name, comp_name, body)
update component config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application | [required] |
**comp_name** | **String** | identifier of the component | [required] |
**body** | [**V1PeriodUpdateApplicationComponentRequest**](V1PeriodUpdateApplicationComponentRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodComponentBase**](v1.ComponentBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow

> crate::models::V1PeriodSimpleResponse update_workflow(app_name, workflow_name, body)
update application workflow config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | identifier of the application. | [required] |
**workflow_name** | **String** | identifier of the workflow | [required] |
**body** | [**V1PeriodUpdateWorkflowRequest**](V1PeriodUpdateWorkflowRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodSimpleResponse**](v1.SimpleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

