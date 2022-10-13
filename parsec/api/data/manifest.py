# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING, Type

from parsec.api.protocol import DeviceID
from enum import Enum
from parsec._parsec import (
    BlockAccess,
    WorkspaceEntry,
    FolderManifest,
    FileManifest,
    WorkspaceManifest,
    UserManifest,
    manifest_decrypt_and_load,
    manifest_verify_and_load,
    manifest_decrypt_verify_and_load,
    manifest_unverified_load,
)

if TYPE_CHECKING:
    from parsec._parsec import AnyRemoteManifest
else:
    AnyRemoteManifest = Type["AnyRemoteManifest"]

LOCAL_AUTHOR_LEGACY_PLACEHOLDER = DeviceID(
    "LOCAL_AUTHOR_LEGACY_PLACEHOLDER@LOCAL_AUTHOR_LEGACY_PLACEHOLDER"
)

__all__ = [
    "WorkspaceEntry",
    "FileManifest",
    "FolderManifest",
    "UserManifest",
    "WorkspaceManifest",
    "BlockAccess",
    "manifest_decrypt_and_load",
    "manifest_decrypt_verify_and_load",
    "manifest_verify_and_load",
    "manifest_unverified_load",
]


class ManifestType(Enum):
    FILE_MANIFEST = "file_manifest"
    FOLDER_MANIFEST = "folder_manifest"
    WORKSPACE_MANIFEST = "workspace_manifest"
    USER_MANIFEST = "user_manifest"
