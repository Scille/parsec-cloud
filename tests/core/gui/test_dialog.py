# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtPrintSupport import QPrinter
import pytest
import multiprocessing
from parsec._subprocess_dialog import PrintHelper
from parsec.core.gui.custom_dialogs import QDialogInProcess


@pytest.fixture
def close_process_pool():
    if multiprocessing.get_start_method(allow_none=True) is None:
        multiprocessing.set_start_method("spawn")
    assert multiprocessing.get_start_method() == "spawn"
    yield
    QDialogInProcess.pool.terminate()
    QDialogInProcess.pool.join()


def dump_printer(printer):
    result = {}
    for name in dir(printer):
        if name.startswith("_") or name[0].isupper():
            continue
        if name in ["printEngine", "paintEngine", "margins", "abort", "newPage"]:
            continue
        method = getattr(printer, name)
        try:
            result[name] = method()
        except TypeError:
            continue
    return result


def assert_equal_printers(a, b):
    assert dump_printer(a) == dump_printer(b)


@pytest.mark.gui
@pytest.mark.trio
async def test_file_dialog_in_process(gui, close_process_pool):
    assert QDialogInProcess.getOpenFileName(gui, "title", dir="dir", testing=True) == (
        "getOpenFileName",
        ("title",),
        {"dir": "dir"},
    )
    assert QDialogInProcess.getOpenFileNames(gui, "title", dir="dir", testing=True) == (
        "getOpenFileNames",
        ("title",),
        {"dir": "dir"},
    )
    assert QDialogInProcess.getSaveFileName(gui, "title", dir="dir", testing=True) == (
        "getSaveFileName",
        ("title",),
        {"dir": "dir"},
    )
    assert QDialogInProcess.getExistingDirectory(gui, "title", dir="dir", testing=True) == (
        "getExistingDirectory",
        ("title",),
        {"dir": "dir"},
    )

    printer = QDialogInProcess.get_printer(gui, testing_print=True)
    assert_equal_printers(printer, QPrinter(QPrinter.HighResolution))

    with pytest.raises(TypeError) as ctx:
        QDialogInProcess.getOpenFileName(gui, i_dont_exist=42)
    assert "'i_dont_exist' is not a valid keyword argument" in str(ctx.value)


@pytest.mark.gui
def test_print_helper():
    printer = QPrinter(QPrinter.HighResolution)
    for setter, values in [
        (printer.setCollateCopies, [False, True]),
        (printer.setColorMode, [0, 1]),
        (printer.setCopyCount, [1, 3]),
        (printer.setCreator, ["", "some_creator"]),
        (printer.setDocName, ["", "some_doc_name"]),
        (printer.setDoubleSidedPrinting, [False, True]),
        (printer.setDuplex, [0, 1]),
        (printer.setFontEmbeddingEnabled, [False, True]),
        (printer.setFullPage, [False, True]),
        (printer.setOrientation, [0, 1]),
        (printer.setOutputFileName, ["", "some_path/some_file.pdf"]),
        (printer.setOutputFormat, [0, 1]),
        (printer.setPageOrder, [0, 1]),
        (printer.setPaperName, ["A4", "A5"]),
        (printer.setPaperSize, [0, 1]),
        (printer.setPaperSource, [6, 4]),
        (printer.setPdfVersion, [0, 1]),
        (printer.setPrintProgram, ["", "some_program"]),
        (printer.setPrintRange, [0, 1]),
        (printer.setPrinterName, ["", "some_printer"]),
        (printer.setResolution, [1200, 1000]),
        (printer.setPageOrientation, [0, 1]),
        (printer.setPageSize, [0, 1]),
        (printer.setPageMargins, [(0, 0, 0, 0, 0), (1, 2, 3, 4, 5)]),
        (printer.setFromTo, [(0, 0), (2, 8)]),
    ]:
        for arg in values:
            setter(*arg) if isinstance(arg, tuple) else setter(arg)
            assert_equal_printers(
                PrintHelper.dict_to_printer(PrintHelper.printer_to_dict(printer)), printer
            )
