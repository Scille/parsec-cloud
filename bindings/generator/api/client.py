# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .addr import ParsecOrganizationAddr
from .common import (
    U64,
    Bytes,
    DateTime,
    DeviceID,
    DeviceLabel,
    EntryName,
    Enum,
    EnumItemUnit,
    ErrorVariant,
    Handle,
    HumanHandle,
    NonZeroU8,
    OrganizationID,
    Path,
    PKIEnrollmentID,
    RealmRole,
    Ref,
    Result,
    SizeInt,
    Structure,
    UserID,
    UserProfile,
    Variant,
    VariantItemTuple,
    VariantItemUnit,
    VlobID,
)
from .config import ClientConfig
from .device import DeviceAccessStrategy
from .invite import AvailableDevice, DeviceSaveStrategy
from .pki import (
    PkiEnrollmentAcceptError,
    PkiEnrollmentRejectError,
    X509CertificateReference,
    PkiEnrollmentListItem,
    PkiEnrollmentListError,
)


def list_started_clients() -> list[tuple[Handle, DeviceID]]:
    raise NotImplementedError


class WaitForDeviceAvailableError(ErrorVariant):
    class Internal:
        pass


async def wait_for_device_available(
    config_dir: Ref[Path],
    device_id: DeviceID,
) -> Result[None, WaitForDeviceAvailableError]:
    raise NotImplementedError


class ClientStartError(ErrorVariant):
    class DeviceUsedByAnotherProcess:
        pass

    class LoadDeviceInvalidPath:
        pass

    class LoadDeviceInvalidData:
        pass

    class LoadDeviceDecryptionFailed:
        pass

    class LoadDeviceRemoteOpaqueKeyFetchOffline:
        pass

    class LoadDeviceRemoteOpaqueKeyFetchFailed:
        pass

    class Internal:
        pass


async def client_start(
    config: ClientConfig,
    access: DeviceAccessStrategy,
) -> Result[Handle, ClientStartError]:
    raise NotImplementedError


class ClientStopError(ErrorVariant):
    class Internal:
        pass


async def client_stop(client: Handle) -> Result[None, ClientStopError]:
    raise NotImplementedError


class ClientInfoError(ErrorVariant):
    class Stopped:
        pass

    class Internal:
        pass


class ActiveUsersLimit(Variant):
    LimitedTo = VariantItemTuple(U64)
    NoLimit = VariantItemUnit


class ServerConfig(Structure):
    user_profile_outsider_allowed: bool
    active_users_limit: ActiveUsersLimit


class ClientInfo(Structure):
    organization_addr: ParsecOrganizationAddr
    organization_id: OrganizationID
    device_id: DeviceID
    user_id: UserID
    device_label: DeviceLabel
    human_handle: HumanHandle
    current_profile: UserProfile
    server_config: ServerConfig
    is_server_online: bool
    is_organization_expired: bool
    must_accept_tos: bool


async def client_info(
    client: Handle,
) -> Result[ClientInfo, ClientInfoError]:
    raise NotImplementedError


class Tos(Structure):
    per_locale_urls: dict[str, str]
    updated_on: DateTime


class ClientGetTosError(ErrorVariant):
    class NoTos:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def client_get_tos(
    client: Handle,
) -> Result[Tos, ClientGetTosError]:
    raise NotImplementedError


class ClientAcceptTosError(ErrorVariant):
    class NoTos:
        pass

    class TosMismatch:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def client_accept_tos(
    client: Handle,
    tos_updated_on: DateTime,
) -> Result[None, ClientAcceptTosError]:
    raise NotImplementedError


class UserInfo(Structure):
    id: UserID
    human_handle: HumanHandle
    current_profile: UserProfile
    created_on: DateTime
    created_by: DeviceID | None
    revoked_on: DateTime | None
    revoked_by: DeviceID | None


