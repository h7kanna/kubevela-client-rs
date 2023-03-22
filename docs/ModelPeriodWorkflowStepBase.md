# ModelPeriodWorkflowStepBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alias** | **String** |  | 
**depends_on** | **Vec<String>** |  | 
**description** | **String** |  | 
**r#if** | Option<**String**> |  | [optional]
**inputs** | Option<[**Vec<crate::models::V1alpha1PeriodInputItem>**](v1alpha1.InputItem.md)> |  | [optional]
**meta** | Option<[**crate::models::V1alpha1PeriodWorkflowStepMeta**](v1alpha1.WorkflowStepMeta.md)> |  | [optional]
**name** | **String** |  | 
**order_index** | **i32** |  | 
**outputs** | Option<[**Vec<crate::models::V1alpha1PeriodOutputItem>**](v1alpha1.OutputItem.md)> |  | [optional]
**properties** | Option<[**serde_json::Value**](.md)> |  | [optional]
**timeout** | Option<**String**> |  | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


