# V1alpha3ResourceSliceSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**all_nodes** | Option<**bool**> | AllNodes indicates that all nodes have access to the resources in the pool.  Exactly one of NodeName, NodeSelector and AllNodes must be set. | [optional]
**devices** | Option<[**Vec<crate::models::V1alpha3Device>**](v1alpha3.Device.md)> | Devices lists some or all of the devices in this pool.  Must not have more than 128 entries. | [optional]
**driver** | **String** | Driver identifies the DRA driver providing the capacity information. A field selector can be used to list only ResourceSlice objects with a certain driver name.  Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. This field is immutable. | 
**node_name** | Option<**String**> | NodeName identifies the node which provides the resources in this pool. A field selector can be used to list only ResourceSlice objects belonging to a certain node.  This field can be used to limit access from nodes to ResourceSlices with the same node name. It also indicates to autoscalers that adding new nodes of the same type as some old node might also make new resources available.  Exactly one of NodeName, NodeSelector and AllNodes must be set. This field is immutable. | [optional]
**node_selector** | Option<[**crate::models::V1NodeSelector**](v1.NodeSelector.md)> |  | [optional]
**pool** | [**crate::models::V1alpha3ResourcePool**](v1alpha3.ResourcePool.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

