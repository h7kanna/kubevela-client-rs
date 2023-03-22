# V1PeriodWorkflowStep

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alias** | Option<**String**> |  | [optional]
**depends_on** | Option<**Vec<String>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**r#if** | Option<**String**> |  | [optional]
**inputs** | Option<[**Vec<crate::models::V1alpha1PeriodInputItem>**](v1alpha1.InputItem.md)> |  | [optional]
**meta** | Option<[**crate::models::V1alpha1PeriodWorkflowStepMeta**](v1alpha1.WorkflowStepMeta.md)> |  | [optional]
**name** | **String** |  | 
**outputs** | Option<[**Vec<crate::models::V1alpha1PeriodOutputItem>**](v1alpha1.OutputItem.md)> |  | [optional]
**properties** | Option<[**serde_json::Value**](.md)> |  | [optional]
**sub_steps** | Option<[**Vec<crate::models::V1PeriodWorkflowStepBase>**](v1.WorkflowStepBase.md)> |  | [optional]
**timeout** | Option<**String**> |  | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


