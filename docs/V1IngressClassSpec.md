# V1IngressClassSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**controller** | Option<**String**> | controller refers to the name of the controller that should handle this class. This allows for different \"flavors\" that are controlled by the same controller. For example, you may have different parameters for the same implementing controller. This should be specified as a domain-prefixed path no more than 250 characters in length, e.g. \"acme.io/ingress-controller\". This field is immutable. | [optional]
**parameters** | Option<[**crate::models::V1IngressClassParametersReference**](v1.IngressClassParametersReference.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


