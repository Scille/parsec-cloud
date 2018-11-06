import os

from PyQt5.QtCore import QSettings

from parsec.core.gui.core_call import core_call


KEYS = ["mountpoint"]


def get_value(key, default=None):
    if key not in KEYS:
        raise KeyError(key)
    return QSettings(
        os.path.join(core_call().get_config().base_settings_path, "parsec_gui.cfg"),
        QSettings.NativeFormat,
    ).value(key, default)


def set_value(key, value):
    if key not in KEYS:
        raise KeyError(key)
    settings = QSettings(
        os.path.join(core_call().get_config().base_settings_path, "parsec_gui.cfg"),
        QSettings.NativeFormat,
    )
    settings.setValue(key, value)
    settings.sync()
