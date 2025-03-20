# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from base64 import b64decode, b64encode
from collections.abc import Buffer
from enum import Enum, auto
from typing import Annotated, Final

from pydantic import GetPydanticSchema, PlainSerializer, PlainValidator, ValidationError
from pydantic_core import core_schema

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    DeviceID,
    GreetingAttemptID,
    InvitationStatus,
    InvitationToken,
    OrganizationID,
    RealmRole,
    SequesterServiceID,
    UserID,
    UserProfile,
    VlobID,
)


# Unset singleton used as default value in function parameter when `None`
# can be a valid value.
# We implement this as an enum to satisfy type checker (see
# https://github.com/python/typing/issues/689#issuecomment-561425237)
class UnsetType(Enum):
    Unset = auto()


Unset: Final = UnsetType.Unset


class BadOutcome:
    pass


class BadOutcomeEnum(BadOutcome, Enum):
    pass


def base64_bytes_validator(val: object) -> Buffer:
    match val:
        case str():
            return b64decode(val)
        case Buffer():
            return val
        case _:
            raise ValidationError()


def base64_bytes_serializer(val: Buffer) -> str:
    return b64encode(val).decode("ascii")


# Why a custom type since Pydantic already has a `Base64Bytes` field ?
#
# Long story short, Pydantic's `Base64Bytes`:
# - Adds `\n` in the the base64 output
# - Want the input to deserialize to be bytes, which doesn't work for JSON since
#   a str is provided.
#
# In a nutshell, Pydantic's base64 serialization only works for serialization
# formats that natively support the bytes type, i.e. formats that don't need
# base64 in the first place :/
Base64Bytes = Annotated[
    bytes, PlainValidator(base64_bytes_validator), PlainSerializer(base64_bytes_serializer)
]


OrganizationIDField = Annotated[
    OrganizationID,
    PlainValidator(lambda x: x if isinstance(x, OrganizationID) else OrganizationID(x)),
    PlainSerializer(lambda x: x.str, return_type=str),
]

user_id_from_str_schema = core_schema.chain_schema(
    [  # check if it's a valid string, then that it can be converted to UserID
        core_schema.str_schema(),
        core_schema.no_info_plain_validator_function(lambda v: UserID.from_hex(v)),
    ]
)
UserIDField = Annotated[
    UserID,
    GetPydanticSchema(
        lambda _source_type, _handler: core_schema.json_or_python_schema(
            json_schema=user_id_from_str_schema,
            python_schema=core_schema.union_schema(
                [
                    # check if it's a UserID first, if not use schema from str
                    core_schema.is_instance_schema(UserID),
                    user_id_from_str_schema,
                ]
            ),
            serialization=core_schema.plain_serializer_function_ser_schema(
                lambda v: v.hex if isinstance(v, UserID) else v
            ),
        ),
    ),
]
DeviceIDField = Annotated[
    DeviceID,
    PlainValidator(lambda x: x if isinstance(x, DeviceID) else DeviceID.from_hex(x)),
    PlainSerializer(lambda x: x.hex, return_type=str),
]
SequesterServiceIDField = Annotated[
    SequesterServiceID,
    PlainValidator(
        lambda x: x if isinstance(x, SequesterServiceID) else SequesterServiceID.from_hex(x)
    ),
    PlainSerializer(lambda x: x.hex, return_type=str),
]
InvitationTokenField = Annotated[
    InvitationToken,
    PlainValidator(lambda x: x if isinstance(x, InvitationToken) else InvitationToken.from_hex(x)),
    PlainSerializer(lambda x: x.hex, return_type=str),
]
GreetingAttemptIDField = Annotated[
    GreetingAttemptID,
    PlainValidator(
        lambda x: x if isinstance(x, GreetingAttemptID) else GreetingAttemptID.from_hex(x)
    ),
    PlainSerializer(lambda x: x.hex, return_type=str),
]
InvitationStatusField = Annotated[
    InvitationStatus,
    PlainValidator(
        lambda x: x if isinstance(x, InvitationStatus) else InvitationStatus.from_str(x)
    ),
    PlainSerializer(lambda x: x.str, return_type=str),
]
RealmRoleField = Annotated[
    RealmRole,
    PlainValidator(lambda x: x if isinstance(x, RealmRole) else RealmRole.from_str(x)),
    PlainSerializer(lambda x: x.str, return_type=str),
]
VlobIDField = Annotated[
    VlobID,
    PlainValidator(lambda x: x if isinstance(x, VlobID) else VlobID.from_hex(x)),
    PlainSerializer(lambda x: x.hex, return_type=str),
]
DateTimeField = Annotated[
    DateTime,
    PlainValidator(lambda x: x if isinstance(x, DateTime) else DateTime.from_rfc3339(x)),
    PlainSerializer(lambda x: x.to_rfc3339(), return_type=str),
]
UserProfileField = Annotated[
    UserProfile,
    PlainValidator(lambda x: x if isinstance(x, UserProfile) else UserProfile.from_str(x)),
    PlainSerializer(lambda x: x.str, return_type=str),
]
ActiveUsersLimitField = Annotated[
    ActiveUsersLimit,
    PlainValidator(
        lambda x: x if isinstance(x, ActiveUsersLimit) else ActiveUsersLimit.from_maybe_int(x)
    ),
    PlainSerializer(lambda x: x.to_maybe_int(), return_type=int | None),
]
