# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

"""
Helper module used by `parsec.core.gui.custom_dialogs.QDialogInProcess`
to run the file selection dialog in a subprocess.
"""

import sys
import contextlib
import importlib_resources


def set_parsec_icon(app):
    from PyQt5.QtGui import QIcon, QPixmap

    # TODO: This end up importing the core and all the corresponding dependencies
    # This slows down the startup time of the first dialog, we might want to do better
    filename = "parsec.icns" if sys.platform == "darwin" else "parsec.ico"
    icon_data = importlib_resources.read_binary("parsec.core.resources", filename)
    pixmap = QPixmap()
    pixmap.loadFromData(icon_data)
    icon = QIcon(pixmap)
    app.setWindowIcon(icon)


@contextlib.contextmanager
def safe_app():
    from PyQt5.QtWidgets import QApplication
    from PyQt5.QtCore import QEventLoop

    app = QApplication(sys.argv)
    try:
        # The icon is already set properly for frozen executables.
        # Also, the icon is not set properly for native dialogs due to a bug in pyqt.
        # See the QFileDialog documentation for more information about native dialogs.
        # In the end, `set_parsec_icon` is mostly useful for linux since parsec in the
        # snap package is not frozen and explicitely disable the use of native widgets.
        frozen = getattr(sys, "frozen", False)
        if not frozen:
            set_parsec_icon(app)
        yield
    finally:
        # Exiting the app, necessary on macos
        app.exit()
        # Let the app process its last events
        app.processEvents(QEventLoop.AllEvents, 0)
        # Make sure we don't keep a reference to the app in this scope
        del app


def run_dialog(cls, method, *args, **kwargs):

    with safe_app():
        method = getattr(cls, method)
        # Bypass the actual dialog method, used for testing
        if kwargs.pop("testing", None):
            return method.__name__, args, kwargs
        # Run the dialog method with a `None` parent
        return method(None, *args, **kwargs)
