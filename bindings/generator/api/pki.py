# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
    Bytes,
    ErrorVariant,
    Result,
    Sha256BoxData,
    Structure,
    Variant,
    VariantItemTuple,
)


class CertificateHash(Variant):
    class SHA256:
        data: Sha256BoxData


class CertificateReferenceIdOrHash(Structure):
    id: Bytes
    hash: CertificateHash


class CertificateReference(Variant):
    Id = VariantItemTuple(Bytes)
    Hash = VariantItemTuple(CertificateHash)
    IdOrHash = VariantItemTuple(CertificateReferenceIdOrHash)


class ShowCertificateSelectionDialogError(ErrorVariant):
    class CannotOpenStore: ...

    class CannotGetCertificateInfo: ...


async def show_certificate_selection_dialog_windows_only() -> Result[
    Optional[CertificateReference], ShowCertificateSelectionDialogError
]:
    raise NotImplementedError
