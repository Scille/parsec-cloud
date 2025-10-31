# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import ClassVar

from .common import (
    Bytes,
    BytesBasedType,
    DateTime,
    EnrollmentID,
    ErrorVariant,
    Result,
    StrBasedType,
    Structure,
    UnitStructure,
    Variant,
    VariantItemTuple,
)


class X509CertificateHash(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { <libparsec::X509CertificateHash as std::str::FromStr>::from_str(s.as_str()).map_err(|e| e.to_string()) }"
    custom_to_rs_string = (
        "|x: libparsec::X509CertificateHash| -> Result<_, &'static str> { Ok(x.to_string()) }"
    )


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
