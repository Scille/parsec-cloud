import os

from PyQt5.QtCore import QSettings

from parsec.core.gui.core_call import core_call


def identity(raw):
    return raw


ENTRIES = {
    "last_device": {"encode": identity, "decode": identity},
    "mountpoint": {"encode": identity, "decode": identity},
    "mountpoint_enabled": {
        "encode": lambda raw: "true" if raw else "false",
        "decode": lambda raw: raw.lower() == "true",
    },
}


def get_value(key, default=None):
    decoder = ENTRIES[key]["decode"]
    value = QSettings(
        os.path.join(core_call().get_config().base_settings_path, "parsec_gui.cfg"),
        QSettings.NativeFormat,
    ).value(key, default)
    if value is not None:
        value = decoder(value)
    return value


def set_value(key, value):
    encode = ENTRIES[key]["encode"]
    settings = QSettings(
        os.path.join(core_call().get_config().base_settings_path, "parsec_gui.cfg"),
        QSettings.NativeFormat,
    )
    settings.setValue(key, encode(value))
    settings.sync()
