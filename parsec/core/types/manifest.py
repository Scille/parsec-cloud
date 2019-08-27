# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from typing import Optional, Tuple
from pendulum import Pendulum, now as pendulum_now

from parsec.types import DeviceID
from parsec.serde import fields, OneOfSchema, validate, post_load
from parsec.api.data import UserManifest, BaseSchema, BaseData
from parsec.core.types import WorkspaceEntry, WorkspaceEntrySchema, EntryID, EntryIDField


class LocalUserManifestSchema(BaseSchema):
    type = fields.CheckedConstant("local_user_manifest", required=True)
    base = fields.Nested(UserManifest.SCHEMA_CLS, required=True, allow_none=True)
    id = EntryIDField(required=True)
    need_sync = fields.Boolean(required=True)
    updated = fields.DateTime(required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))
    workspaces = fields.FrozenList(fields.Nested(WorkspaceEntrySchema), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        return LocalUserManifest(**data)


class LocalManifest(BaseData):
    class SCHEMA_CLS(OneOfSchema):
        type_field = "type"
        type_field_remove = False
        type_schemas = {
            "local_user_manifest": LocalUserManifestSchema,
            # "local_workspace_manifest": LocalWorkspaceManifestSchema,
            # "local_file_manifest": LocalFileManifestSchema,
            # "local_folder_manifest": LocalFolderManifestSchema,
        }

        def get_obj_type(self, obj):
            return obj["type"]


class LocalUserManifest(LocalManifest):
    SCHEMA_CLS = LocalUserManifestSchema

    base: Optional[UserManifest]
    id: EntryID
    need_sync: bool
    updated: Pendulum
    last_processed_message: int
    workspaces: Tuple[WorkspaceEntry, ...]

    @classmethod
    def new_placeholder(cls, id: EntryID) -> "LocalUserManifest":
        return cls(
            base=None,
            id=id,
            need_sync=True,
            updated=pendulum_now(),
            last_processed_message=0,
            workspaces=(),
        )

    @property
    def created(self):
        return self.base.created if self.base else self.updated

    @property
    def base_version(self):
        return self.base.version if self.base else 0

    @property
    def is_placeholder(self):
        return self.base is None

    @classmethod
    def from_remote(cls, remote: UserManifest) -> "LocalUserManifest":
        return cls(
            base=remote,
            id=remote.id,
            need_sync=False,
            updated=remote.updated,
            last_processed_message=remote.last_processed_message,
            workspaces=remote.workspaces,
        )

    def to_remote(self, author: DeviceID, timestamp: Pendulum) -> UserManifest:
        return UserManifest(
            author=author,
            timestamp=timestamp,
            id=self.id,
            version=self.base_version + 1,
            created=self.created,
            updated=self.updated,
            last_processed_message=self.last_processed_message,
            workspaces=self.workspaces,
        )

    def get_workspace_entry(self, workspace_id: EntryID) -> WorkspaceEntry:
        return next((w for w in self.workspaces if w.id == workspace_id), None)

    def evolve_and_mark_updated(self, **data) -> "LocalUserManifest":
        if "updated" not in data:
            data["updated"] = pendulum_now()
        data.setdefault("need_sync", True)
        return attr.evolve(self, **data)

    def evolve(self, **data) -> "LocalUserManifest":
        return attr.evolve(self, **data)

    def evolve_workspaces_and_mark_updated(self, *data) -> "LocalUserManifest":
        workspaces = {**{w.id: w for w in self.workspaces}, **{w.id: w for w in data}}
        return self.evolve_and_mark_updated(workspaces=tuple(workspaces.values()))

    def evolve_workspaces(self, *data) -> "LocalUserManifest":
        workspaces = {**{w.id: w for w in self.workspaces}, **{w.id: w for w in data}}
        return self.evolve(workspaces=tuple(workspaces.values()))
