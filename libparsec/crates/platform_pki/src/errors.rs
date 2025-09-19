// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

error_set::error_set! {
    BaseCertStoreError = {
        #[display("Cannot open certificate store: {0}")]
        CannotOpenStore(std::io::Error),
        #[display("Cannot find certificate")]
        NotFound,
        #[display("Cannot get certificate info: {0}")]
        CannotGetCertificateInfo(std::io::Error),
    };
    BaseKeyPairError = {
        #[display("Cannot acquire keypair related to certificate: {0}")]
        CannotAcquireKeypair(std::io::Error),
    };
    GetDerEncodedCertificateError = BaseCertStoreError;
    SignMessageError = BaseCertStoreError || BaseKeyPairError || {
        #[display("Cannot sign message: {0}")]
        CannotSign(std::io::Error),
    };
    EncryptMessageError = BaseCertStoreError || BaseKeyPairError || {
        #[display("Cannot encrypt message: {0}")]
        CannotEncrypt(std::io::Error),
    };
    DecryptMessageError = BaseCertStoreError || BaseKeyPairError || {
        #[display("Cannot decrypt message: {0}")]
        CannotDecrypt(std::io::Error),
    };
}
