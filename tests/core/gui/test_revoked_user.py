# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5.QtWidgets import QLabel


@pytest.mark.gui
@pytest.mark.trio
async def test_revoked_notification(
    aqtbot, running_backend, backend, autoclose_dialog, logged_gui, alice, bob
):

    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None

    await backend.user.revoke_user(
        organization_id=alice.organization_id,
        user_id=bob.user_id,
        revoked_user_certificate=b"dummy",
        revoked_user_certifier=alice.device_id,
    )

    # Assert dialog
    def _revoked_notified():
        assert autoclose_dialog.dialogs == [("Error", "This device is revoked")]

    await aqtbot.wait_until(_revoked_notified)

    # Users widget
    users_w = await logged_gui.test_switch_to_users_widget(error=True)
    assert users_w.layout_users.count() == 1
    assert isinstance(users_w.layout_users.itemAt(0).widget(), QLabel)

    # Devices widget
    devices_w = await logged_gui.test_switch_to_devices_widget(error=True)
    assert isinstance(devices_w.layout_devices.itemAt(0).widget(), QLabel)
