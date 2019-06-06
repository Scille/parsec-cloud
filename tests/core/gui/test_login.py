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
    llw = gui.test_get_login_login_widget()

    assert lw is not None
    assert llw is not None

    # Available device is automatically selected for login
    assert llw.combo_login.currentText() == f"{alice.organization_id}:{alice.device_id}"

    # Auth by password by default
    assert llw.check_box_use_pkcs11.checkState() == QtCore.Qt.Unchecked

    await aqtbot.key_clicks(llw.line_edit_password, password)

    async with aqtbot.wait_signals([lw.login_with_password_clicked, gui.logged_in]):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)

    lw = gui.test_get_login_widget()
    assert lw is None

    cw = gui.test_get_central_widget()
    assert cw is not None
