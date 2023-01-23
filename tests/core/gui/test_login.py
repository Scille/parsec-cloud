# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pathlib

import pytest
from PyQt5 import QtCore, QtWidgets

from parsec._parsec import list_available_devices, save_device_with_password_in_config
from parsec.core.gui.central_widget import CentralWidget
from parsec.core.gui.login_widget import (
    LoginAccountsWidget,
    LoginNoDevicesWidget,
    LoginPasswordInputWidget,
    LoginWidget,
)
from parsec.core.gui.parsec_application import ParsecApp
from parsec.core.local_device import load_device_file


@pytest.mark.gui
@pytest.mark.trio
async def test_login(aqtbot, gui_factory, autoclose_dialog, core_config, alice, monkeypatch):
    # Create an existing device before starting the gui
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    tabw = gui.test_get_tab()

    def _devices_listed():
        assert lw.widget.layout().count() > 0

    await aqtbot.wait_until(_devices_listed)

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w

    # Fix the return value of ensure_string_size, because it can depend of the size of the window
    monkeypatch.setattr(
        "parsec.core.gui.main_window.ensure_string_size", lambda s, size, font: (s[:16] + "...")
    )

    # Only one device, we skip the device selection

    def _password_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)

    await aqtbot.wait_until(_password_widget_shown)

    password_w = lw.widget.layout().itemAt(0).widget()

    await aqtbot.key_clicks(password_w.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        aqtbot.mouse_click(password_w.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None
    assert (
        central_widget.button_user.text()
        == f"{alice.organization_id.str}\n{alice.short_user_display}"
    )
    assert gui.tab_center.tabText(0) == "CoolOrg - Alicey..."


@pytest.mark.gui
@pytest.mark.trio
async def test_login_back_to_account_list(
    aqtbot, gui_factory, autoclose_dialog, core_config, alice, bob
):
    # Create an existing device before starting the gui
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)
    await save_device_with_password_in_config(core_config.config_dir, bob, password)

    gui = await gui_factory()
    lw = gui.test_get_login_widget()

    def _devices_listed():
        assert lw.widget.layout().count() > 0

    await aqtbot.wait_until(_devices_listed)

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w

    async with aqtbot.wait_signal(accounts_w.account_clicked):
        aqtbot.mouse_click(
            accounts_w.accounts_widget.layout().itemAt(0).widget(), QtCore.Qt.LeftButton
        )

    def _password_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)

    await aqtbot.wait_until(_password_widget_shown)

    password_w = lw.widget.layout().itemAt(0).widget()

    async with aqtbot.wait_signal(password_w.back_clicked):
        aqtbot.mouse_click(password_w.button_back, QtCore.Qt.LeftButton)

    def _account_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginAccountsWidget)


@pytest.mark.gui
@pytest.mark.trio
async def test_login_no_devices(aqtbot, gui_factory, autoclose_dialog):
    gui = await gui_factory(skip_dialogs=False)
    lw = gui.test_get_login_widget()

    def _devices_listed():
        assert lw.widget.layout().count() > 0

    await aqtbot.wait_until(_devices_listed)

    no_device_w = lw.widget.layout().itemAt(0).widget()
    assert isinstance(no_device_w, LoginNoDevicesWidget)


@pytest.mark.gui
@pytest.mark.trio
async def test_login_device_list(
    aqtbot, gui_factory, autoclose_dialog, core_config, alice, bob, alice2, adam, otheralice
):
    password = "P@ssw0rd"

    local_devices = [alice, bob, alice2, otheralice, adam]
    devices = []

    for d in local_devices:
        path = await save_device_with_password_in_config(core_config.config_dir, d, password)
        devices.append(load_device_file(pathlib.Path(path)))

    # Settings the last device used
    core_config = core_config.evolve(gui_last_device=bob.device_id.str)

    gui = await gui_factory(core_config=core_config)
    lw = gui.test_get_login_widget()

    def _accounts_widget_listed():
        assert lw.widget.layout().count() == 1

    await aqtbot.wait_until(_accounts_widget_listed)

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w

    def _devices_listed():
        # 5 devices, 1 spacer
        assert accounts_w.accounts_widget.layout().count() == len(devices) + 1

    await aqtbot.wait_until(_devices_listed)

    for idx in range(len(devices)):
        acc_w = accounts_w.accounts_widget.layout().itemAt(idx).widget()
        assert acc_w.device in devices
        assert acc_w.device.device_display == acc_w.label_device.text()
        assert acc_w.device.short_user_display == acc_w.label_name.text()
        assert acc_w.device.organization_id.str == acc_w.label_organization.text()
        # We set the last_device in the config, the first one in the list should be bob
        if idx == 0:
            assert acc_w.device.device_id == bob.device_id
        devices.remove(acc_w.device)

    assert len(devices) == 0


