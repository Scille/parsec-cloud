# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Type

from parsec.serde import fields
from parsec._parsec import BlockID

__all__ = ("BlockID", "BlockIDField", "block_create_serializer", "block_read_serializer")


BlockIDField: Type[fields.Field] = fields.uuid_based_field_factory(BlockID)

from parsec._parsec import (
    BlockReadReq,
    BlockReadRep,
    BlockCreateReq,
    BlockCreateRep,
)


class BlockCreateSerializer:
    @staticmethod
    def req_dumps(req) -> bytes:  # type: ignore[no-untyped-def]
        return BlockCreateReq(req["block_id"], req["realm_id"], req["block"]).dump()

    @staticmethod
    def rep_loads(raw: bytes) -> BlockCreateRep:
        try:
            return BlockCreateRep.load(raw)
        except ValueError as exc:
            raise exc


class BlockReadSerializer:
    @staticmethod
    def req_dumps(req) -> bytes:  # type: ignore[no-untyped-def]
        return BlockReadReq(req["block_id"]).dump()

    @staticmethod
    def rep_loads(raw: bytes) -> BlockReadRep:
        try:
            return BlockReadRep.load(raw)
        except ValueError as exc:
            raise exc


block_create_serializer = BlockCreateSerializer
block_read_serializer = BlockReadSerializer
