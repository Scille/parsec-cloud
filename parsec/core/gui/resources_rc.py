# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

try:
    from parsec.core.gui._resources_rc import *  # noqa
except ImportError as exc:
    raise ModuleNotFoundError(
        """PyQt resources bundle hasn't been generated.
Running `python misc/generate_pyqt.py build` should fix the issue
"""
    ) from exc


if __name__ == "__main__":
    from PyQt5.QtCore import QDirIterator

    it = QDirIterator(":", QDirIterator.Subdirectories)
    while it.hasNext():
        print(it.next())
