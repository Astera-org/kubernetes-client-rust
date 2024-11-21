# V1LoadBalancerIngress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hostname** | Option<**String**> | Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers) | [optional]
**ip** | Option<**String**> | IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers) | [optional]
**ip_mode** | Option<**String**> | IPMode specifies how the load-balancer IP behaves, and may only be specified when the ip field is specified. Setting this to \"VIP\" indicates that traffic is delivered to the node with the destination set to the load-balancer's IP and port. Setting this to \"Proxy\" indicates that traffic is delivered to the node or pod with the destination set to the node's IP and node port or the pod's IP and port. Service implementations may use this information to adjust traffic routing. | [optional]
**ports** | Option<[**Vec<crate::models::V1PortStatus>**](v1.PortStatus.md)> | Ports is a list of records of service ports If used, every port defined in the service should have an entry in it | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


