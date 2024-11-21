# V1alpha1StorageVersionMigrationStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | Option<[**Vec<crate::models::V1alpha1MigrationCondition>**](v1alpha1.MigrationCondition.md)> | The latest available observations of the migration's current state. | [optional]
**resource_version** | Option<**String**> | ResourceVersion to compare with the GC cache for performing the migration. This is the current resource version of given group, version and resource when kube-controller-manager first observes this StorageVersionMigration resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


