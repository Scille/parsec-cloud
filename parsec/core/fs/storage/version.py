# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path

from parsec.core.types import LocalDevice, EntryID


STORAGE_REVISION = 1
USER_STORAGE_NAME = f"user_data-v{STORAGE_REVISION}.sqlite"
WORKSPACE_DATA_STORAGE_NAME = f"workspace_data-v{STORAGE_REVISION}.sqlite"
WORKSPACE_CACHE_STORAGE_NAME = f"workspace_cache-v{STORAGE_REVISION}.sqlite"


def get_user_data_storage_db_path(data_base_dir: Path, device: LocalDevice) -> Path:
    return data_base_dir / device.slug / USER_STORAGE_NAME


def get_workspace_data_storage_db_path(
    data_base_dir: Path, device: LocalDevice, workspace_id: EntryID
) -> Path:
    return data_base_dir / device.slug / workspace_id.str / WORKSPACE_DATA_STORAGE_NAME


def get_workspace_cache_storage_db_path(
    data_base_dir: Path, device: LocalDevice, workspace_id: EntryID
) -> Path:
    return data_base_dir / device.slug / workspace_id.str / WORKSPACE_CACHE_STORAGE_NAME
