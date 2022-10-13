# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from PyQt5 import QtCore
from unittest.mock import Mock
from pathlib import Path
from parsec._parsec import DateTime

from parsec.api.data import EntryName, EntryID
from parsec.core.types import WorkspaceRole
from parsec.core.core_events import CoreEvent
from parsec.core.fs import FSWorkspaceNoReadAccess, FsPath
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.timestamped_workspace_widget import TimestampedWorkspaceWidget
from parsec.core.gui.lang import translate, format_datetime

from tests.common import freeze_time, customize_fixtures


@pytest.fixture
def catch_timestamped_workspace_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.timestamped_workspace_widget.TimestampedWorkspaceWidget"
    )


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
    aqtbot.mouse_click(w_w.button_add_workspace, QtCore.Qt.LeftButton)

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
            assert wk_button.name == EntryName("Workspace1")
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
    await core.user_fs.workspace_create(EntryName("Workspace1"))

    def _workspace_displayed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == EntryName("Workspace1")

    await aqtbot.wait_until(_workspace_displayed)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    # Now do the rename
    workspace_name = ".." if invalid_name else "Workspace1_Renamed"
    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.get_text_input", lambda *args, **kwargs: (workspace_name)
    )
    aqtbot.mouse_click(wk_button.button_rename, QtCore.Qt.LeftButton)

    def _outcome_occured():
        assert w_w.layout_workspaces.count() == 1
        new_wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(new_wk_button, WorkspaceButton)
        assert new_wk_button.workspace_fs is wk_button.workspace_fs
        if invalid_name:
            assert wk_button.name == EntryName("Workspace1")
            assert autoclose_dialog.dialogs == [
                (
                    "Error",
                    "Could not rename the workspace. This name is not a valid workspace name.",
                )
            ]
        else:
            assert wk_button.name == EntryName("Workspace1_Renamed")
            assert not autoclose_dialog.dialogs

    await aqtbot.wait_until(_outcome_occured)


@pytest.mark.gui
@pytest.mark.trio
async def test_mountpoint_remote_error_event(aqtbot, running_backend, logged_gui, snackbar_catcher):
    c_w = logged_gui.test_get_central_widget()

    async with aqtbot.wait_signal(c_w.new_notification):
        c_w.event_bus.send(
            CoreEvent.MOUNTPOINT_REMOTE_ERROR,
            exc=FSWorkspaceNoReadAccess("Cannot get workspace roles: no read access"),
            path=FsPath("/bar"),
            mountpoint=Path("/foo"),
            operation="open",
            workspace_id=EntryID.new(),
            timestamp=None,
        )

    assert "Read access to " in snackbar_catcher.snackbars[0][1]
    snackbar_catcher.reset()

    async with aqtbot.wait_signal(c_w.new_notification):
        c_w.event_bus.send(
            CoreEvent.MOUNTPOINT_UNHANDLED_ERROR,
            exc=RuntimeError("D'Oh !"),
            path=FsPath("/bar"),
            mountpoint=Path("/foo"),
            operation="unlink",
            workspace_id=EntryID.new(),
        )

    assert "Error with the mountpoint " in snackbar_catcher.snackbars[0][1]
    snackbar_catcher.reset()


