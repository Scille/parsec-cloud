# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import (
    Bytes,
    DateTime,
    EmailAddress,
    ErrorVariant,
    Result,
    Structure,
    Variant,
    VariantItemTuple,
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


class DistinguishedNameValue(Variant):
    CommonName = VariantItemTuple(str)
    EmailAddress = VariantItemTuple(str)


class CertificateDetails(Structure):
    name: str | None
    subject: list[DistinguishedNameValue]
    issuer: list[DistinguishedNameValue]
    not_before: DateTime
    not_after: DateTime
    serial: Bytes
    emails: list[EmailAddress]
    can_sign: bool
    can_encrypt: bool


class InvalidCertificateReason(Variant):
    class UnableToParseTime:
        pass

    class UnableToParseCert:
        pass

    class InvalidEmail:
        pass


class CertificateWithDetails(Variant):
    class Valid:
        handle: X509CertificateReference
        friendly_name: str | None
        details: CertificateDetails

    class Invalid:
        handle: X509CertificateReference
        friendly_name: str | None
        invalid_reason: InvalidCertificateReason


class ListUserCertificatesError(ErrorVariant):
    class CannotOpenStore:
        pass


async def list_user_certificates_with_details() -> Result[
    list[CertificateWithDetails], ListUserCertificatesError
]:
    raise NotImplementedError
