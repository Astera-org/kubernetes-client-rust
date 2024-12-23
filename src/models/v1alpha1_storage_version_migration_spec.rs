/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1StorageVersionMigrationSpec : Spec of the storage version migration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha1StorageVersionMigrationSpec {
    /// The token used in the list options to get the next chunk of objects to migrate. When the .status.conditions indicates the migration is \"Running\", users can use this token to check the progress of the migration.
    #[serde(rename = "continueToken", skip_serializing_if = "Option::is_none")]
    pub continue_token: Option<String>,
    #[serde(rename = "resource")]
    pub resource: Box<crate::models::V1alpha1GroupVersionResource>,
}

impl V1alpha1StorageVersionMigrationSpec {
    /// Spec of the storage version migration.
    pub fn new(resource: crate::models::V1alpha1GroupVersionResource) -> V1alpha1StorageVersionMigrationSpec {
        V1alpha1StorageVersionMigrationSpec {
            continue_token: None,
            resource: Box::new(resource),
        }
    }
}


