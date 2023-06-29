# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import base64
import binascii
import json
import os
import sys
from pathlib import Path
from typing import Any, FrozenSet, Mapping

import attr
from structlog import get_logger

from parsec.api.data import EntryID
from parsec.core.types import BackendAddr

DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE = 512 * 1024 * 1024

logger = get_logger()


def get_default_data_base_dir(environ: Mapping[str, str]) -> Path:
    if sys.platform == "win32":
        return Path(environ["APPDATA"]) / "parsec/data"
    else:
        path = environ.get("XDG_DATA_HOME")
        if not path:
            path = f"{environ['HOME']}/.local/share"
        return Path(path) / "parsec"


def get_default_config_dir(environ: Mapping[str, str]) -> Path:
    if sys.platform == "win32":
        return Path(environ["APPDATA"]) / "parsec/config"
    else:
        path = environ.get("XDG_CONFIG_HOME")
        if not path:
            path = f"{environ.get('HOME')}/.config"
        return Path(path) / "parsec"


def get_default_mountpoint_base_dir(environ: Mapping[str, str]) -> Path:
    return Path.home() / "Parsec"


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True)
class CoreConfig:
    config_dir: Path
    data_base_dir: Path
    mountpoint_base_dir: Path
    prevent_sync_pattern_path: Path | None = None  # Use `default_pattern.ignore` by default
    preferred_org_creation_backend_addr: BackendAddr
    debug: bool = False

    backend_max_cooldown: int = 30
    backend_connection_keepalive: int | None = 29
    backend_max_connections: int = 4

    invitation_token_size: int = 8

    mountpoint_enabled: bool = False
    mountpoint_in_directory: bool = False
    # A personal workspace is one whose name matches a defined pattern (such as
    # "Drive"). These workspaces will be mounted in a specific directory
    # Base dir to mount personal workspaces
    personal_workspace_base_dir: Path | None = None
    # Naming pattern for personal workspaces
    personal_workspace_name_pattern: str | None = None
    disabled_workspaces: FrozenSet[EntryID] = frozenset()

    sentry_dsn: str | None = None
    sentry_environment: str = ""
    telemetry_enabled: bool = True
    workspace_storage_cache_size: int = DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE
    pki_extra_trust_roots: FrozenSet[Path] = frozenset()

    gui_last_device: str | None = None
    gui_tray_enabled: bool = True
    gui_language: str | None = None
    gui_first_launch: bool = True
    gui_last_version: str | None = None
    gui_check_version_at_startup: bool = True
    gui_check_version_url: str = "https://github.com/Scille/parsec-cloud/releases/latest"
    gui_check_version_api_url: str = "https://api.github.com/repos/Scille/parsec-cloud/releases"
    gui_check_version_allow_pre_release: bool = False
    gui_confirmation_before_close: bool = True
    gui_allow_multiple_instances: bool = False
    gui_show_confined: bool = False
    gui_geometry: bytes | None = None
    gui_hide_unmounted: bool = False

    ipc_win32_mutex_name: str = "parsec-cloud"

    def evolve(self, **kwargs: Any) -> CoreConfig:
        return attr.evolve(self, **kwargs)

    @property
    def ipc_socket_file(self) -> Path:
        return self.data_base_dir / "parsec-cloud.lock"


