// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::fmt::Debug;

use crate::{
    AvailablePkiCertificate, PkiCertificate, PkiScwsConfig, PkiSystemInitError,
    PkiSystemListUserCertificateError, PkiSystemOpenCertificateError,
};
use futures::{StreamExt, TryStreamExt};
use libparsec_types::prelude::*;
use wasm_bindgen::JsCast;

pub struct PlatformPkiSystem {
    scws: scwsapi::Scws,
}

impl Debug for PlatformPkiSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlatformPkiSystem").finish_non_exhaustive()
    }
}

impl PlatformPkiSystem {
    pub async fn init(
        config_dir: &std::path::Path,
        scws_config: Option<PkiScwsConfig>,
    ) -> Result<Self, PkiSystemInitError> {
        let Some(config) = scws_config else {
            log::warn!("No config provided cannot prepare SCWS");
            return Err(PkiSystemInitError::NotAvailable);
        };
        let (scws, cert) = get_scws_values_from_global_context()?;
        let (svc_rep, challenge) = request_server_challenge_from_scws(&scws, &cert).await?;
        let cmd = libparsec_client_connection::AnonymousServerCmds::new(
            config_dir,
            config.parsec_addr,
            config.proxy,
        )
        .map_err(|e| PkiSystemInitError::Internal(e.context("Cannot start HTTP client")))?;

        let signature = ask_server_to_sign_scws_challenge(cmd, svc_rep, challenge).await?;

        scws.create_environment(&signature)
            .await
            .map_err(|e| match e {
                scwsapi::CreateEnvironmentError::CreateError(js_value) => {
                    PkiSystemInitError::Internal(anyhow::anyhow!(
                        "Failed to complete the initialization of SCWS ({js_value:?})"
                    ))
                }
            })?;

        Ok(Self { scws })
    }

    pub async fn open_certificate(
        &self,
        _cert_ref: &X509CertificateReference,
    ) -> Result<PkiCertificate, PkiSystemOpenCertificateError> {
        unimplemented!("platform not supported")
    }

    pub async fn list_user_certificates(
        &self,
    ) -> Result<Vec<AvailablePkiCertificate>, PkiSystemListUserCertificateError> {
        self.scws
            .iter_working_reader()
            .then(|token| async move {
                futures::stream::iter(token.iter_objects().await.filter_map(|obj| {
                    if let scwsapi::Object::Certificate(cert) = obj {
                        Some(cert)
                    } else {
                        None
                    }
                }))
                .then(|raw_cert| async move {
                    let cert = super::PlatformPkiCertificate::from(raw_cert);
                    cert.get_der()
                        .await
                        .map(|der| {
                            AvailablePkiCertificate::load_der(
                                Some(cert.certificate.ck_label()),
                                der.as_ref(),
                            )
                        })
                        .map_err(|e| {
                            PkiSystemListUserCertificateError::Internal(anyhow::anyhow!(
                                "Cannot obtain DER for certificate {name}: {e}",
                                name = cert.certificate.ck_label()
                            ))
                        })
                })
                .try_collect::<Vec<_>>()
                .await
            })
            .try_concat()
            .await
    }
}

