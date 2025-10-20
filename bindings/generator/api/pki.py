# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
    Bytes,
    ErrorVariant,
    Result,
    StrBasedType,
    Structure,
)


class X509CertificateHash(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { <libparsec::X509CertificateHash as std::str::FromStr>::from_str(s.as_str()).map_err(|e| e.to_string()) }"
    custom_to_rs_string = (
        "|x: libparsec::X509CertificateHash| -> Result<_, &'static str> { Ok(x.to_string()) }"
    )


class X509CertificateReference(Structure):
    uri: Optional[Bytes]
    hash: X509CertificateHash


class ShowCertificateSelectionDialogError(ErrorVariant):
    class CannotOpenStore: ...

    class CannotGetCertificateInfo: ...


async def show_certificate_selection_dialog_windows_only() -> Result[
    Optional[X509CertificateReference], ShowCertificateSelectionDialogError
]:
    raise NotImplementedError
