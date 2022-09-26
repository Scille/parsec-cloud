# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Generic, TypeVar


# Meta-types, not part of the API but to be used to describe the API


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


class OrganizationID(StrBasedType):
    pass


class DeviceID(StrBasedType):
    pass


class DeviceLabel(StrBasedType):
    pass


class PathBuf(StrBasedType):
    pass


Path = Ref[PathBuf]


class HelloError(Variant):
    class EmptySubject:
        pass

    class YouAreADog:
        hello: str


def hello_world(subject: Ref[str]) -> Result[str, HelloError]:
    ...
