# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore


@pytest.fixture
def catch_password_change_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.password_change_widget.PasswordChangeWidget")


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_invalid_old_password(
    aqtbot, running_backend, logged_gui, catch_password_change_widget, autoclose_dialog
):
    d_w = await logged_gui.test_switch_to_devices_widget()

    assert d_w.layout_devices.count() == 1
    db_w = d_w.layout_devices.itemAt(0).widget()
    assert db_w.is_current_device is True

    await aqtbot.run(db_w.change_password_clicked.emit)
    pc_w = await catch_password_change_widget()

    await aqtbot.key_clicks(pc_w.line_edit_old_password, "0123456789")
    await aqtbot.key_clicks(pc_w.line_edit_password, "P@ssw0rd2")
    await aqtbot.key_clicks(pc_w.line_edit_password_check, "P@ssw0rd2")
    await aqtbot.mouse_click(pc_w.button_change, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [
        ("Error", "You did not provide the right password for this device.")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_invalid_password_check(
    aqtbot, running_backend, logged_gui, catch_password_change_widget, autoclose_dialog
):
    d_w = await logged_gui.test_switch_to_devices_widget()

    assert d_w.layout_devices.count() == 1
    db_w = d_w.layout_devices.itemAt(0).widget()
    assert db_w.is_current_device is True

    await aqtbot.run(db_w.change_password_clicked.emit)
    pc_w = await catch_password_change_widget()

    await aqtbot.key_clicks(pc_w.line_edit_old_password, "P@ssw0rd")
    await aqtbot.key_clicks(pc_w.line_edit_password, "P@ssw0rd2")
    await aqtbot.key_clicks(pc_w.line_edit_password_check, "P@ssw0rd3")
    await aqtbot.mouse_click(pc_w.button_change, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [
        ("Error", "The password and the password confirmation do no match.")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_success(
    aqtbot, running_backend, logged_gui, catch_password_change_widget, autoclose_dialog
):
    d_w = await logged_gui.test_switch_to_devices_widget()

    assert d_w.layout_devices.count() == 1
    db_w = d_w.layout_devices.itemAt(0).widget()
    assert db_w.is_current_device is True

    await aqtbot.run(db_w.change_password_clicked.emit)
    pc_w = await catch_password_change_widget()

    await aqtbot.key_clicks(pc_w.line_edit_old_password, "P@ssw0rd")
    await aqtbot.key_clicks(pc_w.line_edit_password, "P@ssw0rd2")
    await aqtbot.key_clicks(pc_w.line_edit_password_check, "P@ssw0rd2")
    await aqtbot.mouse_click(pc_w.button_change, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [("", "The password has been successfully changed.")]
    autoclose_dialog.reset()

    central_widget = logged_gui.test_get_central_widget()
    tabw = logged_gui.test_get_tab()
    assert central_widget is not None

    def _trigger_logout_menu():
        central_widget.button_user.menu().actions()[0].trigger()

    async with aqtbot.wait_signal(tabw.logged_out):
        await qt_thread_gateway.send_action(_trigger_logout_menu)

    lw = logged_gui.test_get_login_widget()
    tabw = logged_gui.test_get_tab()

    assert lw is not None

    await aqtbot.key_clicks(lw.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signal(lw.login_with_password_clicked):
        await aqtbot.mouse_click(lw.button_login, QtCore.Qt.LeftButton)

    assert len(autoclose_dialog.dialogs) == 1
    assert autoclose_dialog.dialogs[0][0] == "Error"
    assert autoclose_dialog.dialogs[0][1] == "The password is incorrect."

    # Retry to login...
    await logged_gui.test_logout_and_switch_to_login_widget()

    # ...with old password...
    await logged_gui.test_proceed_to_login("P@ssw0rd", error=True)
    assert autoclose_dialog.dialogs == [("Error", "The password is incorrect.")]

    # ...and new password
    await logged_gui.test_proceed_to_login("P@ssw0rd2")
