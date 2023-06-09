# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from marshmallow import post_load, pre_dump, pre_load, validate

from parsec.serde import fields
from parsec.serde.exceptions import SerdeError, SerdePackingError, SerdeValidationError
from parsec.serde.packing import Unpacker, packb, unpackb
from parsec.serde.schema import BaseSchema, OneOfSchema
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
