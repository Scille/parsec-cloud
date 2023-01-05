# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.api.data import EntryName
from parsec.core.gui.lang import translate
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


@pytest.mark.gui
@pytest.mark.trio
async def test_backend_desync_notification(
    aqtbot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    caplog,
    snackbar_catcher,
):
    central_widget = logged_gui.test_get_central_widget()
    local_device = central_widget.core.device
    assert central_widget is not None

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

    def _assert_desync_log():
        caplog.assert_occurred(
            "[info     ] Backend connection is desync   [parsec.core.backend_connection.authenticated]"
        )

    # Wait until we're online
    local_device.time_provider.mock_time(speed=1000.0)
    await aqtbot.wait_until(_online)

    # Shift by 5 minutes
    local_device.time_provider.mock_time(shift=5 * 60)

    # Wait until we get the notification
    async with aqtbot.wait_signal(central_widget.systray_notification):

        # Force sync by creating a workspace
        await central_widget.core.user_fs.workspace_create(EntryName("test1"))

        # Wait until we're offline
        local_device.time_provider.mock_time(speed=1000.0)
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
                await aqtbot.wait_until(_assert_desync_log)

    # There should be no new dialog
    assert len(autoclose_dialog.dialogs) == 0

    # Re-sync
    local_device.time_provider.mock_time(shift=0)
    local_device.time_provider.mock_time(speed=1000.0)
    await aqtbot.wait_until(_online)

    # Shift again
    local_device.time_provider.mock_time(shift=5 * 60)

    # Force sync by creating a workspace
    await central_widget.core.user_fs.workspace_create(EntryName("test2"))

    # Wait until we get the notification
    local_device.time_provider.mock_time(speed=1000.0)
    await aqtbot.wait_until(_offline)

    # There should be no new dialog
    assert len(autoclose_dialog.dialogs) == 0