@pytest.mark.gui
@pytest.mark.trio
async def test_mountpoint_open_in_explorer_button(aqtbot, running_backend, logged_gui, monkeypatch):
    # Disable actual mount given we are only testing the GUI here
    open_workspace_mock = Mock()
    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.WorkspacesWidget.open_workspace", open_workspace_mock
    )

    # Create a new workspace
    core = logged_gui.test_get_core()
    wid = await core.user_fs.workspace_create(EntryName("wksp1"))

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def get_wk_button():
        assert w_w.layout_workspaces.count() == 1
        item = w_w.layout_workspaces.itemAt(0)
        assert item
        wk_button = item.widget()

        assert isinstance(wk_button, WorkspaceButton)
        return wk_button

    # New workspace should show up mounted

    wk_button = None

    def _initially_mounted():
        nonlocal wk_button
        wk_button = get_wk_button()
        assert wk_button.button_open.isEnabled()
        assert wk_button.switch_button.isChecked()
        # Be sure that the workspave is mounted
        assert core.mountpoint_manager.is_workspace_mounted(wid)

    await aqtbot.wait_until(_initially_mounted)

    # Now switch to umounted
    aqtbot.mouse_click(wk_button.switch_button, QtCore.Qt.LeftButton)

    def _unmounted():
        nonlocal wk_button
        wk_button = get_wk_button()
        assert not wk_button.button_open.isEnabled()
        assert not wk_button.switch_button.isChecked()
        assert not core.mountpoint_manager.is_workspace_mounted(wid)

    await aqtbot.wait_until(_unmounted)

    def _mounted():
        nonlocal wk_button
        wk_button = get_wk_button()
        assert wk_button.button_open.isEnabled()
        assert wk_button.switch_button.isChecked()
        # Be sure that the workspave is mounted
        assert core.mountpoint_manager.is_workspace_mounted(wid)

    # Now switch back to mounted
    aqtbot.mouse_click(wk_button.switch_button, QtCore.Qt.LeftButton)
    await aqtbot.wait_until(_mounted)

    # Test open button

    def _wk_opened():
        open_workspace_mock.assert_called_once()

    aqtbot.mouse_click(wk_button.button_open, QtCore.Qt.LeftButton)
    await aqtbot.wait_until(_wk_opened)


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_filter_user(
    aqtbot, running_backend, logged_gui, autoclose_dialog, alice_user_fs, bob, bob_user_fs, alice
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()
    wid_alice = await alice_user_fs.workspace_create(EntryName("Workspace1"))
    wid_bob = await bob_user_fs.workspace_create(EntryName("Workspace2"))
    await bob_user_fs.workspace_create(EntryName("Workspace3"))

    await alice_user_fs.workspace_share(wid_alice, bob.user_id, WorkspaceRole.MANAGER)
    await bob_user_fs.workspace_share(wid_bob, alice.user_id, WorkspaceRole.READER)

    await alice_user_fs.process_last_messages()
    await alice_user_fs.sync()
    await bob_user_fs.process_last_messages()
    await bob_user_fs.sync()

    def _workspace_listed():
        assert w_w.layout_workspaces.count() == 3
        wk_button1 = w_w.layout_workspaces.itemAt(0).widget()
        wk_button2 = w_w.layout_workspaces.itemAt(1).widget()
        wk_button3 = w_w.layout_workspaces.itemAt(2).widget()
        assert isinstance(wk_button1, WorkspaceButton)
        assert isinstance(wk_button2, WorkspaceButton)
        assert isinstance(wk_button3, WorkspaceButton)
        assert not w_w.filter_remove_button.isVisible()

    await aqtbot.wait_until(_workspace_listed)

    u_w = await logged_gui.test_switch_to_users_widget()

    # Force click on user filter menu
    assert u_w.layout_users.count() == 3
    for i in range(u_w.layout_users.count()):
        button = u_w.layout_users.itemAt(i).widget()
        if not button.is_current_user and button.user_info.user_id == alice.user_id:
            button.filter_user_workspaces_clicked.emit(button.user_info)
            break
    else:
        raise ValueError("Can not find Alice user")

    def _workspace_filtered():
        assert w_w.isVisible()
        assert w_w.layout_workspaces.count() == 2
        wk_button_1 = w_w.layout_workspaces.itemAt(0).widget()
        wk_button_2 = w_w.layout_workspaces.itemAt(1).widget()
        assert isinstance(wk_button_1, WorkspaceButton)
        assert isinstance(wk_button_2, WorkspaceButton)
        assert wk_button_1.name in [EntryName("Workspace1"), EntryName("Workspace2")]
        assert wk_button_2.name in [EntryName("Workspace1"), EntryName("Workspace2")]
        assert w_w.filter_remove_button.isVisible()
        assert w_w.filter_label.text() == "Common workspaces with {}".format(
            alice.short_user_display
        )

    await aqtbot.wait_until(_workspace_filtered)

    # Remove filter

    aqtbot.mouse_click(w_w.filter_remove_button, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(_workspace_listed)


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_filter_user_new_workspace(
    aqtbot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    alice_user_fs,
    bob,
    bob_user_fs,
    alice,
    monkeypatch,
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()
    wid_alice = await alice_user_fs.workspace_create(EntryName("Workspace1"))

    await alice_user_fs.workspace_share(wid_alice, bob.user_id, WorkspaceRole.MANAGER)

    await alice_user_fs.process_last_messages()
    await alice_user_fs.sync()

    def _workspace_listed():
        assert w_w.layout_workspaces.count() == 1
        wk_button1 = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button1, WorkspaceButton)
        assert not w_w.filter_remove_button.isVisible()

    await aqtbot.wait_until(_workspace_listed)

    u_w = await logged_gui.test_switch_to_users_widget()

    # Force click on user filter menu
    assert u_w.layout_users.count() == 3
    for i in range(u_w.layout_users.count()):
        button = u_w.layout_users.itemAt(i).widget()
        if not button.is_current_user and button.user_info.user_id == alice.user_id:
            button.filter_user_workspaces_clicked.emit(button.user_info)
            break
    else:
        raise ValueError("Can not find Alice user")

    def _workspace_filtered():
        assert w_w.isVisible()
        assert w_w.layout_workspaces.count() == 1
        wk_button_1 = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button_1, WorkspaceButton)
        assert wk_button_1.name == EntryName("Workspace1")
        assert w_w.filter_remove_button.isVisible()
        assert w_w.filter_label.text() == "Common workspaces with {}".format(
            alice.short_user_display
        )

    await aqtbot.wait_until(_workspace_filtered)

    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.get_text_input", lambda *args, **kwargs: ("Workspace2")
    )
    aqtbot.mouse_click(w_w.button_add_workspace, QtCore.Qt.LeftButton)

    def _new_workspace_listed():
        assert w_w.layout_workspaces.count() == 2
        wk_button1 = w_w.layout_workspaces.itemAt(0).widget()
        wk_button2 = w_w.layout_workspaces.itemAt(1).widget()
        assert isinstance(wk_button1, WorkspaceButton)
        assert isinstance(wk_button2, WorkspaceButton)
        assert wk_button1.name in [EntryName("Workspace1"), EntryName("Workspace2")]
        assert wk_button2.name in [EntryName("Workspace1"), EntryName("Workspace2")]
        assert not w_w.filter_remove_button.isVisible()

    await aqtbot.wait_until(_new_workspace_listed)


