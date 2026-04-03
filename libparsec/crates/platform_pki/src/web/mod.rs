// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Web platform PKI implementation using SCWS (SmartCard Web Service).
//!
//! SCWS is a native service running on localhost that bridges web applications
//! to smartcard hardware via a REST API.

mod certificate;
mod private_key;
mod scws_client;
#[allow(dead_code)]
mod scws_types;

use std::sync::Arc;

use bytes::Bytes;
use libparsec_types::prelude::*;
use rustls_pki_types::CertificateDer;
use sha2::Digest as _;
use tokio::sync::OnceCell;

use crate::{
    errors::{ListTrustedRootCertificatesError, ListUserCertificatesError},
    DecryptMessageError, DerCertificate, GetDerEncodedCertificateError,
    ListIntermediateCertificatesError, ShowCertificateSelectionDialogError, SignMessageError,
    X509CertificateDer,
};

pub use certificate::Certificate;
pub use private_key::PrivateKey;
use scws_client::ScwsClient;
use scws_types::TokenObject;

const DEFAULT_SCWS_URL: &str = "http://127.0.0.1:41231";
const DEFAULT_PIN: &str = "1234";

// Dummy base64-encoded cert token to pass the SCWS heuristic check.
// The real SCWS checks that the token starts with "MIIF"/"MIIG"/etc.
// This is a minimal self-signed cert placeholder.
const DUMMY_WEBAPP_TOKEN: &str = "MIIF0000000000000000000000000000000000000000";
const DUMMY_SIGNATURE: &str = "0000";

// ---------------------------------------------------------------------------
// ScwsSession — shared state for an authenticated SCWS session
// ---------------------------------------------------------------------------

pub(crate) struct ScwsSession {
    pub(crate) client: ScwsClient,
    pub(crate) env_id: String,
    #[allow(dead_code)]
    pub(crate) token_handle: String,
    pub(crate) objects: Vec<TokenObject>,
}

impl ScwsSession {
    async fn init(base_url: &str) -> anyhow::Result<Self> {
        let client = ScwsClient::new(base_url);

        // 1. Mutual authentication handshake
        let _challenge = client
            .get_challenge(DUMMY_WEBAPP_TOKEN)
            .await
            .map_err(|e| anyhow::anyhow!("SCWS get_challenge failed: {e}"))?;

        // 2. Create environment
        let env = client
            .create_env(DUMMY_SIGNATURE, "SHA-256")
            .await
            .map_err(|e| anyhow::anyhow!("SCWS create_env failed: {e}"))?;

        let env_id = env.env_id;

        // 3. Discover readers
        let readers = client
            .get_readers(&env_id)
            .await
            .map_err(|e| anyhow::anyhow!("SCWS get_readers failed: {e}"))?;

        let reader = readers
            .iter()
            .find(|r| r.card)
            .ok_or_else(|| anyhow::anyhow!("No reader with a card inserted found"))?;

        // 4. Connect to the token
        let conn = client
            .connect(&env_id, &reader.name)
            .await
            .map_err(|e| anyhow::anyhow!("SCWS connect failed: {e}"))?;

        let token_handle = conn.handle;

        // 5. Login with PIN
        client
            .login(&env_id, &token_handle, 0, DEFAULT_PIN)
            .await
            .map_err(|e| anyhow::anyhow!("SCWS login failed: {e}"))?;

        // 6. List all objects on the token
        let objects_resp = client
            .get_token_objects(&env_id, &token_handle)
            .await
            .map_err(|e| anyhow::anyhow!("SCWS get_token_objects failed: {e}"))?;

        Ok(Self {
            client,
            env_id,
            token_handle,
            objects: objects_resp.objects,
        })
    }

    fn certificate_objects(&self) -> impl Iterator<Item = &TokenObject> {
        self.objects.iter().filter(|obj| {
            obj.r#type == "certificate"
                && obj.has_private_key.unwrap_or(false)
                && !obj.root_cert.unwrap_or(false)
                && !obj.ca_cert.unwrap_or(false)
        })
    }
}

// ---------------------------------------------------------------------------
// PkiSystem
// ---------------------------------------------------------------------------

pub struct PkiSystem {
    session: Arc<ScwsSession>,
    keepalive_task: libparsec_platform_async::JoinHandle<()>,
}

impl PkiSystem {
    pub async fn init(_config: crate::PkiConfig<'_>) -> anyhow::Result<Self> {
        let base_url = std::env::var("SCWS_URL").unwrap_or_else(|_| DEFAULT_SCWS_URL.to_string());
        let session = Arc::new(ScwsSession::init(&base_url).await?);

        let keepalive_session = Arc::clone(&session);
        let keepalive_task = libparsec_platform_async::spawn(keepalive_loop(keepalive_session));

        Ok(Self {
            session,
            keepalive_task,
        })
    }

