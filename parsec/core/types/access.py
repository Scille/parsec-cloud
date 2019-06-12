# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from hashlib import sha256
from typing import Optional

from parsec.api.protocole import RealmRole, RealmRoleField
from parsec.crypto import SecretKey, HashDigest
from parsec.serde import UnknownCheckedSchema, fields, validate, post_load
from parsec.core.types.base import (
    BlockID,
    BlockIDField,
    EntryID,
    EntryIDField,
    serializer_factory,
    EntryNameField,
)


# TODO: legacy stuff, remove me
@attr.s(slots=True, frozen=True, auto_attribs=True)
class ManifestAccess:
    id: EntryID
    realm_id: EntryID
    key: SecretKey
    encryption_revision: int


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BlockAccess:
    id: BlockID
    key: SecretKey
    offset: int
    size: int
    digest: HashDigest

    @classmethod
    def from_block(cls, block: bytes, offset: int) -> "BlockAccess":
        return cls(
            id=BlockID(),
            key=SecretKey.generate(),
            offset=offset,
            size=len(block),
            digest=sha256(block).hexdigest(),
        )


class BlockAccessSchema(UnknownCheckedSchema):
    id = BlockIDField(required=True)
    key = fields.SecretKey(required=True)
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
    id: BlockID
    key: SecretKey
    offset: int
    size: int


class DirtyBlockAccessSchema(UnknownCheckedSchema):
    id = BlockIDField(required=True)
    key = fields.SecretKey(required=True)
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))

    @post_load
    def make_obj(self, data):
        return DirtyBlockAccess(**data)


dirty_block_access_serializer = serializer_factory(DirtyBlockAccessSchema)


# Republishing under a better name
WorkspaceRole = RealmRole
WorkspaceRoleField = RealmRoleField


# Not stricly speaking an access, but close enough...


@attr.s(slots=True, frozen=True, auto_attribs=True)
class WorkspaceEntry:
    name: str
    id: EntryID = attr.ib(factory=EntryID)
    key: SecretKey = attr.ib(factory=SecretKey.generate)
    encryption_revision: int = 1
    role_cached_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)
    role: Optional[WorkspaceRole] = WorkspaceRole.OWNER

    def is_revoked(self) -> bool:
        return self.role is None

    def evolve(self, **kwargs) -> "WorkspaceEntry":
        return attr.evolve(self, **kwargs)

    def evolve_and_mark_updated(self, **data) -> "WorkspaceEntry":
        if "role_cached_on" not in data:
            data["role_cached_on"] = pendulum.now()
        return attr.evolve(self, **data)


class WorkspaceEntrySchema(UnknownCheckedSchema):
    name = EntryNameField(validate=validate.Length(min=1, max=256), required=True)
    id = EntryIDField(required=True)
    key = fields.SecretKey(required=True)
    encryption_revision = fields.Int(required=True, validate=validate.Range(min=0))
    role_cached_on = fields.DateTime(required=True)
    role = WorkspaceRoleField(allow_none=True, missing=None)

    @post_load
    def make_obj(self, data):
        return WorkspaceEntry(**data)


workspace_entry_serializer = serializer_factory(WorkspaceEntrySchema)
