# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

"""
Helper module used by `parsec.core.gui.custom_dialogs.QDialogInProcess`
to run the file selection dialog in a subprocess.
"""

import sys


def run_dialog(cls, method, *args, **kwargs):
    from PyQt5.QtWidgets import QApplication

    method = getattr(cls, method)
    if kwargs.pop("testing", None):
        return method.__name__, args, kwargs
    app = QApplication(sys.argv)  # noqa: F841
    return method(None, *args, **kwargs)
