# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.serde import fields

from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import BlockIDField, EntryIDField


__all__ = ("block_create_serializer", "block_read_serializer")


class BlockCreateReqSchema(BaseReqSchema):
    block_id = BlockIDField(required=True)
    realm_id = EntryIDField(required=True)
    block = fields.Bytes(required=True)


class BlockCreateRepSchema(BaseRepSchema):
    pass


block_create_serializer = CmdSerializer(BlockCreateReqSchema, BlockCreateRepSchema)


class BlockReadReqSchema(BaseReqSchema):
    block_id = BlockIDField(required=True)


class BlockReadRepSchema(BaseRepSchema):
    block = fields.Bytes(required=True)


block_read_serializer = CmdSerializer(BlockReadReqSchema, BlockReadRepSchema)
