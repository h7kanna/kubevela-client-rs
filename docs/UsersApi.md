# \UsersApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /api/v1/users | create a user
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /api/v1/users/{username} | delete a user
[**detail_user**](UsersApi.md#detail_user) | **GET** /api/v1/users/{username} | get user detail
[**disable_user**](UsersApi.md#disable_user) | **GET** /api/v1/users/{username}/disable | disable a user
[**enable_user**](UsersApi.md#enable_user) | **GET** /api/v1/users/{username}/enable | enable a user
[**list_user**](UsersApi.md#list_user) | **GET** /api/v1/users | list users
[**update_user**](UsersApi.md#update_user) | **PUT** /api/v1/users/{username} | update a user's alias or password



## create_user

> crate::models::V1PeriodUserBase create_user(body)
create a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1PeriodCreateUserRequest**](V1PeriodCreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodUserBase**](v1.UserBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> serde_json::Value delete_user(username)
delete a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | identifier of a user | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detail_user

> crate::models::V1PeriodDetailUserResponse detail_user(username)
get user detail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | identifier of a user | [required] |

### Return type

[**crate::models::V1PeriodDetailUserResponse**](v1.DetailUserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_user

> serde_json::Value disable_user(username)
disable a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | identifier of a user | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_user

> serde_json::Value enable_user(username)
enable a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | identifier of a user | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user

> crate::models::V1PeriodListUserResponse list_user(page, page_size, name, email, alias)
list users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | query the page number |  |
**page_size** | Option<**i32**> | query the page size number |  |
**name** | Option<**String**> | fuzzy search based on name |  |
**email** | Option<**String**> | fuzzy search based on email |  |
**alias** | Option<**String**> | fuzzy search based on alias |  |

### Return type

[**crate::models::V1PeriodListUserResponse**](v1.ListUserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::V1PeriodUserBase update_user(username, body)
update a user's alias or password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | identifier of a user | [required] |
**body** | [**V1PeriodUpdateUserRequest**](V1PeriodUpdateUserRequest.md) |  | [required] |

### Return type

[**crate::models::V1PeriodUserBase**](v1.UserBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

