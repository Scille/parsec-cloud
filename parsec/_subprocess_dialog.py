# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Any, Iterator, Type, cast

from PyQt5.QtWidgets import QApplication, QWidget

"""
Helper module used by `parsec.core.gui.custom_dialogs.QDialogInProcess`
to run the file selection dialog in a subprocess.
"""

import sys
import contextlib
import importlib.resources
import functools


@functools.lru_cache()
def get_parsec_icon_data() -> bytes:
    filename = "parsec.icns" if sys.platform == "darwin" else "parsec.ico"
    return importlib.resources.files("parsec.core.resources").joinpath(filename).read_bytes()


def set_parsec_icon(app: QApplication) -> None:
    from PyQt5.QtGui import QIcon, QPixmap

    icon_data = get_parsec_icon_data()
    pixmap = QPixmap()
    pixmap.loadFromData(icon_data)
    icon = QIcon(pixmap)
    app.setWindowIcon(icon)


class PrintHelper:
    @classmethod
    def print_html(cls, parent: QWidget, html: str) -> int:
        from PyQt5.QtPrintSupport import QPrinter, QPrintDialog
        from PyQt5.QtGui import QTextDocument

        printer = QPrinter(QPrinter.HighResolution)
        dialog = QPrintDialog(printer, parent)
        result = dialog.exec_()
        if result == dialog.Accepted:
            doc = QTextDocument()
            doc.setHtml(html)
            doc.print_(printer)
        return result


@contextlib.contextmanager
def safe_app() -> Iterator[None]:
    from PyQt5.QtWidgets import QApplication
    from PyQt5.QtCore import QEventLoop

    app = QApplication(sys.argv)
    try:
        # The icon is already set properly for frozen executables.
        # Also, the icon is not set properly for native dialogs due to a bug in pyqt.
        # See the QFileDialog documentation for more information about native dialogs.
        # In the end, `set_parsec_icon` is mostly useful for linux since parsec in the
        # snap package is not frozen and explicitly disable the use of native widgets.
        frozen = getattr(sys, "frozen", False)
        if not frozen:
            set_parsec_icon(app)

        yield

    finally:
        # Exiting the app, necessary on macos
        app.exit()
        # Let the app process its last events
        QApplication.processEvents(QEventLoop.AllEvents, cast(QEventLoop.ProcessEventsFlag, 0))
        # Make sure we don't keep a reference to the app in this scope
        del app


def load_resources(with_printer: bool = False) -> None:

    # Loading resources require an application
    with safe_app():

        # First printer instantiation might take a long time on windows
        # when network printers are involved. See the bug report:
        # https://bugreports.qt.io/browse/QTBUG-49560
        if with_printer:
            from PyQt5.QtPrintSupport import QPrinter

            QPrinter(QPrinter.HighResolution)


def run_dialog(
    sub_cls: Type[Any],
    method_name: str,
    *args: Any,
    **kwargs: Any,
) -> Any:
    with safe_app():
        method = getattr(sub_cls, method_name)
        # Bypass the actual dialog method, used for testing
        if kwargs.pop("testing", None):
            return method.__name__, args, kwargs
        # Run the dialog method with a `None` parent
        return method(None, *args, **kwargs)