class DevicePurpose(Enum):
    Standard = EnumItemUnit
    ShamirRecovery = EnumItemUnit
    PassphraseRecovery = EnumItemUnit
    Registration = EnumItemUnit


class DeviceInfo(Structure):
    id: DeviceID
    purpose: DevicePurpose
    device_label: DeviceLabel
    created_on: DateTime
    created_by: DeviceID | None


class ClientRevokeUserError(ErrorVariant):
    class Stopped:
        pass

    class Offline:
        pass

    class UserIsSelf:
        pass

    class UserNotFound:
        pass

    class AuthorNotAllowed:
        pass

    class TimestampOutOfBallpark:
        pass

    class NoKey:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class Internal:
        pass


async def client_revoke_user(
    client: Handle,
    user: UserID,
) -> Result[None, ClientRevokeUserError]:
    raise NotImplementedError


class ClientGetUserInfoError(ErrorVariant):
    class Stopped:
        pass

    class NonExisting:
        pass

    class Internal:
        pass


async def client_get_user_info(
    client: Handle,
    user_id: UserID,
) -> Result[UserInfo, ClientGetUserInfoError]:
    raise NotImplementedError


class ClientListUsersError(ErrorVariant):
    class Stopped:
        pass

    class Internal:
        pass


async def client_list_users(
    client: Handle,
    skip_revoked: bool,
    # offset: Optional[int],
    # limit: Optional[int],
) -> Result[list[UserInfo], ClientListUsersError]:
    raise NotImplementedError


class ClientListUserDevicesError(ErrorVariant):
    class Stopped:
        pass

    class Internal:
        pass


async def client_list_user_devices(
    client: Handle,
    user: UserID,
) -> Result[list[DeviceInfo], ClientListUserDevicesError]:
    raise NotImplementedError


class ClientGetUserDeviceError(ErrorVariant):
    class Stopped:
        pass

    class NonExisting:
        pass

    class Internal:
        pass


async def client_get_user_device(
    client: Handle,
    device: DeviceID,
) -> Result[tuple[UserInfo, DeviceInfo], ClientGetUserDeviceError]:
    raise NotImplementedError


class WorkspaceUserAccessInfo(Structure):
    user_id: UserID
    human_handle: HumanHandle
    current_profile: UserProfile
    current_role: RealmRole


class ClientListWorkspaceUsersError(ErrorVariant):
    class Stopped:
        pass

    class Internal:
        pass


async def client_list_workspace_users(
    client: Handle,
    realm_id: VlobID,
) -> Result[list[WorkspaceUserAccessInfo], ClientListWorkspaceUsersError]:
    raise NotImplementedError


class ClientListWorkspacesError(ErrorVariant):
    class Internal:
        pass


class WorkspaceInfo(Structure):
    id: VlobID
    current_name: EntryName
    current_self_role: RealmRole
    is_started: bool
    is_bootstrapped: bool


async def client_list_workspaces(
    client: Handle,
) -> Result[list[WorkspaceInfo], ClientListWorkspacesError]:
    raise NotImplementedError


class ClientCreateWorkspaceError(ErrorVariant):
    class Stopped:
        pass

    class Internal:
        pass


async def client_create_workspace(
    client: Handle,
    name: EntryName,
) -> Result[VlobID, ClientCreateWorkspaceError]:
    raise NotImplementedError


class ClientRenameWorkspaceError(ErrorVariant):
    class WorkspaceNotFound:
        pass

    class AuthorNotAllowed:
        pass

    class Offline:
        pass

    class Stopped:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class NoKey:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidEncryptedRealmName:
        pass

    class Internal:
        pass


async def client_rename_workspace(
    client: Handle,
    realm_id: VlobID,
    new_name: EntryName,
) -> Result[None, ClientRenameWorkspaceError]:
    raise NotImplementedError


class ClientShareWorkspaceError(ErrorVariant):
    class Stopped:
        pass

    class RecipientIsSelf:
        pass

    class RecipientNotFound:
        pass

    class WorkspaceNotFound:
        pass

    class RecipientRevoked:
        pass

    class AuthorNotAllowed:
        pass

    class RoleIncompatibleWithOutsider:
        pass

    class Offline:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class Internal:
        pass


