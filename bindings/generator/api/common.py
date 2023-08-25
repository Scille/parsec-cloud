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
    custom_getters: dict[str, str] = {
        "email": """fn {fn_name}<'a>(obj: &'a libparsec::HumanHandle) -> &'a str {{
            obj.email()
        }}""",
        "label": """fn {fn_name}<'a>(obj: &'a libparsec::HumanHandle) -> &'a str {{
            obj.label()
        }}""",
    }
    custom_init_fn: str = """
        fn {fn_name}(email: impl AsRef<str>, label: impl AsRef<str>) -> Result<libparsec::HumanHandle, String> {{
            libparsec::HumanHandle::new(email.as_ref(), label.as_ref()).map_err(|e| e.to_string())
        }}
    """


class DateTime(StrBasedType):
    custom_from_rs_string_fn = """
        fn {fn_name}(s: String) -> Result<libparsec::DateTime, String> {{
            libparsec::DateTime::from_rfc3339(&s).map_err(|e| e.to_string())
        }}
    """
    custom_to_rs_string_fn = """
        fn {fn_name}(dt: libparsec::DateTime) -> Result<String, &'static str> {{
            Ok(dt.to_rfc3339())
        }}
    """


class Password(StrBasedType):
    custom_from_rs_string_fn = """
        fn {fn_name}(s: String) -> Result<libparsec::Password, String> {{
            Ok(s.into())
        }}
    """


class BackendAddr(StrBasedType):
    custom_from_rs_string_fn = """
        fn {fn_name}(s: String) -> Result<libparsec::BackendAddr, String> {{
            libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string())
        }}
    """
    custom_to_rs_string_fn = """
        #[allow(dead_code)]
        fn {fn_name}(addr: libparsec::BackendAddr) -> Result<String, &'static str> {{
            Ok(addr.to_url().into())
        }}
    """


class BackendOrganizationAddr(StrBasedType):
    custom_from_rs_string_fn = """
        #[allow(dead_code)]
        fn {fn_name}(s: String) -> Result<libparsec::BackendOrganizationAddr, String> {{
            libparsec::BackendOrganizationAddr::from_any(&s).map_err(|e| e.to_string())
        }}
    """
    custom_to_rs_string_fn = """
        #[allow(dead_code)]
        fn {fn_name}(addr: libparsec::BackendOrganizationAddr) -> Result<String, &'static str> {{
            Ok(addr.to_url().into())
        }}
    """


class BackendOrganizationBootstrapAddr(StrBasedType):
    custom_from_rs_string_fn = """
        fn {fn_name}(s: String) -> Result<libparsec::BackendOrganizationBootstrapAddr, String> {{
            libparsec::BackendOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string())
        }}
    """
    custom_to_rs_string_fn = """
        fn {fn_name}(addr: libparsec::BackendOrganizationBootstrapAddr) -> Result<String, &'static str> {{
            Ok(addr.to_url().into())
        }}
    """


class BackendInvitationAddr(StrBasedType):
    custom_from_rs_string_fn = """
        fn {fn_name}(s: String) -> Result<libparsec::BackendInvitationAddr, String> {{
            libparsec::BackendInvitationAddr::from_any(&s).map_err(|e| e.to_string())
        }}
    """
    custom_to_rs_string_fn = """
        #[allow(dead_code)]
        fn {fn_name}(addr: libparsec::BackendInvitationAddr) -> Result<String, &'static str> {{
            Ok(addr.to_url().into())
        }}
    """


class Path(StrBasedType):
    custom_from_rs_string_fn = """
        fn {fn_name}(s: String) -> Result<std::path::PathBuf, &'static str> {{
            Ok(std::path::PathBuf::from(s))
        }}
    """
    custom_to_rs_string_fn = """
        fn {fn_name}(path: std::path::PathBuf) -> Result<String, &'static str> {{
            path.into_os_string().into_string().map_err(|_| "Path contains non-utf8 characters")
        }}
    """


class SequesterVerifyKeyDer(BytesBasedType):
    pass


class SASCode(StrBasedType):
    pass


class EntryName(StrBasedType):
    custom_from_rs_string_fn = """
        fn {fn_name}(s: String) -> Result<libparsec::EntryName, String> {{
            s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
        }}
    """


class EntryID(BytesBasedType):
    custom_from_rs_bytes_fn = """
        fn {fn_name}(x: &[u8]) -> Result<libparsec::EntryID, String> {{
            libparsec::EntryID::try_from(x).map_err(|e| e.to_string())
        }}
    """
    custom_to_rs_bytes_fn = """
        fn {fn_name}(x: libparsec::EntryID) -> Result<Vec<u8>, &'static str> {{
            Ok(x.as_bytes().to_vec())
        }}
    """


class InvitationToken(BytesBasedType):
    custom_from_rs_bytes_fn = """
        fn {fn_name}(x: &[u8]) -> Result<libparsec::InvitationToken, String> {{
            libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
        }}
    """
    custom_to_rs_bytes_fn = """
        fn {fn_name}(x: libparsec::InvitationToken) -> Result<Vec<u8>, &'static str> {{
            Ok(x.as_bytes().to_owned())
        }}
    """


class UserProfile(Variant):
    Admin = VariantItemUnit
    Standard = VariantItemUnit
    Outsider = VariantItemUnit
