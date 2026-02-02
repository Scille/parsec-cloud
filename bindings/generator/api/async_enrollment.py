# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .addr import ParsecAsyncEnrollmentAddr
from .common import (
    AsyncEnrollmentID,
    DateTime,
    DeviceLabel,
    ErrorVariant,
    Handle,
    HumanHandle,
    Path,
    Ref,
    Result,
    Structure,
    UserProfile,
    Variant,
    VariantItemUnit,
    X509CertificateReference,
)
from .config import ClientConfig
from .device import AvailableDevice, DeviceSaveStrategy


class ClientGetAsyncEnrollmentAddrError(ErrorVariant):
    class Internal:
        pass


def client_get_async_enrollment_addr(
    client: Handle,
) -> Result[ParsecAsyncEnrollmentAddr, ClientGetAsyncEnrollmentAddrError]:
    raise NotImplementedError


class SubmitAsyncEnrollmentIdentityStrategy(Variant):
    class OpenBao:
        requested_human_handle: HumanHandle
        openbao_server_url: str
        openbao_transit_mount_path: str
        openbao_secret_mount_path: str
        openbao_entity_id: str
        openbao_auth_token: str
        openbao_preferred_auth_id: str

    class PKI:
        certificate_reference: X509CertificateReference


class AcceptFinalizeAsyncEnrollmentIdentityStrategy(Variant):
    class OpenBao:
        openbao_server_url: str
        openbao_transit_mount_path: str
        openbao_secret_mount_path: str
        openbao_entity_id: str
        openbao_auth_token: str

    class PKI:
        certificate_reference: X509CertificateReference


class ClientListAsyncEnrollmentsError(ErrorVariant):
    class Offline:
        pass

    class AuthorNotAllowed:
        pass

    class Internal:
        pass


class AsyncEnrollmentIdentitySystem(Variant):
    class PKI:
        x509_root_certificate_common_name: str
        x509_root_certificate_subject: bytes

    class PKICorrupted:
        reason: str

    OpenBao = VariantItemUnit()


class AsyncEnrollmentUntrusted(Structure):
    enrollment_id: AsyncEnrollmentID
    submitted_on: DateTime
    untrusted_requested_device_label: DeviceLabel
    untrusted_requested_human_handle: HumanHandle
    identity_system: AsyncEnrollmentIdentitySystem


async def client_list_async_enrollments(
    client: Handle,
) -> Result[list[AsyncEnrollmentUntrusted], ClientListAsyncEnrollmentsError]:
    raise NotImplementedError


class ClientRejectAsyncEnrollmentError(ErrorVariant):
    class Offline:
        pass

    class AuthorNotAllowed:
        pass

    class EnrollmentNotFound:
        pass

    class EnrollmentNoLongerAvailable:
        pass

    class Internal:
        pass


async def client_reject_async_enrollment(
    client: Handle,
    enrollment_id: AsyncEnrollmentID,
) -> Result[None, ClientRejectAsyncEnrollmentError]:
    raise NotImplementedError


class ClientAcceptAsyncEnrollmentError(ErrorVariant):
    class Offline:
        pass

    class AuthorNotAllowed:
        pass

    class EnrollmentNotFound:
        pass

    class EnrollmentNoLongerAvailable:
        pass

    class BadSubmitPayload:
        pass

    class IdentityStrategyMismatch:
        pass

    class ActiveUsersLimitReached:
        pass

    class HumanHandleAlreadyTaken:
        pass

    class TimestampOutOfBallpark:
        pass

    class Internal:
        pass

    class OpenBaoBadURL:
        pass

    class OpenBaoNoServerResponse:
        pass

    class OpenBaoBadServerResponse:
        pass

    class PKIServerInvalidX509Trustchain:
        pass

    class PKICannotOpenCertificateStore:
        pass

    class PKIUnusableX509CertificateReference:
        pass


async def client_accept_async_enrollment(
    client: Handle,
    profile: UserProfile,
    enrollment_id: AsyncEnrollmentID,
    identity_strategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
) -> Result[None, ClientAcceptAsyncEnrollmentError]:
    raise NotImplementedError


