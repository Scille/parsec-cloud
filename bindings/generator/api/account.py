# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .addr import ParsecAddr, ParsecInvitationAddrAndRedirectionURL
from .client import ActiveUsersLimit
from .common import (
    AccessToken,
    AccountAuthMethodID,
    DateTime,
    DeviceLabel,
    EmailAddress,
    ErrorVariant,
    Handle,
    HumanHandle,
    InvitationType,
    KeyDerivation,
    OrganizationID,
    Password,
    Path,
    Ref,
    Result,
    Structure,
    UserID,
    UserProfile,
    Variant,
)
from .device import AvailableDevice, DeviceAccessStrategy, DeviceSaveStrategy


def list_started_accounts() -> list[Handle]:
    raise NotImplementedError


class AccountCreateSendValidationEmailError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass

    class EmailRecipientRefused:
        pass

    class EmailServerUnavailable:
        pass

    class EmailSendingRateLimited:
        wait_until: DateTime


async def account_create_1_send_validation_email(
    config_dir: Ref[Path],
    addr: ParsecAddr,
    email: EmailAddress,
) -> Result[None, AccountCreateSendValidationEmailError]:
    raise NotImplementedError


class AccountCreateError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass

    class InvalidValidationCode:
        pass

    class SendValidationEmailRequired:
        pass


async def account_create_2_check_validation_code(
    config_dir: Ref[Path],
    addr: ParsecAddr,
    validation_code: Ref[str],
    email: EmailAddress,
) -> Result[None, AccountCreateError]:
    raise NotImplementedError


class AccountAuthMethodStrategy(Variant):
    class Password:
        password: Password

    class MasterSecret:
        master_secret: KeyDerivation


async def account_create_3_proceed(
    config_dir: Ref[Path],
    addr: ParsecAddr,
    validation_code: Ref[str],
    human_handle: HumanHandle,
    auth_method_strategy: AccountAuthMethodStrategy,
) -> Result[None, AccountCreateError]:
    raise NotImplementedError


class AccountCreateAuthMethodError(ErrorVariant):
    class BadVaultKeyAccess:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def account_create_auth_method(
    account: Handle,
    auth_method_strategy: AccountAuthMethodStrategy,
) -> Result[None, AccountCreateAuthMethodError]:
    raise NotImplementedError


class AccountInfo(Structure):
    server_addr: ParsecAddr
    in_use_auth_method: AccountAuthMethodID
    human_handle: HumanHandle


class AccountInfoError(ErrorVariant):
    class Internal:
        pass


def account_info(
    account: Handle,
) -> Result[AccountInfo, AccountInfoError]:
    raise NotImplementedError


class AuthMethodInfo(Structure):
    auth_method_id: AccountAuthMethodID
    created_on: DateTime
    created_by_ip: str
    created_by_user_agent: str
    use_password: bool


class AccountListAuthMethodsError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def account_list_auth_methods(
    account: Handle,
) -> Result[list[AuthMethodInfo], AccountListAuthMethodsError]:
    raise NotImplementedError


class AccountDisableAuthMethodError(ErrorVariant):
    class Offline:
        pass

    class AuthMethodNotFound:
        pass

    class AuthMethodAlreadyDisabled:
        pass

    class SelfDisableNotAllowed:
        pass

    class Internal:
        pass


async def account_disable_auth_method(
    account: Handle,
    auth_method_id: AccountAuthMethodID,
) -> Result[None, AccountDisableAuthMethodError]:
    raise NotImplementedError


class AccountLoginStrategy(Variant):
    class Password:
        email: EmailAddress
        password: Password

    class MasterSecret:
        master_secret: KeyDerivation


class AccountLoginError(ErrorVariant):
    class BadPasswordAlgorithm:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def account_login(
    config_dir: Path,
    addr: ParsecAddr,
    login_strategy: AccountLoginStrategy,
) -> Result[Handle, AccountLoginError]:
    raise NotImplementedError


class AccountLogoutError(ErrorVariant):
    class Internal:
        pass


def account_logout(account: Handle) -> Result[None, AccountLogoutError]:
    raise NotImplementedError


class AccountListInvitationsError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def account_list_invitations(
    account: Handle,
) -> Result[
    list[
        tuple[
            ParsecInvitationAddrAndRedirectionURL,
            OrganizationID,
            AccessToken,
            InvitationType,
        ]
    ],
    AccountListInvitationsError,
]:
    raise NotImplementedError


