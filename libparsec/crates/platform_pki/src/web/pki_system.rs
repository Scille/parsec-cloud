// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    AvailablePkiCertificate, PkiCertificate, PkiScwsConfig, PkiSystemInitError,
    PkiSystemListUserCertificateError, PkiSystemOpenCertificateError,
};
use libparsec_types::prelude::*;

#[derive(Debug)]
pub struct PlatformPkiSystem {}

impl PlatformPkiSystem {
    pub async fn init(_scws_config: Option<PkiScwsConfig>) -> Result<Self, PkiSystemInitError> {
        // Retrieve the `SCWS` object from the global scope.
        // It has been set there by the SharedWorker's TypeScript wrapper (see `web_shared_worker.ts`)
        // which dynamically imported `scwsapi.js` before calling `pki_init_for_scws` (which itself is
        // expected to call us).
        let global = js_sys::global();
        let scws = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("SCWS"))
            .map_err(|e| {
                PkiSystemInitError::Internal(anyhow::anyhow!(
                    "Failed to access global `SCWS` object: {e:?}"
                ))
            })?;
        if scws.is_undefined() || scws.is_null() {
            return Err(PkiSystemInitError::Internal(anyhow::anyhow!(
                "Global `SCWS` object not found, `scwsapi.js` was not loaded"
            )));
        }

        Err(PkiSystemInitError::NotAvailable)
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
        unimplemented!("platform not supported");
    }
}
