# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = ("blockstore_create_serializer", "blockstore_read_serializer")


# TODO: should specify beacon ID in order to secure history garbage collection


class BlockstoreCreateReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    block = fields.Bytes(required=True)


class BlockstoreCreateRepSchema(BaseRepSchema):
    pass


blockstore_create_serializer = CmdSerializer(BlockstoreCreateReqSchema, BlockstoreCreateRepSchema)


class BlockstoreReadReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)


class BlockstoreReadRepSchema(BaseRepSchema):
    block = fields.Bytes(required=True)


blockstore_read_serializer = CmdSerializer(BlockstoreReadReqSchema, BlockstoreReadRepSchema)
