# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from libparsec.types import DateTime

from parsec.api.data import EntryName
from parsec.core.gui.lang import translate
from parsec.core.backend_connection.authenticated import DESYNC_RETRY_TIME

from tests.common import real_clock_timeout


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


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
async def test_backend_desync_notification(
    aqtbot,
    frozen_clock,
    running_backend,
    logged_gui,
    monkeypatch,
    autoclose_dialog,
    caplog,
    snackbar_catcher,
):
    central_widget = logged_gui.test_get_central_widget()
    assert central_widget is not None
    timestamp_shift_minutes = 0

    def _timestamp(self):
        return DateTime.now().subtract(minutes=timestamp_shift_minutes)

    monkeypatch.setattr("parsec.api.protocol.BaseClientHandshake.timestamp", _timestamp)
    monkeypatch.setattr("parsec.core.types.local_device.LocalDevice.timestamp", _timestamp)

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
    await frozen_clock.sleep_with_autojump(DESYNC_RETRY_TIME)
    await aqtbot.wait_until(_online)

    # Shift by 5 minutes
    timestamp_shift_minutes = 5

    # Wait until we get the notification
    async with aqtbot.wait_signal(central_widget.systray_notification):

        # Force sync by creating a workspace
        await central_widget.core.user_fs.workspace_create(EntryName("test1"))

        # Wait until we're offline
        await frozen_clock.sleep_with_autojump(DESYNC_RETRY_TIME)
        await aqtbot.wait_until(_offline)

    # Wait for the dialog
    await aqtbot.wait_until(_assert_desync_dialog)

    # Clear dialogs
    autoclose_dialog.dialogs.clear()

    # Wait for a few reconnections
    for _ in range(3):
        caplog.clear()
        async with real_clock_timeout():
            with caplog.at_level("INFO"):
                await frozen_clock.sleep_with_autojump(DESYNC_RETRY_TIME)
            caplog.assert_occured(
                "[info     ] Backend connection is desync   [parsec.core.backend_connection.authenticated]"
            )

    # There should be no new dialog
    assert len(autoclose_dialog.dialogs) == 0

    # Re-sync
    timestamp_shift_minutes = 0
    await frozen_clock.sleep_with_autojump(DESYNC_RETRY_TIME)
    await aqtbot.wait_until(_online)

    # Shift again
    timestamp_shift_minutes = 5

    # Force sync by creating a workspace
    await central_widget.core.user_fs.workspace_create(EntryName("test2"))

    # Wait until we get the notification
    await frozen_clock.sleep_with_autojump(DESYNC_RETRY_TIME)
    await aqtbot.wait_until(_offline)

    # There should be no new dialog
    assert len(autoclose_dialog.dialogs) == 0
