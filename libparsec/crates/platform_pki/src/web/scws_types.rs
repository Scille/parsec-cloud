// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! JSON response types for the SCWS (SmartCard Web Service) REST API.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChallengeResponse {
    pub challenge: String,
    pub cryptogram: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateEnvResponse {
    pub env_id: String,
    pub pub_key_pem: String,
    pub encalg: String,
    pub credential_enc_alg: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReaderInfo {
    pub name: String,
    pub card: bool,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConnectResponse {
    pub handle: String,
    pub serial: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TokenInfoResponse {
    pub infos: Vec<TokenSlotInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TokenSlotInfo {
    pub user_logged: bool,
    pub pin_label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TokenObjectsResponse {
    pub objects: Vec<TokenObject>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TokenObject {
    pub r#type: String,
    pub handle: String,
    #[serde(default)]
    pub ck_id: Option<String>,
    #[serde(default)]
    pub has_private_key: Option<bool>,
    #[serde(default)]
    pub subject_name: Option<String>,
    #[serde(default)]
    pub issuer_name: Option<String>,
    #[serde(default)]
    pub root_cert: Option<bool>,
    #[serde(default)]
    pub ca_cert: Option<bool>,
    #[serde(default)]
    pub container: Option<String>,
    #[serde(default)]
    pub ck_label: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CertTrustResponse {
    pub trust_status: String,
    #[serde(default)]
    pub cert_path: Vec<CertPathEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CertPathEntry {
    pub handle: String,
    #[serde(default)]
    pub root_cert: Option<bool>,
    #[serde(default)]
    pub ca_cert: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScwsErrorResponse {
    pub err_string: String,
}
