// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::{PkiSystem, X509CertificateDer};

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to verify.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: libparsec_types::X509CertificateHash,
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    // Config dir is only used for testbed, we can safely provide an empty path here
    let pki = PkiSystem::init(std::path::Path::new(""), None)
        .await
        .context("Failed to initialize PKI system")?;

    let cert_ref: libparsec_types::X509CertificateReference = args.certificate_hash.into();
    let cert = pki
        .open_certificate(&cert_ref)
        .await
        .context("Failed to find certificate")?
        .context("Certificate not found")?;
    let validation_path = cert
        .get_validation_path()
        .await
        .context("Failed to get validation path")?;

    println!("trusted path:");
    println!(
        "  root: subject={}",
        display_x509_raw_name(&validation_path.root.subject)
    );
    if !validation_path.intermediates.is_empty() {
        println!("  intermediate certificates:");
        for (i, cert) in validation_path.intermediates.iter().enumerate() {
            println!("    {:02}. {}", i + 1, display_x509_certificate_der(cert)?);
        }
    }
    println!(
        "  leaf: {}",
        display_x509_certificate_der(&validation_path.leaf)?
    );

    Ok(())
}

fn display_x509_raw_name(raw_name: &[u8]) -> String {
    use x509_cert::der::{DecodeValue, Header, SliceReader, Tag};

    let components = x509_cert::name::Name::decode_value(
        &mut SliceReader::new(raw_name).unwrap(),
        Header::new(Tag::Sequence, raw_name.len()).unwrap(),
    )
    .map(libparsec_platform_pki::x509::extract_dn_list_from_rnd_seq)
    .unwrap_or_default();

    format!("{:?}", components)
}

fn display_x509_certificate_der(cert: &X509CertificateDer<'_>) -> anyhow::Result<String> {
    let details = libparsec_platform_pki::UserX509CertificateDetails::load_der(cert.as_ref())
        .context("Invalid certificate DER")?;

    Ok(format!(
        "subject={subject:?}, issuer={issuer:?}",
        subject = details.subject,
        issuer = details.issuer
    ))
}
