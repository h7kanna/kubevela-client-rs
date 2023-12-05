# V1PeriodManagedPluginDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_secret** | Option<[**crate::models::TypesPeriodKubernetesSecret**](types.KubernetesSecret.md)> |  | [optional]
**auth_type** | Option<**String**> |  | [optional]
**backend** | **bool** |  | 
**backend_service** | [**crate::models::TypesPeriodKubernetesService**](types.KubernetesService.md) |  | 
**backend_type** | **String** |  | 
**base_url** | **String** |  | 
**category** | **String** |  | 
**class** | **String** |  | 
**default_nav_url** | **String** |  | 
**enabled** | **bool** |  | 
**id** | **String** |  | 
**includes** | [**Vec<crate::models::TypesPeriodIncludes>**](types.Includes.md) |  | 
**info** | [**crate::models::TypesPeriodInfo**](types.Info.md) |  | 
**json_setting** | [**serde_json::Value**](.md) |  | 
**kube_permissions** | Option<[**Vec<crate::models::V1PeriodPolicyRule>**](v1.PolicyRule.md)> |  | [optional]
**module** | **String** |  | 
**name** | **String** |  | 
**proxy** | **bool** |  | 
**requirement** | Option<[**crate::models::TypesPeriodRequirement**](types.Requirement.md)> |  | [optional]
**routes** | Option<[**Vec<crate::models::TypesPeriodRoute>**](types.Route.md)> |  | [optional]
**secure_json_fields** | **::std::collections::HashMap<String, bool>** |  | 
**sub_type** | **String** |  | 
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


