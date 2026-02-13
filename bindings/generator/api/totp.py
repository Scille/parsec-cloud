# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .addr import ParsecAddr, ParsecTOTPResetAddr
from .client import ClientConfig
from .common import (
    ErrorVariant,
    Handle,
    OrganizationID,
    Result,
    SecretKey,
    TOTPOpaqueKeyID,
    UserID,
    Variant,
    VariantItemUnit,
)


class TOTPSetupStatus(Variant):
    class Stalled:
        base32_totp_secret: str

    AlreadySetup = VariantItemUnit()


class ClientTotpSetupStatusError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def client_totp_setup_status(
    client: Handle,
) -> Result[TOTPSetupStatus, ClientTotpSetupStatusError]:
    raise NotImplementedError


class TotpSetupStatusAnonymousError(ErrorVariant):
    class BadToken:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def totp_setup_status_anonymous(
    config: ClientConfig,
    addr: ParsecTOTPResetAddr,
) -> Result[TOTPSetupStatus, TotpSetupStatusAnonymousError]:
    raise NotImplementedError


class ClientTOTPSetupConfirmError(ErrorVariant):
    class InvalidOneTimePassword:
        pass

    class AlreadySetup:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def client_totp_setup_confirm(
    client: Handle,
    one_time_password: str,
) -> Result[None, ClientTOTPSetupConfirmError]:
    raise NotImplementedError


class TotpSetupConfirmAnonymousError(ErrorVariant):
    class InvalidOneTimePassword:
        pass

    class BadToken:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def totp_setup_confirm_anonymous(
    config: ClientConfig,
    addr: ParsecTOTPResetAddr,
    one_time_password: str,
) -> Result[None, TotpSetupConfirmAnonymousError]:
    raise NotImplementedError


class ClientTotpCreateOpaqueKeyError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def client_totp_create_opaque_key(
    client: Handle,
) -> Result[tuple[TOTPOpaqueKeyID, SecretKey], ClientTotpCreateOpaqueKeyError]:
    raise NotImplementedError


class TotpFetchOpaqueKeyError(ErrorVariant):
    class InvalidOneTimePassword:
        pass

    class Throttled:
        pass

    class Offline:
        pass

    class Internal:
        pass


async def totp_fetch_opaque_key(
    config: ClientConfig,
    server_addr: ParsecAddr,
    organization_id: OrganizationID,
    user_id: UserID,
    opaque_key_id: TOTPOpaqueKeyID,
    one_time_password: str,
) -> Result[SecretKey, TotpFetchOpaqueKeyError]:
    raise NotImplementedError
