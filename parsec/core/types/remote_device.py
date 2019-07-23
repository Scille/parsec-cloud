# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from typing import Optional
from uuid import UUID

from parsec.types import DeviceID, DeviceName, UserID
from parsec.api.protocol import RealmRole
from parsec.crypto import (
    PublicKey,
    VerifyKey,
    unsecure_read_user_certificate,
    unsecure_read_device_certificate,
    unsecure_read_realm_role_certificate,
)


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class UnverifiedRemoteUser:
    user_certificate: bytes
    fetched_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)

    def __repr__(self):
        info = unsecure_read_user_certificate(self.user_certificate)
        return f"{self.__class__.__name__}({info.user_id})"


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class UnverifiedRemoteDevice:
    device_certificate: bytes
    revoked_device_certificate: bytes = None
    fetched_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)

    def __repr__(self):
        info = unsecure_read_device_certificate(self.device_certificate)
        return f"{self.__class__.__name__}({info.device_id})"


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class UnverifiedRealmRole:
    realm_role_certificate: bytes
    fetched_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)

    def __repr__(self):
        info = unsecure_read_realm_role_certificate(self.realm_role_certificate)
        return (
            f"{self.__class__.__name__}(realm={info.realm_id}, "
            f"user={info.user_id}, role={info.role})"
        )


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class VerifiedRemoteUser:
    fetched_on: pendulum.Pendulum

    user_id: UserID
    public_key: PublicKey

    user_certificate: bytes
    certified_by: DeviceID
    certified_on: pendulum.Pendulum
    is_admin: bool = False

    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class VerifiedRemoteDevice:
    fetched_on: pendulum.Pendulum

    device_id: DeviceID
    verify_key: VerifyKey

    device_certificate: bytes
    certified_by: DeviceID
    certified_on: pendulum.Pendulum

    revoked_device_certificate: bytes = None
    revoked_by: DeviceID = None
    revoked_on: pendulum.Pendulum = None

    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    @property
    def device_name(self) -> DeviceName:
        return self.device_id.device_name

    @property
    def user_id(self) -> UserID:
        return self.device_id.user_id


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class VerifiedRealmRole:
    fetched_on: pendulum.Pendulum

    realm_id: UUID
    user_id: UserID
    role: Optional[RealmRole]

    certified_by: DeviceID
    certified_on: pendulum.Pendulum

    realm_role_certificate: bytes

    def __repr__(self):
        return (
            f"{self.__class__.__name__}(realm={self.realm_id}, "
            f"user={self.user_id}, role={self.role})"
        )
