# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import TYPE_CHECKING, Union
import attr

from parsec.types import UUID4
from parsec.serde import fields
from parsec.api.protocol.base import (
    BaseReqSchema,
    BaseRepSchema,
    BaseTypedReqSchema,
    BaseTypedRepSchema,
    CmdSerializer,
    BaseReq,
    BaseRep,
    cmd_rep_error_type_factory,
    cmd_rep_factory,
)
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


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BlockReadReq(BaseReq):
    class SCHEMA_CLS(BaseTypedReqSchema):
        cmd = fields.CheckedConstant("block_read", required=True)
        block_id = BlockIDField(required=True)

    block_id: BlockID


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BlockReadRepOk(BaseRep):
    class SCHEMA_CLS(BaseTypedRepSchema):
        status = fields.CheckedConstant("ok", required=True)
        block = fields.Bytes(required=True)

    block: bytes


BlockReadRepNotFound = cmd_rep_error_type_factory("BlockReadRepNotFound", "not_found")
BlockReadRepTimeout = cmd_rep_error_type_factory("BlockReadRepTimeout", "timeout")
BlockReadRepNotAllowed = cmd_rep_error_type_factory("BlockReadRepNotAllowed", "not_allowed")
BlockReadRepInMaintenance = cmd_rep_error_type_factory(
    "BlockReadRepInMaintenance", "in_maintenance"
)

BlockReadRep = cmd_rep_factory(
    "BlockReadRep",
    BlockReadRepOk,
    BlockReadRepNotFound,
    BlockReadRepTimeout,
    BlockReadRepNotAllowed,
    BlockReadRepInMaintenance,
)

BlockReadRepType = Union[
    BlockReadRepOk,
    BlockReadRepNotFound,  # type: ignore[valid-type]
    BlockReadRepTimeout,  # type: ignore[valid-type]
    BlockReadRepNotAllowed,  # type: ignore[valid-type]
    BlockReadRepInMaintenance,  # type: ignore[valid-type]
]

block_read_serializer = CmdSerializer.from_typed(BlockReadReq, BlockReadRep.TYPES)

_PyBlockReadReq = BlockReadReq
if not TYPE_CHECKING:
    try:
        from libparsec.types import BlockReadReq as _RsBlockReadReq
    except:
        pass
    else:
        BlockReadReq = _RsBlockReadReq

_PyBlockReadRep = BlockReadRep
if not TYPE_CHECKING:
    try:
        from libparsec.types import BlockReadRep as _RsBlockReadRep
    except:
        pass
    else:
        BlockReadRep = _RsBlockReadRep
