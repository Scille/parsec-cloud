// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{fmt::Debug, str::FromStr};

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::{get_der_encoded_certificate, CertificateHash, CertificateReference};

#[derive(Debug, Clone)]
struct CertificateHashArg(CertificateHash);

impl FromStr for CertificateHashArg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hash_ty, hex_data) = s
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("Missing `:` delimiter"))?;
        let raw_data = hex::decode(hex_data)?;
        if hash_ty.eq_ignore_ascii_case("sha256") {
            Ok(CertificateHashArg(CertificateHash::SHA256(
                raw_data
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid data size"))?,
            )))
        } else {
            Err(anyhow::anyhow!("Unsupported hash type `{hash_ty}`"))
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to get content of.
    certificate_hash: Option<CertificateHashArg>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    let cert_ref: CertificateReference = match args.certificate_hash {
        Some(hash) => CertificateReference::Hash(hash.0),
        #[cfg(target_os = "windows")]
        None => {
            let cert_ref = libparsec_platform_pki::show_certificate_selection_dialog()
                .map_err(anyhow::Error::from)
                .and_then(|res| {
                    res.ok_or_else(|| anyhow::anyhow!("User did not select a certificate"))
                })
                .context("Failed to get certificate from user")?;
            CertificateReference::from(cert_ref)
        }
        #[cfg(not(target_os = "windows"))]
        None => {
            return Err(anyhow::anyhow!(
                "certificate hash is required on non-Windows OS"
            ));
        }
    };

    let cert = get_der_encoded_certificate(&cert_ref).context("Get certificate")?;

    println!("id: {}", hex::encode(cert.cert_ref.id));
    println!("fingerprint: {}", hex::encode(cert.cert_ref.hash));
    println!("content: {}", hex::encode(cert.der_content));

    Ok(())
}
