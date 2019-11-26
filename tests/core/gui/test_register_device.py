# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore
from parsec.core.types import BackendOrganizationClaimDeviceAddr
from parsec.core.gui.register_device_dialog import RegisterDeviceDialog
from unittest.mock import patch

from parsec.core.local_device import save_device_with_password


@pytest.fixture
async def logged_gui(aqtbot, gui_factory, autoclose_dialog, core_config, alice):
    # Create an existing device before starting the gui
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    llw = gui.test_get_login_login_widget()
    tabw = gui.test_get_tab()

    assert llw is not None

    await aqtbot.key_clicks(llw.line_edit_password, password)

    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None

    await aqtbot.mouse_click(central_widget.menu.button_devices, QtCore.Qt.LeftButton)
    yield gui


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_open_modal(aqtbot, logged_gui, running_backend):
    d_w = logged_gui.test_get_devices_widget()

    assert d_w is not None
    async with aqtbot.wait_signal(d_w.list_success):
        pass

    with patch("parsec.core.gui.devices_widget.RegisterDeviceDialog") as register_mock:
        await aqtbot.mouse_click(d_w.taskbar_buttons[0], QtCore.Qt.LeftButton)
        register_mock.assert_called_once_with(parent=d_w, jobs_ctx=d_w.jobs_ctx, core=d_w.core)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_ok(
    aqtbot, gui, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    d_w = logged_gui.test_get_devices_widget()
    assert d_w is not None

    def _claim_device(token, addr, password):
        l_w = gui.test_get_login_widget()

        assert l_w is not None
        l_w.show_claim_device_widget(BackendOrganizationClaimDeviceAddr.from_url(addr))

        claim_w = gui.test_get_claim_device_widget()

        assert claim_w is not None

        aqtbot.qtbot.keyClicks(claim_w.line_edit_token, token)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_password, password)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_password_check, password)
        aqtbot.qtbot.mouseClick(claim_w.button_claim, QtCore.Qt.LeftButton)

    def run_dialog():
        modal = RegisterDeviceDialog(parent=d_w, jobs_ctx=d_w.jobs_ctx, core=d_w.core)
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device")
        aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert modal.line_edit_url.text()
        assert modal.line_edit_device_name.text() == "new_device"
        assert modal.line_edit_token.text()
        with aqtbot.qtbot.waitSignal(modal.device_registered):
            _claim_device(modal.line_edit_token.text(), modal.line_edit_url.text(), "P@ssw0rd!")
        assert (
            "Information",
            "The device has been registered. You may now close this window.",
        ) in autoclose_dialog.dialogs

    await qt_thread_gateway.send_action(run_dialog)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_invalid_device_name(
    aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    d_w = logged_gui.test_get_devices_widget()
    assert d_w is not None

    def run_dialog():
        with patch("parsec.core.gui.register_device_dialog.DeviceName") as type_mock:
            type_mock.side_effect = ValueError()
            modal = RegisterDeviceDialog(parent=d_w, jobs_ctx=d_w.jobs_ctx, core=d_w.core)
            modal.show()
            assert not modal.line_edit_token.text()
            assert not modal.line_edit_url.text()
            aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device")
            with aqtbot.qtbot.waitSignal(modal.registration_error):
                aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)

    await qt_thread_gateway.send_action(run_dialog)
    assert autoclose_dialog.dialogs == [("Error", "The device name is invalid.")]


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_cancel(
    aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    d_w = logged_gui.test_get_devices_widget()
    assert d_w is not None

    def run_dialog():
        modal = RegisterDeviceDialog(parent=d_w, jobs_ctx=d_w.jobs_ctx, core=d_w.core)
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
    d_w = logged_gui.test_get_devices_widget()
    assert d_w is not None

    def run_dialog():
        modal = RegisterDeviceDialog(parent=d_w, jobs_ctx=d_w.jobs_ctx, core=d_w.core)
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device")

        with running_backend.offline():
            with aqtbot.qtbot.waitSignal(modal.registration_error):
                aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)

    await qt_thread_gateway.send_action(run_dialog)

    assert autoclose_dialog.dialogs == [("Error", "Cannot reach the server.")]


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_modal_already_registered(
    aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    d_w = logged_gui.test_get_devices_widget()
    assert d_w is not None

    def run_dialog():
        modal = RegisterDeviceDialog(parent=d_w, jobs_ctx=d_w.jobs_ctx, core=d_w.core)
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "dev1")
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)

    await qt_thread_gateway.send_action(run_dialog)
    assert autoclose_dialog.dialogs == [("Error", "The device already exists.")]


@pytest.mark.gui
@pytest.mark.trio
async def test_register_device_unknown_error(
    monkeypatch, aqtbot, logged_gui, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    d_w = logged_gui.test_get_devices_widget()
    assert d_w is not None

    async def _broken(*args, **kwargs):
        raise RuntimeError()

    monkeypatch.setattr(
        "parsec.core.gui.register_device_dialog.core_invite_and_create_device", _broken
    )

    def run_dialog():
        modal = RegisterDeviceDialog(parent=d_w, jobs_ctx=d_w.jobs_ctx, core=d_w.core)
        modal.show()
        aqtbot.qtbot.keyClicks(modal.line_edit_device_name, "new_device")
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert autoclose_dialog.dialogs == [("Error", "Cannot register the device.")]

    await qt_thread_gateway.send_action(run_dialog)
    # TODO: Make sure a log is emitted


# TODO test with timeout
