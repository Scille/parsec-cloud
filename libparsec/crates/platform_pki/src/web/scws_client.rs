// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Low-level HTTP client for communicating with the SCWS service.

use data_encoding::BASE64;
use rustls_pki_types::{pem::PemObject, CertificateDer};

use super::scws_types::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ScwsClientError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("SCWS returned error: {0}")]
    ScwsError(String),
    #[allow(dead_code)]
    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),
    #[error("Base64 decode error: {0}")]
    Base64Decode(String),
    #[error("PEM decode error: {0}")]
    PemDecode(String),
    #[error("Hex decode error: {0}")]
    HexDecode(String),
}

pub(crate) struct ScwsClient {
    client: reqwest::Client,
    base_url: String,
}

impl ScwsClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/dyn/{}", self.base_url, path)
    }

    async fn check_response(resp: reqwest::Response) -> Result<reqwest::Response, ScwsClientError> {
        if resp.status().is_success() {
            Ok(resp)
        } else {
            let body = resp.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<ScwsErrorResponse>(&body) {
                Err(ScwsClientError::ScwsError(err.err_string))
            } else {
                Err(ScwsClientError::ScwsError(body))
            }
        }
    }

    pub async fn get_challenge(
        &self,
        token: &str,
    ) -> Result<ChallengeResponse, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("get_challenge"))
            .query(&[("token", token)])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        resp.json().await.map_err(Into::into)
    }

    pub async fn create_env(
        &self,
        sig: &str,
        encalg: &str,
    ) -> Result<CreateEnvResponse, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("create_env"))
            .query(&[("sig", sig), ("encalg", encalg)])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        resp.json().await.map_err(Into::into)
    }

    pub async fn get_readers(
        &self,
        env: &str,
    ) -> Result<Vec<ReaderInfo>, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("get_readers"))
            .query(&[("env", env)])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        resp.json().await.map_err(Into::into)
    }

    pub async fn connect(
        &self,
        env: &str,
        reader: &str,
    ) -> Result<ConnectResponse, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("connect"))
            .query(&[("env", env), ("reader", reader)])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        resp.json().await.map_err(Into::into)
    }

    #[allow(dead_code)]
    pub async fn get_token_info(
        &self,
        env: &str,
        token: &str,
    ) -> Result<TokenInfoResponse, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("get_token_info"))
            .query(&[("env", env), ("token", token)])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        resp.json().await.map_err(Into::into)
    }

    pub async fn get_token_objects(
        &self,
        env: &str,
        token: &str,
    ) -> Result<TokenObjectsResponse, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("get_token_objects"))
            .query(&[("env", env), ("token", token)])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        resp.json().await.map_err(Into::into)
    }

    /// Export a certificate object and return its DER bytes.
    ///
    /// SCWS returns `base64(PEM_bytes)`, so we base64-decode to get PEM, then parse PEM to DER.
    pub async fn export_object_der(
        &self,
        env: &str,
        object: &str,
    ) -> Result<Vec<u8>, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("export_object"))
            .query(&[("env", env), ("object", object), ("format", "PEM")])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        let b64_text = resp.text().await?;

        // SCWS wraps the PEM in an additional base64 layer
        let pem_bytes = BASE64
            .decode(b64_text.trim().as_bytes())
            .map_err(|e| ScwsClientError::Base64Decode(e.to_string()))?;

        let der = CertificateDer::from_pem_slice(&pem_bytes)
            .map_err(|e| ScwsClientError::PemDecode(e.to_string()))?;

        Ok(der.to_vec())
    }

    pub async fn login(
        &self,
        env: &str,
        token: &str,
        pin_index: u32,
        pin: &str,
    ) -> Result<(), ScwsClientError> {
        let resp = self
            .client
            .post(self.url("login"))
            .query(&[
                ("env", env),
                ("token", token),
                ("pin", &pin_index.to_string()),
                ("e", "false"),
            ])
            .body(pin.to_string())
            .send()
            .await?;
        Self::check_response(resp).await?;
        Ok(())
    }

    /// Sign a pre-computed hash using RSA-PSS.
    ///
    /// The hash must be hex-encoded. Returns the raw signature bytes.
    pub async fn sign(
        &self,
        env: &str,
        key: &str,
        hash_hex: &str,
        mgf_alg: &str,
        salt_len: u32,
    ) -> Result<Vec<u8>, ScwsClientError> {
        let resp = self
            .client
            .post(self.url("sign"))
            .query(&[
                ("env", env),
                ("key", key),
                ("mgfAlg", mgf_alg),
                ("saltLen", &salt_len.to_string()),
            ])
            .body(hash_hex.to_string())
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        let hex_str = resp.text().await?;
        hex_decode(hex_str.trim())
    }

    /// Decrypt ciphertext using RSA-OAEP.
    ///
    /// The ciphertext must be hex-encoded. Returns the raw plaintext bytes.
    pub async fn decrypt(
        &self,
        env: &str,
        key: &str,
        ciphertext_hex: &str,
    ) -> Result<Vec<u8>, ScwsClientError> {
        let resp = self
            .client
            .post(self.url("decrypt"))
            .query(&[
                ("env", env),
                ("key", key),
                ("alg", "oaep"),
                ("hashAlg", "sha256"),
                ("mgf", "sha256"),
            ])
            .body(ciphertext_hex.to_string())
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        let hex_str = resp.text().await?;
        hex_decode(hex_str.trim())
    }

    pub async fn get_certificate_trust(
        &self,
        env: &str,
        object: &str,
    ) -> Result<CertTrustResponse, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("get_certificate_trust"))
            .query(&[("env", env), ("object", object)])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        resp.json().await.map_err(Into::into)
    }

    pub async fn get_event(
        &self,
        env: &str,
    ) -> Result<Option<serde_json::Value>, ScwsClientError> {
        let resp = self
            .client
            .get(self.url("get_event"))
            .query(&[("env", env)])
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(None);
        }
        let resp = Self::check_response(resp).await?;
        resp.json().await.map_err(Into::into)
    }
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, ScwsClientError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| ScwsClientError::HexDecode(e.to_string()))
        })
        .collect()
}

fn _hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    _hex_encode(bytes)
}
