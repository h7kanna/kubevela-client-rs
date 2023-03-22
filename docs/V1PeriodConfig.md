# V1PeriodConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**args_escaped** | Option<**bool**> |  | [optional]
**attach_stderr** | Option<**bool**> |  | [optional]
**attach_stdin** | Option<**bool**> |  | [optional]
**attach_stdout** | Option<**bool**> |  | [optional]
**cmd** | Option<**Vec<String>**> |  | [optional]
**domainname** | Option<**String**> |  | [optional]
**entrypoint** | Option<**Vec<String>**> |  | [optional]
**env** | Option<**Vec<String>**> |  | [optional]
**exposed_ports** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**healthcheck** | Option<[**crate::models::V1PeriodHealthConfig**](v1.HealthConfig.md)> |  | [optional]
**hostname** | Option<**String**> |  | [optional]
**image** | Option<**String**> |  | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**network_disabled** | Option<**bool**> |  | [optional]
**on_build** | Option<**Vec<String>**> |  | [optional]
**open_stdin** | Option<**bool**> |  | [optional]
**shell** | Option<**Vec<String>**> |  | [optional]
**stdin_once** | Option<**bool**> |  | [optional]
**stop_signal** | Option<**String**> |  | [optional]
**tty** | Option<**bool**> |  | [optional]
**user** | Option<**String**> |  | [optional]
**volumes** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**working_dir** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


