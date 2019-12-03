# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.gui.workspace_button import WorkspaceButton


@pytest.fixture
@pytest.mark.trio
async def workspace_fs(alice_user_fs, running_backend):
    wid = await alice_user_fs.workspace_create("Workspace")
    workspace = alice_user_fs.get_workspace(wid)
    return workspace


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button(qtbot, workspace_fs):

    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        is_shared=False,
        is_creator=False,
        files=[],
        enable_workspace_color=False,
        parent=None,
    )
    qtbot.addWidget(w)
    w.show()

    assert w.label_empty.isVisible() is True
    assert w.widget_files.isVisible() is False
    assert w.label_owner.isVisible() is False
    assert w.label_shared.isVisible() is False
    assert w.name == "Workspace"
    assert w.label_workspace.text() == "Workspace"


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_shared_by(qtbot, workspace_fs):

    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        is_shared=True,
        is_creator=False,
        files=[],
        enable_workspace_color=False,
        parent=None,
    )

    qtbot.addWidget(w)
    w.show()
    assert w.label_empty.isVisible() is True
    assert w.widget_files.isVisible() is False
    assert w.label_owner.isVisible() is False
    assert w.label_shared.isVisible() is True
    assert w.name == "Workspace"
    # assert w.label_workspace.text() == "Workspace (shared with you)"


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_shared_with(qtbot, workspace_fs):

    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        is_shared=True,
        is_creator=True,
        files=[],
        enable_workspace_color=False,
        parent=None,
    )

    qtbot.addWidget(w)
    w.show()
    assert w.label_empty.isVisible() is True
    assert w.widget_files.isVisible() is False
    assert w.label_owner.isVisible() is True
    assert w.label_shared.isVisible() is True
    assert w.name == "Workspace"
    assert w.label_workspace.text() == "Workspace (shared with others)"


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_files(qtbot, workspace_fs):
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        is_shared=True,
        is_creator=True,
        files=["File1.txt", "File2.txt", "Dir1"],
        enable_workspace_color=False,
        parent=None,
    )

    qtbot.addWidget(w)
    w.show()
    assert w.label_empty.isVisible() is False
    assert w.widget_files.isVisible() is True
    assert w.label_owner.isVisible() is True
    assert w.label_shared.isVisible() is True
    assert w.name == "Workspace"
    assert w.label_workspace.text() == "Workspace (shared with others)"
    assert w.label_file1.text() == "File1.txt"
    assert w.label_file2.text() == "File2.txt"
    assert w.label_file3.text() == "Dir1"
    assert w.label_file4.text() == ""


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_clicked(qtbot, workspace_fs):
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        is_shared=True,
        is_creator=True,
        files=[],
        enable_workspace_color=False,
        parent=None,
    )

    qtbot.addWidget(w)
    with qtbot.waitSignal(w.clicked, timeout=500) as blocker:
        qtbot.mouseClick(w, QtCore.Qt.LeftButton)
    assert blocker.args == [workspace_fs]


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_share_clicked(qtbot, workspace_fs):
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        is_shared=False,
        is_creator=True,
        files=[],
        enable_workspace_color=False,
        parent=None,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.share_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_share, QtCore.Qt.LeftButton)
    assert blocker.args == [workspace_fs]


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_rename_clicked(qtbot, workspace_fs):
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        is_shared=False,
        is_creator=True,
        files=[],
        enable_workspace_color=False,
        parent=None,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.rename_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_rename, QtCore.Qt.LeftButton)
    assert blocker.args == [w]


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_delete_clicked(qtbot, workspace_fs):
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        is_shared=False,
        is_creator=True,
        files=[],
        enable_workspace_color=False,
        parent=None,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.delete_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_delete, QtCore.Qt.LeftButton)
    assert blocker.args == [workspace_fs]
