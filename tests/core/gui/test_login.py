# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.local_device import save_device_with_password
from parsec.core.gui.login_widget import (
    LoginPasswordInputWidget,
    LoginAccountsWidget,
    LoginNoDevicesWidget,
)


@pytest.mark.gui
@pytest.mark.trio
async def test_login(aqtbot, gui_factory, autoclose_dialog, core_config, alice):
    # Create an existing device before starting the gui
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    tabw = gui.test_get_tab()

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w

    async with aqtbot.wait_signal(accounts_w.account_clicked):
        await aqtbot.mouse_click(
            accounts_w.accounts_widget.layout().itemAt(0).widget(), QtCore.Qt.LeftButton
        )

    def _password_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)

    await aqtbot.wait_until(_password_widget_shown)

    password_w = lw.widget.layout().itemAt(0).widget()

    await aqtbot.key_clicks(password_w.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(password_w.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None
    assert (
        central_widget.button_user.text() == f"{alice.organization_id}\n{alice.short_user_display}"
    )


@pytest.mark.gui
@pytest.mark.trio
async def test_login_back_to_account_list(
    aqtbot, gui_factory, autoclose_dialog, core_config, alice
):
    # Create an existing device before starting the gui
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)

    gui = await gui_factory()
    lw = gui.test_get_login_widget()

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w

    async with aqtbot.wait_signal(accounts_w.account_clicked):
        await aqtbot.mouse_click(
            accounts_w.accounts_widget.layout().itemAt(0).widget(), QtCore.Qt.LeftButton
        )

    def _password_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)

    await aqtbot.wait_until(_password_widget_shown)

    password_w = lw.widget.layout().itemAt(0).widget()

    async with aqtbot.wait_signal(password_w.back_clicked):
        await aqtbot.mouse_click(password_w.button_back, QtCore.Qt.LeftButton)

    def _account_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginAccountsWidget)


@pytest.mark.gui
@pytest.mark.trio
async def test_login_no_devices(aqtbot, gui_factory, autoclose_dialog, core_config):
    gui = await gui_factory()
    lw = gui.test_get_login_widget()

    no_device_w = lw.widget.layout().itemAt(0).widget()
    assert isinstance(no_device_w, LoginNoDevicesWidget)


@pytest.mark.gui
@pytest.mark.trio
async def test_login_device_list(aqtbot, gui_factory, autoclose_dialog, core_config, alice):
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)

    gui = await gui_factory()
    lw = gui.test_get_login_widget()

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w

    assert accounts_w.accounts_widget.layout().count() == 2
    alice_w = accounts_w.accounts_widget.layout().itemAt(0).widget()

    assert alice_w.label_device.text() == "My dev1 machine"
    assert alice_w.label_name.text() == "Alicey McAliceFace"
    assert alice_w.label_organization.text() == "CoolOrg"
