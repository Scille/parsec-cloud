# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from marshmallow import validate, pre_dump, post_load, pre_load

from parsec.serde import fields
from parsec.serde.exceptions import SerdeError, SerdeValidationError, SerdePackingError
from parsec.serde.schema import BaseSchema, OneOfSchema, BaseCmdSchema
from parsec.serde.packing import packb, unpackb, Unpacker
from parsec.serde.serializer import (
    BaseSerializer,
    JSONSerializer,
    MsgpackSerializer,
    ZipMsgpackSerializer,
)

__all__ = (
    "SerdeError",
    "SerdeValidationError",
    "SerdePackingError",
    "BaseSchema",
    "OneOfSchema",
    "BaseCmdSchema",
    "validate",
    "pre_dump",
    "pre_load",
    "post_load",
    "fields",
    "packb",
    "unpackb",
    "Unpacker",
    "BaseSerializer",
    "JSONSerializer",
    "MsgpackSerializer",
    "ZipMsgpackSerializer",
)
