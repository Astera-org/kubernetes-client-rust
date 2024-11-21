/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1ClusterTrustBundleSpec : ClusterTrustBundleSpec contains the signer and trust anchors.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha1ClusterTrustBundleSpec {
    /// signerName indicates the associated signer, if any.  In order to create or update a ClusterTrustBundle that sets signerName, you must have the following cluster-scoped permission: group=certificates.k8s.io resource=signers resourceName=<the signer name> verb=attest.  If signerName is not empty, then the ClusterTrustBundle object must be named with the signer name as a prefix (translating slashes to colons). For example, for the signer name `example.com/foo`, valid ClusterTrustBundle object names include `example.com:foo:abc` and `example.com:foo:v1`.  If signerName is empty, then the ClusterTrustBundle object's name must not have such a prefix.  List/watch requests for ClusterTrustBundles can filter on this field using a `spec.signerName=NAME` field selector.
    #[serde(rename = "signerName", skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<String>,
    /// trustBundle contains the individual X.509 trust anchors for this bundle, as PEM bundle of PEM-wrapped, DER-formatted X.509 certificates.  The data must consist only of PEM certificate blocks that parse as valid X.509 certificates.  Each certificate must include a basic constraints extension with the CA bit set.  The API server will reject objects that contain duplicate certificates, or that use PEM block headers.  Users of ClusterTrustBundles, including Kubelet, are free to reorder and deduplicate certificate blocks in this file according to their own logic, as well as to drop PEM block headers and inter-block data.
    #[serde(rename = "trustBundle")]
    pub trust_bundle: String,
}

impl V1alpha1ClusterTrustBundleSpec {
    /// ClusterTrustBundleSpec contains the signer and trust anchors.
    pub fn new(trust_bundle: String) -> V1alpha1ClusterTrustBundleSpec {
        V1alpha1ClusterTrustBundleSpec {
            signer_name: None,
            trust_bundle,
        }
    }
}


