# \AuthenticationApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_configured**](AuthenticationApi.md#admin_configured) | **GET** /api/v1/auth/admin_configured | check admin is configured
[**configure_admin**](AuthenticationApi.md#configure_admin) | **PUT** /api/v1/auth/init_admin | initialize admin
[**get_dex_config**](AuthenticationApi.md#get_dex_config) | **GET** /api/v1/auth/dex_config | get Dex config
[**get_login_type**](AuthenticationApi.md#get_login_type) | **GET** /api/v1/auth/login_type | get login type
[**get_login_user_info**](AuthenticationApi.md#get_login_user_info) | **GET** /api/v1/auth/user_info | get login user detail info
[**login**](AuthenticationApi.md#login) | **POST** /api/v1/auth/login | handle login request
[**refresh_token**](AuthenticationApi.md#refresh_token) | **GET** /api/v1/auth/refresh_token | refresh token



## admin_configured

> crate::models::V1PeriodAdminConfiguredResponse admin_configured()
check admin is configured

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodAdminConfiguredResponse**](v1.AdminConfiguredResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configure_admin

> crate::models::V1PeriodInitAdminResponse configure_admin(body)
initialize admin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodInitAdminRequest**](V1PeriodInitAdminRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodInitAdminResponse**](v1.InitAdminResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dex_config

> crate::models::V1PeriodDexConfigResponse get_dex_config()
get Dex config

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodDexConfigResponse**](v1.DexConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_login_type

> crate::models::V1PeriodGetLoginTypeResponse get_login_type()
get login type

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodGetLoginTypeResponse**](v1.GetLoginTypeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_login_user_info

> crate::models::V1PeriodLoginUserInfoResponse get_login_user_info()
get login user detail info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodLoginUserInfoResponse**](v1.LoginUserInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> crate::models::V1PeriodLoginResponse login(body)
handle login request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodLoginRequest**](V1PeriodLoginRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodLoginResponse**](v1.LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_token

> crate::models::V1PeriodRefreshTokenResponse refresh_token()
refresh token

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1PeriodRefreshTokenResponse**](v1.RefreshTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

