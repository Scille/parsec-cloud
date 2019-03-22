# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import attr
import json
from typing import Optional
from pathlib import Path
from structlog import get_logger


logger = get_logger()


def get_default_data_base_dir(environ: dict):
    if os.name == "nt":
        return Path(environ["APPDATA"]) / "parsec/data"
    else:
        path = environ.get("XDG_DATA_HOME")
        if not path:
            path = f"{environ['HOME']}/.local/share"
        return Path(path) / "parsec"


def get_default_cache_base_dir(environ: dict):
    if os.name == "nt":
        return Path(environ["APPDATA"]) / "parsec/cache"
    else:
        path = environ.get("XDG_CACHE_HOME")
        if not path:
            path = f"{environ.get('HOME')}/.cache"
        return Path(path) / "parsec"


def get_default_config_dir(environ: dict):
    if os.name == "nt":
        return Path(environ["APPDATA"]) / "parsec/config"
    else:
        path = environ.get("XDG_CONFIG_HOME")
        if not path:
            path = f"{environ.get('HOME')}/.config"
        return Path(path) / "parsec"


def get_default_mountpoint_base_dir(environ: dict):
    return Path.home() / "parsec_mnt"


@attr.s(slots=True, frozen=True, auto_attribs=True)
class GuiConfig:
    last_device: Optional[str] = None
    tray_enabled: bool = True
    language: Optional[str] = "en"
    first_launch: bool = True
    check_version: bool = True
    sentry_logging: bool = True

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)


def gui_config_factory(
    last_device: str = None,
    tray_enabled: bool = True,
    language: str = "en",
    first_launch: bool = True,
    check_version: bool = True,
    sentry_logging: bool = True,
) -> GuiConfig:
    return GuiConfig(
        last_device=last_device,
        tray_enabled=tray_enabled,
        language=language,
        first_launch=first_launch,
        check_version=check_version,
        sentry_logging=sentry_logging,
    )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class CoreConfig:
    config_dir: Path
    data_base_dir: Path
    cache_base_dir: Path
    mountpoint_base_dir: Path

    debug: bool = False
    backend_watchdog: int = 0
    backend_max_connections: int = 4

    invitation_token_size: int = 8

    mountpoint_enabled: bool = False

    sentry_url: Optional[str] = None

    ssl_keyfile: Optional[str] = None
    ssl_certfile: Optional[str] = None

    gui: GuiConfig = GuiConfig()

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)


def config_factory(
    config_dir: Path = None,
    data_base_dir: Path = None,
    cache_base_dir: Path = None,
    mountpoint_base_dir: Path = None,
    mountpoint_enabled: bool = False,
    backend_watchdog: int = 0,
    backend_max_connections: int = 4,
    debug: bool = False,
    ssl_keyfile: str = None,
    ssl_certfile: str = None,
    gui: GuiConfig = GuiConfig(),
    environ: dict = {},
) -> CoreConfig:
    return CoreConfig(
        config_dir=config_dir or get_default_config_dir(environ),
        data_base_dir=data_base_dir or get_default_data_base_dir(environ),
        cache_base_dir=cache_base_dir or get_default_cache_base_dir(environ),
        mountpoint_base_dir=mountpoint_base_dir or get_default_mountpoint_base_dir(environ),
        debug=debug,
        backend_watchdog=backend_watchdog,
        ssl_keyfile=ssl_keyfile,
        ssl_certfile=ssl_certfile,
        sentry_url=environ.get("SENTRY_URL") or None,
        gui=gui,
    )


def load_config(config_dir: Path, **extra_config) -> CoreConfig:

    config_file = config_dir / "config.json"
    try:
        raw_conf = config_file.read_text()
        data_conf = json.loads(raw_conf)

    except OSError:
        # Config file not created yet, fallback to default
        data_conf = {}

    except (ValueError, json.JSONDecodeError) as exc:
        # Config file broken, fallback to default
        logger.warning(f"Ignoring invalid config in {config_file} ({exc})")
        data_conf = {}

    try:
        data_conf["data_base_dir"] = Path(data_conf["data_base_dir"])
    except (KeyError, ValueError):
        pass

    try:
        data_conf["cache_base_dir"] = Path(data_conf["cache_base_dir"])
    except (KeyError, ValueError):
        pass

    try:
        data_conf["mountpoint_base_dir"] = Path(data_conf["mountpoint_base_dir"])
    except (KeyError, ValueError):
        pass

    return config_factory(
        config_dir=config_dir,
        gui=gui_config_factory(**data_conf.pop("gui", {})),
        **data_conf,
        **extra_config,
        environ=os.environ,
    )


def reload_config(config: CoreConfig) -> CoreConfig:
    return load_config(config.config_dir, debug=config.debug)


def save_config(config: CoreConfig):
    config_path = config.config_dir
    config_path.mkdir(parents=True, exist_ok=True)
    config_path /= "config.json"
    config_path.touch(exist_ok=True)
    config_path.write_text(
        json.dumps(
            {
                "data_base_dir": str(config.data_base_dir),
                "cache_base_dir": str(config.cache_base_dir),
                "mountpoint_base_dir": str(config.mountpoint_base_dir),
                "backend_watchdog": config.backend_watchdog,
                "gui": {
                    "last_device": config.gui.last_device,
                    "tray_enabled": config.gui.tray_enabled,
                    "language": config.gui.language,
                    "first_launch": config.gui.first_launch,
                    "check_version": config.gui.check_version,
                    "sentry_logging": config.gui.sentry_logging,
                },
            }
        )
    )
