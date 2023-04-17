# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from PyQt5 import QtCore

from parsec._parsec import DateTime
from parsec.api.data import EntryID, EntryName
from parsec.core.gui.file_table import FileTable
from parsec.core.gui.lang import switch_language


@pytest.mark.gui
def test_file_table_parent_folder(qtbot, core_config):
    switch_language(core_config, "en")

    w = FileTable(parent=None)
    qtbot.add_widget(w)

    assert w.rowCount() == 0
    assert w.columnCount() == 5

    w.add_parent_folder()
    assert w.rowCount() == 1
    assert w.item(0, 1).text() == "Parent folder"


@pytest.mark.gui
def test_file_table_parent_workspace(qtbot, core_config):
    switch_language(core_config, "en")

    w = FileTable(parent=None)
    qtbot.add_widget(w)

    assert w.rowCount() == 0
    assert w.columnCount() == 5

    w.add_parent_workspace()
    assert w.rowCount() == 1
    assert w.item(0, 1).text() == "Workspaces list"


@pytest.mark.gui
def test_file_table_clear(qtbot):
    w = FileTable(parent=None)
    qtbot.add_widget(w)

    w.add_parent_workspace()
    assert w.rowCount() == 1
    w.clear()
    assert w.rowCount() == 0


@pytest.mark.gui
def test_file_table_sort(qtbot, core_config):
    switch_language(core_config, "en")

    w = FileTable(parent=None)
    qtbot.add_widget(w)
    w.add_parent_workspace()
    w.add_folder(EntryName("Dir1"), EntryID.new(), True, False)
    w.add_file(
        EntryName("File1.txt"),
        EntryID.new(),
        100,
        DateTime(2000, 1, 15),
        DateTime(2000, 1, 20),
        True,
        False,
    )
    w.add_file(
        EntryName("AnotherFile.txt"),
        EntryID.new(),
        80,
        DateTime(2000, 1, 10),
        DateTime(2000, 1, 25),
        True,
        False,
    )
    assert w.rowCount() == 4
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "Dir1"
    assert w.item(2, 1).text() == "File1.txt"
    assert w.item(3, 1).text() == "AnotherFile.txt"

    # Name
    w.sortByColumn(1, QtCore.Qt.AscendingOrder)
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "AnotherFile.txt"
    assert w.item(2, 1).text() == "Dir1"
    assert w.item(3, 1).text() == "File1.txt"

    w.sortByColumn(1, QtCore.Qt.DescendingOrder)
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "File1.txt"
    assert w.item(2, 1).text() == "Dir1"
    assert w.item(3, 1).text() == "AnotherFile.txt"

    # Created
    w.sortByColumn(2, QtCore.Qt.AscendingOrder)
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "Dir1"
    assert w.item(2, 1).text() == "AnotherFile.txt"
    assert w.item(3, 1).text() == "File1.txt"

    w.sortByColumn(2, QtCore.Qt.DescendingOrder)
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "File1.txt"
    assert w.item(2, 1).text() == "AnotherFile.txt"
    assert w.item(3, 1).text() == "Dir1"

    # Updated
    w.sortByColumn(3, QtCore.Qt.AscendingOrder)
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "Dir1"
    assert w.item(2, 1).text() == "File1.txt"
    assert w.item(3, 1).text() == "AnotherFile.txt"

    w.sortByColumn(3, QtCore.Qt.DescendingOrder)
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "AnotherFile.txt"
    assert w.item(2, 1).text() == "File1.txt"
    assert w.item(3, 1).text() == "Dir1"

    # Size
    w.sortByColumn(4, QtCore.Qt.AscendingOrder)
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "Dir1"
    assert w.item(2, 1).text() == "AnotherFile.txt"
    assert w.item(3, 1).text() == "File1.txt"

    w.sortByColumn(4, QtCore.Qt.DescendingOrder)
    assert w.item(0, 1).text() == "Workspaces list"
    assert w.item(1, 1).text() == "File1.txt"
    assert w.item(2, 1).text() == "AnotherFile.txt"
    assert w.item(3, 1).text() == "Dir1"
