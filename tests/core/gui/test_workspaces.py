# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.local_device import save_device_with_password


@pytest.fixture
async def logged_gui(aqtbot, gui_factory, autoclose_dialog, core_config, alice, bob):
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

    save_device_with_password(core_config.config_dir, bob, "P@ssw0rd")

    yield gui


@pytest.mark.gui
@pytest.mark.trio
async def test_add_workspace(aqtbot, running_backend, logged_gui, monkeypatch):
    w_w = logged_gui.test_get_workspaces_widget()

    assert w_w is not None
    async with aqtbot.wait_signal(w_w.list_success):
        pass
    assert w_w.layout_workspaces.count() == 0

    c_w = logged_gui.test_get_central_widget()
    assert c_w is not None

    add_button = c_w.widget_taskbar.layout().itemAt(0).widget()
    assert add_button is not None

    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.TextInputDialog.get_text",
        classmethod(lambda *args, **kwargs: ("Workspace1")),
    )

    async with aqtbot.wait_signals([w_w.create_success, w_w.list_success]):
        aqtbot.qtbot.mouseClick(add_button, QtCore.Qt.LeftButton)

    assert w_w.layout_workspaces.count() == 1
    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    assert wk_button.name == "Workspace1"


@pytest.mark.gui
@pytest.mark.trio
async def test_rename_workspace(aqtbot, running_backend, logged_gui, monkeypatch):
    w_w = logged_gui.test_get_workspaces_widget()

    assert w_w is not None
    async with aqtbot.wait_signal(w_w.list_success):
        pass
    assert w_w.layout_workspaces.count() == 0

    c_w = logged_gui.test_get_central_widget()
    assert c_w is not None

    add_button = c_w.widget_taskbar.layout().itemAt(0).widget()
    assert add_button is not None

    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.TextInputDialog.get_text",
        classmethod(lambda *args, **kwargs: ("Workspace1")),
    )

    async with aqtbot.wait_signals([w_w.create_success, w_w.list_success]):
        aqtbot.qtbot.mouseClick(add_button, QtCore.Qt.LeftButton)

    assert w_w.layout_workspaces.count() == 1
    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    assert wk_button.name == "Workspace1"

    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.TextInputDialog.get_text",
        classmethod(lambda *args, **kwargs: ("Workspace1_Renamed")),
    )

    async with aqtbot.wait_signal(w_w.rename_success):
        aqtbot.qtbot.mouseClick(wk_button.button_rename, QtCore.Qt.LeftButton)
    assert wk_button.name == "Workspace1_Renamed"