def config_factory(
    config_dir: Path | None = None,
    data_base_dir: Path | None = None,
    mountpoint_base_dir: Path | None = None,
    prevent_sync_pattern_path: Path | None = None,
    mountpoint_enabled: bool = False,
    mountpoint_in_directory: bool = False,
    personal_workspace_base_dir: Path | None = None,
    personal_workspace_name_pattern: str | None = None,
    disabled_workspaces: FrozenSet[EntryID] = frozenset(),
    backend_max_cooldown: int = 30,
    backend_connection_keepalive: int | None = 29,
    backend_max_connections: int = 4,
    sentry_dsn: str | None = None,
    sentry_environment: str = "",
    telemetry_enabled: bool = True,
    workspace_storage_cache_size: int = DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
    pki_extra_trust_roots: FrozenSet[Path] = frozenset(),
    debug: bool = False,
    gui_last_device: str | None = None,
    gui_tray_enabled: bool = True,
    gui_language: str | None = None,
    gui_first_launch: bool = True,
    gui_last_version: str | None = None,
    gui_check_version_at_startup: bool = True,
    gui_check_version_allow_pre_release: bool = False,
    gui_allow_multiple_instances: bool = False,
    preferred_org_creation_backend_addr: BackendAddr | None = None,
    gui_show_confined: bool = False,
    gui_geometry: bytes | None = None,
    gui_hide_unmounted: bool = False,
    ipc_win32_mutex_name: str = "parsec-cloud",
    environ: Mapping[str, str] = {},
    **_: object,
) -> CoreConfig:
    # The environment variable we always be used first, and if it is not present,
    # we'll use the value from the configuration file.
    backend_addr_env = environ.get("PREFERRED_ORG_CREATION_BACKEND_ADDR")
    if backend_addr_env:
        preferred_org_creation_backend_addr = BackendAddr.from_url(backend_addr_env)
    if not preferred_org_creation_backend_addr:
        preferred_org_creation_backend_addr = BackendAddr.from_url(
            "parsec://localhost:6777?no_ssl=true"
        )

    if mountpoint_base_dir is None:
        mountpoint_base_dir = get_default_mountpoint_base_dir(environ)

    data_base_dir = data_base_dir or get_default_data_base_dir(environ)
    core_config = CoreConfig(
        config_dir=config_dir or get_default_config_dir(environ),
        data_base_dir=data_base_dir,
        mountpoint_base_dir=mountpoint_base_dir,
        prevent_sync_pattern_path=prevent_sync_pattern_path,
        mountpoint_enabled=mountpoint_enabled,
        mountpoint_in_directory=mountpoint_in_directory,
        personal_workspace_base_dir=personal_workspace_base_dir,
        personal_workspace_name_pattern=personal_workspace_name_pattern,
        disabled_workspaces=disabled_workspaces,
        backend_max_cooldown=backend_max_cooldown,
        backend_connection_keepalive=backend_connection_keepalive,
        backend_max_connections=backend_max_connections,
        telemetry_enabled=telemetry_enabled,
        workspace_storage_cache_size=workspace_storage_cache_size,
        pki_extra_trust_roots=pki_extra_trust_roots,
        debug=debug,
        sentry_dsn=sentry_dsn,
        sentry_environment=sentry_environment,
        gui_last_device=gui_last_device,
        gui_tray_enabled=gui_tray_enabled,
        gui_language=gui_language,
        gui_first_launch=gui_first_launch,
        gui_last_version=gui_last_version,
        gui_check_version_at_startup=gui_check_version_at_startup,
        gui_check_version_allow_pre_release=gui_check_version_allow_pre_release,
        gui_allow_multiple_instances=gui_allow_multiple_instances,
        preferred_org_creation_backend_addr=preferred_org_creation_backend_addr,
        gui_show_confined=gui_show_confined,
        gui_geometry=gui_geometry,
        gui_hide_unmounted=gui_hide_unmounted,
        ipc_win32_mutex_name=ipc_win32_mutex_name,
    )

    # Make sure the directories exist on the system
    core_config.config_dir.mkdir(mode=0o700, parents=True, exist_ok=True)
    core_config.data_base_dir.mkdir(mode=0o700, parents=True, exist_ok=True)

    # Mountpoint base directory is not used on windows
    if sys.platform != "win32" or core_config.mountpoint_in_directory:
        core_config.mountpoint_base_dir.mkdir(mode=0o700, parents=True, exist_ok=True)

        if core_config.personal_workspace_base_dir:
            core_config.personal_workspace_base_dir.mkdir(mode=0o700, parents=True, exist_ok=True)

    return core_config