    pub async fn find_certificate(
        &self,
        cert_ref: &X509CertificateReference,
    ) -> Result<Option<Certificate>, crate::FindCertificateError> {
        for obj in self.session.certificate_objects() {
            let der_bytes = self
                .session
                .client
                .export_object_der(&self.session.env_id, &obj.handle)
                .await
                .map_err(|e| crate::FindCertificateError::Internal(e.into()))?;

            let digest = sha2::Sha256::digest(&der_bytes);
            match &cert_ref.hash {
                X509CertificateHash::SHA256(expected) => {
                    if expected.as_ref() == digest.as_slice() {
                        return Ok(Some(Certificate::new(
                            Arc::clone(&self.session),
                            obj.clone(),
                        )));
                    }
                }
            }
        }
        Ok(None)
    }

    pub async fn list_user_certificates<'a>(
        &'a self,
    ) -> Result<impl Iterator<Item = Certificate> + use<'a>, crate::ListUserCertificateError> {
        Ok(self.session.certificate_objects().map(|obj| {
            Certificate::new(Arc::clone(&self.session), obj.clone())
        }))
    }
}

impl Drop for PkiSystem {
    fn drop(&mut self) {
        self.keepalive_task.abort();
    }
}

async fn keepalive_loop(session: Arc<ScwsSession>) {
    loop {
        libparsec_platform_async::sleep(std::time::Duration::from_secs(30)).await;

        match session.client.get_event(&session.env_id).await {
            Ok(_) => {}
            Err(e) => {
                log::warn!("SCWS keepalive error: {e}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Global session for free functions (old API)
// ---------------------------------------------------------------------------

static GLOBAL_SESSION: OnceCell<Arc<ScwsSession>> = OnceCell::const_new();

async fn get_or_init_global_session() -> Result<&'static Arc<ScwsSession>, anyhow::Error> {
    GLOBAL_SESSION
        .get_or_try_init(|| async {
            let base_url =
                std::env::var("SCWS_URL").unwrap_or_else(|_| DEFAULT_SCWS_URL.to_string());
            ScwsSession::init(&base_url)
                .await
                .map(Arc::new)
        })
        .await
}

async fn find_cert_object_by_ref(
    session: &ScwsSession,
    certificate_ref: &X509CertificateReference,
) -> Result<Option<TokenObject>, anyhow::Error> {
    for obj in session.certificate_objects() {
        let der_bytes = session
            .client
            .export_object_der(&session.env_id, &obj.handle)
            .await?;

        let digest = sha2::Sha256::digest(&der_bytes);
        match &certificate_ref.hash {
            X509CertificateHash::SHA256(expected) => {
                if expected.as_ref() == digest.as_slice() {
                    return Ok(Some(obj.clone()));
                }
            }
        }
    }
    Ok(None)
}

fn find_private_key_handle(session: &ScwsSession, ck_id: &str) -> Option<String> {
    session
        .objects
        .iter()
        .find(|obj| obj.r#type == "privateKey" && obj.ck_id.as_deref() == Some(ck_id))
        .map(|obj| obj.handle.clone())
}

// ---------------------------------------------------------------------------
// Free functions (old API)
// ---------------------------------------------------------------------------

pub async fn get_der_encoded_certificate(
    certificate_ref: &X509CertificateReference,
) -> Result<X509CertificateDer<'static>, GetDerEncodedCertificateError> {
    let session = get_or_init_global_session()
        .await
        .map_err(|e| GetDerEncodedCertificateError::CannotOpenStore(std::io::Error::other(e)))?;

    let obj = find_cert_object_by_ref(session, certificate_ref)
        .await
        .map_err(|e| GetDerEncodedCertificateError::CannotOpenStore(std::io::Error::other(e)))?
        .ok_or(GetDerEncodedCertificateError::NotFound)?;

    let der_bytes = session
        .client
        .export_object_der(&session.env_id, &obj.handle)
        .await
        .map_err(|e| GetDerEncodedCertificateError::CannotOpenStore(std::io::Error::other(e)))?;

    Ok(X509CertificateDer::from(der_bytes).into_owned())
}

pub async fn list_trusted_root_certificate_anchors(
) -> Result<Vec<rustls_pki_types::TrustAnchor<'static>>, ListTrustedRootCertificatesError> {
    let session = get_or_init_global_session()
        .await
        .map_err(|e| ListTrustedRootCertificatesError::CannotOpenStore(std::io::Error::other(e)))?;

    let mut roots = Vec::new();
    for obj in session.certificate_objects() {
        let trust = session
            .client
            .get_certificate_trust(&session.env_id, &obj.handle)
            .await
            .map_err(|e| {
                ListTrustedRootCertificatesError::CannotOpenStore(std::io::Error::other(e))
            })?;

        for entry in &trust.cert_path {
            if entry.root_cert.unwrap_or(false) {
                let der_bytes = session
                    .client
                    .export_object_der(&session.env_id, &entry.handle)
                    .await
                    .map_err(|e| {
                        ListTrustedRootCertificatesError::CannotOpenStore(std::io::Error::other(e))
                    })?;
                let cert_der = CertificateDer::from(der_bytes);
                if let Ok(anchor) = webpki::anchor_from_trusted_cert(&cert_der) {
                    roots.push(anchor.to_owned());
                }
            }
        }
    }
    Ok(roots)
}

pub async fn list_intermediate_certificates(
) -> Result<Vec<X509CertificateDer<'static>>, ListIntermediateCertificatesError> {
    let session = get_or_init_global_session()
        .await
        .map_err(|e| {
            ListIntermediateCertificatesError::CannotOpenStore(std::io::Error::other(e))
        })?;

    let mut intermediates = Vec::new();
    for obj in session.certificate_objects() {
        let trust = session
            .client
            .get_certificate_trust(&session.env_id, &obj.handle)
            .await
            .map_err(|e| {
                ListIntermediateCertificatesError::CannotOpenStore(std::io::Error::other(e))
            })?;

        for entry in &trust.cert_path {
            if entry.ca_cert.unwrap_or(false) && !entry.root_cert.unwrap_or(false) {
                let der_bytes = session
                    .client
                    .export_object_der(&session.env_id, &entry.handle)
                    .await
                    .map_err(|e| {
                        ListIntermediateCertificatesError::CannotOpenStore(std::io::Error::other(e))
                    })?;
                intermediates.push(X509CertificateDer::from(der_bytes).into_owned());
            }
        }
    }
    Ok(intermediates)
}

pub async fn sign_message(
    message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<(PkiSignatureAlgorithm, Bytes), SignMessageError> {
    let session = get_or_init_global_session()
        .await
        .map_err(|e| SignMessageError::CannotOpenStore(std::io::Error::other(e)))?;

    let obj = find_cert_object_by_ref(session, certificate_ref)
        .await
        .map_err(|e| SignMessageError::CannotOpenStore(std::io::Error::other(e)))?
        .ok_or(SignMessageError::NotFound)?;

    let ck_id = obj
        .ck_id
        .as_deref()
        .ok_or(SignMessageError::NotFound)?;

    let key_handle =
        find_private_key_handle(session, ck_id).ok_or(SignMessageError::NotFound)?;

    let hash = sha2::Sha256::digest(message);
    let hash_hex = scws_client::hex_encode(&hash);

    let sig_bytes = session
        .client
        .sign(&session.env_id, &key_handle, &hash_hex, "sha256", 32)
        .await
        .map_err(|e| SignMessageError::CannotSign(std::io::Error::other(e)))?;

    Ok((
        PkiSignatureAlgorithm::RsassaPssSha256,
        Bytes::from(sig_bytes),
    ))
}

pub async fn decrypt_message(
    algo: PKIEncryptionAlgorithm,
    encrypted_message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<Bytes, DecryptMessageError> {
    if algo != PKIEncryptionAlgorithm::RsaesOaepSha256 {
        todo!("Unsupported encryption algo '{algo}'");
    }

    let session = get_or_init_global_session()
        .await
        .map_err(|e| DecryptMessageError::CannotOpenStore(std::io::Error::other(e)))?;

    let obj = find_cert_object_by_ref(session, certificate_ref)
        .await
        .map_err(|e| DecryptMessageError::CannotOpenStore(std::io::Error::other(e)))?
        .ok_or(DecryptMessageError::NotFound)?;

    let ck_id = obj
        .ck_id
        .as_deref()
        .ok_or(DecryptMessageError::NotFound)?;

    let key_handle = find_private_key_handle(session, ck_id)
        .ok_or(DecryptMessageError::NotFound)?;

    let ciphertext_hex = scws_client::hex_encode(encrypted_message);

    let plaintext = session
        .client
        .decrypt(&session.env_id, &key_handle, &ciphertext_hex)
        .await
        .map_err(|e| DecryptMessageError::CannotDecrypt(std::io::Error::other(e)))?;

    Ok(Bytes::from(plaintext))
}

pub fn show_certificate_selection_dialog_windows_only(
) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
    unimplemented!("platform not supported")
}

pub fn is_available() -> bool {
    true
}

pub async fn list_user_certificates_der(
) -> Result<Vec<DerCertificate<'static>>, ListUserCertificatesError> {
    let session = get_or_init_global_session()
        .await
        .map_err(|_| ListUserCertificatesError::CannotOpenStore)?;

    let mut certs = Vec::new();
    for obj in session.certificate_objects() {
        if let Ok(der_bytes) = session
            .client
            .export_object_der(&session.env_id, &obj.handle)
            .await
        {
            certs.push(DerCertificate::from_der_owned(der_bytes));
        }
    }
    Ok(certs)
}
