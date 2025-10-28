// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use std::fmt::Debug;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::get_der_encoded_certificate;
use libparsec_types::{X509CertificateHash, X509CertificateReference};
use sha2::Digest;

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to get content of.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: Option<X509CertificateHash>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    let cert_ref: X509CertificateReference = match args.certificate_hash {
        Some(hash) => hash.into(),
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

    let cert = get_der_encoded_certificate(&cert_ref).context("Get certificate")?;

    println!("id: {}", &cert.cert_ref.uris().next().unwrap());
    println!("fingerprint: {}", cert.cert_ref.hash);
    {
        let digest = sha2::Sha256::digest(&cert.der_content);
        let hash = X509CertificateHash::SHA256(Box::new(digest.into()));
        println!("Manually calculated fingerprint: {hash}");
    }
    println!(
        "content: {}",
        data_encoding::BASE64.encode_display(&cert.der_content)
    );

    Ok(())
}
