# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pendulum
import pytest
from parsec.core.gui.lang import translate


@pytest.mark.gui
@pytest.mark.trio
async def test_offline_notification(aqtbot, running_backend, logged_gui):
    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None

    # Assert connected
    def _online():
        assert central_widget.menu.label_connection_state.text() == translate(
            "TEXT_BACKEND_STATE_CONNECTED"
        )

    await aqtbot.wait_until(_online)

    # Assert offline
    def _offline():
        assert central_widget.menu.label_connection_state.text() == translate(
            "TEXT_BACKEND_STATE_DISCONNECTED"
        )

    async with aqtbot.wait_signal(central_widget.systray_notification):
        with running_backend.offline():
            await aqtbot.wait_until(_offline)


@pytest.mark.gui
@pytest.mark.trio
async def test_backend_desync_notification(
    aqtbot, running_backend, logged_gui, monkeypatch, autoclose_dialog
):
    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None
    minutes = 0

    def _timestamp(self):
        return pendulum.now().subtract(minutes=minutes)

    monkeypatch.setattr("parsec.api.protocol.BaseClientHandshake.timestamp", _timestamp)
    monkeypatch.setattr("parsec.core.types.local_device.LocalDevice.timestamp", _timestamp)
    monkeypatch.setattr("parsec.core.backend_connection.authenticated.DESYNC_RETRY_TIME", 0.1)

    def _online():
        assert central_widget.menu.label_connection_state.text() == translate(
            "TEXT_BACKEND_STATE_CONNECTED"
        )

    def _offline():
        assert central_widget.menu.label_connection_state.text() == translate(
            "TEXT_BACKEND_STATE_DISCONNECTED"
        )

    def _assert_desync_dialog():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [("Error", translate("TEXT_BACKEND_STATE_DESYNC"))]

    # Wait until we're online
    await aqtbot.wait_until(_online)

    # Shift by 5 minutes
    minutes = 5

    # Wait until we get the notification
    async with aqtbot.wait_signal(central_widget.systray_notification):
        await aqtbot.wait_until(_offline)

    # Wait for the dialog
    await aqtbot.wait_until(_assert_desync_dialog)

    # Wait half a second
    await aqtbot.wait(500)

    # There should not more more dialogs than before
    await aqtbot.wait_until(_assert_desync_dialog)

    # Re-sync
    minutes = 0
    await aqtbot.wait_until(_online)

    # Shift again
    minutes = 5

    # Wait until we get the notification
    await aqtbot.wait_until(_offline)

    # There should not more more dialogs than before
    await aqtbot.wait_until(_assert_desync_dialog)
