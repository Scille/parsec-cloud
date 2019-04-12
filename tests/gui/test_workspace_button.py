# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.gui.workspace_button import WorkspaceButton


@pytest.mark.gui
def test_workspace_button(qtbot):

    w = WorkspaceButton(
        workspace_name="Workspace1",
        is_owner=True,
        creator="Tester",
        files={},
        shared_with=[],
        parent=None,
    )
    qtbot.addWidget(w)
    w.show()

    assert w.label_workspace.text() == "Workspace1"
    assert w.participants == []
    assert w.label_empty.isVisible() is True
    assert w.widget_files.isVisible() is False


@pytest.mark.gui
def test_workspace_button_shared_by(qtbot):

    w = WorkspaceButton(
        workspace_name="Workspace1",
        is_owner=False,
        creator="Tester",
        files={},
        shared_with=["Me"],
        parent=None,
    )
    qtbot.addWidget(w)
    w.show()
    assert w.label_workspace.text() == "Workspace1 (shared by Tester)"
    assert w.label_empty.isVisible() is True
    assert w.widget_files.isVisible() is False


@pytest.mark.gui
def test_workspace_button_shared_with(qtbot):

    w = WorkspaceButton(
        workspace_name="Workspace1",
        is_owner=True,
        creator="Tester",
        files={},
        shared_with=["Other"],
        parent=None,
    )
    qtbot.addWidget(w)
    w.show()
    assert w.label_workspace.text() == "Workspace1 (shared)"
    assert w.label_empty.isVisible() is True
    assert w.widget_files.isVisible() is False


@pytest.mark.gui
def test_workspace_button_long_name(qtbot):
    w = WorkspaceButton(
        workspace_name="A_Very_Long_Workspace_Name_To_Check_If_It_Is_Shortened",
        is_owner=True,
        creator="Tester",
        files={},
        shared_with=[],
        parent=None,
    )
    qtbot.addWidget(w)
    w.show()

    assert w.label_workspace.text() == "A_Very_Long_Workspac..."
    assert w.label_empty.isVisible() is True
    assert w.widget_files.isVisible() is False


@pytest.mark.gui
def test_workspace_button_files(qtbot):
    w = WorkspaceButton(
        workspace_name="Workspace1",
        is_owner=True,
        creator="Tester",
        files={"File1.txt": False, "File2.txt": False, "Dir1": True},
        shared_with=[],
        parent=None,
    )
    qtbot.addWidget(w)
    w.show()

    assert w.label_workspace.text() == "Workspace1"
    assert w.label_empty.isVisible() is False
    assert w.widget_files.isVisible() is True
    assert w.line_edit_file1.text() == "File1.txt"
    assert w.line_edit_file2.text() == "File2.txt"
    assert w.line_edit_file3.text() == "Dir1"
    assert w.line_edit_file4.text() == ""

    with qtbot.waitSignal(w.file_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.line_edit_file1, QtCore.Qt.LeftButton)
    assert blocker.args == ["Workspace1", "File1.txt", False]

    with qtbot.waitSignal(w.file_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.line_edit_file2, QtCore.Qt.LeftButton)
    assert blocker.args == ["Workspace1", "File2.txt", False]

    with qtbot.waitSignal(w.file_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.line_edit_file3, QtCore.Qt.LeftButton)
    assert blocker.args == ["Workspace1", "Dir1", True]


@pytest.mark.gui
def test_workspace_button_clicked(qtbot):
    w = WorkspaceButton(
        workspace_name="Workspace1",
        is_owner=True,
        creator="Tester",
        files={},
        shared_with=[],
        parent=None,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.clicked, timeout=500) as blocker:
        qtbot.mouseClick(w, QtCore.Qt.LeftButton)
    assert blocker.args == ["Workspace1"]


@pytest.mark.gui
def test_workspace_button_share_clicked(qtbot):
    w = WorkspaceButton(
        workspace_name="Workspace1",
        is_owner=True,
        creator="Tester",
        files={},
        shared_with=[],
        parent=None,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.share_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_share, QtCore.Qt.LeftButton)
    assert blocker.args == [w]


@pytest.mark.gui
def test_workspace_button_rename_clicked(qtbot):
    w = WorkspaceButton(
        workspace_name="Workspace1",
        is_owner=True,
        creator="Tester",
        files={},
        shared_with=[],
        parent=None,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.rename_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_rename, QtCore.Qt.LeftButton)
    assert blocker.args == [w]


@pytest.mark.gui
def test_workspace_button_delete_clicked(qtbot):
    w = WorkspaceButton(
        workspace_name="Workspace1",
        is_owner=True,
        creator="Tester",
        files={},
        shared_with=[],
        parent=None,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.delete_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_delete, QtCore.Qt.LeftButton)
    assert blocker.args == [w]
