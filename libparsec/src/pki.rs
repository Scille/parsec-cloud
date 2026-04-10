// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_async::lock::Mutex as AsyncMutex;
pub use libparsec_platform_pki::{
    x509::DistinguishedNameValue, AvailablePkiCertificate, PkiCertificateRequestPrivateKeyError,
    PkiScwsConfig, PkiSystemInitError, PkiSystemListUserCertificateError,
    PkiSystemOpenCertificateError, ShowCertificateSelectionDialogError, UserX509CertificateDetails,
    UserX509CertificateLoadError,
};
use libparsec_types::prelude::*;

use crate::handle::{
    borrow_from_handle, register_handle, take_and_close_handle, Handle, HandleItem,
};

pub async fn show_certificate_selection_dialog_windows_only(
) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
    libparsec_platform_pki::show_certificate_selection_dialog_windows_only()
}

static PKI_SYSTEM: AsyncMutex<Option<std::sync::Arc<libparsec_platform_pki::PkiSystem>>> =
    AsyncMutex::new(None);

pub async fn pki_init_for_native(config_dir: &std::path::Path) -> Result<(), PkiSystemInitError> {
    let mut guard = PKI_SYSTEM.lock().await;
    if guard.is_some() {
        // PKI already initialized, go idempotent
        return Ok(());
    }

    let pki_system = libparsec_platform_pki::PkiSystem::init(config_dir, None).await?;

    *guard = Some(std::sync::Arc::new(pki_system));

    Ok(())
}

pub async fn pki_init_for_scws(
    config_dir: &std::path::Path,
    parsec_addr: libparsec_types::ParsecAddr,
) -> Result<(), PkiSystemInitError> {
    let mut guard = PKI_SYSTEM.lock().await;
    if guard.is_some() {
        // PKI already initialized, go idempotent
        return Ok(());
    }

    let scws_config = PkiScwsConfig {
        parsec_addr,
        proxy: libparsec_client_connection::ProxyConfig::default(),
    };
    let pki_system = libparsec_platform_pki::PkiSystem::init(config_dir, Some(scws_config)).await?;

    *guard = Some(std::sync::Arc::new(pki_system));
    Ok(())
}

async fn get_pki_system() -> anyhow::Result<std::sync::Arc<libparsec_platform_pki::PkiSystem>> {
    let guard = PKI_SYSTEM.lock().await;
    guard
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("PKI system not initialized"))
        .cloned()
}

pub async fn pki_list_user_certificates(
) -> Result<Vec<AvailablePkiCertificate>, PkiSystemListUserCertificateError> {
    let pki_system = get_pki_system()
        .await
        .map_err(libparsec_platform_pki::PkiSystemListUserCertificateError::Internal)?;
    pki_system.list_user_certificates().await
}

pub async fn pki_open_certificate(
    cert_ref: &X509CertificateReference,
) -> Result<Handle, PkiSystemOpenCertificateError> {
    let pki_system = get_pki_system()
        .await
        .map_err(libparsec_platform_pki::PkiSystemOpenCertificateError::Internal)?;
    let cert = pki_system.open_certificate(cert_ref).await?;
    let handle = register_handle(HandleItem::PkiCertificate(Arc::new(cert)));
    Ok(handle)
}

#[derive(Debug, thiserror::Error)]
pub enum PkiCertificateCloseError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn pki_certificate_close(handle: Handle) -> Result<(), PkiCertificateCloseError> {
    let pki_certificate = take_and_close_handle(handle, |x| match *x {
        HandleItem::PkiCertificate(pki_certificate) => Ok(pki_certificate),
        _ => Err(x),
    })?;

    drop(pki_certificate);
    Ok(())
}

pub async fn pki_certificate_open_private_key(
    handle: Handle,
) -> Result<Handle, PkiCertificateRequestPrivateKeyError> {
    let pki_certificate = borrow_from_handle(handle, |x| match x {
        HandleItem::PkiCertificate(pki_certificate) => Some(pki_certificate.clone()),
        _ => None,
    })
    .map_err(PkiCertificateRequestPrivateKeyError::Internal)?;

    let pki_private_key = pki_certificate.request_private_key().await?;
    let handle = register_handle(HandleItem::PkiPrivateKey {
        certificate: pki_certificate.clone(),
        private_key: Arc::new(pki_private_key),
    });
    Ok(handle)
}

#[derive(Debug, thiserror::Error)]
pub enum PkiPrivateKeyCloseError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn pki_private_key_close(handle: Handle) -> Result<(), PkiPrivateKeyCloseError> {
    take_and_close_handle(handle, |x| match *x {
        // Nothing to take, we just want the arc objects to be dropped
        HandleItem::PkiPrivateKey { .. } => Ok(()),
        _ => Err(x),
    })?;
    Ok(())
}
