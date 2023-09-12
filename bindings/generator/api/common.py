# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, Generic, TypeVar

#
# Meta-types
#
# These types are not part of the API but are meant to be used to describe it.
#

OK = TypeVar("OK")
ERR = TypeVar("ERR")
REFERENCED = TypeVar("REFERENCED")


class Result(Generic[OK, ERR]):
    pass


class Enum:
    pass


class EnumItemUnit:
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
    custom_getters: dict[str, str] = {
        "email": "|obj| { fn access(obj: &libparsec::HumanHandle) -> &str { obj.email() } access(obj) }",
        "label": "|obj| { fn access(obj: &libparsec::HumanHandle) -> &str { obj.label() } access(obj) }",
    }
    custom_init: str = """
        |email: String, label: String| -> Result<_, String> {
            libparsec::HumanHandle::new(&email, &label).map_err(|e| e.to_string())
        }
    """


class DateTime(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::DateTime::from_rfc3339(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = (
        "|dt: libparsec::DateTime| -> Result<String, &'static str> { Ok(dt.to_rfc3339()) }"
    )


class Password(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { Ok(s.into()) }"


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


# EntryID, RealmID and InvitationToken, are defined as strings (instead of
# Uint8Array) so that the Typescript code only manipulates strings without
# conversion or parsing.


class EntryID(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<libparsec::EntryID, _> { libparsec::EntryID::from_hex(s.as_str()).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|x: libparsec::EntryID| -> Result<String, &'static str> { Ok(x.hex()) }"


class RealmID(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<libparsec::RealmID, _> { libparsec::RealmID::from_hex(s.as_str()).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|x: libparsec::RealmID| -> Result<String, &'static str> { Ok(x.hex()) }"


class InvitationToken(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<libparsec::InvitationToken, _> { libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string()) }"
    custom_to_rs_string = (
        "|x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) }"
    )


class UserProfile(Enum):
    Admin = EnumItemUnit
    Standard = EnumItemUnit
    Outsider = EnumItemUnit
