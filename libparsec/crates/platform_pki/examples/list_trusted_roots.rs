// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_pki::{list_trusted_root_certificate_der, x509, Certificate};
use rustls_pki_types::TrustAnchor;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let roots = list_trusted_root_certificate_der()?;

    println!("Found {} trusted roots", roots.len());

    for (i, der) in roots.iter().enumerate() {
        match x509::Certificate::load_der(der) {
            Ok(cert) => {
                println!("Certificate #{i}:");
                println!("  subject: {:?}", cert.subject);
                println!("  issuer: {:?}", cert.issuer);
                println!("  ext: {:?}", cert.extensions);

                match TrustAnchor::try_from(&Certificate::from_der(der.as_ref())) {
                    Ok(anchor) => println!("  anchor: {anchor:?}"),
                    Err(e) => println!("anchor: fail to parse: {e}"),
                }
            }
            Err(e) => println!("Failed to load certificate #{i}: {e}"),
        }
    }
    Ok(())
}
