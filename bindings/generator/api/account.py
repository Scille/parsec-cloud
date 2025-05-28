# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .common import (
    HumanHandle,
    Password,
    Ref,
    Path,
    Result,
    EmailAddress,
    ErrorVariant,
    Variant,
    ValidationCode,
)
from .addr import ParsecAddr


class AccountSendEmailValidationTokenError(ErrorVariant):
    class Stopped:
        pass

    class Offline:
        pass

    class Internal:
        pass

    class EmailRecipientRefused:
        pass

    class EmailServerUnavailable:
        pass

    class EmailParseError:
        pass


class AccountCreateProceedError(ErrorVariant):
    class Stopped:
        pass

    class Offline:
        pass

    class Internal:
        pass

    class InvalidValidationCode:
        pass

    class AuthMethodIdAlreadyExists:
        pass

    class CryptoError:
        pass


async def account_create_send_validation_email(
    email: Ref[str],
    config_dir: Ref[Path],
    addr: ParsecAddr,
) -> Result[None, AccountSendEmailValidationTokenError]:
    raise NotImplementedError


class AccountCreateStep(Variant):
    class CheckCode:
        validation_code: ValidationCode
        email: EmailAddress

    class Create:
        human_handle: HumanHandle
        password: Password
        validation_code: ValidationCode


async def account_create_proceed(
    step: AccountCreateStep,
    config_dir: Ref[Path],
    addr: ParsecAddr,
) -> Result[None, AccountCreateProceedError]:
    raise NotImplementedError
