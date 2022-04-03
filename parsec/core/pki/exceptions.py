# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

"""
PkiEnrollmentError: all PKI enrollment related errors
+- PkiEnrollmentLocalPendingError: all local pending enrollment errors
   +- PkiEnrollmentLocalPendingNotFoundError: when the file corresponding to a given enrollment id cannot be not found
   +- PkiEnrollmentLocalPendingValidationError: when a local pending enrollment cannot be loaded
   +- PkiEnrollmentLocalPendingPackingError: when a local pending enrollment cannot be loaded
   +- PkiEnrollmentLocalPendingCryptoError: when the private keys cannot be encrypted/decrypted
+- PkiEnrollmentCertificateError: all certificate errors
   +- PkiEnrollmentCertificateNotFoundError: when the certificate corresponding to a given certificate id cannot be found
   +- PkiEnrollmentCertificateValidationError: when the provided certificate cannot be validated using the configured trust roots
   +- PkiEnrollmentCertificateSignatureError: when the provided signature does not correspond to the certificate public key
   +- PkiEnrollmentCertificateCryptoError: when any of the required certificate-replated crypto operation fails
+- PkiEnrollmentPayloadError: all the enrollment payload errors
   +- PkiEnrollmentPayloadValidationError: when some enrollement information cannot be properly loaded
+- PkiEnrollmentRemoteError: all the errors coming from a enrollment command on the backend
   +- PkiEnrollmentSubmitError: all the errors coming from the pki_enrollment_submit command
   +- PkiEnrollmentSubmitEnrollmentIdAlreadyUsedError: when the enrollment ID is alread used
   +- PkiEnrollmentSubmitCertificateAlreadySubmittedError: when the certificate has already been submited
   +- PkiEnrollmentSubmitCertificateAlreadyEnrolledError: when the certificate is already enrolled
"""


class PkiEnrollmentError(Exception):
    @property
    def message(self):
        """All PkiEnrollmentError should have a message as first argument."""
        return str(self.args[0])

    def __str__(self):
        return self.message


# Local pending enrollment errors


class PkiEnrollmentLocalPendingError(PkiEnrollmentError):
    pass


class PkiEnrollmentLocalPendingNotFoundError(PkiEnrollmentLocalPendingError):
    pass


class PkiEnrollmentLocalPendingValidationError(PkiEnrollmentLocalPendingError):
    pass


class PkiEnrollmentLocalPendingPackingError(PkiEnrollmentLocalPendingError):
    pass


class PkiEnrollmentLocalPendingCryptoError(PkiEnrollmentLocalPendingError):
    pass


# Certificate errors


class PkiEnrollmentCertificateError(PkiEnrollmentError):
    pass


class PkiEnrollmentCertificateNotFoundError(PkiEnrollmentCertificateError):
    pass


class PkiEnrollmentCertificateValidationError(PkiEnrollmentCertificateError):
    pass


class PkiEnrollmentCertificateSignatureError(PkiEnrollmentCertificateError):
    pass


class PkiEnrollmentCertificateCryptoError(PkiEnrollmentCertificateError):
    pass


# Payload errors


class PkiEnrollmentPayloadError(PkiEnrollmentError):
    pass


class PkiEnrollmentPayloadValidationError(PkiEnrollmentPayloadError):
    pass


# Submit Errors


class PkiEnrollmentRemoteError(PkiEnrollmentError):
    pass


class PkiEnrollmentSubmitError(PkiEnrollmentRemoteError):
    pass


class PkiEnrollmentSubmitEnrollmentIdAlreadyUsedError(PkiEnrollmentSubmitError):
    pass


class PkiEnrollmentSubmitCertificateAlreadySubmittedError(PkiEnrollmentSubmitError):
    pass


class PkiEnrollmentSubmitCertificateAlreadyEnrolledError(PkiEnrollmentSubmitError):
    pass