class AccountListOrganizationsError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


class AccountOrganizationsOrganizationConfig(Structure):
    is_expired: bool
    user_profile_outsider_allowed: bool
    active_users_limit: ActiveUsersLimit


class AccountOrganizationsActiveUser(Structure):
    organization_id: OrganizationID
    user_id: UserID
    created_on: DateTime
    is_frozen: bool
    current_profile: UserProfile
    organization_config: AccountOrganizationsOrganizationConfig


class AccountOrganizationsRevokedUser(Structure):
    organization_id: OrganizationID
    user_id: UserID
    created_on: DateTime
    revoked_on: DateTime
    current_profile: UserProfile


class AccountOrganizations(Structure):
    active: list[AccountOrganizationsActiveUser]
    revoked: list[AccountOrganizationsRevokedUser]


async def account_list_organizations(
    account: Handle,
) -> Result[
    AccountOrganizations,
    AccountListOrganizationsError,
]:
    raise NotImplementedError


class AccountListRegistrationDevicesError(ErrorVariant):
    class BadVaultKeyAccess:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def account_list_registration_devices(
    account: Handle,
) -> Result[set[tuple[OrganizationID, UserID]], AccountListRegistrationDevicesError]:
    raise NotImplementedError


class AccountCreateRegistrationDeviceError(ErrorVariant):
    class LoadDeviceInvalidPath:
        pass

    class LoadDeviceInvalidData:
        pass

    class LoadDeviceDecryptionFailed:
        pass

    class BadVaultKeyAccess:
        pass

    class Offline:
        pass

    class Internal:
        pass

    class TimestampOutOfBallpark:
        pass

    class RemoteOpaqueKeyFetchOffline:
        pass

    class RemoteOpaqueKeyFetchFailed:
        pass


async def account_create_registration_device(
    account: Handle,
    existing_local_device_access: DeviceAccessStrategy,
) -> Result[None, AccountCreateRegistrationDeviceError]:
    raise NotImplementedError


class AccountRegisterNewDeviceError(ErrorVariant):
    class BadVaultKeyAccess:
        pass

    class UnknownRegistrationDevice:
        pass

    class CorruptedRegistrationDevice:
        pass

    class Offline:
        pass

    class Internal:
        pass

    class NoSpaceAvailable:
        pass

    class InvalidPath:
        pass

    class TimestampOutOfBallpark:
        pass

    class RemoteOpaqueKeyUploadOffline:
        pass

    class RemoteOpaqueKeyUploadFailed:
        pass


async def account_register_new_device(
    account: Handle,
    organization_id: OrganizationID,
    user_id: UserID,
    new_device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result[AvailableDevice, AccountRegisterNewDeviceError]:
    raise NotImplementedError


class AccountDeleteSendValidationEmailError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass

    class EmailRecipientRefused:
        pass

    class EmailServerUnavailable:
        pass

    class EmailSendingRateLimited:
        wait_until: DateTime


async def account_delete_1_send_validation_email(
    account: Handle,
) -> Result[None, AccountDeleteSendValidationEmailError]:
    raise NotImplementedError


class AccountDeleteProceedError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass

    class InvalidValidationCode:
        pass

    class SendValidationEmailRequired:
        pass


async def account_delete_2_proceed(
    account: Handle,
    validation_code: Ref[str],
) -> Result[None, AccountDeleteProceedError]:
    raise NotImplementedError


class AccountRecoverSendValidationEmailError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass

    class EmailRecipientRefused:
        pass

    class EmailServerUnavailable:
        pass

    class EmailSendingRateLimited:
        wait_until: DateTime


async def account_recover_1_send_validation_email(
    config_dir: Ref[Path],
    addr: ParsecAddr,
    email: EmailAddress,
) -> Result[None, AccountRecoverSendValidationEmailError]:
    raise NotImplementedError


class AccountRecoverProceedError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass

    class InvalidValidationCode:
        pass

    class SendValidationEmailRequired:
        pass


async def account_recover_2_proceed(
    config_dir: Ref[Path],
    addr: ParsecAddr,
    validation_code: Ref[str],
    email: EmailAddress,
    auth_method_strategy: AccountAuthMethodStrategy,
) -> Result[None, AccountRecoverProceedError]:
    raise NotImplementedError
