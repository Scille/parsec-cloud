# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import TYPE_CHECKING

from parsec.types import UUID4
from parsec.serde import fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.realm import RealmIDField


__all__ = ("BlockID", "BlockIDField", "block_create_serializer", "block_read_serializer")


class BlockID(UUID4):
    __slots__ = ()


_PyBlockID = BlockID
if not TYPE_CHECKING:
    try:
        from libparsec.types import BlockID as _RsBlockID
    except:
        pass
    else:
        BlockID = _RsBlockID


BlockIDField = fields.uuid_based_field_factory(BlockID)


class BlockCreateReqSchema(BaseReqSchema):
    block_id = BlockIDField(required=True)
    realm_id = RealmIDField(required=True)
    block = fields.Bytes(required=True)


class BlockCreateRepSchema(BaseRepSchema):
    pass


block_create_serializer = CmdSerializer(BlockCreateReqSchema, BlockCreateRepSchema)


class BlockReadReqSchema(BaseReqSchema):
    block_id = BlockIDField(required=True)


class BlockReadRepSchema(BaseRepSchema):
    block = fields.Bytes(required=True)


block_read_serializer = CmdSerializer(BlockReadReqSchema, BlockReadRepSchema)
