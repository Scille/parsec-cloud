# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Generic, TypeVar

#
# Meta-types
#
# Those types are not part of the API but to be used to describe it.
#


class Result(Generic[TypeVar("OK"), TypeVar("ERR")]):
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
    def __init__(self, *items):
        self.items = items


# Similar to a variant, but:
# - also provide an `error` field that contains the `to_string()` of the value.
# - Doesn't allow js-to-rust conversion (given this is only a type returned by the Rust API)
class ErrorVariant(Variant):
    pass


class Structure:
    pass


# Represent passing parameter in function by reference
class Ref(Generic[TypeVar("REFERENCED")]):
    pass


# A type that should be converted from/into string
class StrBasedType:
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


class LoggedCoreHandle(I32BasedType):
    pass


class OrganizationID(StrBasedType):
    pass


class DeviceLabel(StrBasedType):
    pass


class HumanHandle(StrBasedType):
    pass


class BackendAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class DeviceID(StrBasedType):
    pass


class ClientHandle(U32BasedType):
    pass


class Path(StrBasedType):
    custom_from_rs_string = (
        "|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) }"
    )
    custom_to_rs_string = '|path: std::path::PathBuf| -> Result<_, _> { path.into_os_string().into_string().map_err(|_| "Path contains non-utf8 characters") }'
