from .common import *


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
    preferred_org_creation_backend_addr: BackendAddr
    workspace_storage_cache_size: WorkspaceStorageCacheSize