class SubmitAsyncEnrollmentError(ErrorVariant):
    class Offline:
        pass

    class EmailAlreadySubmitted:
        pass

    class EmailAlreadyEnrolled:
        pass

    class StorageNotAvailable:
        pass

    class InvalidPath:
        pass

    class Internal:
        pass

    class OpenBaoBadURL:
        pass

    class OpenBaoNoServerResponse:
        pass

    class OpenBaoBadServerResponse:
        pass

    class PKIServerInvalidX509Trustchain:
        pass

    class PKICannotOpenCertificateStore:
        pass

    class PKIUnusableX509CertificateReference:
        pass


class AvailablePendingAsyncEnrollmentIdentitySystem(Variant):
    class OpenBao:
        openbao_entity_id: str
        openbao_preferred_auth_id: str

    class PKI:
        certificate_ref: X509CertificateReference


class AvailablePendingAsyncEnrollment(Structure):
    file_path: Path
    submitted_on: DateTime
    addr: ParsecAsyncEnrollmentAddr
    enrollment_id: AsyncEnrollmentID
    requested_device_label: DeviceLabel
    requested_human_handle: HumanHandle
    identity_system: AvailablePendingAsyncEnrollmentIdentitySystem


async def submit_async_enrollment(
    config: ClientConfig,
    addr: ParsecAsyncEnrollmentAddr,
    force: bool,
    requested_device_label: DeviceLabel,
    identity_strategy: SubmitAsyncEnrollmentIdentityStrategy,
) -> Result[AvailablePendingAsyncEnrollment, SubmitAsyncEnrollmentError]:
    raise NotImplementedError


class SubmitterListLocalAsyncEnrollmentsError(ErrorVariant):
    class StorageNotAvailable:
        pass

    class Internal:
        pass


async def submitter_list_async_enrollments(
    config_dir: Ref[Path],
) -> Result[list[AvailablePendingAsyncEnrollment], SubmitterListLocalAsyncEnrollmentsError]:
    raise NotImplementedError


class SubmitterGetAsyncEnrollmentInfoError(ErrorVariant):
    class Offline:
        pass

    class EnrollmentNotFound:
        pass

    class Internal:
        pass


class PendingAsyncEnrollmentInfo(Variant):
    class Submitted:
        submitted_on: DateTime

    class Cancelled:
        submitted_on: DateTime
        cancelled_on: DateTime

    class Rejected:
        submitted_on: DateTime
        rejected_on: DateTime

    class Accepted:
        submitted_on: DateTime
        accepted_on: DateTime


async def submitter_get_async_enrollment_info(
    config: ClientConfig,
    addr: ParsecAsyncEnrollmentAddr,
    enrollment_id: AsyncEnrollmentID,
) -> Result[PendingAsyncEnrollmentInfo, SubmitterGetAsyncEnrollmentInfoError]:
    raise NotImplementedError


class SubmitterFinalizeAsyncEnrollmentError(ErrorVariant):
    class Offline:
        pass

    class StorageNotAvailable:
        pass

    class NotAccepted:
        pass

    class BadAcceptPayload:
        pass

    class IdentityStrategyMismatch:
        pass

    class EnrollmentFileInvalidPath:
        pass

    class EnrollmentFileCannotRetrieveCiphertextKey:
        pass

    class EnrollmentFileInvalidData:
        pass

    class EnrollmentNotFoundOnServer:
        pass

    class SaveDeviceInvalidPath:
        pass

    class SaveDeviceRemoteOpaqueKeyUploadOffline:
        pass

    class SaveDeviceRemoteOpaqueKeyUploadFailed:
        pass

    class Internal:
        pass

    class OpenBaoBadURL:
        pass

    class OpenBaoNoServerResponse:
        pass

    class OpenBaoBadServerResponse:
        pass

    class PKICannotOpenCertificateStore:
        pass

    class PKIUnusableX509CertificateReference:
        pass


async def submitter_finalize_async_enrollment(
    config: ClientConfig,
    enrollment_file: Ref[Path],
    new_device_save_strategy: DeviceSaveStrategy,
    identity_strategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
) -> Result[AvailableDevice, SubmitterFinalizeAsyncEnrollmentError]:
    raise NotImplementedError


class SubmitterCancelAsyncEnrollmentError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class StorageNotAvailable:
        pass

    class Internal:
        pass


async def submitter_cancel_async_enrollment(
    config: ClientConfig,
    addr: ParsecAsyncEnrollmentAddr,
    enrollment_id: AsyncEnrollmentID,
) -> Result[None, SubmitterCancelAsyncEnrollmentError]:
    raise NotImplementedError
