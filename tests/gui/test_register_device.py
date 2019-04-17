# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore
from parsec.core.gui.register_device_dialog import RegisterDeviceDialog
from unittest.mock import patch

from parsec.core.local_device import save_device_with_password


@pytest.fixture
async def logged_gui(aqtbot, gui_factory, autoclose_dialog, core_config, alice):
    # Create an existing device before starting the gui
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)

    gui = await gui_factory()
    lw = gui.login_widget

    await aqtbot.key_clicks(lw.login_widget.line_edit_password, password)

    async with aqtbot.wait_signals([lw.login_with_password_clicked, gui.logged_in]):
        await aqtbot.mouse_click(lw.login_widget.button_login, QtCore.Qt.LeftButton)

    await aqtbot.mouse_click(gui.central_widget.menu.button_devices, QtCore.Qt.LeftButton)
    yield gui


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_open_modal(aqtbot, logged_gui):
    with patch("parsec.core.gui.devices_widget.RegisterDeviceDialog") as register_mock:
        await aqtbot.mouse_click(
            logged_gui.central_widget.devices_widget.taskbar_buttons[0], QtCore.Qt.LeftButton
        )
        register_mock.assert_called_once_with(
            parent=logged_gui.central_widget.devices_widget,
            portal=logged_gui.central_widget.devices_widget.portal,
            core=logged_gui.central_widget.devices_widget.core,
        )


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_ok(
    aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    def _claim_device(user_id, device_name, token, addr, password):
        claim_w = logged_gui.login_widget.claim_device_widget

        aqtbot.qtbot.keyClicks(claim_w.line_edit_login, user_id)
        claim_w.line_edit_device.setText("")
        aqtbot.qtbot.keyClicks(claim_w.line_edit_device, device_name)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_token, token)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_url, addr)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_password, password)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_password_check, password)
        aqtbot.qtbot.mouseClick(claim_w.button_claim, QtCore.Qt.LeftButton)

    def run_dialog():
        modal = RegisterDeviceDialog(
            parent=logged_gui.central_widget.devices_widget,
            portal=logged_gui.central_widget.devices_widget.portal,
            core=logged_gui.central_widget.devices_widget.core,
        )
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device")
        aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert modal.line_edit_url.text()
        assert modal.line_edit_device_name.text() == "new_device"
        assert modal.line_edit_token.text()
        with aqtbot.qtbot.waitSignal(modal.device_registered):
            _claim_device(
                "alice",
                modal.line_edit_device_name.text(),
                modal.line_edit_token.text(),
                modal.line_edit_url.text(),
                "P@ssw0rd!",
            )
        assert (
            "Information",
            "Device has been registered. You may now close this window.",
        ) in autoclose_dialog.dialogs

    await qt_thread_gateway.send_action(run_dialog)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_invalid_device_name(
    aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    def run_dialog():
        modal = RegisterDeviceDialog(
            parent=logged_gui.central_widget.devices_widget,
            portal=logged_gui.central_widget.devices_widget.portal,
            core=logged_gui.central_widget.devices_widget.core,
        )
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device" * 4)
        aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert modal.line_edit_url.text()
        assert modal.line_edit_device_name.text() == ("new_device" * 4)[:32]
        assert modal.line_edit_token.text()

    await qt_thread_gateway.send_action(run_dialog)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_cancel(
    aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    def run_dialog():
        modal = RegisterDeviceDialog(
            parent=logged_gui.central_widget.devices_widget,
            portal=logged_gui.central_widget.devices_widget.portal,
            core=logged_gui.central_widget.devices_widget.core,
        )
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device")
        aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert modal.line_edit_url.text()
        assert modal.line_edit_device_name.text() == "new_device"
        assert modal.line_edit_token.text()
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_cancel, QtCore.Qt.LeftButton)
        assert not modal.widget_registration.isVisible()

    await qt_thread_gateway.send_action(run_dialog)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_offline(
    aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    gui = logged_gui

    def run_dialog():
        modal = RegisterDeviceDialog(
            parent=gui.central_widget.devices_widget,
            portal=gui.central_widget.devices_widget.portal,
            core=gui.central_widget.devices_widget.core,
        )
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device")

        with running_backend.offline():
            with aqtbot.qtbot.waitSignal(modal.registration_error):
                aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)

    await qt_thread_gateway.send_action(run_dialog)

    assert autoclose_dialog.dialogs == [("Error", "Cannot invite a device without being online.")]


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_already_registered(
    aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    def run_dialog():
        modal = RegisterDeviceDialog(
            parent=logged_gui.central_widget.devices_widget,
            portal=logged_gui.central_widget.devices_widget.portal,
            core=logged_gui.central_widget.devices_widget.core,
        )
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "dev1")
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)

    await qt_thread_gateway.send_action(run_dialog)
    assert autoclose_dialog.dialogs == [
        (
            "Error",
            "Cannot register this device (Cannot invite device: Backend error `already_exists`: "
            "Device `alice@dev1` already exists).",
        )
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_unknown_error(
    monkeypatch, aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    async def _broken(*args, **kwargs):
        raise RuntimeError()

    monkeypatch.setattr(
        "parsec.core.gui.register_device_dialog.core_invite_and_create_device", _broken
    )

    def run_dialog():
        modal = RegisterDeviceDialog(
            parent=logged_gui.central_widget.devices_widget,
            portal=logged_gui.central_widget.devices_widget.portal,
            core=logged_gui.central_widget.devices_widget.core,
        )
        modal.show()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device")
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert autoclose_dialog.dialogs == [
            ("Error", "Cannot register this device (Unexpected error: RuntimeError()).")
        ]

    await qt_thread_gateway.send_action(run_dialog)
    # TODO: Make sure a log is emitted


# TODO test with timeout
