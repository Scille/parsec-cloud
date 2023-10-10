# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import Path, Structure, U32BasedType, Variant


class CacheSize(U32BasedType):
    pass


class WorkspaceStorageCacheSize(Variant):
    class Default:
        pass

    class Custom:
        size: CacheSize


class ClientConfig(Structure):
    config_dir: Path
    data_base_dir: Path
    mountpoint_base_dir: Path
    workspace_storage_cache_size: WorkspaceStorageCacheSize


def get_default_data_base_dir() -> Path:
    raise NotImplementedError


def get_default_config_dir() -> Path:
    raise NotImplementedError


def get_default_mountpoint_base_dir() -> Path:
    raise NotImplementedError
