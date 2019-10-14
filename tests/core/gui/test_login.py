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
    tabw = gui.test_get_tab()

    assert lw is not None
    assert llw is not None

    # Available device is automatically selected for login
    assert llw.combo_login.currentText() == f"{alice.organization_id}:{alice.device_id}"

    # Auth by password by default
    assert llw.check_box_use_pkcs11.checkState() == QtCore.Qt.Unchecked

    await aqtbot.key_clicks(llw.line_edit_password, password)

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)

    lw = gui.test_get_login_widget()
    assert lw is None

    cw = gui.test_get_central_widget()
    assert cw is not None


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_org_missing_fields(aqtbot, gui_factory, autoclose_dialog, core_config):
    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    await aqtbot.mouse_click(lw.button_bootstrap_instead, QtCore.Qt.LeftButton)
    bw = gui.test_get_bootstrap_organization_widget()
    assert bw is not None

    assert bw.button_bootstrap.isEnabled() is False

    await aqtbot.key_clicks(bw.line_edit_login, "login")
    assert bw.button_bootstrap.isEnabled() is False

    await aqtbot.key_clicks(bw.line_edit_device, "device")
    assert bw.button_bootstrap.isEnabled() is False

    await aqtbot.key_clicks(bw.line_edit_url, "ws://localhost:5000")
    assert bw.button_bootstrap.isEnabled() is False

    await aqtbot.key_clicks(bw.line_edit_password, "passwor")
    assert bw.button_bootstrap.isEnabled() is False

    await aqtbot.key_clicks(bw.line_edit_password, "d")
    assert bw.button_bootstrap.isEnabled() is False

    await aqtbot.key_clicks(bw.line_edit_password_check, "password")
    assert bw.button_bootstrap.isEnabled() is True

    await aqtbot.key_click(bw.line_edit_password, QtCore.Qt.Key_Backspace)
    assert bw.button_bootstrap.isEnabled() is False


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user_missing_fields(aqtbot, gui_factory, autoclose_dialog, core_config):
    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    await aqtbot.mouse_click(lw.button_register_user_instead, QtCore.Qt.LeftButton)
    ruw = gui.test_get_claim_user_widget()
    assert ruw is not None

    assert ruw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(ruw.line_edit_login, "login")
    assert ruw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(ruw.line_edit_device, "device")
    assert ruw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(ruw.line_edit_url, "ws://localhost:5000")
    assert ruw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(ruw.line_edit_token, "token")
    assert ruw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(ruw.line_edit_password, "passwor")
    assert ruw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(ruw.line_edit_password, "d")
    assert ruw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(ruw.line_edit_password_check, "password")
    assert ruw.button_claim.isEnabled() is True

    await aqtbot.key_click(ruw.line_edit_password, QtCore.Qt.Key_Backspace)
    assert ruw.button_claim.isEnabled() is False


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_device_missing_fields(aqtbot, gui_factory, autoclose_dialog, core_config):
    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    await aqtbot.mouse_click(lw.button_register_device_instead, QtCore.Qt.LeftButton)
    rdw = gui.test_get_claim_device_widget()
    assert rdw is not None

    assert rdw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(rdw.line_edit_login, "login")
    assert rdw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(rdw.line_edit_device, "device")
    assert rdw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(rdw.line_edit_url, "ws://localhost:5000")
    assert rdw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(rdw.line_edit_token, "token")
    assert rdw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(rdw.line_edit_password, "passwor")
    assert rdw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(rdw.line_edit_password, "d")
    assert rdw.button_claim.isEnabled() is False

    await aqtbot.key_clicks(rdw.line_edit_password_check, "password")
    assert rdw.button_claim.isEnabled() is True

    await aqtbot.key_click(rdw.line_edit_password, QtCore.Qt.Key_Backspace)
    assert rdw.button_claim.isEnabled() is False
