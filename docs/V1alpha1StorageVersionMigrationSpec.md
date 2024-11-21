# V1alpha1StorageVersionMigrationSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**continue_token** | Option<**String**> | The token used in the list options to get the next chunk of objects to migrate. When the .status.conditions indicates the migration is \"Running\", users can use this token to check the progress of the migration. | [optional]
**resource** | [**crate::models::V1alpha1GroupVersionResource**](v1alpha1.GroupVersionResource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


