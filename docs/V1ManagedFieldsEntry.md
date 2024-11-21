# V1ManagedFieldsEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | Option<**String**> | APIVersion defines the version of this resource that this field set applies to. The format is \"group/version\" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted. | [optional]
**fields_type** | Option<**String**> | FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: \"FieldsV1\" | [optional]
**fields_v1** | Option<[**serde_json::Value**](.md)> | FieldsV1 holds the first JSON version format as described in the \"FieldsV1\" type. | [optional]
**manager** | Option<**String**> | Manager is an identifier of the workflow managing these fields. | [optional]
**operation** | Option<**String**> | Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'. | [optional]
**subresource** | Option<**String**> | Subresource is the name of the subresource used to update that object, or empty string if the object was updated through the main resource. The value of this field is used to distinguish between managers, even if they share the same name. For example, a status update will be distinct from a regular update using the same manager name. Note that the APIVersion field is not related to the Subresource field and it always corresponds to the version of the main resource. | [optional]
**time** | Option<**String**> | Time is the timestamp of when the ManagedFields entry was added. The timestamp will also be updated if a field is added, the manager changes any of the owned fields value or removes a field. The timestamp does not update when a field is removed from the entry because another manager took it over. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

