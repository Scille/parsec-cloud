# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .common import (
    Enum,
    EnumItemUnit,
    Path,
    Structure,
    U32BasedType,
    Variant,
    VariantItemUnit,
    ErrorVariant,
    Ref,
    Result,
)
from .addr import ParsecAddr


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


class ClientAgentConfig(Variant):
    NativeOnly = VariantItemUnit()
    NativeOrWeb = VariantItemUnit()


class AccountConfig(Variant):
    Disabled = VariantItemUnit()
    EnabledWithVault = VariantItemUnit()
    EnabledWithoutVault = VariantItemUnit()


class OrganizationBootstrapConfig(Variant):
    WithBootstrapToken = VariantItemUnit()
    Spontaneous = VariantItemUnit()


class OpenBaoSecretConfig(Variant):
    class KV2:
        mount_path: str


class OpenBaoAuthConfig(Variant):
    class OIDCHexagone:
        mount_path: str

    class OIDCProConnect:
        mount_path: str


class OpenBaoConfig(Structure):
    server_url: str
    secret: OpenBaoSecretConfig
    auths: list[OpenBaoAuthConfig]


class ServerConfig(Structure):
    client_agent: ClientAgentConfig
    account: AccountConfig
    organization_bootstrap: OrganizationBootstrapConfig
    openbao: OpenBaoConfig | None


class GetServerConfigError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def get_server_config(
    config_dir: Ref[Path],
    addr: ParsecAddr,
) -> Result[ServerConfig, GetServerConfigError]:
    raise NotImplementedError
