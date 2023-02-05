# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.api.data import EntryName
from parsec.core.gui.lang import translate
from parsec.core.gui.remanence_management_widget import RemanenceManagementWidget
from parsec.core.gui.workspace_button import WorkspaceButton


@pytest.fixture
def catch_workspace_remanence_widget(widget_catcher_factory) -> RemanenceManagementWidget:
    return widget_catcher_factory(
        "parsec.core.gui.remanence_management_widget.RemanenceManagementWidget"
    )


@pytest.mark.gui
@pytest.mark.trio
async def test_remanence_dialog_monitor_disabled(
    aqtbot,
    logged_gui,
    catch_workspace_remanence_widget,
    autoclose_dialog,
    running_backend,
    remanence_monitor_event,  # Keep the remanence monitor disabled
):
    core = logged_gui.test_get_core()
    await core.user_fs.workspace_create(EntryName("MyWorkspace"))

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def _wait_workspace_mounted():
        assert w_w.layout_workspaces.count() == 1
        assert isinstance(w_w.layout_workspaces.itemAt(0).widget(), WorkspaceButton)

    await aqtbot.wait_until(_wait_workspace_mounted)

    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    w_w._on_manage_remanence_clicked(wk_button.workspace_fs)

    rem_w = await catch_workspace_remanence_widget()
    assert isinstance(rem_w, RemanenceManagementWidget)

    def _wait_error_message_shown():
        assert autoclose_dialog.dialogs == [
            ("Error", translate("TEXT_GET_REMANENCE_STATUS_FAILED"))
        ]

    await aqtbot.wait_until(_wait_error_message_shown)


@pytest.mark.gui
@pytest.mark.trio
async def test_remanence_dialog_empty_workspace(
    aqtbot, running_backend, logged_gui, catch_workspace_remanence_widget, remanence_monitor_event
):

    remanence_monitor_event.set()

    core = logged_gui.test_get_core()
    await core.user_fs.workspace_create(EntryName("MyWorkspace"))

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def _wait_workspace_mounted():
        assert w_w.layout_workspaces.count() == 1
        assert isinstance(w_w.layout_workspaces.itemAt(0).widget(), WorkspaceButton)

    await aqtbot.wait_until(_wait_workspace_mounted)

    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    w_w._on_manage_remanence_clicked(wk_button.workspace_fs)

    rem_w = await catch_workspace_remanence_widget()
    assert isinstance(rem_w, RemanenceManagementWidget)

    assert rem_w.dialog.label_title.text() == translate(
        "TEXT_REMANENCE_DIALOG_TITLE_workspace_name"
    ).format(workspace_name=wk_button.workspace_fs.get_workspace_name().str)

    def _wait_dialog_updated():
        assert rem_w.switch_button.isChecked() is False
        assert rem_w.progress_bar.text() == translate("TEXT_WORKSPACE_IS_EMPTY")

    await aqtbot.wait_until(_wait_dialog_updated)


@pytest.mark.gui
@pytest.mark.trio
async def test_remanence_dialog_non_empty_workspace(
    aqtbot, logged_gui, catch_workspace_remanence_widget, running_backend, remanence_monitor_event
):

    remanence_monitor_event.set()

    core = logged_gui.test_get_core()
    await core.user_fs.workspace_create(EntryName("MyWorkspace"))

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def _wait_workspace_mounted():
        assert w_w.layout_workspaces.count() == 1
        assert isinstance(w_w.layout_workspaces.itemAt(0).widget(), WorkspaceButton)

    await aqtbot.wait_until(_wait_workspace_mounted)

    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    assert isinstance(wk_button, WorkspaceButton)

    async with await wk_button.workspace_fs.open_file("/file1", "wb") as dest_file:
        await dest_file.write(b"a" * 1024)

    await wk_button.workspace_fs.sync()

    w_w._on_manage_remanence_clicked(wk_button.workspace_fs)

    rem_w = await catch_workspace_remanence_widget()
    assert isinstance(rem_w, RemanenceManagementWidget)

    def _wait_dialog_updated():
        assert rem_w.switch_button.isChecked() is False
        assert rem_w.label_info.text() == translate(
            "TEXT_BLOCK_REMANENCE_DISABLED_cache_size"
        ).format(cache_size="512 MB")
        assert (
            rem_w.progress_bar.text()
            == f"{translate('TEXT_REMANENCE_PROGRESS_BAR_TITLE_IDLE')} - 1.00 KB / 1.00 KB (100%)"
        )

    await aqtbot.wait_until(_wait_dialog_updated)


@pytest.mark.gui
@pytest.mark.trio
async def test_remanence_dialog_turn_on(
    aqtbot,
    logged_gui,
    catch_workspace_remanence_widget,
    running_backend,
    remanence_monitor_event,
    snackbar_catcher,
):

    remanence_monitor_event.set()

    core = logged_gui.test_get_core()
    await core.user_fs.workspace_create(EntryName("MyWorkspace"))

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def _wait_workspace_mounted():
        assert w_w.layout_workspaces.count() == 1
        assert isinstance(w_w.layout_workspaces.itemAt(0).widget(), WorkspaceButton)

    await aqtbot.wait_until(_wait_workspace_mounted)

    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    assert isinstance(wk_button, WorkspaceButton)

    async with await wk_button.workspace_fs.open_file("/file1", "wb") as dest_file:
        await dest_file.write(b"a" * 1024)

    await wk_button.workspace_fs.sync()

    w_w._on_manage_remanence_clicked(wk_button.workspace_fs)

    rem_w = await catch_workspace_remanence_widget()
    assert isinstance(rem_w, RemanenceManagementWidget)

    def _wait_dialog_updated():
        assert rem_w.switch_button.isChecked() is False

    await aqtbot.wait_until(_wait_dialog_updated)

    aqtbot.mouse_click(rem_w.switch_button, QtCore.Qt.LeftButton)

    def _wait_remanence_turned_on():
        assert rem_w.switch_button.isChecked()
        assert rem_w.label_info.text() == translate("TEXT_BLOCK_REMANENCE_ENABLED")
        assert snackbar_catcher.snackbars == [
            ("CONGRATULATE", translate("TEXT_ENABLE_BLOCK_REMANENCE_SUCCESS"))
        ]

    await aqtbot.wait_until(_wait_remanence_turned_on)
