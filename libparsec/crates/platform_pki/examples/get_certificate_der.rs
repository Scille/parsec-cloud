// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use std::fmt::Debug;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::get_der_encoded_certificate;
use libparsec_types::{X509CertificateHash, X509CertificateReference, X509URIFlavorValue};
use sha2::Digest;

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to get content of.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: Option<X509CertificateHash>,
    #[arg(value_parser = utils::Base64SerdeParser::<X509URIFlavorValue>::default())]
    certificate_uri: Option<X509URIFlavorValue>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:x?}");

    let cert_ref: X509CertificateReference = match args.certificate_hash {
        Some(hash) => {
            let h: X509CertificateReference = hash.into();
            if let Some(uri) = args.certificate_uri {
                h.add_or_replace_uri_wrapped(uri)
            } else {
                h
            }
        }
        #[cfg(target_os = "windows")]
        None => libparsec_platform_pki::show_certificate_selection_dialog_windows_only()
            .map_err(anyhow::Error::from)
            .and_then(|res| res.ok_or_else(|| anyhow::anyhow!("User did not select a certificate")))
            .context("Failed to get certificate from user")?,
        #[cfg(not(target_os = "windows"))]
        None => {
            return Err(anyhow::anyhow!(
                "certificate hash is required on non-Windows OS"
            ));
        }
    };

    if let Some(uri) = cert_ref.uris().next() {
        println!("original uri: {uri}");
    }
    println!("original fingerprint: {}", cert_ref.hash);
    let certificate = get_der_encoded_certificate(&cert_ref).context("Get certificate")?;

    let uri = cert_ref.uris().next().unwrap();
    println!("uri: {uri}");
    println!("serialized uri: {}", display_serialized_uri(uri));
    println!("fingerprint: {}", cert_ref.hash);
    {
        let digest = sha2::Sha256::digest(&certificate);
        let hash = X509CertificateHash::SHA256(Box::new(digest.into()));
        println!("Manually calculated fingerprint: {hash}");
    }
    println!(
        "content: {}",
        data_encoding::BASE64.encode_display(&certificate)
    );

    Ok(())
}

fn display_serialized_uri(uri: &X509URIFlavorValue) -> impl std::fmt::Display {
    let encoded = rmp_serde::encode::to_vec(uri).unwrap();
    data_encoding::BASE64.encode(&encoded)
}
