# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.local_device import save_device_with_password
from parsec.core.gui.password_change_dialog import PasswordChangeDialog


@pytest.fixture
async def logged_gui(aqtbot, gui_factory, running_backend, autoclose_dialog, core_config, alice):
    save_device_with_password(core_config.config_dir, alice, "P@ssw0rd")

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    llw = gui.test_get_login_login_widget()
    tabw = gui.test_get_tab()

    assert llw is not None

    await aqtbot.key_clicks(llw.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None

    await aqtbot.mouse_click(central_widget.menu.button_devices, QtCore.Qt.LeftButton)

    yield gui


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_invalid_old_password(
    aqtbot, running_backend, logged_gui, qt_thread_gateway, autoclose_dialog
):
    d_w = logged_gui.test_get_devices_widget()

    assert d_w is not None
    async with aqtbot.wait_signal(d_w.list_success):
        pass
    assert d_w.layout_devices.count() == 2
    item = d_w.layout_devices.itemAt(0)
    assert item.widget().is_current_device is True

    def _create_change_password_dialog():
        dlg = PasswordChangeDialog(core=d_w.core, parent=d_w)
        dlg.line_edit_old_password.setText("0123456789")
        dlg.line_edit_password.setText("P@ssw0rd2")
        dlg.line_edit_password_check.setText("P@ssw0rd2")
        return dlg

    dlg = await qt_thread_gateway.send_action(_create_change_password_dialog)

    await aqtbot.mouse_click(dlg.button_change, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [("Error", "Your old password is invalid.")]


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_invalid_password_check(
    aqtbot, running_backend, logged_gui, qt_thread_gateway, autoclose_dialog
):
    d_w = logged_gui.test_get_devices_widget()

    assert d_w is not None
    async with aqtbot.wait_signal(d_w.list_success):
        pass
    assert d_w.layout_devices.count() == 2
    item = d_w.layout_devices.itemAt(0)
    assert item.widget().is_current_device is True

    def _create_change_password_dialog():
        dlg = PasswordChangeDialog(core=d_w.core, parent=d_w)
        dlg.line_edit_old_password.setText("P@ssw0rd")
        dlg.line_edit_password.setText("P@ssw0rd2")
        dlg.line_edit_password_check.setText("P@ssw0rd3")
        return dlg

    dlg = await qt_thread_gateway.send_action(_create_change_password_dialog)

    await aqtbot.mouse_click(dlg.button_change, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [("Error", "Passwords don't match.")]


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_success(
    aqtbot, running_backend, logged_gui, qt_thread_gateway, autoclose_dialog
):
    d_w = logged_gui.test_get_devices_widget()

    assert d_w is not None
    async with aqtbot.wait_signal(d_w.list_success):
        pass
    assert d_w.layout_devices.count() == 2
    item = d_w.layout_devices.itemAt(0)
    assert item.widget().is_current_device is True

    def _create_change_password_dialog():
        dlg = PasswordChangeDialog(core=d_w.core, parent=d_w)
        dlg.line_edit_old_password.setText("P@ssw0rd")
        dlg.line_edit_password.setText("P@ssw0rd2")
        dlg.line_edit_password_check.setText("P@ssw0rd2")
        return dlg

    dlg = await qt_thread_gateway.send_action(_create_change_password_dialog)

    await aqtbot.mouse_click(dlg.button_change, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [
        ("Information", "Your password has been successfully changed.")
    ]
    autoclose_dialog.reset()

    central_widget = logged_gui.test_get_central_widget()
    tabw = logged_gui.test_get_tab()
    assert central_widget is not None

    async with aqtbot.wait_signal(tabw.logged_out):
        await aqtbot.mouse_click(central_widget.menu.button_logout, QtCore.Qt.LeftButton)

    lw = logged_gui.test_get_login_widget()
    llw = logged_gui.test_get_login_login_widget()
    tabw = logged_gui.test_get_tab()

    assert llw is not None

    await aqtbot.key_clicks(llw.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signal(lw.login_with_password_clicked):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [("Error", "Authentication failed.")]
    await aqtbot.key_clicks(llw.line_edit_password, "2")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)
