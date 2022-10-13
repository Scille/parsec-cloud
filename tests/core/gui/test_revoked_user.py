# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from PyQt5.QtWidgets import QLabel


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("wait_idle_core", [False, True])
async def test_revoked_notification(
    aqtbot,
    running_backend,
    backend,
    autoclose_dialog,
    logged_gui,
    alice,
    bob,
    wait_idle_core,
    snackbar_catcher,
):

    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None

    # Revokation might come when the core is busy, or idle
    # We're testing this because the internal detection of the revokation might differ,
    # but we still want to make sure the dialog pops up properly
    # TODO: this parametrization could go directly to the logged_gui fixture
    if wait_idle_core:
        await central_widget.core.wait_idle_monitors()

    await backend.user.revoke_user(
        organization_id=alice.organization_id,
        user_id=bob.user_id,
        revoked_user_certificate=b"dummy",
        revoked_user_certifier=alice.device_id,
    )

    # Assert dialog
    def _revoked_notified():
        assert snackbar_catcher.snackbars == [("WARN", "This device is revoked")]

    await aqtbot.wait_until(_revoked_notified)

    # Users widget
    users_w = await logged_gui.test_switch_to_users_widget(error=True)
    assert users_w.layout_users.count() == 1
    assert isinstance(users_w.layout_users.itemAt(0).widget(), QLabel)

    # Devices widget
    devices_w = await logged_gui.test_switch_to_devices_widget(error=True)
    assert isinstance(devices_w.layout_devices.itemAt(0).widget(), QLabel)
