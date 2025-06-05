# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from base64 import b64decode, b64encode
from collections.abc import Buffer
from enum import Enum, auto
from typing import Annotated, Final

from pydantic import GetPydanticSchema, PlainSerializer, PlainValidator
from pydantic_core.core_schema import (
    chain_schema,
    int_schema,
    is_instance_schema,
    json_or_python_schema,
    no_info_plain_validator_function,
    none_schema,
    plain_serializer_function_ser_schema,
    str_schema,
    union_schema,
)

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    DeviceID,
    EmailAddress,
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
from parsec.config import AccountVaultStrategy, AllowedClientAgent


# The Unset singleton is used as default value in functions when `None` can be a
# valid value (so `None` and `Unset` will have a different meaning).
# This idiom is commonly known as a "sentinel value" and there is not a standard
# way to implement it (see https://peps.python.org/pep-0661/).
# We implement it here as an enum to satisfy type checker (see
# https://github.com/python/typing/issues/689#issuecomment-561425237)
class UnsetType(Enum):
    Unset = auto()


Unset: Final = UnsetType.Unset


# -- Bad outcome types --------------------------------------------------------


class BadOutcome:
    """
    Base class for bad outcomes returned from Parsec API.

    Inherit from this class if the bad outcome needs to include additional
    attributes. For example:

    ```python
    @dataclass(slots=True)
    class BadKeyIndex(BadOutcome):
        last_realm_certificate_timestamp: DateTime
    ```
    """

    pass


class BadOutcomeEnum(BadOutcome, Enum):
    """
    Base class for bad outcomes enums returned from Parsec API.

    Inherit from this class if the bad outcome does not need to include
    additional attributes, only . For example:

    ```python
    class SendEmailBadOutcome(BadOutcomeEnum):
        SERVER_UNAVAILABLE = auto()
        RECIPIENT_REFUSED = auto()
        BAD_SMTP_CONFIG = auto()
    ```
    """

    pass


# -- Pydantic schema function helper ------------------------------------------


def get_pydantic_schema(
    type, validator_func, serializer_func, from_schema=None, allowed_none=False
):
    """
    Helper to get a pydantic schema for custom `type` annotation.

    By default, the schemas used to deserialize JSON/Python inputs are based on `str_schema`_
    but you can specify a different `from_schema` (such as `int_schema`, `bool_schema`, etc).
    See `pydantic_core_schema`_ for more information.

    Args:
        type: the custom type to generate the pydantic schema
        validator_func: the function to deserialize an input into an instance of `type`
        serializer_func: the function to serialize an instance of `type`
        from_schema: the base schema to be used for input (`str_schema` by default)
        allowed_none: set to `True` if the input value can be `None` (`False` by default)

    .. _pydantic_core_schema: https://docs.pydantic.dev/latest/api/pydantic_core_schema/
    .. _str_schema: https://docs.pydantic.dev/latest/api/pydantic_core_schema/#pydantic_core.core_schema.str_schema
    """
    # If not specified, use a str_schema as it is the most common case
    from_schema = str_schema() if from_schema is None else from_schema

    # Construct a schema to deserialize a value from base input type (e.g. `str_schema` or `int_schema`)
    # into an instance of `type`. Note this is a `chain_schema` so the input will be
    # validated against all schemas in the specified order.
    type_from_base_schema = chain_schema(
        [
            # First, check if it's a valid value from base type,
            from_schema if not allowed_none else union_schema([none_schema(), from_schema]),
            # then that it can be converted to the concrete type
            no_info_plain_validator_function(validator_func),
        ]
    )
    return GetPydanticSchema(
        lambda _source_type, _handler: json_or_python_schema(
            # schema to deserialize a JSON input into an instance of `type`
            json_schema=type_from_base_schema,
            # schema to deserialize a Python input into an instance of `type`
            # checks if it's already a `type` instance or if it can be converted from base type
            python_schema=union_schema(
                [
                    is_instance_schema(type),
                    type_from_base_schema,
                ]
            ),
            # schema to serialize an instance of `type`
            serialization=plain_serializer_function_ser_schema(serializer_func),
        )
    )


# -- Field types annotations --------------------------------------------------


# Base64BytesField
#
# Why a custom type if Pydantic already has `Base64Bytes` type?
#
# Long story short, Pydantic's `Base64Bytes` wants the input to be bytes,
# which doesn't work for JSON since a str is provided.
#
# In a nutshell, Pydantic's base64 serialization only works for serialization
# formats that natively support the bytes type, i.e. formats that don't need
# base64 in the first place :/
#
# We might have use `Base64Str` but this is a str type, not a bytes type.


