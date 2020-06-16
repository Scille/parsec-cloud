# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.local_device import save_device_with_password


@pytest.mark.gui
@pytest.mark.trio
async def test_login(aqtbot, gui_factory, autoclose_dialog, core_config, alice):
    # Create an existing device before starting the gui
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    tabw = gui.test_get_tab()

    assert lw is not None

    # Available device is automatically selected for login
    assert lw.combo_username.currentText() == f"{alice.organization_id}:{alice.device_id}"

    await aqtbot.key_clicks(lw.line_edit_password, password)

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(lw.button_login, QtCore.Qt.LeftButton)

    lw = gui.test_get_login_widget()
    assert lw is None

    cw = gui.test_get_central_widget()
    assert cw is not None

    assert cw.button_user.text() == f"{alice.organization_id}\n{alice.device_id.user_id}"
