# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .common import Enum, EnumItemUnit, Path, Structure, U32BasedType, Variant, VariantItemUnit


class CacheSize(U32BasedType):
    pass


class WorkspaceStorageCacheSize(Variant):
    class Default:
        pass

    class Custom:
        size: CacheSize


class MountpointMountStrategy(Variant):
    class Directory:
        base_dir: Path

    # Only allowed on Windows
    DriveLetter = VariantItemUnit()
    # Use this on web
    Disabled = VariantItemUnit()


class LogLevel(Enum):
    Error = EnumItemUnit()
    Warn = EnumItemUnit()
    Info = EnumItemUnit()
    Debug = EnumItemUnit()
    Trace = EnumItemUnit()


class ClientConfig(Structure):
    config_dir: Path
    data_base_dir: Path
    mountpoint_mount_strategy: MountpointMountStrategy
    workspace_storage_cache_size: WorkspaceStorageCacheSize
    with_monitors: bool
    prevent_sync_pattern: str | None
    log_level: LogLevel | None


def get_default_data_base_dir() -> Path:
    raise NotImplementedError


def get_default_config_dir() -> Path:
    raise NotImplementedError


def get_default_mountpoint_base_dir() -> Path:
    raise NotImplementedError
