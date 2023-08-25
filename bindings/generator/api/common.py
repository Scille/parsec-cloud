# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, Generic, TypeVar

#
# Meta-types
#
# Those types are not part of the API but to be used to describe it.
#

OK = TypeVar("OK")
ERR = TypeVar("ERR")
REFERENCED = TypeVar("REFERENCED")


class Result(Generic[OK, ERR]):
    pass


# e.g.
#
#       enum Foo {
#           A{a: u32},
#           B(u32, u32),
#           C
#       }
#
# represented as:
#
#     class Foo(Variant):
#         class A:
#             a: int
#         B: VariantItemTuple(int, int)
#         C: VariantItemUnit
class Variant:
    pass


class VariantItemUnit:
    pass


class VariantItemTuple:
    def __init__(self, *items: Any):
        self.items = items


# Similar to a variant, but:
# - also provide an `error` field that contains the `to_string()` of the value.
# - Doesn't allow js-to-rust conversion (given this is only a type returned by the Rust API)
class ErrorVariant(Variant):
    pass


class Structure:
    pass


# Represent passing parameter in function by reference
class Ref(Generic[REFERENCED]):
    pass


# A type that should be converted from/into string
class StrBasedType:
    pass


# A type that should be converted from/into bytes
class BytesBasedType:
    pass


# A type that should be converted from/into int
class I32BasedType:
    pass


class U32BasedType:
    pass


class CustomConversionType:
    pass


#
# Common types
#


class Handle(U32BasedType):
    pass


class OrganizationID(StrBasedType):
    pass


class UserID(StrBasedType):
    pass


class DeviceID(StrBasedType):
    pass


class DeviceLabel(StrBasedType):
    pass


class HumanHandle(Structure):
    email: str
    label: str
    custom_getters: dict[str, str] = {"email": "email", "label": "label"}
    custom_init: dict[str, str] = {
        "web": 'libparsec::HumanHandle::new(&email, &label).map_err(|e| TypeError::new(&format!("Invalid HumanHandle: {e}")).into())',
        "electron": "libparsec::HumanHandle::new(&email, &label).or_throw(cx)",
    }


class DateTime(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::DateTime::from_rfc3339(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = (
        "|dt: libparsec::DateTime| -> Result<String, &'static str> { Ok(dt.to_rfc3339()) }"
    )


class Password(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { Ok(s.into()) }"


class BackendAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class BackendOrganizationAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendOrganizationAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendOrganizationAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class BackendOrganizationBootstrapAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class BackendInvitationAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendInvitationAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendInvitationAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class Path(StrBasedType):
    custom_from_rs_string = (
        "|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) }"
    )
    custom_to_rs_string = '|path: std::path::PathBuf| -> Result<_, _> { path.into_os_string().into_string().map_err(|_| "Path contains non-utf8 characters") }'


class SequesterVerifyKeyDer(BytesBasedType):
    pass


class SASCode(StrBasedType):
    pass


class EntryName(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, _> { s.parse::<libparsec::EntryName>().map_err(|e| e.to_string()) }"


class EntryID(BytesBasedType):
    custom_from_rs_bytes = (
        "|x: &[u8]| -> Result<_, _> { libparsec::EntryID::try_from(x).map_err(|e| e.to_string()) }"
    )
    custom_to_rs_bytes = (
        "|x: libparsec::EntryID| -> Result<_, &'static str> { Ok(x.as_bytes().to_owned()) }"
    )


class RealmID(BytesBasedType):
    custom_from_rs_bytes = (
        "|x: &[u8]| -> Result<_, _> { libparsec::RealmID::try_from(x).map_err(|e| e.to_string()) }"
    )
    custom_to_rs_bytes = (
        "|x: libparsec::RealmID| -> Result<_, &'static str> { Ok(x.as_bytes().to_owned()) }"
    )


class InvitationToken(BytesBasedType):
    custom_from_rs_bytes = "|x: &[u8]| -> Result<_, _> { libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string()) }"
    custom_to_rs_bytes = (
        "|x: libparsec::InvitationToken| -> Result<_, &'static str> { Ok(x.as_bytes().to_owned()) }"
    )


class UserProfile(Variant):
    Admin = VariantItemUnit
    Standard = VariantItemUnit
    Outsider = VariantItemUnit
