# \WebhookApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_application_webhook**](WebhookApi.md#handle_application_webhook) | **POST** /api/v1/webhook/{token} | handle application webhook request



## handle_application_webhook

> crate::models::V1PeriodApplicationDeployResponse handle_application_webhook(token, body)
handle application webhook request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | webhook token | [required] |
**body** | [**V1PeriodHandleApplicationTriggerWebhookRequest**](V1PeriodHandleApplicationTriggerWebhookRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodApplicationDeployResponse**](v1.ApplicationDeployResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

