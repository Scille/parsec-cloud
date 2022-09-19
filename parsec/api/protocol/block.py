# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Type

from parsec._parsec import (
    BlockID,
    BlockReadReq,
    BlockReadRep,
    BlockCreateReq,
    BlockCreateRep,
)
from parsec.api.protocol.base import ApiCommandSerializer
from parsec.serde import fields

__all__ = (
    "BlockID",
    "BlockIDField",
    "block_create_serializer",
    "block_read_serializer",
)

BlockIDField: Type[fields.Field] = fields.uuid_based_field_factory(BlockID)

block_create_serializer = ApiCommandSerializer(BlockCreateReq, BlockCreateRep)
block_read_serializer = ApiCommandSerializer(BlockReadReq, BlockReadRep)
