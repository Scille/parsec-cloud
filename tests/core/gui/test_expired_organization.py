# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.backend_connection import (
    BackendConnectionRefused,
    backend_authenticated_cmds_factory,
)
from parsec.core.local_device import save_device_with_password
from tests.common import freeze_time


@pytest.mark.gui
@pytest.mark.trio
async def test_expired_notification_logging(
    aqtbot, running_backend, backend, autoclose_dialog, expiredorgalice, gui_factory, core_config
):

    # Log has alice on an expired organization
    save_device_with_password(core_config.config_dir, expiredorgalice, "P@ssw0rd")

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    tabw = gui.test_get_tab()

    await aqtbot.key_clicks(lw.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(lw.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None

    # Assert dialog
    def _expired_notified():
        assert autoclose_dialog.dialogs == [("Error", "The organization has expired")]

    await aqtbot.wait_until(_expired_notified)


@pytest.mark.gui
@pytest.mark.trio
async def test_on_expired_notification(
    aqtbot, running_backend, backend, autoclose_dialog, expiredorgalice, gui_factory, core_config
):
    save_device_with_password(core_config.config_dir, expiredorgalice, "P@ssw0rd")
    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    tabw = gui.test_get_tab()

    # Force logging on an expired organization
    with freeze_time("1989-12-17"):
        await aqtbot.key_clicks(lw.line_edit_password, "P@ssw0rd")

        async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
            await aqtbot.mouse_click(lw.button_login, QtCore.Qt.LeftButton)

        central_widget = gui.test_get_central_widget()
        assert central_widget is not None

        # Assert logged in
        def _notified():
            assert autoclose_dialog.dialogs == []

        await aqtbot.wait_until(_notified)

    # Trigger another handshake
    with pytest.raises(BackendConnectionRefused):
        async with backend_authenticated_cmds_factory(
            expiredorgalice.organization_addr,
            expiredorgalice.device_id,
            expiredorgalice.signing_key,
        ) as cmds:
            async with cmds.acquire_transport():
                # This shall never happen, we shall have been rejected while acquiring the transport
                assert False

    # Assert dialog
    def _expired_notified():
        assert autoclose_dialog.dialogs == [("Error", "The organization has expired")]

    await aqtbot.wait_until(_expired_notified)
