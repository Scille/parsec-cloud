# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from uuid import uuid4
from hashlib import sha256
from typing import Union

from parsec.crypto import SymetricKey, HashDigest, generate_secret_key
from parsec.serde import UnknownCheckedSchema, fields, validate, post_load
from parsec.core.types.base import TrustSeed, AccessID, TrustSeedField, serializer_factory


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ManifestAccess:
    id: AccessID = attr.ib(factory=uuid4)
    key: SymetricKey = attr.ib(factory=generate_secret_key)


class ManifestAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.SymetricKey(required=True)

    @post_load
    def make_obj(self, data):
        return ManifestAccess(**data)


manifest_access_serializer = serializer_factory(ManifestAccessSchema)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BlockAccess:
    id: AccessID
    key: SymetricKey
    offset: int
    size: int
    digest: HashDigest

    @classmethod
    def from_block(cls, block: bytes, offset: int) -> "BlockAccess":
        return cls(
            id=uuid4(),
            key=generate_secret_key(),
            offset=offset,
            size=len(block),
            digest=sha256(block).hexdigest(),
        )


class BlockAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    # TODO: provide digest as hexa string
    digest = fields.String(required=True, validate=validate.Length(min=1, max=64))

    @post_load
    def make_obj(self, data):
        return BlockAccess(**data)


block_access_serializer = serializer_factory(BlockAccessSchema)


# TODO: not used yet, useful ?
@attr.s(slots=True, frozen=True, auto_attribs=True)
class DirtyBlockAccess:
    id: AccessID
    key: SymetricKey
    offset: int
    size: int


class DirtyBlockAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))

    @post_load
    def make_obj(self, data):
        return DirtyBlockAccess(**data)


dirty_block_access_serializer = serializer_factory(DirtyBlockAccessSchema)


Access = Union[ManifestAccess, BlockAccess, DirtyBlockAccess]
