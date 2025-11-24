// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use anyhow::Context;

use clap::Parser;

mod utils;

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    cert: utils::CertificateOrRef,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    env_logger::init();

    let raw_cert = args
        .cert
        .get_certificate()
        .context("Invalid cert arguments")?;
    let cert =
        libparsec_platform_pki::x509::X509CertificateInformation::load_der(raw_cert.as_ref())
            .context("Invalid cert content")?;
    println!("{cert:#x?}");

    Ok(())
}
