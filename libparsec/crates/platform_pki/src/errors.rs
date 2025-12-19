// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

error_set::error_set! {
    BaseCertStoreError := {
        #[display("Cannot open certificate store: {0}")]
        CannotOpenStore(std::io::Error),
        #[display("Cannot find certificate")]
        NotFound,
        #[display("Cannot get certificate info: {0}")]
        CannotGetCertificateInfo(std::io::Error),
    }
    BaseKeyPairError := {
        #[display("Cannot acquire keypair related to certificate: {0}")]
        CannotAcquireKeypair(std::io::Error),
    }
    GetDerEncodedCertificateError := BaseCertStoreError
    ListTrustedRootCertificatesError := BaseCertStoreError || {
        #[display("Invalid root certificate: {0}")]
       InvalidRootCertificate(webpki::Error)
    }
    ListIntermediateCertificatesError := BaseCertStoreError
    SignMessageError := BaseCertStoreError || BaseKeyPairError || {
        #[display("Cannot sign message: {0}")]
        CannotSign(std::io::Error),
    }
    EncryptMessageError := BaseCertStoreError || BaseKeyPairError || {
        #[display("Cannot encrypt message: {0}")]
        CannotEncrypt(std::io::Error),
    }
    DecryptMessageError := BaseCertStoreError || BaseKeyPairError || {
        #[display("Cannot decrypt message: {0}")]
        CannotDecrypt(std::io::Error),
    }
    InvalidCertificateDer := {
        #[display("Invalid certificate: {0}")]
        InvalidCertificateDer(webpki::Error),
    }
    VerifySignatureError := {
        #[display("Invalid signature for the given message and certificate")]
        InvalidSignature,
        #[display("Unexpected signature will verifying signature of a message: {0}")]
        UnexpectedError(webpki::Error)
    }
    InvalidPemContent := {
        #[display("Invalid PEM content: {0}")]
        InvalidPemContent(rustls_pki_types::pem::Error)
    }
    CreateLocalPendingError := EncryptMessageError
    VerifyCertificateError := {
        #[display("Time out of valid range: {0}")]
        DateTimeOutOfRange(chrono::OutOfRangeError),
        #[display("The provided certificate cannot be trusted: {0}")]
        Untrusted(webpki::Error),
    }
    ValidatePayloadError := InvalidCertificateDer
        || VerifyCertificateError
        || VerifySignatureError
    DataError := {
        DataError(libparsec_types::DataError)
    }
    LoadSubmitPayloadError := ValidatePayloadError || DataError
    LoadAnswerPayloadError := ValidatePayloadError
        || DataError
        || ListTrustedRootCertificatesError
    GetValidationPathForCertError := ListTrustedRootCertificatesError
        || ListIntermediateCertificatesError
        || InvalidCertificateDer
        || VerifyCertificateError
}
