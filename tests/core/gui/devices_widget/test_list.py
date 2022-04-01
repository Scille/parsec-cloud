# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from PyQt5.QtWidgets import QLabel
from tests.common import customize_fixtures
from parsec.core.gui.lang import translate


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)  # Use alice given she has multiple devices
async def test_list_devices(aqtbot, running_backend, logged_gui):
    d_w = await logged_gui.test_switch_to_devices_widget()

    assert d_w.layout_devices.count() == 2
    item = d_w.layout_devices.itemAt(0)
    label_device0 = item.widget().label_device_name.text()
    label_is_current0 = item.widget().label_is_current.text()
    item = d_w.layout_devices.itemAt(1)
    label_device1 = item.widget().label_device_name.text()
    label_is_current1 = item.widget().label_is_current.text()

    if label_device0 == "My dev1 machine":
        assert label_device1 == "My dev2 machine"
        assert label_is_current0 == "(current)"
    else:
        assert label_device0 == "My dev2 machine"
        assert label_device1 == "My dev1 machine"
        assert label_is_current1 == "(current)"


@pytest.mark.gui
@pytest.mark.trio
async def test_list_devices_offline(aqtbot, logged_gui, autoclose_dialog):
    d_w = await logged_gui.test_switch_to_devices_widget(error=True)
    assert d_w.layout_devices.count() == 1
    error_msg = d_w.layout_devices.itemAt(0).widget()
    assert isinstance(error_msg, QLabel)
    assert error_msg.text() == translate("TEXT_DEVICE_LIST_RETRIEVABLE_FAILURE")

    assert not autoclose_dialog.dialogs
