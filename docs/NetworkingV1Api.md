# \NetworkingV1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ingress_class**](NetworkingV1Api.md#create_ingress_class) | **POST** /apis/networking.k8s.io/v1/ingressclasses | 
[**create_namespaced_ingress**](NetworkingV1Api.md#create_namespaced_ingress) | **POST** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses | 
[**create_namespaced_network_policy**](NetworkingV1Api.md#create_namespaced_network_policy) | **POST** /apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies | 
[**delete_collection_ingress_class**](NetworkingV1Api.md#delete_collection_ingress_class) | **DELETE** /apis/networking.k8s.io/v1/ingressclasses | 
[**delete_collection_namespaced_ingress**](NetworkingV1Api.md#delete_collection_namespaced_ingress) | **DELETE** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses | 
[**delete_collection_namespaced_network_policy**](NetworkingV1Api.md#delete_collection_namespaced_network_policy) | **DELETE** /apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies | 
[**delete_ingress_class**](NetworkingV1Api.md#delete_ingress_class) | **DELETE** /apis/networking.k8s.io/v1/ingressclasses/{name} | 
[**delete_namespaced_ingress**](NetworkingV1Api.md#delete_namespaced_ingress) | **DELETE** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses/{name} | 
[**delete_namespaced_network_policy**](NetworkingV1Api.md#delete_namespaced_network_policy) | **DELETE** /apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name} | 
[**get_api_resources**](NetworkingV1Api.md#get_api_resources) | **GET** /apis/networking.k8s.io/v1/ | 
[**list_ingress_class**](NetworkingV1Api.md#list_ingress_class) | **GET** /apis/networking.k8s.io/v1/ingressclasses | 
[**list_ingress_for_all_namespaces**](NetworkingV1Api.md#list_ingress_for_all_namespaces) | **GET** /apis/networking.k8s.io/v1/ingresses | 
[**list_namespaced_ingress**](NetworkingV1Api.md#list_namespaced_ingress) | **GET** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses | 
[**list_namespaced_network_policy**](NetworkingV1Api.md#list_namespaced_network_policy) | **GET** /apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies | 
[**list_network_policy_for_all_namespaces**](NetworkingV1Api.md#list_network_policy_for_all_namespaces) | **GET** /apis/networking.k8s.io/v1/networkpolicies | 
[**patch_ingress_class**](NetworkingV1Api.md#patch_ingress_class) | **PATCH** /apis/networking.k8s.io/v1/ingressclasses/{name} | 
[**patch_namespaced_ingress**](NetworkingV1Api.md#patch_namespaced_ingress) | **PATCH** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses/{name} | 
[**patch_namespaced_ingress_status**](NetworkingV1Api.md#patch_namespaced_ingress_status) | **PATCH** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses/{name}/status | 
[**patch_namespaced_network_policy**](NetworkingV1Api.md#patch_namespaced_network_policy) | **PATCH** /apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name} | 
[**read_ingress_class**](NetworkingV1Api.md#read_ingress_class) | **GET** /apis/networking.k8s.io/v1/ingressclasses/{name} | 
[**read_namespaced_ingress**](NetworkingV1Api.md#read_namespaced_ingress) | **GET** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses/{name} | 
[**read_namespaced_ingress_status**](NetworkingV1Api.md#read_namespaced_ingress_status) | **GET** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses/{name}/status | 
[**read_namespaced_network_policy**](NetworkingV1Api.md#read_namespaced_network_policy) | **GET** /apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name} | 
[**replace_ingress_class**](NetworkingV1Api.md#replace_ingress_class) | **PUT** /apis/networking.k8s.io/v1/ingressclasses/{name} | 
[**replace_namespaced_ingress**](NetworkingV1Api.md#replace_namespaced_ingress) | **PUT** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses/{name} | 
[**replace_namespaced_ingress_status**](NetworkingV1Api.md#replace_namespaced_ingress_status) | **PUT** /apis/networking.k8s.io/v1/namespaces/{namespace}/ingresses/{name}/status | 
[**replace_namespaced_network_policy**](NetworkingV1Api.md#replace_namespaced_network_policy) | **PUT** /apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name} | 



## create_ingress_class

> crate::models::V1IngressClass create_ingress_class(body, pretty, dry_run, field_manager, field_validation)


create an IngressClass

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1IngressClass**](V1IngressClass.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |

### Return type

[**crate::models::V1IngressClass**](v1.IngressClass.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespaced_ingress

> crate::models::V1Ingress create_namespaced_ingress(namespace, body, pretty, dry_run, field_manager, field_validation)


create an Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Ingress**](V1Ingress.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |

### Return type

[**crate::models::V1Ingress**](v1.Ingress.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespaced_network_policy

> crate::models::V1NetworkPolicy create_namespaced_network_policy(namespace, body, pretty, dry_run, field_manager, field_validation)


create a NetworkPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1NetworkPolicy**](V1NetworkPolicy.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |

### Return type

[**crate::models::V1NetworkPolicy**](v1.NetworkPolicy.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_ingress_class

> crate::models::V1Status delete_collection_ingress_class(pretty, _continue, dry_run, field_selector, grace_period_seconds, label_selector, limit, orphan_dependents, propagation_policy, resource_version, resource_version_match, send_initial_events, timeout_seconds, body)


delete collection of IngressClass

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**send_initial_events** | Option<**bool**> | `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic \"Bookmark\" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `\"k8s.io/initial-events-end\": \"true\"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.  When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan   is interpreted as \"data at least as new as the provided `resourceVersion`\"   and the bookmark event is send when the state is synced   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.   If `resourceVersion` is unset, this is interpreted as \"consistent read\" and the   bookmark event is send when the state is synced at least to the moment   when request started being processed. - `resourceVersionMatch` set to any other value or unset   Invalid error is returned.  Defaults to true if `resourceVersion=\"\"` or `resourceVersion=\"0\"` (for backward compatibility reasons) and to false otherwise. |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_namespaced_ingress

> crate::models::V1Status delete_collection_namespaced_ingress(namespace, pretty, _continue, dry_run, field_selector, grace_period_seconds, label_selector, limit, orphan_dependents, propagation_policy, resource_version, resource_version_match, send_initial_events, timeout_seconds, body)


delete collection of Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**send_initial_events** | Option<**bool**> | `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic \"Bookmark\" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `\"k8s.io/initial-events-end\": \"true\"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.  When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan   is interpreted as \"data at least as new as the provided `resourceVersion`\"   and the bookmark event is send when the state is synced   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.   If `resourceVersion` is unset, this is interpreted as \"consistent read\" and the   bookmark event is send when the state is synced at least to the moment   when request started being processed. - `resourceVersionMatch` set to any other value or unset   Invalid error is returned.  Defaults to true if `resourceVersion=\"\"` or `resourceVersion=\"0\"` (for backward compatibility reasons) and to false otherwise. |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_namespaced_network_policy

> crate::models::V1Status delete_collection_namespaced_network_policy(namespace, pretty, _continue, dry_run, field_selector, grace_period_seconds, label_selector, limit, orphan_dependents, propagation_policy, resource_version, resource_version_match, send_initial_events, timeout_seconds, body)


delete collection of NetworkPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**send_initial_events** | Option<**bool**> | `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic \"Bookmark\" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `\"k8s.io/initial-events-end\": \"true\"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.  When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan   is interpreted as \"data at least as new as the provided `resourceVersion`\"   and the bookmark event is send when the state is synced   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.   If `resourceVersion` is unset, this is interpreted as \"consistent read\" and the   bookmark event is send when the state is synced at least to the moment   when request started being processed. - `resourceVersionMatch` set to any other value or unset   Invalid error is returned.  Defaults to true if `resourceVersion=\"\"` or `resourceVersion=\"0\"` (for backward compatibility reasons) and to false otherwise. |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ingress_class

> crate::models::V1Status delete_ingress_class(name, pretty, dry_run, grace_period_seconds, orphan_dependents, propagation_policy, body)


delete an IngressClass

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the IngressClass | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespaced_ingress

> crate::models::V1Status delete_namespaced_ingress(name, namespace, pretty, dry_run, grace_period_seconds, orphan_dependents, propagation_policy, body)


delete an Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Ingress | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespaced_network_policy

> crate::models::V1Status delete_namespaced_network_policy(name, namespace, pretty, dry_run, grace_period_seconds, orphan_dependents, propagation_policy, body)


delete a NetworkPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the NetworkPolicy | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_resources

> crate::models::V1ApiResourceList get_api_resources()


get available resources

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1ApiResourceList**](v1.APIResourceList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ingress_class

> crate::models::V1IngressClassList list_ingress_class(pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, send_initial_events, timeout_seconds, watch)


list or watch objects of kind IngressClass

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**send_initial_events** | Option<**bool**> | `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic \"Bookmark\" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `\"k8s.io/initial-events-end\": \"true\"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.  When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan   is interpreted as \"data at least as new as the provided `resourceVersion`\"   and the bookmark event is send when the state is synced   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.   If `resourceVersion` is unset, this is interpreted as \"consistent read\" and the   bookmark event is send when the state is synced at least to the moment   when request started being processed. - `resourceVersionMatch` set to any other value or unset   Invalid error is returned.  Defaults to true if `resourceVersion=\"\"` or `resourceVersion=\"0\"` (for backward compatibility reasons) and to false otherwise. |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1IngressClassList**](v1.IngressClassList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ingress_for_all_namespaces

> crate::models::V1IngressList list_ingress_for_all_namespaces(allow_watch_bookmarks, _continue, field_selector, label_selector, limit, pretty, resource_version, resource_version_match, send_initial_events, timeout_seconds, watch)


list or watch objects of kind Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**send_initial_events** | Option<**bool**> | `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic \"Bookmark\" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `\"k8s.io/initial-events-end\": \"true\"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.  When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan   is interpreted as \"data at least as new as the provided `resourceVersion`\"   and the bookmark event is send when the state is synced   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.   If `resourceVersion` is unset, this is interpreted as \"consistent read\" and the   bookmark event is send when the state is synced at least to the moment   when request started being processed. - `resourceVersionMatch` set to any other value or unset   Invalid error is returned.  Defaults to true if `resourceVersion=\"\"` or `resourceVersion=\"0\"` (for backward compatibility reasons) and to false otherwise. |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1IngressList**](v1.IngressList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaced_ingress

> crate::models::V1IngressList list_namespaced_ingress(namespace, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, send_initial_events, timeout_seconds, watch)


list or watch objects of kind Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**send_initial_events** | Option<**bool**> | `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic \"Bookmark\" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `\"k8s.io/initial-events-end\": \"true\"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.  When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan   is interpreted as \"data at least as new as the provided `resourceVersion`\"   and the bookmark event is send when the state is synced   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.   If `resourceVersion` is unset, this is interpreted as \"consistent read\" and the   bookmark event is send when the state is synced at least to the moment   when request started being processed. - `resourceVersionMatch` set to any other value or unset   Invalid error is returned.  Defaults to true if `resourceVersion=\"\"` or `resourceVersion=\"0\"` (for backward compatibility reasons) and to false otherwise. |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1IngressList**](v1.IngressList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaced_network_policy

> crate::models::V1NetworkPolicyList list_namespaced_network_policy(namespace, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, send_initial_events, timeout_seconds, watch)


list or watch objects of kind NetworkPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**send_initial_events** | Option<**bool**> | `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic \"Bookmark\" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `\"k8s.io/initial-events-end\": \"true\"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.  When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan   is interpreted as \"data at least as new as the provided `resourceVersion`\"   and the bookmark event is send when the state is synced   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.   If `resourceVersion` is unset, this is interpreted as \"consistent read\" and the   bookmark event is send when the state is synced at least to the moment   when request started being processed. - `resourceVersionMatch` set to any other value or unset   Invalid error is returned.  Defaults to true if `resourceVersion=\"\"` or `resourceVersion=\"0\"` (for backward compatibility reasons) and to false otherwise. |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1NetworkPolicyList**](v1.NetworkPolicyList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_network_policy_for_all_namespaces

> crate::models::V1NetworkPolicyList list_network_policy_for_all_namespaces(allow_watch_bookmarks, _continue, field_selector, label_selector, limit, pretty, resource_version, resource_version_match, send_initial_events, timeout_seconds, watch)


list or watch objects of kind NetworkPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**send_initial_events** | Option<**bool**> | `sendInitialEvents=true` may be set together with `watch=true`. In that case, the watch stream will begin with synthetic events to produce the current state of objects in the collection. Once all such events have been sent, a synthetic \"Bookmark\" event  will be sent. The bookmark will report the ResourceVersion (RV) corresponding to the set of objects, and be marked with `\"k8s.io/initial-events-end\": \"true\"` annotation. Afterwards, the watch stream will proceed as usual, sending watch events corresponding to changes (subsequent to the RV) to objects watched.  When `sendInitialEvents` option is set, we require `resourceVersionMatch` option to also be set. The semantic of the watch request is as following: - `resourceVersionMatch` = NotOlderThan   is interpreted as \"data at least as new as the provided `resourceVersion`\"   and the bookmark event is send when the state is synced   to a `resourceVersion` at least as fresh as the one provided by the ListOptions.   If `resourceVersion` is unset, this is interpreted as \"consistent read\" and the   bookmark event is send when the state is synced at least to the moment   when request started being processed. - `resourceVersionMatch` set to any other value or unset   Invalid error is returned.  Defaults to true if `resourceVersion=\"\"` or `resourceVersion=\"0\"` (for backward compatibility reasons) and to false otherwise. |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1NetworkPolicyList**](v1.NetworkPolicyList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_ingress_class

> crate::models::V1IngressClass patch_ingress_class(name, body, pretty, dry_run, field_manager, field_validation, force)


partially update the specified IngressClass

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the IngressClass | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1IngressClass**](v1.IngressClass.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_ingress

> crate::models::V1Ingress patch_namespaced_ingress(name, namespace, body, pretty, dry_run, field_manager, field_validation, force)


partially update the specified Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Ingress | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1Ingress**](v1.Ingress.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_ingress_status

> crate::models::V1Ingress patch_namespaced_ingress_status(name, namespace, body, pretty, dry_run, field_manager, field_validation, force)


partially update status of the specified Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Ingress | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1Ingress**](v1.Ingress.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_network_policy

> crate::models::V1NetworkPolicy patch_namespaced_network_policy(name, namespace, body, pretty, dry_run, field_manager, field_validation, force)


partially update the specified NetworkPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the NetworkPolicy | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1NetworkPolicy**](v1.NetworkPolicy.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_ingress_class

> crate::models::V1IngressClass read_ingress_class(name, pretty)


read the specified IngressClass

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the IngressClass | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |

### Return type

[**crate::models::V1IngressClass**](v1.IngressClass.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_ingress

> crate::models::V1Ingress read_namespaced_ingress(name, namespace, pretty)


read the specified Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Ingress | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |

### Return type

[**crate::models::V1Ingress**](v1.Ingress.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_ingress_status

> crate::models::V1Ingress read_namespaced_ingress_status(name, namespace, pretty)


read status of the specified Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Ingress | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |

### Return type

[**crate::models::V1Ingress**](v1.Ingress.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_network_policy

> crate::models::V1NetworkPolicy read_namespaced_network_policy(name, namespace, pretty)


read the specified NetworkPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the NetworkPolicy | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |

### Return type

[**crate::models::V1NetworkPolicy**](v1.NetworkPolicy.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_ingress_class

> crate::models::V1IngressClass replace_ingress_class(name, body, pretty, dry_run, field_manager, field_validation)


replace the specified IngressClass

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the IngressClass | [required] |
**body** | [**V1IngressClass**](V1IngressClass.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |

### Return type

[**crate::models::V1IngressClass**](v1.IngressClass.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_ingress

> crate::models::V1Ingress replace_namespaced_ingress(name, namespace, body, pretty, dry_run, field_manager, field_validation)


replace the specified Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Ingress | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Ingress**](V1Ingress.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |

### Return type

[**crate::models::V1Ingress**](v1.Ingress.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_ingress_status

> crate::models::V1Ingress replace_namespaced_ingress_status(name, namespace, body, pretty, dry_run, field_manager, field_validation)


replace status of the specified Ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Ingress | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Ingress**](V1Ingress.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |

### Return type

[**crate::models::V1Ingress**](v1.Ingress.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_network_policy

> crate::models::V1NetworkPolicy replace_namespaced_network_policy(name, namespace, body, pretty, dry_run, field_manager, field_validation)


replace the specified NetworkPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the NetworkPolicy | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1NetworkPolicy**](V1NetworkPolicy.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. Defaults to 'false' unless the user-agent indicates a browser or command-line HTTP tool (curl and wget). |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**field_validation** | Option<**String**> | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |  |

### Return type

[**crate::models::V1NetworkPolicy**](v1.NetworkPolicy.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

