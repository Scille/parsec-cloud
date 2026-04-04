# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .addr import ParsecAddr
from .common import (
    ErrorVariant,
    Handle,
    Path,
    Ref,
    Result,
    X509CertificateReference,
)


class ShowCertificateSelectionDialogError(ErrorVariant):
    class CannotOpenStore: ...

    class CannotGetCertificateInfo: ...


async def show_certificate_selection_dialog_windows_only() -> Result[
    X509CertificateReference | None, ShowCertificateSelectionDialogError
]:
    raise NotImplementedError


class PkiSystemInitError(ErrorVariant):
    class NotAvailable: ...

    class Internal: ...


async def pki_init(
    config_dir: Ref[Path],
) -> Result[None, PkiSystemInitError]:
    raise NotImplementedError


async def pki_init_for_scws(
    config_dir: Ref[Path],
    parsec_addr: Ref[ParsecAddr],
) -> Result[None, PkiSystemInitError]:
    raise NotImplementedError


class PkiSystemFindCertificateError(ErrorVariant):
    class Internal: ...


async def pki_open_certificate(
    cert_ref: Ref[X509CertificateReference],
) -> Result[Handle | None, PkiSystemFindCertificateError]:
    raise NotImplementedError


class PkiCertificateCloseError(ErrorVariant):
    class Internal: ...


def pki_certificate_close(
    handle: Handle,
) -> Result[None, PkiCertificateCloseError]:
    raise NotImplementedError


class PkiCertificateRequestPrivateKeyError(ErrorVariant):
    class NotFound: ...

    class Internal: ...


async def pki_certificate_open_private_key(
    handle: Handle,
) -> Result[Handle, PkiCertificateRequestPrivateKeyError]:
    raise NotImplementedError


class PkiPrivateKeyCloseError(ErrorVariant):
    class Internal: ...


async def pki_private_key_close(
    handle: Handle,
) -> Result[None, PkiPrivateKeyCloseError]:
    raise NotImplementedError
