# V1ClusterTrustBundleProjection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label_selector** | Option<[**crate::models::V1LabelSelector**](v1.LabelSelector.md)> |  | [optional]
**name** | Option<**String**> | Select a single ClusterTrustBundle by object name.  Mutually-exclusive with signerName and labelSelector. | [optional]
**optional** | Option<**bool**> | If true, don't block pod startup if the referenced ClusterTrustBundle(s) aren't available.  If using name, then the named ClusterTrustBundle is allowed not to exist.  If using signerName, then the combination of signerName and labelSelector is allowed to match zero ClusterTrustBundles. | [optional]
**path** | **String** | Relative path from the volume root to write the bundle. | 
**signer_name** | Option<**String**> | Select all ClusterTrustBundles that match this signer name. Mutually-exclusive with name.  The contents of all selected ClusterTrustBundles will be unified and deduplicated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