// Retrieve the `SCWS` object from the global scope.
// It has been set there by the SharedWorker's TypeScript wrapper (see `web_shared_worker.ts`)
// which dynamically imported `scwsapi.js` before calling `pki_init_for_scws` (which itself is
// expected to call us).
fn get_scws_values_from_global_context() -> Result<(scwsapi::Scws, String), PkiSystemInitError> {
    let global = js_sys::global();
    let raw_scws = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("SCWS"))
        .map_err(|e| {
            PkiSystemInitError::Internal(anyhow::anyhow!(
                "Failed to access global `SCWS` object: {e:?}"
            ))
        })?;
    let raw_webapp_cert =
        js_sys::Reflect::get(&global, &"SCWS_WEBAPP_CERT".into()).map_err(|e| {
            PkiSystemInitError::Internal(anyhow::anyhow!(
                "Failed to access global `SCWS_WEBAPP_CERT` object: {e:?}"
            ))
        })?;
    // If `SCWS` is not an object or `SCWS_WEBAPP_CERT` is not a string,
    // we should abort the initialization, we would not be able to continue anyway.
    if !raw_scws.is_object() || !raw_webapp_cert.is_string() {
        return Err(PkiSystemInitError::NotAvailable);
    }
    let scws: scwsapi::Scws = raw_scws
        .dyn_into::<scwsapi_sys::Scws>()
        .map_err(|e| {
            PkiSystemInitError::Internal(anyhow::anyhow!(
                "Cannot cast SCWS object to correct type ({e:?})"
            ))
        })
        .map(Into::into)?;
    let cert = raw_webapp_cert
        .dyn_into::<js_sys::JsString>()
        .map_err(|e| {
            PkiSystemInitError::Internal(anyhow::anyhow!(
                "Cannot cast SCWS_WEBAPP_CERT object to string ({e:?})"
            ))
        })?;

    Ok((scws, cert.into()))
}

async fn request_server_challenge_from_scws(
    scws: &scwsapi::Scws,
    cert: &str,
) -> Result<(scwsapi::ServiceResponse, AccessToken), PkiSystemInitError> {
    let challenge = libparsec_types::AccessToken::default();
    let svc_rep = scws
        .find_service(cert, challenge.as_bytes())
        .await
        .map_err(|e| match e {
            scwsapi::FindServiceError::FindError(js_value) => PkiSystemInitError::Internal(
                anyhow::anyhow!("error while looking for SCWS service ({js_value:?})"),
            ),
            scwsapi::FindServiceError::InvalidChallenge(e) => PkiSystemInitError::Internal(
                anyhow::anyhow!("SCWS return an invalid challenge ({e})"),
            ),
            scwsapi::FindServiceError::InvalidCryptogram(_) => PkiSystemInitError::Internal(
                anyhow::anyhow!("SCWS return an invalid signature ({e})"),
            ),
        })?;

    Ok((svc_rep, challenge))
}

async fn ask_server_to_sign_scws_challenge(
    cmd: libparsec_client_connection::AnonymousServerCmds,
    svc_rep: scwsapi::ServiceResponse,
    challenge: AccessToken,
) -> Result<Bytes, PkiSystemInitError> {
    use libparsec_protocol::anonymous_server_cmds::latest::scws_service_mutual_challenges::{
        Rep, Req,
    };

    let srv_challenge = Req {
        scws_service_challenge_key_id: svc_rep.key_id as u64,
        scws_service_challenge_payload: Bytes::copy_from_slice(challenge.as_bytes()),
        scws_service_challenge_signature: Bytes::from(svc_rep.cryptogram),
        web_application_challenge_payload: Bytes::from(svc_rep.challenge),
    };
    let srv_rep = cmd.send(srv_challenge).await.map_err(|e| {
        PkiSystemInitError::Internal(anyhow::anyhow!("Communication error with server: {e}"))
    })?;
    match srv_rep {
        Rep::InvalidScwsServiceChallengeSignature => Err(PkiSystemInitError::Internal(
            anyhow::anyhow!("Server does not validate the signature made by SCWS"),
        )),
        Rep::InvalidWebApplicationChallengePayload => Err(PkiSystemInitError::Internal(
            anyhow::anyhow!("Server think our challenge is invalid"),
        )),
        Rep::NotAvailable => Err(PkiSystemInitError::NotAvailable),
        Rep::Ok {
            web_application_challenge_signature,
        } => Ok(web_application_challenge_signature),
        Rep::UnknownScwsServiceChallengeKeyId => Err(PkiSystemInitError::Internal(
            anyhow::anyhow!("Server does not know the public key used by SCWS"),
        )),
        Rep::UnknownStatus {
            unknown_status,
            reason,
        } => {
            log::warn!("Unknown server status {unknown_status} (reason: {reason:?})");
            Err(PkiSystemInitError::NotAvailable)
        }
    }
}
