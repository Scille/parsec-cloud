# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Type, Any

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


class ApiCommandSerializer:
    def __init__(self, req_schema: Any, rep_schema: Any) -> None:
        self.req_schema = req_schema
        self.rep_schema = rep_schema

    def req_dumps(self, req: dict[str, Any]) -> bytes:
        req.pop("cmd")
        return self.req_schema(**req).dump()

    def rep_loads(self, raw: bytes) -> Any:
        return self.rep_schema.load(raw)


block_create_serializer = ApiCommandSerializer(BlockCreateReq, BlockCreateRep)
block_read_serializer = ApiCommandSerializer(BlockReadReq, BlockReadRep)
