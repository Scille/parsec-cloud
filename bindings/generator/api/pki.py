# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .addr import ParsecPkiEnrollmentAddr
from .common import (
    DISPLAY_TO_STRING,
    Bytes,
    BytesBasedType,
    DateTime,
    DeviceID,
    DeviceLabel,
    ErrorVariant,
    HumanHandle,
    PKIEnrollmentID,
    Result,
    StrBasedType,
    Structure,
    UserID,
    UserProfile,
    X509CertificateReference,
)
from .config import ClientConfig
from .device import AvailableDevice, DeviceSaveStrategy


class PkiSignatureAlgorithm(StrBasedType):
    custom_to_rs_string = DISPLAY_TO_STRING


class ShowCertificateSelectionDialogError(ErrorVariant):
    class CannotOpenStore: ...

    class CannotGetCertificateInfo: ...


async def show_certificate_selection_dialog_windows_only() -> Result[
    X509CertificateReference | None, ShowCertificateSelectionDialogError
]:
    raise NotImplementedError


class PkiEnrollmentListItem(Structure):
    enrollment_id: PKIEnrollmentID
    submitted_on: DateTime
    der_x509_certificate: Bytes
    payload_signature: Bytes
    payload_signature_algorithm: PkiSignatureAlgorithm
    payload: Bytes


class PkiEnrollmentListError(ErrorVariant):
    class Offline: ...

    class AuthorNotAllowed: ...

    class Internal: ...


async def is_pki_available() -> bool:
    raise NotImplementedError


class PkiEnrollmentRejectError(ErrorVariant):
    class Offline: ...

    class Internal: ...

    class AuthorNotAllowed: ...

    class EnrollmentNoLongerAvailable: ...

    class EnrollmentNotFound: ...


class PkiEnrollmentSubmitError(ErrorVariant):
    class Offline: ...

    class Internal: ...

    class AlreadyEnrolled: ...

    class AlreadySubmitted: ...

    class EmailAlreadyUsed: ...

    class IdAlreadyUsed: ...

    class InvalidPayload: ...

    class PkiOperationError: ...


async def pki_enrollment_submit(
    config: ClientConfig,
    addr: ParsecPkiEnrollmentAddr,
    cert_ref: X509CertificateReference,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    force: bool,
) -> Result[DateTime, PkiEnrollmentSubmitError]:
    raise NotImplementedError


class PkiEnrollmentAcceptError(ErrorVariant):
    class Offline: ...

    class Internal: ...

    class AuthorNotAllowed: ...

    class EnrollmentNoLongerAvailable: ...

    class EnrollmentNotFound: ...

    class ActiveUsersLimitReached: ...

    class HumanHandleAlreadyTaken: ...

    class PkiOperationError: ...


class VerifyKey(BytesBasedType): ...


class PublicKey(BytesBasedType): ...


class PkiEnrollmentAnswerPayload(Structure):
    user_id: UserID
    device_id: DeviceID
    device_label: DeviceLabel
    human_handle: HumanHandle
    profile: UserProfile
    root_verify_key: VerifyKey


class PkiEnrollmentSubmitPayload(Structure):
    verify_key: VerifyKey
    public_key: PublicKey
    device_label: DeviceLabel
    human_handle: HumanHandle


class PKIEncryptionAlgorithm(StrBasedType):
    custom_to_rs_string = DISPLAY_TO_STRING


class PKILocalPendingEnrollment(Structure):
    cert_ref: X509CertificateReference
    addr: ParsecPkiEnrollmentAddr
    submitted_on: DateTime
    enrollment_id: PKIEnrollmentID
    payload: PkiEnrollmentSubmitPayload
    encrypted_key: Bytes
    encrypted_key_algo: PKIEncryptionAlgorithm
    ciphertext: Bytes


class PkiEnrollmentFinalizeError(ErrorVariant):
    class SaveError: ...

    class Internal: ...


async def pki_enrollment_finalize(
    config: ClientConfig,
    save_strategy: DeviceSaveStrategy,
    accepted: PkiEnrollmentAnswerPayload,
    local_pending: PKILocalPendingEnrollment,
) -> Result[AvailableDevice, PkiEnrollmentFinalizeError]:
    raise NotImplementedError
