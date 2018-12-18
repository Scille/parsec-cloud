import os
import attr
import json
from typing import Optional, Dict
from pathlib import Path


def get_default_data_dir(environ):
    if os.name == "nt":
        return Path(environ["APPDATA"]) / "parsec/data"
    else:
        path = environ.get("XDG_DATA_HOME")
        if not path:
            path = f"{environ.get('HOME')}/.local/share"
        return Path(path) / "parsec"


def get_default_cache_dir(environ):
    if os.name == "nt":
        return Path(environ["APPDATA"]) / "parsec/cache"
    else:
        path = environ.get("XDG_CACHE_HOME")
        if not path:
            path = f"{environ.get('HOME')}/.cache"
        return Path(path) / "parsec"


def get_default_config_dir(environ):
    if os.name == "nt":
        return Path(environ["APPDATA"]) / "parsec/config"
    else:
        path = environ.get("XDG_CONFIG_HOME")
        if not path:
            path = f"{environ.get('HOME')}/.config"
        return Path(path) / "parsec"


@attr.s(slots=True, frozen=True, auto_attribs=True)
class CoreConfig:
    config_dir: Path
    data_dir: Path
    cache_dir: Path

    debug: bool = False
    backend_watchdog: int = 0
    backend_max_connections: int = 4

    invitation_token_size: int = 8

    sentry_url: Optional[str] = None


def config_factory(
    config_dir: Path = None,
    data_dir: Path = None,
    cache_dir: Path = None,
    backend_watchdog: int = 0,
    backend_max_connections: int = 4,
    debug: bool = False,
    environ: dict = {},
) -> CoreConfig:
    return CoreConfig(
        config_dir=config_dir or get_default_config_dir(environ),
        data_dir=data_dir or get_default_data_dir(environ),
        cache_dir=cache_dir or get_default_cache_dir(environ),
        debug=debug,
        backend_watchdog=backend_watchdog,
        sentry_url=environ.get("SENTRY_URL") or None,
    )


def load_config(config_dir: Path, **extra_config) -> CoreConfig:

    try:
        raw_conf = (config_dir / "config.json").read_text()
        data_conf = json.loads(raw_conf)
    except (ValueError, OSError, json.JSONDecodeError):
        # Config file not created yet, fallback to
        data_conf = {}

    try:
        data_conf["data_dir"] = Path(data_conf["data_dir"])
    except (KeyError, ValueError):
        pass

    try:
        data_conf["cache_dir"] = Path(data_conf["cache_dir"])
    except (KeyError, ValueError):
        pass

    return config_factory(config_dir=config_dir, **data_conf, **extra_config)


def save_config(config: CoreConfig):
    (config.config_dir / "config.json").write_text(
        json.dumps(
            {
                "data_dir": str(config.data_dir),
                "cache_dir": str(config.cache_dir),
                "backend_watchdog": config.backend_watchdog,
                "sentry_url": config.sentry_url,
            }
        )
    )
