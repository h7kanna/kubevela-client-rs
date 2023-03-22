# V1alpha1PeriodWorkflowRunStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | Option<[**Vec<crate::models::ConditionPeriodCondition>**](condition.Condition.md)> |  | [optional]
**context_backend** | Option<[**crate::models::V1PeriodObjectReference**](v1.ObjectReference.md)> |  | [optional]
**end_time** | Option<**String**> |  | [optional]
**finished** | **bool** |  | 
**message** | Option<**String**> |  | [optional]
**mode** | [**crate::models::V1alpha1PeriodWorkflowExecuteMode**](v1alpha1.WorkflowExecuteMode.md) |  | 
**start_time** | Option<**String**> |  | [optional]
**status** | **String** |  | 
**steps** | Option<[**Vec<crate::models::V1alpha1PeriodWorkflowStepStatus>**](v1alpha1.WorkflowStepStatus.md)> |  | [optional]
**suspend** | **bool** |  | 
**suspend_state** | Option<**String**> |  | [optional]
**terminated** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


