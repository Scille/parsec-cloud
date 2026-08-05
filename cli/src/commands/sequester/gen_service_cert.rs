use anyhow::Context as _;
use futures::TryFutureExt;
use itertools::Itertools;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

use libparsec_crypto::{SequesterPublicKeyDer, SequesterSigningKeyDer};
use libparsec_types::{DateTime, SequesterServiceCertificate, SequesterServiceID};

const SEQUESTER_SERVICE_CERTIFICATE_PEM_HEADER: &[u8] =
    b"-----BEGIN PARSEC SEQUESTER SERVICE CERTIFICATE-----";
const SEQUESTER_SERVICE_CERTIFICATE_PEM_FOOTER: &[u8] =
    b"-----END PARSEC SEQUESTER SERVICE CERTIFICATE-----";

/// Generate a signed service certificate
#[derive(clap::Args)]
pub struct Args {
    /// Service name
    #[arg(long, value_hint = clap::ValueHint::Other)]
    service_label: String,
    /// File containing the public key to use when generating the service's certificate
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    service_public_key: PathBuf,
    /// File containing the private key used to sign the service's certificate
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    authority_private_key: PathBuf,
    /// Timestamp at which the certificate is valid (similar to `notBefore` in a x509 certificate)
    #[arg(long, default_value_t = DateTime::now())]
    datetime: DateTime,
    /// Write the certificate at the given path.
    #[arg(value_hint = clap::ValueHint::FilePath)]
    output: crate::utils::OutputFile,
}

pub async fn main(_ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let Args {
        service_label,
        service_public_key,
        authority_private_key,
        output,
        datetime,
    } = args;
    let (svc_pubkey_pem, authority_pkey_pem) = tokio::try_join!(
        tokio::fs::read_to_string(&service_public_key).map_err(|e| anyhow::Error::from(e).context(
            format!(
                "Cannot read service public key file '{}'",
                service_public_key.display()
            )
        )),
        tokio::fs::read_to_string(&authority_private_key).map_err(|e| anyhow::Error::from(e)
            .context(format!(
                "Cannot read authority private key file '{}'",
                authority_private_key.display()
            )))
    )?;

    let encryption_key_der = SequesterPublicKeyDer::load_pem(&svc_pubkey_pem)
        .context("Failed to parse service's public key")?;
    let authority_pkey = SequesterSigningKeyDer::load_pem(&authority_pkey_pem)
        .context("Failed to parse authority private key")?;

    let service_id = SequesterServiceID::default();

    let certificate = SequesterServiceCertificate {
        timestamp: datetime,
        service_id,
        service_label,
        encryption_key_der,
    };

    let data = authority_pkey.sign(&certificate.dump());
    let b64_data = data_encoding::BASE64.encode(&data);
    let formatted_data = textwrap::wrap(&b64_data, 64);
    let mut buffers = Vec::from_iter(
        // TODO: Replace `Itertools::intersperse` with `Iterator::intersperse` once stabilized
        // https://github.com/rust-lang/rust/issues/79524
        Itertools::intersperse(
            std::iter::once(std::io::IoSlice::new(
                SEQUESTER_SERVICE_CERTIFICATE_PEM_HEADER,
            ))
            .chain(
                formatted_data
                    .iter()
                    .map(|s| std::io::IoSlice::new(s.as_bytes())),
            )
            .chain(std::iter::once(std::io::IoSlice::new(
                SEQUESTER_SERVICE_CERTIFICATE_PEM_FOOTER,
            ))),
            std::io::IoSlice::new(b"\n"),
        )
        .chain(std::iter::once(std::io::IoSlice::new(b"\n"))),
    );

    output
        .with_created_file(async |writer| {
            let mut buffers = buffers.as_mut_slice();
            while !buffers.is_empty() {
                let written = writer.write_vectored(buffers).await?;
                std::io::IoSlice::advance_slices(&mut buffers, written);
            }
            Ok(())
        })
        .await
        .context("Failed to write certificate to output file")
}
