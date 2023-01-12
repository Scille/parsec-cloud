# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Generic, List, Optional, TypeVar


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


# A type that should be converted from/into int
class IntBasedType:
    pass


class LoggedCoreHandle(IntBasedType):
    pass


class OrganizationID(StrBasedType):
    pass


class DeviceLabel(StrBasedType):
    pass


class HumanHandle(StrBasedType):
    pass


class StrPath(StrBasedType):
    pass


class DeviceID(StrBasedType):
    pass


class DeviceFileType(Variant):
    class Password:
        pass

    class Recovery:
        pass

    class Smartcard:
        pass


class AvailableDevice(Structure):
    key_file_path: StrPath
    organization_id: OrganizationID
    device_id: DeviceID
    human_handle: Optional[HumanHandle]
    device_label: Optional[DeviceLabel]
    slug: str
    ty: DeviceFileType


async def list_available_devices(path: StrPath) -> List[AvailableDevice]:
    ...


async def test_gen_default_devices():
    ...


async def login(test_device_id: DeviceID) -> LoggedCoreHandle:
    ...


class LoggedCoreError(Variant):
    class InvalidHandle:
        handle: LoggedCoreHandle

    class Disconnected:
        pass


async def logged_core_get_test_device_id(
    handle: LoggedCoreHandle,
) -> Result[DeviceID, LoggedCoreError]:
    ...
