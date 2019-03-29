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
    lw = gui.login_widget

    # Available device is automatically selected for login
    assert lw.login_widget.combo_login.currentText() == f"{alice.organization_id}:{alice.device_id}"

    # Auth by password by default
    assert lw.login_widget.check_box_use_pkcs11.checkState() == QtCore.Qt.Unchecked

    await aqtbot.key_clicks(lw.login_widget.line_edit_password, password)

    async with aqtbot.wait_signals([lw.login_with_password_clicked, gui.logged_in]):
        await aqtbot.mouse_click(lw.login_widget.button_login, QtCore.Qt.LeftButton)

    assert not lw.isVisible()
    assert gui.central_widget.isVisible()
