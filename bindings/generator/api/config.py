# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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