async def client_share_workspace(
    client: Handle,
    realm_id: VlobID,
    recipient: UserID,
    role: RealmRole | None,
) -> Result[None, ClientShareWorkspaceError]:
    raise NotImplementedError


class ClientForgetAllCertificatesError(ErrorVariant):
    class Stopped:
        pass

    class Internal:
        pass


async def client_forget_all_certificates(
    client: Handle,
) -> Result[None, ClientForgetAllCertificatesError]:
    raise NotImplementedError


def is_keyring_available() -> bool:
    raise NotImplementedError


class ImportRecoveryDeviceError(ErrorVariant):
    class StorageNotAvailable:
        pass

    class Internal:
        pass

    class Stopped:
        pass

    class Offline:
        pass

    class InvalidCertificate:
        pass

    class InvalidPath:
        pass

    class InvalidData:
        pass

    class InvalidPassphrase:
        pass

    class DecryptionFailed:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class RemoteOpaqueKeyUploadOffline:
        pass

    class RemoteOpaqueKeyUploadFailed:
        pass


class ClientExportRecoveryDeviceError(ErrorVariant):
    class Internal:
        pass

    class Stopped:
        pass

    class Offline:
        pass

    class InvalidCertificate:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float


async def import_recovery_device(
    config: ClientConfig,
    recovery_device: Ref[bytes],
    passphrase: str,
    device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result[AvailableDevice, ImportRecoveryDeviceError]:
    raise NotImplementedError


async def client_export_recovery_device(
    client_handle: Handle,
    device_label: DeviceLabel,
) -> Result[tuple[str, bytes], ClientExportRecoveryDeviceError]:
    raise NotImplementedError


class ClientListFrozenUsersError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass

    class AuthorNotAllowed:
        pass


async def client_list_frozen_users(
    client_handle: Handle,
) -> Result[list[UserID], ClientListFrozenUsersError]:
    raise NotImplementedError


class ClientSetupShamirRecoveryError(ErrorVariant):
    class Internal:
        pass

    class Stopped:
        pass

    class Offline:
        pass

    class InvalidCertificate:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class ThresholdBiggerThanSumOfShares:
        pass

    class TooManyShares:
        pass

    class AuthorAmongRecipients:
        pass

    class RecipientNotFound:
        pass

    class RecipientRevoked:
        pass

    class ShamirRecoveryAlreadyExists:
        pass


async def client_setup_shamir_recovery(
    client_handle: Handle,
    per_recipient_shares: dict[UserID, NonZeroU8],
    threshold: NonZeroU8,
) -> Result[None, ClientSetupShamirRecoveryError]:
    raise NotImplementedError


class ClientDeleteShamirRecoveryError(ErrorVariant):
    class Internal:
        pass

    class Stopped:
        pass

    class Offline:
        pass

    class InvalidCertificate:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float


async def client_delete_shamir_recovery(
    client_handle: Handle,
) -> Result[None, ClientDeleteShamirRecoveryError]:
    raise NotImplementedError


class SelfShamirRecoveryInfo(Variant):
    class NeverSetup:
        pass

    class Deleted:
        created_on: DateTime
        created_by: DeviceID
        threshold: NonZeroU8
        per_recipient_shares: dict[UserID, NonZeroU8]
        deleted_on: DateTime
        deleted_by: DeviceID

    class SetupAllValid:
        created_on: DateTime
        created_by: DeviceID
        threshold: NonZeroU8
        per_recipient_shares: dict[UserID, NonZeroU8]

    class SetupWithRevokedRecipients:
        created_on: DateTime
        created_by: DeviceID
        threshold: NonZeroU8
        per_recipient_shares: dict[UserID, NonZeroU8]
        revoked_recipients: set[UserID]

    class SetupButUnusable:
        created_on: DateTime
        created_by: DeviceID
        threshold: NonZeroU8
        per_recipient_shares: dict[UserID, NonZeroU8]
        revoked_recipients: set[UserID]


class ClientGetSelfShamirRecoveryError(ErrorVariant):
    class Internal:
        pass

    class Stopped:
        pass


async def client_get_self_shamir_recovery(
    client_handle: Handle,
) -> Result[SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError]:
    raise NotImplementedError


class OtherShamirRecoveryInfo(Variant):
    class Deleted:
        user_id: UserID
        created_on: DateTime
        created_by: DeviceID
        threshold: NonZeroU8
        per_recipient_shares: dict[UserID, NonZeroU8]
        deleted_on: DateTime
        deleted_by: DeviceID

    class SetupAllValid:
        user_id: UserID
        created_on: DateTime
        created_by: DeviceID
        threshold: NonZeroU8
        per_recipient_shares: dict[UserID, NonZeroU8]

    class SetupWithRevokedRecipients:
        user_id: UserID
        created_on: DateTime
        created_by: DeviceID
        threshold: NonZeroU8
        per_recipient_shares: dict[UserID, NonZeroU8]
        revoked_recipients: set[UserID]

    class SetupButUnusable:
        user_id: UserID
        created_on: DateTime
        created_by: DeviceID
        threshold: NonZeroU8
        per_recipient_shares: dict[UserID, NonZeroU8]
        revoked_recipients: set[UserID]


class ClientListShamirRecoveriesForOthersError(ErrorVariant):
    class Internal:
        pass

    class Stopped:
        pass


async def client_list_shamir_recoveries_for_others(
    client_handle: Handle,
) -> Result[list[OtherShamirRecoveryInfo], ClientListShamirRecoveriesForOthersError]:
    raise NotImplementedError


class ClientUserUpdateProfileError(ErrorVariant):
    class Stopped:
        pass

    class Offline:
        pass

    class UserNotFound:
        pass

    class UserIsSelf:
        pass

    class AuthorNotAllowed:
        pass

    class UserRevoked:
        pass

    class TimestampOutOfBallpark:
        pass

    class InvalidCertificate:
        pass

    class Internal:
        pass


async def client_update_user_profile(
    client_handle: Handle,
    user: UserID,
    new_profile: UserProfile,
) -> Result[None, ClientUserUpdateProfileError]:
    raise NotImplementedError


class OrganizationInfo(Structure):
    total_block_bytes: SizeInt
    total_metadata_bytes: SizeInt


class ClientOrganizationInfoError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def client_organization_info(
    client_handle: Handle,
) -> Result[OrganizationInfo, ClientOrganizationInfoError]:
    raise NotImplementedError


class ClientGetOrganizationBootstrapDateError(ErrorVariant):
    class Internal:
        pass

    class Stopped:
        pass

    class BootstrapDateNotFound:
        pass

    class Offline:
        pass

    class InvalidCertificate:
        pass


async def client_get_organization_bootstrap_date(
    client_handle: Handle,
) -> Result[DateTime, ClientGetOrganizationBootstrapDateError]:
    raise NotImplementedError


async def client_pki_enrollment_reject(
    client_handle: Handle, enrollment_id: PKIEnrollmentID
) -> Result[None, PkiEnrollmentRejectError]:
    raise NotImplementedError


async def client_pki_enrollment_accept(
    client_handle: Handle,
    profile: UserProfile,
    enrollment_id: PKIEnrollmentID,
    human_handle: Ref[HumanHandle],
    cert_ref: Ref[X509CertificateReference],
    submit_payload: Bytes,
) -> Result[None, PkiEnrollmentAcceptError]:
    raise NotImplementedError


async def client_pki_list_enrollments(
    client_handle: Handle,
) -> Result[list[PkiEnrollmentListItem], PkiEnrollmentListError]:
    raise NotImplementedError
