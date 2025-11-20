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
    Path,
    PKIEnrollmentID,
    Ref,
    Result,
    StrBasedType,
    Structure,
    UserID,
    UserProfile,
    Variant,
    X509CertificateReference,
)
from .config import ClientConfig
from .device import AvailableDevice, DeviceSaveStrategy, RemoveDeviceError


class PkiSignatureAlgorithm(StrBasedType):
    custom_to_rs_string = DISPLAY_TO_STRING


class ShowCertificateSelectionDialogError(ErrorVariant):
    class CannotOpenStore: ...

    class CannotGetCertificateInfo: ...


async def show_certificate_selection_dialog_windows_only() -> Result[
    X509CertificateReference | None, ShowCertificateSelectionDialogError
]:
    raise NotImplementedError


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


class InvalidityReason(Variant):
    class InvalidCertificateDer:
        pass

    class InvalidRootCertificate:
        pass

    class CannotOpenStore:
        pass

    class NotFound:
        pass

    class CannotGetCertificateInfo:
        pass

    class DateTimeOutOfRange:
        pass

    class Untrusted:
        pass

    class InvalidSignature:
        pass

    class UnexpectedError:
        pass

    class DataError:
        pass


class PkiEnrollmentListItem(Variant):
    class Valid:
        enrollment_id: PKIEnrollmentID
        submitted_on: DateTime
        payload: PkiEnrollmentSubmitPayload

    class Invalid:
        enrollment_id: PKIEnrollmentID
        submitted_on: DateTime
        reason: InvalidityReason
        details: str


class PkiEnrollmentListError(ErrorVariant):
    class Offline: ...

    class AuthorNotAllowed: ...

    class Internal: ...


class PkiGetAddrError(ErrorVariant):
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


class ListPkiLocalPendingError(ErrorVariant):
    class StorageNotAvailable:
        pass

    class Internal:
        pass


async def list_pki_local_pending_enrollments(
    config_dir: Ref[Path],
) -> Result[list[PKILocalPendingEnrollment], ListPkiLocalPendingError]:
    raise NotImplementedError


class PKIInfoItem(Variant):
    class Accepted:
        answer: PkiEnrollmentAnswerPayload
        submitted_on: DateTime
        accepted_on: DateTime

    class Rejected:
        submitted_on: DateTime
        rejected_on: DateTime

    class Cancelled:
        submitted_on: DateTime
        cancelled_on: DateTime

    class Submitted:
        submitted_on: DateTime


class PkiEnrollmentInfoError(ErrorVariant):
    class Offline:
        pass

    class EnrollmentNotFound:
        pass

    class InvalidAcceptPayload:
        pass

    class Internal:
        pass


async def pki_enrollment_info(
    config: ClientConfig, addr: ParsecPkiEnrollmentAddr, enrollment_id: PKIEnrollmentID
) -> Result[PKIInfoItem, PkiEnrollmentInfoError]:
    raise NotImplementedError


async def pki_remove_local_pending(
    config: ClientConfig, id: PKIEnrollmentID
) -> Result[None, RemoveDeviceError]:
    raise NotImplementedError
