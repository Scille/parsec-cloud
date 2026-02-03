// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use libparsec_tests_lite::parsec_test;

// Root certificates subject fields
const BLACK_MESA_CA_SUBJECT: &[u8] =
    b"1\x160\x14\x06\x03U\x04\x03\x0c\rBlack Mesa CA1\x130\x11\x06\x03U\x04\n\x0c\nBlack Mesa";
const APERTURE_SCIENCE_CA_SUBJECT: &[u8] = b"1\x1c0\x1a\x06\x03U\x04\x03\x0c\x13Aperture Science CA1\x190\x17\x06\x03U\x04\n\x0c\x10Aperture Science";

#[parsec_test]
async fn list_roots() {
    let certifs = crate::list_trusted_root_certificate_anchors()
        .await
        .unwrap();

    // Should obtain at least Black Mesa and Aperture Science certificates.
    // Note we ignore other certificates, as the actual set of root certificates
    // depends of the machine.
    for subject in &[BLACK_MESA_CA_SUBJECT, APERTURE_SCIENCE_CA_SUBJECT] {
        assert!(
            certifs.iter().any(|c| c
                .subject
                .as_ref()
                .windows(subject.len())
                .any(|w| w == *subject)),
            "Missing expected root certificate with subject: {:?}",
            String::from_utf8_lossy(subject),
        );
    }
}
