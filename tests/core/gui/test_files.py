# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.local_device import save_device_with_password

from parsec.core.gui.file_items import FileType, NAME_DATA_INDEX, TYPE_DATA_INDEX


@pytest.fixture
async def logged_gui(
    aqtbot, gui_factory, autoclose_dialog, core_config, alice, running_backend, monkeypatch
):
    save_device_with_password(core_config.config_dir, alice, "P@ssw0rd")

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    llw = gui.test_get_login_login_widget()

    assert llw is not None

    await aqtbot.key_clicks(llw.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, gui.logged_in]):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None

    wk_widget = gui.test_get_workspaces_widget()
    async with aqtbot.wait_signal(wk_widget.list_success):
        pass

    add_button = central_widget.widget_taskbar.layout().itemAt(0).widget()
    assert add_button is not None

    monkeypatch.setattr(
        "parsec.core.gui.custom_widgets.TextInputDialog.get_text",
        classmethod(lambda *args, **kwargs: ("Workspace")),
    )

    async with aqtbot.wait_signals([wk_widget.create_success, wk_widget.list_success]):
        aqtbot.qtbot.mouseClick(add_button, QtCore.Qt.LeftButton)

    assert wk_widget.layout_workspaces.count() == 1
    wk_button = wk_widget.layout_workspaces.itemAt(0).widget()
    assert wk_button.name == "Workspace"

    async with aqtbot.wait_signal(wk_widget.load_workspace_clicked):
        await aqtbot.mouse_click(wk_button, QtCore.Qt.LeftButton)

    yield gui


@pytest.mark.gui
@pytest.mark.trio
async def test_list_files(aqtbot, running_backend, logged_gui):
    w_f = logged_gui.test_get_files_widget()

    assert w_f is not None
    async with aqtbot.wait_signal(w_f.folder_stat_success):
        pass
    assert w_f.table_files.rowCount() == 1
    for i in range(5):
        assert w_f.table_files.item(0, i).data(TYPE_DATA_INDEX) == FileType.ParentWorkspace


@pytest.mark.gui
@pytest.mark.trio
async def test_create_dir(aqtbot, running_backend, logged_gui, monkeypatch):
    w_f = logged_gui.test_get_files_widget()

    assert w_f is not None
    async with aqtbot.wait_signal(w_f.folder_stat_success):
        pass
    assert w_f.table_files.rowCount() == 1

    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None

    add_button = central_widget.widget_taskbar.layout().itemAt(3).widget()
    assert add_button is not None

    monkeypatch.setattr(
        "parsec.core.gui.custom_widgets.TextInputDialog.get_text",
        classmethod(lambda *args, **kwargs: ("Dir1")),
    )

    async with aqtbot.wait_signals([w_f.folder_create_success, w_f.folder_stat_success]):
        aqtbot.qtbot.mouseClick(add_button, QtCore.Qt.LeftButton)
    assert w_f.table_files.rowCount() == 1
