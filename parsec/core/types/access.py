# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from uuid import uuid4
from hashlib import sha256
from typing import Union, Optional

from parsec.types import UserID
from parsec.api.protocole import RealmRole, RealmRoleField
from parsec.crypto import SymetricKey, HashDigest, generate_secret_key
from parsec.serde import UnknownCheckedSchema, fields, validate, post_load
from parsec.core.types.base import AccessID, serializer_factory, EntryNameField


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserAccess:
    id: AccessID
    key: SymetricKey

    @classmethod
    def from_block(cls, user_id: UserID, key: SymetricKey) -> "UserAccess":
        access_id = sha256(user_id.encode("utf8")).hexdigest()
        return cls(id=access_id, key=key)


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


Access = Union[UserAccess, ManifestAccess, BlockAccess, DirtyBlockAccess]


# Republishing under a better name
WorkspaceRole = RealmRole
WorkspaceRoleField = RealmRoleField


# Not stricly speaking an access, but close enough...


@attr.s(slots=True, frozen=True, auto_attribs=True)
class WorkspaceEntry:
    name: str
    access: ManifestAccess = attr.ib(factory=ManifestAccess)
    encryption_revision: int = 1
    granted_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)
    role: Optional[WorkspaceRole] = WorkspaceRole.OWNER

    def is_revoked(self) -> bool:
        return self.role is None

    def evolve(self, **kwargs) -> "WorkspaceEntry":
        return attr.evolve(self, **kwargs)

    def evolve_and_mark_updated(self, **data) -> "WorkspaceEntry":
        if "granted_on" not in data:
            data["granted_on"] = pendulum.now()
        return attr.evolve(self, **data)


class WorkspaceEntrySchema(UnknownCheckedSchema):
    name = EntryNameField(validate=validate.Length(min=1, max=256), required=True)
    access = fields.Nested(ManifestAccessSchema, required=True)
    encryption_revision = fields.Int(required=True, validate=validate.Range(min=0))
    granted_on = fields.DateTime(required=True)
    role = WorkspaceRoleField(allow_none=True, missing=None)

    @post_load
    def make_obj(self, data):
        return WorkspaceEntry(**data)


workspace_entry_serializer = serializer_factory(WorkspaceEntrySchema)
