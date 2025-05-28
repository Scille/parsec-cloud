# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .common import (
    Password,
    Ref,
    Path,
    Result,
    EmailAddress,
    ErrorVariant,
    Variant,
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

    class InvalidEmail:
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

    class InvalidValidationToken:
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
        code: str

    class Create:
        human_label: str
        password: Password


async def account_create_proceed(
    step: AccountCreateStep,
    email: EmailAddress,
    config_dir: Ref[Path],
    addr: ParsecAddr,
) -> Result[None, AccountCreateProceedError]:
    raise NotImplementedError
