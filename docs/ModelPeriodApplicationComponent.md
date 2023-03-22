# ModelPeriodApplicationComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alias** | **String** |  | 
**app_primary_key** | **String** |  | 
**create_time** | **String** |  | 
**creator** | **String** |  | 
**depends_on** | Option<**Vec<String>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**external_revision** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**inputs** | Option<[**Vec<crate::models::V1alpha1PeriodInputItem>**](v1alpha1.InputItem.md)> |  | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**main** | **bool** |  | 
**name** | **String** |  | 
**outputs** | Option<[**Vec<crate::models::V1alpha1PeriodOutputItem>**](v1alpha1.OutputItem.md)> |  | [optional]
**properties** | Option<[**serde_json::Value**](.md)> |  | [optional]
**scopes** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**traits** | Option<[**Vec<crate::models::ModelPeriodApplicationTrait>**](model.ApplicationTrait.md)> |  | [optional]
**r#type** | **String** |  | 
**update_time** | **String** |  | 
**workload_type** | Option<[**crate::models::CommonPeriodWorkloadTypeDescriptor**](common.WorkloadTypeDescriptor.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


