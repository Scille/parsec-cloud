# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Generic, TypeVar

#
# Meta-types
#
# Those types are not part of the API but to be used to describe it.
#


class Result(Generic[TypeVar("OK"), TypeVar("ERR")]):  # noqa
    pass


class Variant:
    pass


class Structure:
    pass


# Represent passing parameter in function by reference
class Ref(Generic[TypeVar("REFERENCED")]):  # noqa
    pass


# A type that should be converted from/into string
class StrBasedType:
    pass


# A type that should be converted from/into int
class I32BasedType:
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


class Path(StrBasedType):
    pass


class StrPath(StrBasedType):
    pass


class BackendAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, _> { libparsec::BackendAddr::from_any(&s) }"
    custom_to_rs_string = "|addr: libparsec::BackendAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class DeviceID(StrBasedType):
    pass


class ClientHandle(I32BasedType):
    pass


class Path(StrBasedType):
    custom_from_rs_string = (
        "|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) }"
    )
    custom_to_rs_string = '|path: std::path::PathBuf| -> Result<_, _> { path.into_os_string().into_string().map_err(|_| "Path contains non-utf8 characters") }'
