/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3ResourcePool : ResourcePool describes the pool that ResourceSlices belong to.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3ResourcePool {
    /// Generation tracks the change in a pool over time. Whenever a driver changes something about one or more of the resources in a pool, it must change the generation in all ResourceSlices which are part of that pool. Consumers of ResourceSlices should only consider resources from the pool with the highest generation number. The generation may be reset by drivers, which should be fine for consumers, assuming that all ResourceSlices in a pool are updated to match or deleted.  Combined with ResourceSliceCount, this mechanism enables consumers to detect pools which are comprised of multiple ResourceSlices and are in an incomplete state.
    #[serde(rename = "generation")]
    pub generation: i64,
    /// Name is used to identify the pool. For node-local devices, this is often the node name, but this is not required.  It must not be longer than 253 characters and must consist of one or more DNS sub-domains separated by slashes. This field is immutable.
    #[serde(rename = "name")]
    pub name: String,
    /// ResourceSliceCount is the total number of ResourceSlices in the pool at this generation number. Must be greater than zero.  Consumers can use this to check whether they have seen all ResourceSlices belonging to the same pool.
    #[serde(rename = "resourceSliceCount")]
    pub resource_slice_count: i64,
}

impl V1alpha3ResourcePool {
    /// ResourcePool describes the pool that ResourceSlices belong to.
    pub fn new(generation: i64, name: String, resource_slice_count: i64) -> V1alpha3ResourcePool {
        V1alpha3ResourcePool {
            generation,
            name,
            resource_slice_count,
        }
    }
}


