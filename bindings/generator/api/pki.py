# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import (
    ErrorVariant,
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


async def is_pki_available() -> bool:
    raise NotImplementedError
