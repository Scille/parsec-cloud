# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.local_device import save_device_with_password
from parsec.core.gui.lang import switch_language


@pytest.fixture
async def logged_gui(
    aqtbot, gui_factory, running_backend, autoclose_dialog, core_config, alice, alice2
):
    save_device_with_password(core_config.config_dir, alice, "P@ssw0rd")

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    llw = gui.test_get_login_login_widget()
    tabw = gui.test_get_tab()

    assert llw is not None

    await aqtbot.key_clicks(llw.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None

    save_device_with_password(core_config.config_dir, alice2, "P@ssw0rd")
    await aqtbot.mouse_click(central_widget.menu.button_devices, QtCore.Qt.LeftButton)

    yield gui


@pytest.mark.gui
@pytest.mark.trio
async def test_list_devices(aqtbot, running_backend, logged_gui):
    d_w = logged_gui.test_get_devices_widget()

    assert d_w is not None
    async with aqtbot.wait_signal(d_w.list_success):
        pass
    assert d_w.layout_devices.count() == 2
    item = d_w.layout_devices.itemAt(0)
    assert item.widget().label_device.text() == "dev1\n(current)"
    item = d_w.layout_devices.itemAt(1)
    assert item.widget().label_device.text() == "dev2"


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("custom_locale", (False, True))
async def test_device_info(aqtbot, running_backend, autoclose_dialog, logged_gui, custom_locale):
    if custom_locale:
        switch_language(None, "fr")
    d_w = logged_gui.test_get_devices_widget()
    assert d_w is not None
    async with aqtbot.wait_signal(d_w.list_success):
        pass
    assert d_w.layout_devices.count() == 2
    item = d_w.layout_devices.itemAt(0)
    item.widget().show_device_info()
    item = d_w.layout_devices.itemAt(1)
    item.widget().show_device_info()
    if custom_locale:
        assert autoclose_dialog.dialogs == [
            ("Information", "dev1\n\nCréé le samedi 1 janvier 2000 00:00"),
            ("Information", "dev2\n\nCréé le samedi 1 janvier 2000 00:00"),
        ]
    else:
        assert autoclose_dialog.dialogs == [
            ("Information", "dev1\n\nCreated on Saturday, January 1, 2000 12:00 AM"),
            ("Information", "dev2\n\nCreated on Saturday, January 1, 2000 12:00 AM"),
        ]


@pytest.mark.gui
@pytest.mark.trio
async def test_filter_devices(aqtbot, running_backend, logged_gui):
    d_w = logged_gui.test_get_devices_widget()
    assert d_w is not None
    async with aqtbot.wait_signal(d_w.list_success):
        pass

    assert d_w.layout_devices.count() == 2
    dev1_w = d_w.layout_devices.itemAt(0).widget()
    dev2_w = d_w.layout_devices.itemAt(1).widget()

    assert dev1_w.isVisible() is True
    assert dev2_w.isVisible() is True

    async with aqtbot.wait_signal(d_w.filter_timer.timeout):
        aqtbot.qtbot.keyClicks(d_w.line_edit_search, "2")
    assert dev1_w.isVisible() is False
    assert dev2_w.isVisible() is True
    async with aqtbot.wait_signal(d_w.filter_timer.timeout):
        d_w.line_edit_search.setText("")
    assert dev1_w.isVisible() is True
    assert dev2_w.isVisible() is True
    async with aqtbot.wait_signal(d_w.filter_timer.timeout):
        aqtbot.qtbot.keyClicks(d_w.line_edit_search, "1")
    assert dev1_w.isVisible() is True
    assert dev2_w.isVisible() is False
