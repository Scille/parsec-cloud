# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

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
    +- PkiEnrollmentCertificatePinCodeUnavailableError: when the user declines to provide the secret key password
+- PkiEnrollmentPayloadError: all the enrollment payload errors
    +- PkiEnrollmentPayloadValidationError: when some enrollement information cannot be properly loaded
+- PkiEnrollmentRemoteError: all the errors coming from a enrollment command on the backend
    +- PkiEnrollmentSubmitError: all the errors coming from the pki_enrollment_submit command
        +- PkiEnrollmentSubmitEnrollmentIdAlreadyUsedError: when the enrollment ID is alread used
        +- PkiEnrollmentSubmitCertificateAlreadySubmittedError: when the certificate has already been submited
        +- PkiEnrollmentSubmitCertificateAlreadyEnrolledError: when the certificate is already enrolled
        +- PkiEnrollementEmailAlreadyUsedError: when the certificate email address is already attributes to an active user
    +- PkiEnrollmentListError: all the errors coming from the pki_enrollment_list command
        +- PkiEnrollmentListNotAllowedError: when listing the enrollments is not allowed
    +- PkiEnrollmentRejectError: all the errors from the pki_enrollment_reject command
        +- PkiEnrollmentRejectNotAllowedError: when rejecting the enrollment is not allowed
        +- PkiEnrollmentRejectNotFoundError: when the enrollment is not found
        +- PkiEnrollmentRejectNoLongerAvailableError: when the enrollment is not longer available
    +- PkiEnrollmentAcceptError: all the errors coming from the pki_enrollment_reject command
        +- PkiEnrollmentAcceptNotAllowedError: when accepting the enrollment is not allowed
        +- PkiEnrollmentAcceptInvalidPayloadDataError: when the payload data is invalid
        +- PkiEnrollmentAcceptInvalidDataError: when the new user data is invalid
        +- PkiEnrollmentAcceptInvalidCertificationError: when the new user certification is invalid
        +- PkiEnrollmentAcceptNotFoundError: when the enrollment is not found
        +- PkiEnrollmentAcceptNoLongerAvailableError: when the enrollment is no longer available
        +- PkiEnrollmentAcceptAlreadyExistsError: when the new user already exists
        +- PkiEnrollmentAcceptActiveUsersLimitReachedError: when the activate users limit has been reached
    +- PkiEnrollmentInfoError: all the errors coming from the pki_enrollment_info command
        +- PkiEnrollmentInfoNotFoundError: when the provided enrollment could not be found
"""


class PkiEnrollmentError(Exception):
    pass


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


class PkiEnrollmentCertificatePinCodeUnavailableError(PkiEnrollmentCertificateError):
    pass


# Payload errors


class PkiEnrollmentPayloadError(PkiEnrollmentError):
    pass


class PkiEnrollmentPayloadValidationError(PkiEnrollmentPayloadError):
    pass


# Enrollment submit errors


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


class PkiEnrollmentSubmitCertificateEmailAlreadyUsedError(PkiEnrollmentSubmitError):
    pass


# Enrollment list errors


class PkiEnrollmentListError(PkiEnrollmentRemoteError):
    pass


class PkiEnrollmentListNotAllowedError(PkiEnrollmentListError):
    pass


# Enrollment reject errors


class PkiEnrollmentRejectError(PkiEnrollmentRemoteError):
    pass


class PkiEnrollmentRejectNotAllowedError(PkiEnrollmentRejectError):
    pass


class PkiEnrollmentRejectNotFoundError(PkiEnrollmentRejectError):
    pass


class PkiEnrollmentRejectNoLongerAvailableError(PkiEnrollmentRejectError):
    pass


# Enrollment accept errors


class PkiEnrollmentAcceptError(PkiEnrollmentRemoteError):
    pass


class PkiEnrollmentAcceptNotAllowedError(PkiEnrollmentAcceptError):
    pass


class PkiEnrollmentAcceptInvalidPayloadDataError(PkiEnrollmentAcceptError):
    pass


class PkiEnrollmentAcceptInvalidDataError(PkiEnrollmentAcceptError):
    pass


class PkiEnrollmentAcceptInvalidCertificationError(PkiEnrollmentAcceptError):
    pass


class PkiEnrollmentAcceptNotFoundError(PkiEnrollmentAcceptError):
    pass


class PkiEnrollmentAcceptNoLongerAvailableError(PkiEnrollmentAcceptError):
    pass


class PkiEnrollmentAcceptAlreadyExistsError(PkiEnrollmentAcceptError):
    pass


class PkiEnrollmentAcceptActiveUsersLimitReachedError(PkiEnrollmentAcceptError):
    pass


# Enrollment info errors


class PkiEnrollmentInfoError(PkiEnrollmentRemoteError):
    pass


class PkiEnrollmentInfoNotFoundError(PkiEnrollmentInfoError):
    pass
