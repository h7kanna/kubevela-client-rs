# \PipelineApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_context_value**](PipelineApi.md#create_context_value) | **POST** /api/v1/projects/{projectName}/pipelines/{pipelineName}/contexts | create pipeline context values
[**create_pipeline**](PipelineApi.md#create_pipeline) | **POST** /api/v1/projects/{projectName}/pipelines | create pipeline
[**delete_context_value**](PipelineApi.md#delete_context_value) | **DELETE** /api/v1/projects/{projectName}/pipelines/{pipelineName}/contexts/{contextName} | delete pipeline context value
[**delete_pipeline**](PipelineApi.md#delete_pipeline) | **DELETE** /api/v1/projects/{projectName}/pipelines/{pipelineName} | delete pipeline
[**delete_pipeline_run**](PipelineApi.md#delete_pipeline_run) | **DELETE** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName} | delete pipeline run
[**get_pipeline**](PipelineApi.md#get_pipeline) | **GET** /api/v1/projects/{projectName}/pipelines/{pipelineName} | get pipeline
[**get_pipeline_run**](PipelineApi.md#get_pipeline_run) | **GET** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName} | get pipeline run
[**get_pipeline_run_input**](PipelineApi.md#get_pipeline_run_input) | **GET** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName}/input | get pipeline run input
[**get_pipeline_run_log**](PipelineApi.md#get_pipeline_run_log) | **GET** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName}/log | get pipeline run log
[**get_pipeline_run_output**](PipelineApi.md#get_pipeline_run_output) | **GET** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName}/output | get pipeline run output
[**get_pipeline_run_status**](PipelineApi.md#get_pipeline_run_status) | **GET** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName}/status | get pipeline run status
[**list_context_values**](PipelineApi.md#list_context_values) | **GET** /api/v1/projects/{projectName}/pipelines/{pipelineName}/contexts | list pipeline context values
[**list_pipeline_runs**](PipelineApi.md#list_pipeline_runs) | **GET** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs | list pipeline runs
[**list_pipelines**](PipelineApi.md#list_pipelines) | **GET** /api/v1/pipelines | list pipelines
[**resume_pipeline_run**](PipelineApi.md#resume_pipeline_run) | **POST** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName}/resume | resume suspend pipeline run
[**run_pipeline**](PipelineApi.md#run_pipeline) | **POST** /api/v1/projects/{projectName}/pipelines/{pipelineName}/run | run pipeline
[**stop_pipeline**](PipelineApi.md#stop_pipeline) | **POST** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName}/stop | stop pipeline run
[**terminate_pipeline_run**](PipelineApi.md#terminate_pipeline_run) | **POST** /api/v1/projects/{projectName}/pipelines/{pipelineName}/runs/{runName}/terminate | resume suspend pipeline run
[**update_context_value**](PipelineApi.md#update_context_value) | **PUT** /api/v1/projects/{projectName}/pipelines/{pipelineName}/contexts/{contextName} | update pipeline context value
[**update_pipeline**](PipelineApi.md#update_pipeline) | **PUT** /api/v1/projects/{projectName}/pipelines/{pipelineName} | update pipeline



## create_context_value

> crate::models::V1PeriodContext create_context_value(project_name, pipeline_name, body)
create pipeline context values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**body** | [**V1PeriodCreateContextValuesRequest**](V1PeriodCreateContextValuesRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodContext**](v1.Context.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pipeline

> crate::models::V1PeriodPipelineBase create_pipeline(project_name, body)
create pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**body** | [**V1PeriodCreatePipelineRequest**](V1PeriodCreatePipelineRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodPipelineBase**](v1.PipelineBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_context_value

> crate::models::V1PeriodContextNameResponse delete_context_value(project_name, pipeline_name, context_name)
delete pipeline context value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**context_name** | **String** | pipeline context name | [required] |

### Return type

[**crate::models::V1PeriodContextNameResponse**](v1.ContextNameResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline

> crate::models::V1PeriodPipelineMetaResponse delete_pipeline(project_name, pipeline_name)
delete pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |

### Return type

[**crate::models::V1PeriodPipelineMetaResponse**](v1.PipelineMetaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline_run

> crate::models::V1PeriodPipelineRunMeta delete_pipeline_run(project_name, pipeline_name, run_name)
delete pipeline run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |

### Return type

[**crate::models::V1PeriodPipelineRunMeta**](v1.PipelineRunMeta.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline

> crate::models::V1PeriodGetPipelineResponse get_pipeline(pipeline_name, project_name)
get pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_name** | **String** | pipeline name | [required] |
**project_name** | **String** | project name | [required] |

### Return type

[**crate::models::V1PeriodGetPipelineResponse**](v1.GetPipelineResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_run

> crate::models::V1PeriodPipelineRunBase get_pipeline_run(project_name, pipeline_name, run_name)
get pipeline run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |

### Return type

[**crate::models::V1PeriodPipelineRunBase**](v1.PipelineRunBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_run_input

> crate::models::V1PeriodGetPipelineRunInputResponse get_pipeline_run_input(step, project_name, pipeline_name, run_name)
get pipeline run input

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**step** | **String** | query by specific step name | [required] |
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |

### Return type

[**crate::models::V1PeriodGetPipelineRunInputResponse**](v1.GetPipelineRunInputResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_run_log

> crate::models::V1PeriodGetPipelineRunLogResponse get_pipeline_run_log(project_name, pipeline_name, run_name, step)
get pipeline run log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |
**step** | Option<**String**> | query by specific step name |  |

### Return type

[**crate::models::V1PeriodGetPipelineRunLogResponse**](v1.GetPipelineRunLogResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_run_output

> crate::models::V1PeriodGetPipelineRunOutputResponse get_pipeline_run_output(step, project_name, pipeline_name, run_name)
get pipeline run output

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**step** | **String** | query by specific step name | [required] |
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |

### Return type

[**crate::models::V1PeriodGetPipelineRunOutputResponse**](v1.GetPipelineRunOutputResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_run_status

> crate::models::V1alpha1PeriodWorkflowRunStatus get_pipeline_run_status(project_name, pipeline_name, run_name)
get pipeline run status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |

### Return type

[**crate::models::V1alpha1PeriodWorkflowRunStatus**](v1alpha1.WorkflowRunStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_context_values

> crate::models::V1PeriodListContextValueResponse list_context_values(project_name, pipeline_name)
list pipeline context values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |

### Return type

[**crate::models::V1PeriodListContextValueResponse**](v1.ListContextValueResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pipeline_runs

> crate::models::V1PeriodListPipelineRunResponse list_pipeline_runs(project_name, pipeline_name, status)
list pipeline runs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**status** | Option<**String**> | query identifier of the status |  |

### Return type

[**crate::models::V1PeriodListPipelineRunResponse**](v1.ListPipelineRunResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pipelines

> crate::models::V1PeriodListPipelineResponse list_pipelines(query, project_name, detailed)
list pipelines

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Fuzzy search based on name or description |  |
**project_name** | Option<**String**> | query pipelines within a project |  |
**detailed** | Option<**bool**> | query pipelines with detail |  |[default to true]

### Return type

[**crate::models::V1PeriodListPipelineResponse**](v1.ListPipelineResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_pipeline_run

> serde_json::Value resume_pipeline_run(project_name, pipeline_name, run_name, step)
resume suspend pipeline run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |
**step** | Option<**String**> | resume from specific step name |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_pipeline

> crate::models::V1PeriodPipelineRun run_pipeline(project_name, pipeline_name, body)
run pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**body** | [**V1PeriodRunPipelineRequest**](V1PeriodRunPipelineRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodPipelineRun**](v1.PipelineRun.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_pipeline

> crate::models::V1PeriodPipelineRunMeta stop_pipeline(project_name, pipeline_name, run_name)
stop pipeline run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |

### Return type

[**crate::models::V1PeriodPipelineRunMeta**](v1.PipelineRunMeta.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_pipeline_run

> serde_json::Value terminate_pipeline_run(project_name, pipeline_name, run_name)
resume suspend pipeline run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**run_name** | **String** | pipeline run name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_context_value

> crate::models::V1PeriodContext update_context_value(project_name, pipeline_name, context_name, body)
update pipeline context value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**context_name** | **String** | pipeline context name | [required] |
**body** | [**V1PeriodUpdateContextValuesRequest**](V1PeriodUpdateContextValuesRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodContext**](v1.Context.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pipeline

> crate::models::V1PeriodPipelineBase update_pipeline(project_name, pipeline_name, body)
update pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | project name | [required] |
**pipeline_name** | **String** | pipeline name | [required] |
**body** | [**V1PeriodUpdatePipelineRequest**](V1PeriodUpdatePipelineRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodPipelineBase**](v1.PipelineBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

