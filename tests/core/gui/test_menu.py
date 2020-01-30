# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.gui.menu_widget import MenuWidget


@pytest.mark.gui
def test_activate_files(qtbot):

    w = MenuWidget(parent=None)
    qtbot.addWidget(w)

    w.button_devices.setChecked(True)
    w.button_users.setChecked(True)
    assert w.button_files.isChecked() is False
    assert w.button_users.isChecked() is True
    assert w.button_devices.isChecked() is True
    w.activate_files()
    assert w.button_files.isChecked() is True
    assert w.button_users.isChecked() is False
    assert w.button_devices.isChecked() is False


@pytest.mark.gui
def test_activate_users(qtbot):

    w = MenuWidget(parent=None)
    qtbot.addWidget(w)

    w.button_files.setChecked(True)
    w.button_devices.setChecked(True)
    assert w.button_files.isChecked() is True
    assert w.button_users.isChecked() is False
    assert w.button_devices.isChecked() is True
    w.activate_users()
    assert w.button_files.isChecked() is False
    assert w.button_users.isChecked() is True
    assert w.button_devices.isChecked() is False


@pytest.mark.gui
def test_activate_devices(qtbot):

    w = MenuWidget(parent=None)
    qtbot.addWidget(w)

    w.button_files.setChecked(True)
    w.button_users.setChecked(True)
    assert w.button_files.isChecked() is True
    assert w.button_users.isChecked() is True
    assert w.button_devices.isChecked() is False
    w.activate_devices()
    assert w.button_files.isChecked() is False
    assert w.button_users.isChecked() is False
    assert w.button_devices.isChecked() is True


@pytest.mark.gui
def test_set_info(qtbot):
    w = MenuWidget(parent=None)
    qtbot.addWidget(w)

    assert w.label_username.text() == ""
    assert w.label_device.text() == ""
    assert w.label_organization.text() == ""

    w.username = "User"
    w.device = "Device"
    w.organization = "Organization"

    assert w.label_username.text() == "User"
    assert w.label_device.text() == "Device"
    assert w.label_organization.text() == "Organization"


@pytest.mark.gui
def test_clicked(qtbot):
    w = MenuWidget(parent=None)
    qtbot.addWidget(w)

    w.button_files.setChecked(True)
    w.button_users.setChecked(True)
    w.button_devices.setChecked(True)

    with qtbot.waitSignal(w.files_clicked, timeout=500):
        qtbot.mouseClick(w.button_files, QtCore.Qt.LeftButton)

    with qtbot.waitSignal(w.users_clicked, timeout=500):
        qtbot.mouseClick(w.button_users, QtCore.Qt.LeftButton)

    with qtbot.waitSignal(w.devices_clicked, timeout=500):
        qtbot.mouseClick(w.button_devices, QtCore.Qt.LeftButton)

    with qtbot.waitSignal(w.logout_clicked, timeout=500):
        qtbot.mouseClick(w.button_logout, QtCore.Qt.LeftButton)
