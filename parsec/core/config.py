import os
import attr
from typing import Optional
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

    sentry_url: Optional[str] = None


def config_factory(
    config_dir=None,
    data_dir=None,
    cache_dir=None,
    backend_watchdog=0,
    backend_max_connections=4,
    debug=False,
    environ={},
):
    return CoreConfig(
        config_dir=config_dir or get_default_config_dir(environ),
        data_dir=data_dir or get_default_data_dir(environ),
        cache_dir=cache_dir or get_default_cache_dir(environ),
        debug=debug,
        backend_watchdog=backend_watchdog,
        sentry_url=environ.get("SENTRY_URL") or None,
    )
