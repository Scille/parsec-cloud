# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
import pendulum

import pytest

from parsec.core.gui.lang import translate


async def wait_for_log(caplog, logger, level, message, timeout=1.0, tick=0.01):
    caplog.clear()
    caplog.set_level(level)
    with trio.fail_after(timeout):
        while True:
            await trio.sleep(tick)
            for record in caplog.records:
                if record.name == logger and message in record.message:
                    return record


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
    aqtbot, running_backend, logged_gui, monkeypatch, autoclose_dialog, caplog, snackbar_catcher
):
    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None
    timestamp_shift_minutes = 0

    def _timestamp(self):
        return pendulum.now().subtract(minutes=timestamp_shift_minutes)

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
        assert snackbar_catcher.snackbars == [("WARN", translate("TEXT_BACKEND_STATE_DESYNC"))]

    # Wait until we're online
    await aqtbot.wait_until(_online)

    # Shift by 5 minutes
    timestamp_shift_minutes = 5

    # Wait until we get the notification
    async with aqtbot.wait_signal(central_widget.systray_notification):

        # Force sync by creating a workspace
        await central_widget.core.user_fs.workspace_create("test1")

        # Wait until we're offline
        await aqtbot.wait_until(_offline, timeout=3000)

    # Wait for the dialog
    await aqtbot.wait_until(_assert_desync_dialog)

    # Clear dialogs
    autoclose_dialog.dialogs.clear()

    # Wait for a few reconnections
    for _ in range(3):
        await wait_for_log(
            caplog,
            "parsec.core.backend_connection.authenticated",
            "INFO",
            "Backend connection is desync",
        )

    # There should be no new dialog
    assert len(autoclose_dialog.dialogs) == 0

    # Re-sync
    # DESYNC_RETRY_TIME has been monkeypatched so this should take less than 100 ms
    timestamp_shift_minutes = 0
    await aqtbot.wait_until(_online)

    # Shift again
    timestamp_shift_minutes = 5

    # Force sync by creating a workspace
    await central_widget.core.user_fs.workspace_create("test2")

    # Wait until we get the notification
    await aqtbot.wait_until(_offline, timeout=3000)

    # There should be no new dialog
    assert len(autoclose_dialog.dialogs) == 0
