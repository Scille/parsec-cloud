# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import attr
from typing import Optional, Tuple, Dict, Any, Type, TypeVar, TYPE_CHECKING
from libparsec.types import DateTime

from parsec.types import FrozenDict
from parsec.crypto import SecretKey, HashDigest
from parsec.serde import fields, validate, post_load, OneOfSchema, pre_load
from parsec.api.protocol import RealmRole, RealmRoleField, DeviceID, BlockID, BlockIDField
from parsec.api.data.base import (
    BaseData,
    BaseSchema,
    BaseAPISignedData,
    BaseSignedDataSchema,
    DataValidationError,
)
from parsec.api.data.entry import EntryID, EntryIDField, EntryName, EntryNameField
from enum import Enum


LOCAL_AUTHOR_LEGACY_PLACEHOLDER = DeviceID(
    "LOCAL_AUTHOR_LEGACY_PLACEHOLDER@LOCAL_AUTHOR_LEGACY_PLACEHOLDER"
)


class ManifestType(Enum):
    FILE_MANIFEST = "file_manifest"
    FOLDER_MANIFEST = "folder_manifest"
    WORKSPACE_MANIFEST = "workspace_manifest"
    USER_MANIFEST = "user_manifest"


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BlockAccess(BaseData):
    class SCHEMA_CLS(BaseSchema):
        id = BlockIDField(required=True)
        key = fields.SecretKey(required=True)
        offset = fields.Integer(required=True, validate=validate.Range(min=0))
        size = fields.Integer(required=True, validate=validate.Range(min=1))
        digest = fields.HashDigest(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "BlockAccess":
            return BlockAccess(**data)

    id: BlockID
    key: SecretKey
    offset: int
    size: int
    digest: HashDigest


_PyBlockAccess = BlockAccess
if not TYPE_CHECKING:
    try:
        from libparsec.types import BlockAccess as _RsBlockAccess
    except:
        pass
    else:
        BlockAccess = _RsBlockAccess


WorkspaceEntryTypeVar = TypeVar("WorkspaceEntryTypeVar", bound="WorkspaceEntry")


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class WorkspaceEntry(BaseData):
    class SCHEMA_CLS(BaseSchema):
        name = EntryNameField(required=True)
        id = EntryIDField(required=True)
        key = fields.SecretKey(required=True)
        encryption_revision = fields.Int(required=True, validate=validate.Range(min=0))
        encrypted_on = fields.DateTime(required=True)
        role_cached_on = fields.DateTime(required=True)
        role = RealmRoleField(required=True, allow_none=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "WorkspaceEntry":
            return WorkspaceEntry(**data)

    name: EntryName
    id: EntryID
    key: SecretKey
    encryption_revision: int
    encrypted_on: DateTime
    role_cached_on: DateTime
    role: Optional[RealmRole]

    @classmethod
    def new(
        cls: Type[WorkspaceEntryTypeVar], name: EntryName, timestamp: DateTime
    ) -> "WorkspaceEntry":
        assert isinstance(name, EntryName)
        return _PyWorkspaceEntry(
            name=name,
            id=EntryID.new(),
            key=SecretKey.generate(),
            encryption_revision=1,
            encrypted_on=timestamp,
            role_cached_on=timestamp,
            role=RealmRole.OWNER,
        )

    def is_revoked(self) -> bool:
        return self.role is None


_PyWorkspaceEntry = WorkspaceEntry
if not TYPE_CHECKING:
    try:
        from libparsec.types import WorkspaceEntry as _RsWorkspaceEntry
    except:
        pass
    else:
        WorkspaceEntry = _RsWorkspaceEntry


T = TypeVar("T")
BaseAPISignedDataTypeVar = TypeVar("BaseAPISignedDataTypeVar", bound="BaseAPISignedData")
BaseManifestTypeVar = TypeVar("BaseManifestTypeVar", bound="BaseManifest")


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BaseManifest(BaseAPISignedData):
    class SCHEMA_CLS(OneOfSchema, BaseSignedDataSchema):
        type_field = "type"
        version = fields.Integer(required=True, validate=validate.Range(min=0))
        id = EntryIDField(required=True)

        @property
        def type_schemas(self) -> Dict[ManifestType, Type[BaseSignedDataSchema]]:  # type: ignore[override]
            return {
                ManifestType.FILE_MANIFEST: _PyFileManifest.SCHEMA_CLS,
                ManifestType.FOLDER_MANIFEST: _PyFolderManifest.SCHEMA_CLS,
                ManifestType.WORKSPACE_MANIFEST: _PyWorkspaceManifest.SCHEMA_CLS,
                ManifestType.USER_MANIFEST: _PyUserManifest.SCHEMA_CLS,
            }

        def get_obj_type(self, obj: Dict[str, T]) -> T:
            return obj["type"]

    version: int
    id: EntryID

    @classmethod
    def verify_and_load(
        cls: Type[BaseManifestTypeVar],
        *args: object,
        expected_id: Optional[EntryID] = None,
        expected_version: Optional[int] = None,
        **kwargs: object,
    ) -> BaseManifestTypeVar:
        data = super().verify_and_load(*args, **kwargs)  # type: ignore[arg-type]
        if expected_id is not None and data.id != expected_id:
            raise DataValidationError(
                f"Invalid entry ID: expected `{expected_id}`, got `{data.id}`"
            )
        if expected_version is not None and data.version != expected_version:
            raise DataValidationError(
                f"Invalid version: expected `{expected_version}`, got `{data.version}`"
            )
        return data


FolderManifestTypeVar = TypeVar("FolderManifestTypeVar", bound="FolderManifest")


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class FolderManifest(BaseManifest):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.EnumCheckedConstant(ManifestType.FOLDER_MANIFEST, required=True)
        id = EntryIDField(required=True)
        parent = EntryIDField(required=True)
        # Version 0 means the data is not synchronized
        version = fields.Integer(required=True, validate=validate.Range(min=0))
        created = fields.DateTime(required=True)
        updated = fields.DateTime(required=True)
        children = fields.FrozenMap(EntryNameField(), EntryIDField(required=True), required=True)

        @pre_load
        def fix_legacy(self, data: Dict[str, Any]) -> Dict[str, Any]:
            # Compatibility with versions <= 1.14
            if data["author"] is None:
                data["author"] = str(LOCAL_AUTHOR_LEGACY_PLACEHOLDER)
            return data

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "FolderManifest":
            data.pop("type")
            return FolderManifest(**data)

    @classmethod
    def verify_and_load(  # type: ignore[override]
        cls: Type["FolderManifest"],
        *args: object,
        expected_parent: Optional[EntryID] = None,
        **kwargs: object,
    ) -> "FolderManifest":
        data = super().verify_and_load(*args, **kwargs)  # type: ignore[arg-type]
        if expected_parent is not None and data.parent != expected_parent:
            raise DataValidationError(
                f"Invalid parent ID: expected `{expected_parent}`, got `{data.parent}`"
            )
        return data

    id: EntryID
    parent: EntryID
    created: DateTime
    updated: DateTime
    children: FrozenDict[EntryName, EntryID]


_PyFolderManifest = FolderManifest
if not TYPE_CHECKING:
    try:
        from libparsec.types import FolderManifest as _RsFolderManifest
    except:
        pass
    else:
        FolderManifest = _RsFolderManifest


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class FileManifest(BaseManifest):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.EnumCheckedConstant(ManifestType.FILE_MANIFEST, required=True)
        id = EntryIDField(required=True)
        parent = EntryIDField(required=True)
        # Version 0 means the data is not synchronized
        version = fields.Integer(required=True, validate=validate.Range(min=0))
        created = fields.DateTime(required=True)
        updated = fields.DateTime(required=True)
        size = fields.Integer(required=True, validate=validate.Range(min=0))
        blocksize = fields.Integer(required=True, validate=validate.Range(min=8))
        blocks = fields.FrozenList(fields.Nested(_PyBlockAccess.SCHEMA_CLS), required=True)

        @pre_load
        def fix_legacy(self, data: Dict[str, T]) -> Dict[str, T]:
            # Compatibility with versions <= 1.14
            if data["author"] is None:
                data["author"] = str(LOCAL_AUTHOR_LEGACY_PLACEHOLDER)
            return data

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "FileManifest":
            data.pop("type")
            return FileManifest(**data)

    @classmethod
    def verify_and_load(  # type: ignore[override]
        cls: Type["FileManifest"],
        *args: object,
        expected_parent: Optional[EntryID] = None,
        **kwargs: object,
    ) -> "FileManifest":
        data = super().verify_and_load(*args, **kwargs)  # type: ignore[arg-type]
        if expected_parent is not None and data.parent != expected_parent:
            raise DataValidationError(
                f"Invalid parent ID: expected `{expected_parent}`, got `{data.parent}`"
            )
        return data

    id: EntryID
    parent: EntryID
    created: DateTime
    updated: DateTime
    size: int
    blocksize: int
    blocks: Tuple[BlockAccess]


_PyFileManifest = FileManifest
if not TYPE_CHECKING:
    try:
        from libparsec.types import FileManifest as _RsFileManifest
    except:
        pass
    else:
        FileManifest = _RsFileManifest


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class WorkspaceManifest(BaseManifest):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.EnumCheckedConstant(ManifestType.WORKSPACE_MANIFEST, required=True)
        id = EntryIDField(required=True)
        # Version 0 means the data is not synchronized
        version = fields.Integer(required=True, validate=validate.Range(min=0))
        created = fields.DateTime(required=True)
        updated = fields.DateTime(required=True)
        children = fields.FrozenMap(EntryNameField(), EntryIDField(required=True), required=True)

        @pre_load
        def fix_legacy(self, data: Dict[str, T]) -> Dict[str, T]:
            # Compatibility with versions <= 1.14
            if data["author"] is None:
                data["author"] = str(LOCAL_AUTHOR_LEGACY_PLACEHOLDER)
            return data

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "WorkspaceManifest":
            data.pop("type")
            return WorkspaceManifest(**data)

    id: EntryID
    created: DateTime
    updated: DateTime
    children: FrozenDict[EntryName, EntryID]


_PyWorkspaceManifest = WorkspaceManifest
if not TYPE_CHECKING:
    try:
        from libparsec.types import WorkspaceManifest as _RsWorkspaceManifest
    except:
        pass
    else:
        WorkspaceManifest = _RsWorkspaceManifest


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class UserManifest(BaseManifest):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.EnumCheckedConstant(ManifestType.USER_MANIFEST, required=True)
        id = EntryIDField(required=True)
        # Version 0 means the data is not synchronized
        version = fields.Integer(required=True, validate=validate.Range(min=0))
        created = fields.DateTime(required=True)
        updated = fields.DateTime(required=True)
        last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))
        workspaces = fields.List(fields.Nested(_PyWorkspaceEntry.SCHEMA_CLS), required=True)

        @pre_load
        def fix_legacy(self, data: Dict[str, T]) -> Dict[str, T]:
            # Compatibility with versions <= 1.14
            if data["author"] is None:
                data["author"] = str(LOCAL_AUTHOR_LEGACY_PLACEHOLDER)
            return data

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "UserManifest":
            data.pop("type")
            return UserManifest(**data)

    id: EntryID
    created: DateTime
    updated: DateTime
    last_processed_message: int
    workspaces: Tuple[WorkspaceEntry, ...] = attr.ib(converter=tuple)

    def get_workspace_entry(self, workspace_id: EntryID) -> Optional[WorkspaceEntry]:
        return next((w for w in self.workspaces if w.id == workspace_id), None)


_PyUserManifest = UserManifest
if not TYPE_CHECKING:
    try:
        from libparsec.types import UserManifest as _RsUserManifest
    except:
        pass
    else:
        UserManifest = _RsUserManifest