@pytest.mark.gui
@pytest.mark.trio
async def test_login_no_available_devices(
    aqtbot, gui_factory, autoclose_dialog, core_config, alice
):
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)

    device = list_available_devices(core_config.config_dir)[0]

    gui = await gui_factory()

    ParsecApp.add_connected_device(device.organization_id, device.device_id)

    lw = gui.test_get_login_widget()

    lw.reload_devices()

    def _devices_listed():
        assert lw.widget.layout().count() > 0

    await aqtbot.wait_until(_devices_listed)

    no_device_w = lw.widget.layout().itemAt(0).widget()
    assert isinstance(no_device_w, LoginNoDevicesWidget)
    # 0 is spacer, 1 is label
    assert no_device_w.layout().itemAt(2).widget().text() == "Create an organization"
    assert no_device_w.layout().itemAt(3).widget().text() == "Join an organization"
    assert no_device_w.layout().itemAt(4).widget().text() == "Recover a device"


@pytest.mark.gui
@pytest.mark.trio
async def test_login_logout_account_list_refresh(
    aqtbot, gui_factory, autoclose_dialog, core_config, alice, bob
):
    # Create two devices before starting the gui
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)
    await save_device_with_password_in_config(core_config.config_dir, bob, password)

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    tabw = gui.test_get_tab()

    def _devices_listed():
        assert lw.widget.layout().count() > 0

    await aqtbot.wait_until(_devices_listed)

    acc_w = lw.widget.layout().itemAt(0).widget()
    assert acc_w

    # 3 because we have a spacer
    assert acc_w.accounts_widget.layout().count() == 3

    async with aqtbot.wait_signal(acc_w.account_clicked):
        aqtbot.mouse_click(acc_w.accounts_widget.layout().itemAt(0).widget(), QtCore.Qt.LeftButton)

    def _password_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)

    await aqtbot.wait_until(_password_widget_shown)

    await aqtbot.wait_until(_devices_listed)

    password_w = lw.widget.layout().itemAt(0).widget()

    await aqtbot.key_clicks(password_w.line_edit_password, password)

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        aqtbot.mouse_click(password_w.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None
    corner_widget = gui.tab_center.cornerWidget(QtCore.Qt.TopLeftCorner)
    assert isinstance(corner_widget, QtWidgets.QPushButton)
    assert corner_widget.isVisible()

    # Now add a new tab
    aqtbot.mouse_click(corner_widget, QtCore.Qt.LeftButton)

    def _switch_to_login_tab():
        assert gui.tab_center.count() == 2
        gui.tab_center.setCurrentIndex(1)
        assert isinstance(gui.test_get_login_widget(), LoginWidget)

    await aqtbot.wait_until(_switch_to_login_tab)

    def _devices_listed():
        assert gui.test_get_login_widget().widget.layout().count() > 0

    await aqtbot.wait_until(_devices_listed)

    acc_w = gui.test_get_login_widget().widget.layout().itemAt(0).widget()
    # Skipping device selection because we have only one device
    assert isinstance(acc_w, LoginPasswordInputWidget)
    assert not acc_w.button_back.isVisible()

    def _switch_to_main_tab():
        gui.tab_center.setCurrentIndex(0)
        assert isinstance(gui.test_get_central_widget(), CentralWidget)

    await aqtbot.wait_until(_switch_to_main_tab)
    await gui.test_logout()

    assert gui.tab_center.count() == 1

    def _wait_devices_refreshed():
        assert gui.test_get_login_widget() is not None
        assert gui.test_get_login_widget().widget.layout().itemAt(0) is not None
        acc_w = gui.test_get_login_widget().widget.layout().itemAt(0).widget()
        assert acc_w.accounts_widget.layout().count() == 3

    await aqtbot.wait_until(_wait_devices_refreshed)
