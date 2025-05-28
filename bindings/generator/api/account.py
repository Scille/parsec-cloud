# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .common import (
    Ref,
    Path,
    Result,
    EmailValidationToken,
    Password,
    ErrorVariant,
)
from .addr import ParsecAnonymousAccountAddr


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


async def account_send_email_validation_token(
    email: Ref[str],
    config_dir: Ref[Path],
    addr: ParsecAnonymousAccountAddr,
) -> Result[None, AccountSendEmailValidationTokenError]:
    raise NotImplementedError


async def account_create_proceed(
    human_label: str,
    validation_token: EmailValidationToken,
    config_dir: Ref[Path],
    addr: ParsecAnonymousAccountAddr,
    password: Password,
) -> Result[None, AccountCreateProceedError]:
    raise NotImplementedError
