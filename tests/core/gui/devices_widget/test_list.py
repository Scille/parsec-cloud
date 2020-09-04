# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from PyQt5.QtWidgets import QLabel
from tests.common import customize_fixtures
from parsec.core.gui.lang import translate
from PyQt5 import QtCore
from PyQt5.Qt import Qt


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
    assert d_w.layout_devices.count() == 1
    error_msg = d_w.layout_devices.itemAt(0).widget()
    assert isinstance(error_msg, QLabel)
    assert error_msg.text() == translate("TEXT_DEVICE_LIST_RETRIEVABLE_FAILURE")

    assert not autoclose_dialog.dialogs


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)  # Use alice given she has multiple devices
async def test_filter_devices(aqtbot, running_backend, logged_gui):
    def _devices_shown(count: int):
        assert d_w.layout_devices.count() == count
        items = (d_w.layout_devices.itemAt(i) for i in range(d_w.layout_devices.count()))
        for item in items:
            widget = item.widget()
            assert widget.isVisible() is True

    def _all_devices_visible(d_w):
        assert d_w.layout_devices.count() == 2
        dev1_w = d_w.layout_devices.itemAt(0).widget()
        dev2_w = d_w.layout_devices.itemAt(1).widget()

        assert dev1_w.isVisible() is True
        assert dev2_w.isVisible() is True

    d_w = await logged_gui.test_switch_to_devices_widget()
    await aqtbot.wait_until(lambda: _all_devices_visible(d_w=d_w))

    async with aqtbot.wait_signal(d_w.list_success):
        await aqtbot.key_clicks(d_w.line_edit_search, "2")
        await aqtbot.mouse_click(d_w.button_devices_filter, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(lambda: _devices_shown(count=1))

    dev2_w = d_w.layout_devices.itemAt(0).widget()

    assert dev2_w.isVisible() is True

    with pytest.raises(AttributeError):
        dev1_w = d_w.layout_devices.itemAt(1).widget()

    async with aqtbot.wait_signal(d_w.list_success):
        await aqtbot.wait_until(lambda: d_w.line_edit_search.setText(""))

    await aqtbot.wait_until(lambda: _all_devices_visible(d_w=d_w))

    async with aqtbot.wait_signal(d_w.list_success):
        await aqtbot.key_clicks(d_w.line_edit_search, "1")
        await aqtbot.key_press(d_w.line_edit_search, Qt.Key_Enter)

    assert d_w.layout_devices.count() == 1

    dev1_w = d_w.layout_devices.itemAt(0).widget()

    assert dev1_w.isVisible() is True

    with pytest.raises(AttributeError):
        dev2_w = d_w.layout_devices.itemAt(1).widget()
