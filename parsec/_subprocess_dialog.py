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


class PrintHelper:
    @classmethod
    def get_printer(cls, parent, testing_print=False):
        from PyQt5.QtPrintSupport import QPrinter, QPrintDialog

        printer = QPrinter(QPrinter.HighResolution)
        dialog = QPrintDialog(printer, parent)
        result = dialog.Accepted if testing_print else dialog.exec_()
        return None if result != dialog.Accepted else cls.printer_to_dict(printer)

    @classmethod
    def printer_to_dict(cls, printer):
        from PyQt5.QtPrintSupport import QPrinter

        result = {}
        setters = [
            "setCollateCopies",
            "setColorMode",
            "setCopyCount",
            "setCreator",
            "setDocName",
            "setDoubleSidedPrinting",
            "setDuplex",
            "setFontEmbeddingEnabled",
            "setFullPage",
            "setOrientation",
            "setOutputFileName",
            "setOutputFormat",
            "setPageOrder",
            "setPaperName",
            "setPaperSize",
            "setPaperSource",
            "setPdfVersion",
            "setPrintProgram",
            "setPrintRange",
            "setPrinterName",
            "setPrinterSelectionOption",
            "setResolution",
        ]

        # Prepare generic setters
        for name in setters:
            getter = name[3].lower() + name[4:]
            value = getattr(printer, getter)()
            result[name] = ((type(value), value),)

        # Prepare layout setters
        layout = printer.pageLayout()
        page_size = layout.pageSize()
        margins = layout.margins()
        orientation = layout.orientation()
        units = layout.units()
        result["setPageOrientation"] = ((type(orientation), orientation),)
        result["setPageSize"] = ((type(page_size), page_size.sizePoints(), page_size.name()),)
        result["setPageMargins"] = (
            (float, margins.left()),
            (float, margins.top()),
            (float, margins.right()),
            (float, margins.bottom()),
            (QPrinter.Unit, units),
        )

        # Prepare from to setters
        result["setFromTo"] = (int, printer.fromPage()), (int, printer.toPage())
        return result

    @classmethod
    def dict_to_printer(cls, dct):
        from PyQt5.QtPrintSupport import QPrinter

        printer = QPrinter(QPrinter.HighResolution)
        for method, args_info in dct.items():
            args = [arg_type(*subargs) for arg_type, *subargs in args_info]
            getattr(printer, method)(*args)
        return printer


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
