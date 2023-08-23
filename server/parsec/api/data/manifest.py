# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING, Type

from parsec._parsec import (
    BlockAccess,
    DeviceID,
    FileManifest,
    FolderManifest,
    UserManifest,
    WorkspaceEntry,
    WorkspaceManifest,
    child_manifest_decrypt_verify_and_load,
    child_manifest_verify_and_load,
)

if TYPE_CHECKING:
    from parsec._parsec import ChildManifest
else:
    ChildManifest = Type["ChildManifest"]

LOCAL_AUTHOR_LEGACY_PLACEHOLDER = DeviceID(
    "LOCAL_AUTHOR_LEGACY_PLACEHOLDER@LOCAL_AUTHOR_LEGACY_PLACEHOLDER"
)

__all__ = [
    "WorkspaceEntry",
    "FileManifest",
    "FolderManifest",
    "UserManifest",
    "WorkspaceManifest",
    "ChildManifest",
    "BlockAccess",
    "child_manifest_decrypt_verify_and_load",
    "child_manifest_verify_and_load",
]
