# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore
from uuid import UUID
import pendulum
from unittest.mock import ANY

from parsec.api.data import WorkspaceEntry
from parsec.core.core_events import CoreEvent
from parsec.core.fs import FSWorkspaceNoReadAccess
from parsec.core.gui.workspace_button import WorkspaceButton


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("invalid_name", (False, True))
async def test_add_workspace(
    aqtbot, running_backend, logged_gui, monkeypatch, autoclose_dialog, invalid_name
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    # Make sure there is no workspaces to display
    assert w_w.layout_workspaces.count() == 1
    assert w_w.layout_workspaces.itemAt(0).widget().text() == "No workspace has been created yet."

    # Add (or try to) a new workspace
    workspace_name = ".." if invalid_name else "Workspace1"
    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.get_text_input", lambda *args, **kwargs: (workspace_name)
    )
    async with aqtbot.wait_signal(w_w.list_success, timeout=2000):
        await aqtbot.mouse_click(w_w.button_add_workspace, QtCore.Qt.LeftButton)

    def _outcome_occured():
        assert w_w.layout_workspaces.count() == 1
        if invalid_name:
            assert (
                w_w.layout_workspaces.itemAt(0).widget().text()
                == "No workspace has been created yet."
            )
            assert autoclose_dialog.dialogs == [
                (
                    "Error",
                    "Could not create the workspace. This name is not a valid workspace name.",
                )
            ]
        else:
            wk_button = w_w.layout_workspaces.itemAt(0).widget()
            assert isinstance(wk_button, WorkspaceButton)
            assert wk_button.name == "Workspace1"
            assert not autoclose_dialog.dialogs

    await aqtbot.wait_until(_outcome_occured)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("invalid_name", (False, True))
async def test_rename_workspace(
    aqtbot, running_backend, logged_gui, monkeypatch, autoclose_dialog, invalid_name
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    # Create a workspace and make sure the workspace is displayed
    core = logged_gui.test_get_core()
    async with aqtbot.wait_signal(w_w.list_success, timeout=2000):
        await core.user_fs.workspace_create("Workspace1")

    def _workspace_displayed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == "Workspace1"

    await aqtbot.wait_until(_workspace_displayed)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    # Now do the rename
    workspace_name = ".." if invalid_name else "Workspace1_Renamed"
    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.get_text_input", lambda *args, **kwargs: (workspace_name)
    )
    await aqtbot.mouse_click(wk_button.button_rename, QtCore.Qt.LeftButton)

    def _outcome_occured():
        assert w_w.layout_workspaces.count() == 1
        new_wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(new_wk_button, WorkspaceButton)
        assert new_wk_button.workspace_fs is wk_button.workspace_fs
        if invalid_name:
            assert wk_button.name == "Workspace1"
            assert autoclose_dialog.dialogs == [
                (
                    "Error",
                    "Could not rename the workspace. This name is not a valid workspace name.",
                )
            ]
        else:
            assert wk_button.name == "Workspace1_Renamed"
            assert not autoclose_dialog.dialogs

    await aqtbot.wait_until(_outcome_occured)


@pytest.mark.skip("No notification center at the moment")
@pytest.mark.gui
@pytest.mark.trio
async def test_mountpoint_remote_error_event(aqtbot, running_backend, logged_gui):
    c_w = logged_gui.test_get_central_widget()

    async with aqtbot.wait_signal(c_w.new_notification):
        c_w.event_bus.send(
            CoreEvent.MOUNTPOINT_REMOTE_ERROR,
            exc=FSWorkspaceNoReadAccess("Cannot get workspace roles: no read access"),
            path="/foo",
            operation="open",
        )
    msg_widget = c_w.notification_center.widget_layout.layout().itemAt(0).widget()
    assert (
        msg_widget.message
        == 'Cannot access "/foo" from the server given you lost read access to the workspace.'
    )

    async with aqtbot.wait_signal(c_w.new_notification):
        c_w.event_bus.send(
            CoreEvent.MOUNTPOINT_UNHANDLED_ERROR,
            exc=RuntimeError("D'Oh !"),
            path="/bar",
            operation="unlink",
        )
    msg_widget = c_w.notification_center.widget_layout.layout().itemAt(0).widget()
    assert (
        msg_widget.message
        == 'Unexpected error while performing "unlink" operation on "/bar": D\'Oh !.'
    )


@pytest.mark.skip("Should be reworked")
@pytest.mark.gui
@pytest.mark.trio
async def test_event_bus_internal_connection(aqtbot, running_backend, logged_gui, autoclose_dialog):
    w_w = await logged_gui.test_switch_to_workspaces_widget()
    uuid = UUID("1bc1e17b-157a-462f-86f2-7f64657ba16a")
    w_entry = WorkspaceEntry(
        name="w",
        id=ANY,
        key=ANY,
        encryption_revision=1,
        encrypted_on=ANY,
        role_cached_on=ANY,
        role=None,
    )

    async with aqtbot.wait_signal(w_w.fs_synced_qt):
        w_w.event_bus.send(CoreEvent.FS_ENTRY_SYNCED, workspace_id=None, id=uuid)

    async with aqtbot.wait_signal(w_w.fs_updated_qt):
        w_w.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, workspace_id=uuid, id=None)

    async with aqtbot.wait_signal(w_w._workspace_created_qt):
        w_w.event_bus.send(CoreEvent.FS_WORKSPACE_CREATED, new_entry=w_entry)

    async with aqtbot.wait_signal(w_w.sharing_updated_qt):
        w_w.event_bus.send(CoreEvent.SHARING_UPDATED, new_entry=w_entry, previous_entry=None)

    async with aqtbot.wait_signal(w_w.entry_downsynced_qt):
        w_w.event_bus.send(CoreEvent.FS_ENTRY_DOWNSYNCED, workspace_id=uuid, id=uuid)

    async with aqtbot.wait_signal(w_w.mountpoint_started):
        w_w.event_bus.send(
            CoreEvent.MOUNTPOINT_STARTED,
            mountpoint=None,
            workspace_id=uuid,
            timestamp=pendulum.now(),
        )

    assert not autoclose_dialog.dialogs
    async with aqtbot.wait_signal(w_w.mountpoint_stopped):
        w_w.event_bus.send(
            CoreEvent.MOUNTPOINT_STOPPED,
            mountpoint=None,
            workspace_id=uuid,
            timestamp=pendulum.now(),
        )
    assert autoclose_dialog.dialogs == [
        (
            "Error",
            "Your permissions on this workspace have been revoked. You no longer have access to theses files.",
        )
    ]
