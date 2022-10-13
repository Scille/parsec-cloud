# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    UserGetReq,
    UserGetRep,
    UserCreateReq,
    UserCreateRep,
    UserRevokeReq,
    UserRevokeRep,
    DeviceCreateReq,
    DeviceCreateRep,
    HumanFindReq,
    HumanFindRep,
)
from parsec.api.protocol.base import ApiCommandSerializer


__all__ = (
    "user_get_serializer",
    "user_create_serializer",
    "user_revoke_serializer",
    "device_create_serializer",
    "human_find_serializer",
)


#### Access user API ####

user_get_serializer = ApiCommandSerializer(UserGetReq, UserGetRep)


#### User creation API ####

user_create_serializer = ApiCommandSerializer(UserCreateReq, UserCreateRep)
user_revoke_serializer = ApiCommandSerializer(UserRevokeReq, UserRevokeRep)


#### Device creation API ####

device_create_serializer = ApiCommandSerializer(DeviceCreateReq, DeviceCreateRep)


# Human search API

human_find_serializer = ApiCommandSerializer(HumanFindReq, HumanFindRep)
