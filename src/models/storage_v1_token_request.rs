/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StorageV1TokenRequest : TokenRequest contains parameters of a service account token.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StorageV1TokenRequest {
    /// audience is the intended audience of the token in \"TokenRequestSpec\". It will default to the audiences of kube apiserver.
    #[serde(rename = "audience")]
    pub audience: String,
    /// expirationSeconds is the duration of validity of the token in \"TokenRequestSpec\". It has the same default value of \"ExpirationSeconds\" in \"TokenRequestSpec\".
    #[serde(rename = "expirationSeconds", skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
}

impl StorageV1TokenRequest {
    /// TokenRequest contains parameters of a service account token.
    pub fn new(audience: String) -> StorageV1TokenRequest {
        StorageV1TokenRequest {
            audience,
            expiration_seconds: None,
        }
    }
}


