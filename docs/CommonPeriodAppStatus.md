# CommonPeriodAppStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**applied_resources** | Option<[**Vec<crate::models::CommonPeriodClusterObjectReference>**](common.ClusterObjectReference.md)> |  | [optional]
**components** | Option<[**Vec<crate::models::V1PeriodObjectReference>**](v1.ObjectReference.md)> |  | [optional]
**conditions** | Option<[**Vec<crate::models::ConditionPeriodCondition>**](condition.Condition.md)> |  | [optional]
**latest_revision** | Option<[**crate::models::CommonPeriodRevision**](common.Revision.md)> |  | [optional]
**observed_generation** | Option<**i64**> |  | [optional]
**policy** | Option<[**Vec<crate::models::CommonPeriodPolicyStatus>**](common.PolicyStatus.md)> |  | [optional]
**services** | Option<[**Vec<crate::models::CommonPeriodApplicationComponentStatus>**](common.ApplicationComponentStatus.md)> |  | [optional]
**status** | Option<**String**> |  | [optional]
**workflow** | Option<[**crate::models::CommonPeriodWorkflowStatus**](common.WorkflowStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


