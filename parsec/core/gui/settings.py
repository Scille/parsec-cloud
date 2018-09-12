from PyQt5.QtCore import QSettings


KEYS = [
    'mountpoint'
]


def get_value(key, default=None):
    if key not in KEYS:
        raise KeyError(key)
    return QSettings().value(key, default)


def set_value(key, value):
    if key not in KEYS:
        raise KeyError(key)
    QSettings().setValue(key, value)