@pytest.mark.gui
@pytest.mark.trio
async def test_display_timestamped_workspace_in_workspaces_list(
    aqtbot, running_backend, logged_gui, monkeypatch, catch_timestamped_workspace_widget, tmp_path
):
    central_widget = logged_gui.test_get_central_widget()
    workspace_name = EntryName("wksp1")

    def _online():
        assert central_widget.menu.label_connection_state.text() == translate(
            "TEXT_BACKEND_STATE_CONNECTED"
        )

    await aqtbot.wait_until(_online)

    w_w = logged_gui.test_get_workspaces_widget()
    await logged_gui.test_get_core().user_fs.sync()
    await logged_gui.test_get_core().wait_idle_monitors()

    # Timeline:
    #   2000: workspace creation
    #   2010: file1.txt creation
    #   2020: file2.txt creation
    #   2022: present day, we want to see the workspace history as it was in 2015
    year_n = DateTime(2000, 1, 1)
    year_n10 = DateTime(2010, 1, 1)
    year_n15 = DateTime(2015, 1, 1)
    year_n20 = DateTime(2020, 1, 1)
    now = DateTime(2022, 1, 1)

    # 2000: Create the workspace
    with freeze_time(year_n):
        user_fs = logged_gui.test_get_core().user_fs
        await user_fs.workspace_create(workspace_name)
        await user_fs.sync()

        # Now wait for GUI to take it into account
        def _workspace_available():
            assert w_w.layout_workspaces.count() == 1
            wk_button = w_w.layout_workspaces.itemAt(0).widget()
            assert isinstance(wk_button, WorkspaceButton)
            assert wk_button.name == workspace_name

        await aqtbot.wait_until(_workspace_available)
        f_w = await logged_gui.test_switch_to_files_widget(workspace_name)

    # Populate some files for import
    out_of_parsec_data = tmp_path / "out_of_parsec_data"
    out_of_parsec_data.mkdir(parents=True)
    (out_of_parsec_data / "file1.txt").touch()
    (out_of_parsec_data / "file2.txt").touch()

    # 2010: file1.txt creation
    with freeze_time(year_n10):
        # Import file 1
        monkeypatch.setattr(
            "parsec.core.gui.custom_dialogs.QDialogInProcess.getOpenFileNames",
            classmethod(lambda *args, **kwargs: ([out_of_parsec_data / "file1.txt"], True)),
        )
        async with aqtbot.wait_signal(f_w.import_success):
            aqtbot.mouse_click(f_w.button_import_files, QtCore.Qt.LeftButton)
        await f_w.workspace_fs.sync()

    # 2020: file1.txt creation
    with freeze_time(year_n20):
        # Import file 2
        monkeypatch.setattr(
            "parsec.core.gui.custom_dialogs.QDialogInProcess.getOpenFileNames",
            classmethod(lambda *args, **kwargs: ([out_of_parsec_data / "file2.txt"], True)),
        )
        async with aqtbot.wait_signal(f_w.import_success):
            aqtbot.mouse_click(f_w.button_import_files, QtCore.Qt.LeftButton)
        await f_w.workspace_fs.sync()

    def _wait_for_files():
        assert f_w.table_files.rowCount() == 3
        assert f_w.table_files.item(1, 1).text() == "file1.txt"
        assert f_w.table_files.item(1, 2).text() == format_datetime(year_n10)
        assert f_w.table_files.item(1, 3).text() == format_datetime(year_n10)
        assert f_w.table_files.item(2, 1).text() == "file2.txt"
        assert f_w.table_files.item(2, 2).text() == format_datetime(year_n20)
        assert f_w.table_files.item(2, 3).text() == format_datetime(year_n20)

    await aqtbot.wait_until(_wait_for_files)

    # 2022: see history as it was 2015
    with freeze_time(now):
        aqtbot.mouse_click(f_w.button_back, QtCore.Qt.LeftButton)

        def _wait_workspace_refreshed():
            assert w_w.layout_workspaces.count() == 1
            wk_button = w_w.layout_workspaces.itemAt(0).widget()
            assert isinstance(wk_button, WorkspaceButton)
            assert wk_button.name == workspace_name

        await aqtbot.wait_until(_wait_workspace_refreshed)

        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        aqtbot.mouse_click(wk_button.button_remount_ts, QtCore.Qt.LeftButton)
        ts_wk_w = await catch_timestamped_workspace_widget()

        assert isinstance(ts_wk_w, TimestampedWorkspaceWidget)

        def _timestamped_widget_ready():
            assert ts_wk_w.isEnabled()
            assert ts_wk_w.calendar_widget.minimumDate() == QtCore.QDate(
                year_n.year, year_n.month, year_n.day
            )
            assert ts_wk_w.calendar_widget.maximumDate() == QtCore.QDate(
                now.year, now.month, now.day
            )

        await aqtbot.wait_until(_timestamped_widget_ready)

        # Approximately 15 years from now
        selected_date = year_n15
        ts_wk_w.calendar_widget.setSelectedDate(
            QtCore.QDate(selected_date.year, selected_date.month, selected_date.day)
        )

        def _widget_updated_with_new_date():
            assert ts_wk_w.date == QtCore.QDate(
                selected_date.year, selected_date.month, selected_date.day
            )
            assert ts_wk_w.time == QtCore.QTime(0, 0)

        await aqtbot.wait_until(_widget_updated_with_new_date)

        async with aqtbot.wait_signal(w_w.mount_success):
            aqtbot.mouse_click(ts_wk_w.button_show, QtCore.Qt.LeftButton)

        def _new_workspace_listed():
            assert w_w.layout_workspaces.count() == 2
            wk_button = w_w.layout_workspaces.itemAt(0).widget()
            ts_wk_button = w_w.layout_workspaces.itemAt(1).widget()
            assert isinstance(wk_button, WorkspaceButton)
            assert isinstance(ts_wk_button, WorkspaceButton)
            assert not wk_button.is_timestamped
            assert ts_wk_button.is_timestamped

        await aqtbot.wait_until(_new_workspace_listed)

        ts_wk_button = w_w.layout_workspaces.itemAt(1).widget()

        aqtbot.mouse_click(ts_wk_button, QtCore.Qt.LeftButton)

        f_w = logged_gui.test_get_files_widget()
        assert f_w.isVisible()

        def _files_listed():
            f_w = logged_gui.test_get_files_widget()
            assert f_w.isVisible()
            assert f_w.table_files.rowCount() == 2
            assert f_w.table_files.item(1, 1).text() == "file1.txt"
            assert f_w.table_files.item(1, 2).text() == format_datetime(year_n10)
            assert f_w.table_files.item(1, 3).text() == format_datetime(year_n10)

        await aqtbot.wait_until(_files_listed)

        aqtbot.mouse_click(f_w.button_back, QtCore.Qt.LeftButton)

        await aqtbot.wait_until(_new_workspace_listed)

        ts_wk_button = w_w.layout_workspaces.itemAt(1).widget()
        aqtbot.mouse_click(ts_wk_button.button_delete, QtCore.Qt.LeftButton)

        def _timestamped_workspace_delete():
            assert w_w.layout_workspaces.count() == 1
            wk_button = w_w.layout_workspaces.itemAt(0).widget()
            assert isinstance(wk_button, WorkspaceButton)

        await aqtbot.wait_until(_timestamped_workspace_delete)


