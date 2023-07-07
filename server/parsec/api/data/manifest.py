# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from typing import TYPE_CHECKING, Type

from parsec._parsec import (
    BlockAccess,
    DeviceID,
    FileManifest,
    FolderManifest,
    UserManifest,
    WorkspaceEntry,
    WorkspaceManifest,
    manifest_decrypt_verify_and_load,
    manifest_verify_and_load,
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
    "manifest_decrypt_verify_and_load",
    "manifest_verify_and_load",
]
