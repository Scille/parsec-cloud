# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING, Type

from parsec._parsec import (
    BlockAccess,
    FileManifest,
    FolderManifest,
    UserManifest,
    WorkspaceEntry,
    WorkspaceManifest,
    manifest_decrypt_and_load,
    manifest_decrypt_verify_and_load,
    manifest_unverified_load,
    manifest_verify_and_load,
)
from parsec.api.protocol import DeviceID

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
