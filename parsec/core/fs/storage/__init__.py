# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.core.fs.storage.user_storage import UserStorage
from parsec.core.fs.storage.workspace_storage import (
    AnyWorkspaceStorage,
    WorkspaceStorage,
    WorkspaceStorageSnapshot,
)

__all__ = (
    "UserStorage",
    "AnyWorkspaceStorage",
    "WorkspaceStorage",
    "WorkspaceStorageSnapshot",
)
