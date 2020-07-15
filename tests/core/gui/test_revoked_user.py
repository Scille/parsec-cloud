# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
import pytest
from PyQt5.QtWidgets import QLabel

from parsec.api.data import RevokedUserCertificateContent


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
    # Users widget
    users_w = await logged_gui.test_switch_to_users_widget(error=True)
    assert users_w.layout_users.count() == 1
    assert isinstance(users_w.layout_users.itemAt(0).widget(), QLabel)
    # Devices widget
    devices_w = await logged_gui.test_switch_to_devices_widget(error=True)
    assert isinstance(devices_w.layout_devices.itemAt(0).widget(), QLabel)