def base64_bytes_validator(val: object) -> Buffer:
    match val:
        case str():
            # validate to ensure input string does not contain non-alphabet characters
            return b64decode(val, validate=True)
        case Buffer():
            return val
        case _:
            raise ValueError("Invalid base64")


def base64_bytes_serializer(val: Buffer) -> str:
    return b64encode(val).decode("ascii")


Base64BytesField = Annotated[
    bytes, PlainValidator(base64_bytes_validator), PlainSerializer(base64_bytes_serializer)
]


OrganizationIDField = Annotated[
    OrganizationID,
    PlainValidator(lambda x: x if isinstance(x, OrganizationID) else OrganizationID(x)),
    PlainSerializer(lambda x: x.str, return_type=str),
]

UserIDField = Annotated[
    UserID,
    get_pydantic_schema(
        UserID, lambda v: UserID.from_hex(v), lambda v: v.hex if isinstance(v, UserID) else v
    ),
]
DeviceIDField = Annotated[
    DeviceID,
    get_pydantic_schema(
        DeviceID, lambda v: DeviceID.from_hex(v), lambda v: v.hex if isinstance(v, DeviceID) else v
    ),
]
SequesterServiceIDField = Annotated[
    SequesterServiceID,
    get_pydantic_schema(
        SequesterServiceID,
        lambda v: SequesterServiceID.from_hex(v),
        lambda v: v.hex if isinstance(v, SequesterServiceID) else v,
    ),
]
InvitationTokenField = Annotated[
    InvitationToken,
    get_pydantic_schema(
        InvitationToken,
        lambda v: InvitationToken.from_hex(v),
        lambda v: v.hex if isinstance(v, InvitationToken) else v,
    ),
]
GreetingAttemptIDField = Annotated[
    GreetingAttemptID,
    get_pydantic_schema(
        GreetingAttemptID,
        lambda v: GreetingAttemptID.from_hex(v),
        lambda v: v.hex if isinstance(v, GreetingAttemptID) else v,
    ),
]
InvitationStatusField = Annotated[
    InvitationStatus,
    get_pydantic_schema(
        InvitationStatus,
        lambda v: InvitationStatus.from_str(v),
        lambda v: v.str if isinstance(v, InvitationStatus) else v,
    ),
]
RealmRoleField = Annotated[
    RealmRole,
    get_pydantic_schema(
        RealmRole,
        lambda v: RealmRole.from_str(v),
        lambda v: v.str if isinstance(v, RealmRole) else v,
    ),
]
VlobIDField = Annotated[
    VlobID,
    get_pydantic_schema(
        VlobID, lambda v: VlobID.from_hex(v), lambda v: v.hex if isinstance(v, VlobID) else v
    ),
]
DateTimeField = Annotated[
    DateTime,
    get_pydantic_schema(
        DateTime,
        lambda v: DateTime.from_rfc3339(v),
        lambda v: v.to_rfc3339() if isinstance(v, DateTime) else v,
    ),
]

UserProfileField = Annotated[
    UserProfile,
    get_pydantic_schema(
        UserProfile,
        lambda v: UserProfile.from_str(v),
        lambda v: v.str if isinstance(v, UserProfile) else v,
    ),
]
ActiveUsersLimitField = Annotated[
    ActiveUsersLimit,
    get_pydantic_schema(
        ActiveUsersLimit,
        lambda v: ActiveUsersLimit.from_maybe_int(v),
        lambda v: v.to_maybe_int() if isinstance(v, ActiveUsersLimit) else v,
        from_schema=int_schema(),
        allowed_none=True,
    ),
]
EmailAddressField = Annotated[
    EmailAddress,
    get_pydantic_schema(
        EmailAddress,
        lambda v: EmailAddress(v),
        lambda v: v.str if isinstance(v, EmailAddress) else v,
    ),
]
AllowedClientAgentField = Annotated[
    AllowedClientAgent,
    PlainValidator(lambda x: x if isinstance(x, AllowedClientAgent) else AllowedClientAgent(x)),
    PlainSerializer(lambda x: x.value),
]
AccountVaultStrategyField = Annotated[
    AccountVaultStrategy,
    PlainValidator(lambda x: x if isinstance(x, AccountVaultStrategy) else AccountVaultStrategy(x)),
    PlainSerializer(lambda x: x.value),
]