@pytest.mark.gui
@pytest.mark.trio
async def test_hide_unmounted_workspaces(logged_gui, aqtbot):
    w_w = logged_gui.test_get_workspaces_widget()
    core = logged_gui.test_get_core()

    w_w.check_hide_unmounted.setChecked(False)

    workspace_name = EntryName("wksp1")

    # Create the workspace
    user_fs = logged_gui.test_get_core().user_fs
    wid = await user_fs.workspace_create(workspace_name)

    # Now wait for GUI to take it into account
    def _workspace_visible(visible):
        count = 1 if visible else 0
        assert w_w.layout_workspaces.count() == count

    await aqtbot.wait_until(lambda: _workspace_visible(True))

    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    def _mountstate_is(status):
        assert wk_button.switch_button.isChecked() is status
        assert core.mountpoint_manager.is_workspace_mounted(wid) is status

    await aqtbot.wait_until(lambda: _mountstate_is(True))

    # Hide unmounted False, workspace mounted True, workspace should be visible
    await aqtbot.wait_until(lambda: _workspace_visible(True))

    # Hide unmounted False, workspace mounted False, workspace should be visible
    aqtbot.mouse_click(wk_button.switch_button, QtCore.Qt.LeftButton)
    await aqtbot.wait_until(lambda: _mountstate_is(False))
    await aqtbot.wait_until(lambda: _workspace_visible(True))

    # Hide unmounted True, workspace mounted False, workspace should not be visible
    w_w.check_hide_unmounted.setChecked(True)
    aqtbot.mouse_click(wk_button.switch_button, QtCore.Qt.LeftButton)
    await aqtbot.wait_until(lambda: _workspace_visible(False))


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_button(aqtbot, running_backend, logged_gui, autoclose_dialog, bob, adam):
    w_w = await logged_gui.test_switch_to_workspaces_widget()
    core = logged_gui.test_get_core()
    wid = await core.user_fs.workspace_create(EntryName("Workspace"))

    def _workspace_button_shown(people):
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == EntryName("Workspace")
        if people == 0:
            wk_button.label_shared_info.text() == translate("TEXT_WORKSPACE_IS_PRIVATE")
        elif people == 1:
            wk_button.label_shared_info.text() == translate(
                "TEXT_WORKSPACE_IS_SHARED_WITH_user"
            ).format(user="bob")
        else:
            wk_button.label_shared_info.text() == translate(
                "TEXT_WORKSPACE_IS_SHARED_n_USERS"
            ).format(n=people)

    await aqtbot.wait_until(lambda: _workspace_button_shown(people=0))

    await core.user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)
    await aqtbot.wait_until(lambda: _workspace_button_shown(people=1))

    await core.user_fs.workspace_share(wid, adam.user_id, WorkspaceRole.MANAGER)
    await aqtbot.wait_until(lambda: _workspace_button_shown(people=2))

    await core.revoke_user(adam.user_id)
    await aqtbot.wait_until(lambda: _workspace_button_shown(people=1))

    await core.revoke_user(bob.user_id)
    await aqtbot.wait_until(lambda: _workspace_button_shown(people=0))
