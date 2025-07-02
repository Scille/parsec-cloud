# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .common import (
    HumanHandle,
    Password,
    Ref,
    Path,
    Result,
    EmailAddress,
    ErrorVariant,
    ValidationCode,
    Handle,
    OrganizationID,
    UserID,
    DeviceLabel,
    InvitationToken,
    InvitationType,
    KeyDerivation,
)
from .addr import ParsecAddr
from .device import DeviceAccessStrategy, DeviceSaveStrategy, AvailableDevice


class AccountCreateSendValidationEmailError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass

    class EmailRecipientRefused:
        pass

    class EmailServerUnavailable:
        pass


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

    class AuthMethodIdAlreadyExists:
        pass


async def account_create_2_check_validation_code(
    config_dir: Ref[Path],
    addr: ParsecAddr,
    validation_code: ValidationCode,
    email: EmailAddress,
) -> Result[None, AccountCreateError]:
    raise NotImplementedError


async def account_create_3_proceed(
    config_dir: Ref[Path],
    addr: ParsecAddr,
    validation_code: ValidationCode,
    human_handle: HumanHandle,
    password: Ref[Password],
) -> Result[None, AccountCreateError]:
    raise NotImplementedError


class AccountLoginWithPasswordError(ErrorVariant):
    class BadPasswordAlgorithm:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def account_login_with_password(
    config_dir: Path,
    addr: ParsecAddr,
    email: EmailAddress,
    password: Ref[Password],
) -> Result[Handle, AccountLoginWithPasswordError]:
    raise NotImplementedError


class AccountLoginWithMasterSecretError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def account_login_with_master_secret(
    config_dir: Path,
    addr: ParsecAddr,
    auth_method_master_secret: KeyDerivation,
) -> Result[Handle, AccountLoginWithMasterSecretError]:
    raise NotImplementedError


class AccountLogoutError(ErrorVariant):
    class Internal:
        pass


def account_logout(account: Handle) -> Result[None, AccountLogoutError]:
    raise NotImplementedError


class AccountGetHumanHandleError(ErrorVariant):
    class Internal:
        pass


def account_get_human_handle(account: Handle) -> Result[HumanHandle, AccountGetHumanHandleError]:
    raise NotImplementedError


class AccountListInvitationsError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def account_list_invitations(
    account: Handle,
) -> Result[
    list[tuple[OrganizationID, InvitationToken, InvitationType]], AccountListInvitationsError
]:
    raise NotImplementedError


class AccountFetchRegistrationDevicesError(ErrorVariant):
    class BadVaultKeyAccess:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def account_fetch_registration_devices(
    account: Handle,
) -> Result[None, AccountFetchRegistrationDevicesError]:
    raise NotImplementedError


class AccountListRegistrationDevicesError(ErrorVariant):
    class Internal:
        pass


def account_list_registration_devices(
    account: Handle,
) -> Result[set[tuple[OrganizationID, UserID]], AccountListRegistrationDevicesError]:
    raise NotImplementedError


class AccountRegisterNewDeviceError(ErrorVariant):
    class UnknownRegistrationDevice:
        pass

    class Offline:
        pass

    class Internal:
        pass

    class StorageNotAvailable:
        pass

    class InvalidPath:
        pass

    class TimestampOutOfBallpark:
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
    validation_code: ValidationCode,
) -> Result[None, AccountDeleteProceedError]:
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


async def account_create_registration_device(
    account: Handle,
    existing_local_device_access: Ref[DeviceAccessStrategy],
) -> Result[None, AccountCreateRegistrationDeviceError]:
    raise NotImplementedError
