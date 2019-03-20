# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os

from PyQt5.QtCore import QSettings

from parsec.core.config import get_default_config_dir


def identity(raw):
    return raw


ENTRIES = {
    "last_device": {"encode": identity, "decode": identity},
    "global/tray_enabled": {
        "encode": lambda raw: "true" if raw else "false",
        "decode": lambda raw: raw.lower() == "true",
    },
    "global/language": {"encode": identity, "decode": identity},
    "global/first_launch": {
        "encode": lambda raw: "true" if raw else "false",
        "decode": lambda raw: raw.lower() == "true",
    },
    "global/collect_data": {
        "encode": lambda raw: "true" if raw else "false",
        "decode": lambda raw: raw.lower() == "true",
    },
    "global/no_check_version": {
        "encode": lambda raw: "true" if raw else "false",
        "decode": lambda raw: raw.lower() == "true",
    },
    "network/proxy_type": {"encode": identity, "decode": identity},
    "network/proxy_host": {"encode": identity, "decode": identity},
    "network/proxy_username": {"encode": identity, "decode": identity},
    "network/proxy_port": {"encode": lambda raw: str(raw), "decode": lambda raw: int(raw)},
    "network/upload_limit": {"encode": lambda raw: str(raw), "decode": lambda raw: int(raw)},
    "network/download_limit": {"encode": lambda raw: str(raw), "decode": lambda raw: int(raw)},
    "network/simultaneous_connections": {
        "encode": lambda raw: str(raw),
        "decode": lambda raw: int(raw),
    },
}


def get_value(key, default=None):
    decoder = ENTRIES[key]["decode"]
    value = QSettings(
        os.path.join(get_default_config_dir(os.environ), "parsec_gui.cfg"), QSettings.IniFormat
    ).value(key, default)
    if value is not None:
        value = decoder(value)
    return value


def set_value(key, value):
    encode = ENTRIES[key]["encode"]
    settings = QSettings(
        os.path.join(get_default_config_dir(os.environ), "parsec_gui.cfg"), QSettings.IniFormat
    )
    settings.setValue(key, encode(value))
    settings.sync()
