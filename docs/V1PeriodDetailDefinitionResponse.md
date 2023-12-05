# V1PeriodDetailDefinitionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alias** | **String** |  | 
**category** | **String** |  | 
**component** | Option<[**crate::models::V1beta1PeriodComponentDefinitionSpec**](v1beta1.ComponentDefinitionSpec.md)> |  | [optional]
**description** | **String** |  | 
**icon** | **String** |  | 
**labels** | **::std::collections::HashMap<String, String>** |  | 
**name** | **String** |  | 
**owner_addon** | **String** |  | 
**policy** | Option<[**crate::models::V1beta1PeriodPolicyDefinitionSpec**](v1beta1.PolicyDefinitionSpec.md)> |  | [optional]
**schema** | **String** |  | 
**status** | **String** |  | 
**r#trait** | Option<[**crate::models::V1beta1PeriodTraitDefinitionSpec**](v1beta1.TraitDefinitionSpec.md)> |  | [optional]
**ui_schema** | [**Vec<crate::models::SchemaPeriodUiParameter>**](schema.UIParameter.md) |  | 
**workflow_step** | Option<[**crate::models::V1beta1PeriodWorkflowStepDefinitionSpec**](v1beta1.WorkflowStepDefinitionSpec.md)> |  | [optional]
**workload_type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


