# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = ("block_create_serializer", "block_read_serializer")


class BlockCreateReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    vlob_group = fields.UUID(required=True)
    block = fields.Bytes(required=True)


class BlockCreateRepSchema(BaseRepSchema):
    pass


block_create_serializer = CmdSerializer(BlockCreateReqSchema, BlockCreateRepSchema)


class BlockReadReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)


class BlockReadRepSchema(BaseRepSchema):
    block = fields.Bytes(required=True)


block_read_serializer = CmdSerializer(BlockReadReqSchema, BlockReadRepSchema)
