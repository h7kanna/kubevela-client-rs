# V1PeriodDetailAddonResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_versions** | **Vec<String>** |  | 
**definitions** | [**Vec<crate::models::V1PeriodAddonDefinition>**](v1.AddonDefinition.md) |  | 
**dependencies** | Option<[**Vec<crate::models::AddonPeriodDependency>**](addon.Dependency.md)> |  | [optional]
**deploy_to** | Option<[**crate::models::AddonPeriodDeployTo**](addon.DeployTo.md)> |  | [optional]
**description** | **String** |  | 
**detail** | Option<**String**> |  | [optional]
**icon** | **String** |  | 
**invisible** | **bool** |  | 
**name** | **String** |  | 
**need_namespace** | Option<**Vec<String>**> |  | [optional]
**registry_name** | Option<**String**> |  | [optional]
**schema** | **String** |  | 
**system** | Option<[**crate::models::AddonPeriodSystemRequirements**](addon.SystemRequirements.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**ui_schema** | [**Vec<crate::models::SchemaPeriodUiParameter>**](schema.UIParameter.md) |  | 
**url** | Option<**String**> |  | [optional]
**version** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


