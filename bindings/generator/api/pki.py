# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import ClassVar

from .addr import ParsecPkiEnrollmentAddr
from .common import (
    DISPLAY_TO_STRING,
    Bytes,
    BytesBasedType,
    DateTime,
    DeviceLabel,
    EnrollmentID,
    ErrorVariant,
    HumanHandle,
    Result,
    StrBasedType,
    Structure,
    UnitStructure,
    Variant,
    VariantItemTuple,
)
from .config import ClientConfig


class X509CertificateHash(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { <libparsec::X509CertificateHash as std::str::FromStr>::from_str(s.as_str()).map_err(|e| e.to_string()) }"
    custom_to_rs_string = DISPLAY_TO_STRING


class X509WindowsCngURI(BytesBasedType):
    custom_from_rs_bytes = (
        "|v: &[u8]| -> Result<_, String> { Ok(libparsec::Bytes::copy_from_slice(v).into()) }"
    )
    custom_to_rs_bytes = (
        "|v: libparsec::X509WindowsCngURI| -> Result<Vec<u8>, String> { Ok(v.into()) }"
    )


class X509Pkcs11URI(UnitStructure):
    pass


class X509URIFlavorValue(Variant):
    WindowsCNG = VariantItemTuple(X509WindowsCngURI)
    PKCS11 = VariantItemTuple(X509Pkcs11URI)


class X509CertificateReference(Structure):
    uris: list[X509URIFlavorValue]
    hash: X509CertificateHash
    custom_getters: ClassVar = {
        "uris": """
        |o: &libparsec::X509CertificateReference| -> Vec<libparsec::X509URIFlavorValue> {
            o.uris().cloned().collect()
        }
        """,
        "hash": """
        |o: &libparsec::X509CertificateReference| -> libparsec::X509CertificateHash {
            o.hash.clone()
        }
        """,
    }
    custom_init: str = """
        |uris: Vec<libparsec::X509URIFlavorValue>, hash: libparsec::X509CertificateHash| -> Result<_, String> {
            let mut cert_ref = libparsec::X509CertificateReference::from(hash);
            for uri in uris {
                cert_ref = cert_ref.add_or_replace_uri_wrapped(uri);
            }
            Ok(cert_ref)
        }
    """


class ShowCertificateSelectionDialogError(ErrorVariant):
    class CannotOpenStore: ...

    class CannotGetCertificateInfo: ...


async def show_certificate_selection_dialog_windows_only() -> Result[
    X509CertificateReference | None, ShowCertificateSelectionDialogError
]:
    raise NotImplementedError


class PkiEnrollmentListItem(Structure):
    enrollment_id: EnrollmentID
    submitted_on: DateTime
    der_x509_certificate: Bytes
    payload_signature: Bytes
    payload: Bytes


class PkiEnrollmentListError(ErrorVariant):
    class Offline: ...

    class AuthorNotAllowed: ...

    class Internal: ...


async def is_pki_available() -> bool:
    raise NotImplementedError


class PkiEnrollmentRejectError(ErrorVariant):
    class Offline: ...

    class Internal: ...

    class AuthorNotAllowed: ...

    class EnrollmentNoLongerAvailable: ...

    class EnrollmentNotFound: ...


class PkiEnrollmentSubmitError(ErrorVariant):
    class Offline: ...

    class Internal: ...

    class AlreadyEnrolled: ...

    class AlreadySubmitted: ...

    class EmailAlreadyUsed: ...

    class IdAlreadyUsed: ...

    class InvalidPayload: ...

    class PkiOperationError: ...


async def pki_enrollment_submit(
    config: ClientConfig,
    addr: ParsecPkiEnrollmentAddr,
    cert_ref: X509CertificateReference,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    force: bool,
) -> Result[DateTime, PkiEnrollmentSubmitError]:
    raise NotImplementedError


class PkiEnrollmentAcceptError(ErrorVariant):
    class Offline: ...

    class Internal: ...

    class AuthorNotAllowed: ...

    class EnrollmentNoLongerAvailable: ...

    class EnrollmentNotFound: ...

    class ActiveUsersLimitReached: ...

    class HumanHandleAlreadyTaken: ...

    class PkiOperationError: ...
