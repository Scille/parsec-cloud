# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
import pytest
from PyQt5 import QtCore
from PyQt5.QtWidgets import QLabel

from parsec.api.data import RevokedUserCertificateContent
from parsec.core.local_device import save_device_with_password


@pytest.fixture
async def logged_gui(aqtbot, gui_factory, running_backend, autoclose_dialog, core_config, bob):
    save_device_with_password(core_config.config_dir, bob, "P@ssw0rd")
    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    tabw = gui.test_get_tab()

    # assert lw.combo_username.currentText() == f"{bob.organization_id}:{bob.device_id}"
    await aqtbot.key_clicks(lw.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(lw.button_login, QtCore.Qt.LeftButton)

    return gui


@pytest.mark.gui
@pytest.mark.trio
async def test_revoked_notification(
    aqtbot, running_backend, autoclose_dialog, logged_gui, alice_user_fs, bob
):
    now = pendulum.now()
    revoked_device_certificate = RevokedUserCertificateContent(
        author=alice_user_fs.device.device_id, timestamp=now, user_id=bob.user_id
    ).dump_and_sign(alice_user_fs.device.signing_key)
    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None
    async with aqtbot.wait_signal(central_widget.new_notification, timeout=3000):
        await alice_user_fs.backend_cmds.user_revoke(revoked_device_certificate)

    # Assert dialog
    assert len(autoclose_dialog.dialogs) == 1
    dialog = autoclose_dialog.dialogs[0]
    assert dialog[0] == "Error"
    assert dialog[1] == "This device is revoked"

    users_w = logged_gui.test_get_users_widget()
    async with aqtbot.wait_signals(
        [central_widget.menu.button_users.clicked, users_w.list_error], timeout=3000
    ):
        await aqtbot.mouse_click(central_widget.menu.button_users, QtCore.Qt.LeftButton)

    assert users_w.layout_users.count() == 1
    assert isinstance(users_w.layout_users.itemAt(0).widget(), QLabel)

    devices_w = logged_gui.test_get_devices_widget()
    async with aqtbot.wait_signals(
        [central_widget.menu.button_devices.clicked, devices_w.list_error], timeout=3000
    ):
        await aqtbot.mouse_click(central_widget.menu.button_devices, QtCore.Qt.LeftButton)

    assert isinstance(devices_w.layout_devices.itemAt(0).widget(), QLabel)
