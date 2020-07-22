# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore, QtWidgets

from tests.common import customize_fixtures


@pytest.fixture
def catch_file_history_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.file_history_widget.FileHistoryWidget")


async def create_workspace(aqtbot, logged_gui, monkeypatch):
    w_w = await logged_gui.test_switch_to_workspaces_widget()
    add_button = w_w.button_add_workspace
    assert add_button is not None

    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.get_text_input", lambda *args, **kwargs: ("Workspace")
    )
    async with aqtbot.wait_signals(
        [w_w.create_success, w_w.list_success, w_w.mountpoint_started], timeout=2000
    ):
        await aqtbot.mouse_click(add_button, QtCore.Qt.LeftButton)

    def _workspace_button_ready():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert not isinstance(wk_button, QtWidgets.QLabel)

    await aqtbot.wait_until(_workspace_button_ready, timeout=2000)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    assert wk_button.name == "Workspace"

    async with aqtbot.wait_signal(w_w.load_workspace_clicked):
        await aqtbot.mouse_click(wk_button, QtCore.Qt.LeftButton)

    return w_w


async def create_directories(logged_gui, aqtbot, monkeypatch, dir_names):
    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None

    f_w = logged_gui.test_get_files_widget()
    assert f_w is not None

    add_button = f_w.button_create_folder

    for dir_name in dir_names:
        monkeypatch.setattr(
            "parsec.core.gui.files_widget.get_text_input", lambda *args, **kwargs: (dir_name)
        )
        async with aqtbot.wait_signal(f_w.folder_create_success):
            await aqtbot.mouse_click(add_button, QtCore.Qt.LeftButton)

    async with aqtbot.wait_signals([f_w.folder_stat_success, f_w.fs_synced_qt], timeout=3000):
        pass

    f_w.table_files.rowCount() == 1

    def _folder_synced():
        item = f_w.table_files.item(1, 0)
        assert item.is_synced

    await aqtbot.wait_until(_folder_synced, timeout=3000)
    return f_w


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_file_history(
    aqtbot,
    running_backend,
    logged_gui,
    monkeypatch,
    autoclose_dialog,
    qt_thread_gateway,
    catch_file_history_widget,
    alice,
):
    await create_workspace(aqtbot, logged_gui, monkeypatch)
    f_w = await create_directories(logged_gui, aqtbot, monkeypatch, ["dir1"])

    await aqtbot.run(
        f_w.table_files.setRangeSelected, QtWidgets.QTableWidgetSelectionRange(1, 0, 1, 0), True
    )

    await qt_thread_gateway.send_action(f_w.show_history)
    hf_w = await catch_file_history_widget()

    def _history_filled():
        assert hf_w.layout_history.count() == 1
        assert hf_w.layout_history.itemAt(0).widget()

    await aqtbot.wait_until(_history_filled, timeout=5000)
    history_button = hf_w.layout_history.itemAt(0).widget()
    assert history_button.label_user.text() == alice.human_handle.label
    assert history_button.label_version.text() == "1"
