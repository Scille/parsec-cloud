# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from tests.common import customize_fixtures


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)  # Use alice given she has multiple devices
async def test_list_devices(aqtbot, running_backend, logged_gui):
    d_w = await logged_gui.test_switch_to_devices_widget()

    assert d_w.layout_devices.count() == 2
    item = d_w.layout_devices.itemAt(0)
    assert item.widget().label_device_name.text() == "My dev1 machine"
    assert item.widget().label_is_current.text() == "(current)"
    item = d_w.layout_devices.itemAt(1)
    assert item.widget().label_device_name.text() == "My dev2 machine"


@pytest.mark.gui
@pytest.mark.trio
async def test_list_devices_offline(aqtbot, logged_gui, autoclose_dialog):
    d_w = await logged_gui.test_switch_to_devices_widget(error=True)
    assert d_w.layout_devices.count() == 0
    assert not autoclose_dialog.dialogs


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)  # Use alice given she has multiple devices
async def test_filter_devices(aqtbot, running_backend, logged_gui):
    d_w = await logged_gui.test_switch_to_devices_widget()

    assert d_w.layout_devices.count() == 2
    dev1_w = d_w.layout_devices.itemAt(0).widget()
    dev2_w = d_w.layout_devices.itemAt(1).widget()

    assert dev1_w.isVisible() is True
    assert dev2_w.isVisible() is True

    async with aqtbot.wait_signal(d_w.filter_timer.timeout):
        await aqtbot.key_clicks(d_w.line_edit_search, "2")

    assert dev1_w.isVisible() is False
    assert dev2_w.isVisible() is True

    async with aqtbot.wait_signal(d_w.filter_timer.timeout):
        await aqtbot.run(lambda: d_w.line_edit_search.setText(""))

    assert dev1_w.isVisible() is True
    assert dev2_w.isVisible() is True

    async with aqtbot.wait_signal(d_w.filter_timer.timeout):
        await aqtbot.key_clicks(d_w.line_edit_search, "1")

    assert dev1_w.isVisible() is True
    assert dev2_w.isVisible() is False