def load_config(config_dir: Path | None = None, **extra_config: Any) -> CoreConfig:
    if config_dir is None:
        config_dir = get_default_config_dir(os.environ)

    config_file = config_dir / "config.json"
    try:
        raw_conf = config_file.read_text()
        data_conf = json.loads(raw_conf)

    except OSError:
        # Config file not created yet, fallback to default
        data_conf = {}

    except (ValueError, json.JSONDecodeError) as exc:
        # Config file broken, fallback to default
        logger.warning("Ignoring invalid config", config_file=config_file, error=str(exc))
        data_conf = {}

    try:
        data_conf["data_base_dir"] = Path(data_conf["data_base_dir"])
    except (KeyError, ValueError):
        pass

    try:
        data_conf["prevent_sync_pattern_path"] = Path(data_conf["prevent_sync_pattern_path"])
    except (KeyError, ValueError):
        pass

    try:
        data_conf["disabled_workspaces"] = frozenset(
            map(EntryID.from_hex, data_conf["disabled_workspaces"])
        )
    except (KeyError, ValueError):
        pass

    try:
        extra_trust_roots_from_extra_config = list(extra_config.pop("pki_extra_trust_roots", []))
        extra_trust_roots_from_data_conf = list(data_conf.get("pki_extra_trust_roots", []))
        data_conf["pki_extra_trust_roots"] = frozenset(
            map(Path, extra_trust_roots_from_data_conf + extra_trust_roots_from_extra_config)
        )
    except (KeyError, ValueError):
        pass

    try:
        data_conf["preferred_org_creation_backend_addr"] = BackendAddr.from_url(
            data_conf["preferred_org_creation_backend_addr"]
        )
    except KeyError:
        pass
    except ValueError as exc:
        logger.warning("Invalid value for `preferred_org_creation_backend_addr`", error=str(exc))
        data_conf["preferred_org_creation_backend_addr"] = None

    try:
        data_conf["gui_geometry"] = base64.b64decode(data_conf["gui_geometry"].encode("ascii"))
    except (AttributeError, KeyError, UnicodeEncodeError, binascii.Error):
        data_conf["gui_geometry"] = None

    # Work around versioning issue with parsec releases:
    # - v1.12.0, v1.11.4, v1.11.3, v1.11.2, v1.11.1, v1.11.0 and v1.10.0
    # A `v` has been incorrectly added to `parsec.__version__`, potentially
    # affecting the `gui_last_version` entry in the configuration file.
    if data_conf.get("gui_last_version"):
        data_conf["gui_last_version"] = data_conf["gui_last_version"].lstrip("v")

    return config_factory(config_dir=config_dir, **data_conf, **extra_config, environ=os.environ)


def save_config(config: CoreConfig) -> None:
    config_path = config.config_dir
    config_path.mkdir(mode=0o700, parents=True, exist_ok=True)
    config_path /= "config.json"
    config_path.touch(exist_ok=True)
    config_path.write_text(
        json.dumps(
            {
                "data_base_dir": str(config.data_base_dir),
                "prevent_sync_pattern": str(config.prevent_sync_pattern_path),
                "telemetry_enabled": config.telemetry_enabled,
                "disabled_workspaces": list(map(lambda e: e.hex, config.disabled_workspaces)),
                "backend_max_cooldown": config.backend_max_cooldown,
                "backend_connection_keepalive": config.backend_connection_keepalive,
                "workspace_storage_cache_size": config.workspace_storage_cache_size,
                "pki_extra_trust_roots": list(map(str, config.pki_extra_trust_roots)),
                "gui_last_device": config.gui_last_device,
                "gui_tray_enabled": config.gui_tray_enabled,
                "gui_language": config.gui_language,
                "gui_first_launch": config.gui_first_launch,
                "gui_last_version": config.gui_last_version,
                "gui_check_version_at_startup": config.gui_check_version_at_startup,
                "gui_check_version_allow_pre_release": config.gui_check_version_allow_pre_release,
                "gui_allow_multiple_instances": config.gui_allow_multiple_instances,
                "preferred_org_creation_backend_addr": config.preferred_org_creation_backend_addr.to_url(),
                "gui_show_confined": config.gui_show_confined,
                "gui_geometry": base64.b64encode(config.gui_geometry).decode("ascii")
                if config.gui_geometry
                else None,
                "gui_hide_unmounted": config.gui_hide_unmounted,
                "ipc_win32_mutex_name": config.ipc_win32_mutex_name,
            },
            indent=True,
        )
    )
