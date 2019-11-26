# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore
from parsec.core.types import BackendOrganizationClaimUserAddr
from parsec.core.gui.register_user_dialog import RegisterUserDialog
from unittest.mock import patch

from parsec.core.local_device import save_device_with_password


async def logged_gui(aqtbot, gui_factory, autoclose_dialog, core_config, user):
    # Create an existing device before starting the gui
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, user, password)

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

    await aqtbot.mouse_click(central_widget.menu.button_users, QtCore.Qt.LeftButton)
    return gui


@pytest.fixture
async def logged_gui_alice(aqtbot, gui_factory, autoclose_dialog, core_config, alice):
    yield await logged_gui(aqtbot, gui_factory, autoclose_dialog, core_config, alice)


@pytest.fixture
async def logged_gui_bob(aqtbot, gui_factory, autoclose_dialog, core_config, bob):
    yield await logged_gui(aqtbot, gui_factory, autoclose_dialog, core_config, bob)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_user_open_modal(aqtbot, logged_gui_alice, running_backend):
    u_w = logged_gui_alice.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    with patch("parsec.core.gui.users_widget.RegisterUserDialog") as register_mock:
        await aqtbot.mouse_click(u_w.taskbar_buttons[0], QtCore.Qt.LeftButton)
        register_mock.assert_called_once_with(parent=u_w, jobs_ctx=u_w.jobs_ctx, core=u_w.core)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_user_modal_ok(
    aqtbot, gui, logged_gui_alice, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    u_w = logged_gui_alice.test_get_users_widget()
    assert u_w is not None

    def _claim_user(device_name, token, addr, password):
        l_w = gui.test_get_login_widget()

        assert l_w is not None
        l_w.show_claim_user_widget(BackendOrganizationClaimUserAddr.from_url(addr))

        claim_w = gui.test_get_claim_user_widget()

        assert claim_w is not None

        aqtbot.qtbot.keyClicks(claim_w.line_edit_device, device_name)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_token, token)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_password, password)
        aqtbot.qtbot.keyClicks(claim_w.line_edit_password_check, password)
        aqtbot.qtbot.mouseClick(claim_w.button_claim, QtCore.Qt.LeftButton)

    def run_dialog():
        modal = RegisterUserDialog(parent=u_w, jobs_ctx=u_w.jobs_ctx, core=u_w.core)
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_username, "new_user")
        aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert modal.line_edit_url.text()
        assert modal.line_edit_username.text() == "new_user"
        assert modal.line_edit_token.text()
        with aqtbot.qtbot.waitSignal(modal.user_registered):
            _claim_user(
                "laptop", modal.line_edit_token.text(), modal.line_edit_url.text(), "P@ssw0rd!"
            )
        assert (
            "Information",
            "The user has been registered. You may now close this window.",
        ) in autoclose_dialog.dialogs

    await qt_thread_gateway.send_action(run_dialog)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_user_modal_invalid_user_id(
    aqtbot, logged_gui_alice, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    u_w = logged_gui_alice.test_get_users_widget()
    assert u_w is not None

    def run_dialog():
        with patch("parsec.core.gui.register_user_dialog.UserID") as type_mock:
            type_mock.side_effect = ValueError()
            modal = RegisterUserDialog(parent=u_w, jobs_ctx=u_w.jobs_ctx, core=u_w.core)
            modal.show()
            assert not modal.line_edit_token.text()
            assert not modal.line_edit_url.text()
            aqtbot.qtbot.keyClicks(modal.line_edit_username, "new_user" * 5)
            aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)

    await qt_thread_gateway.send_action(run_dialog)
    assert autoclose_dialog.dialogs == [("Error", "User name is invalid.")]


@pytest.mark.gui
@pytest.mark.trio
async def test_register_user_modal_cancel(
    aqtbot, logged_gui_alice, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    u_w = logged_gui_alice.test_get_users_widget()
    assert u_w is not None

    def run_dialog():
        modal = RegisterUserDialog(parent=u_w, jobs_ctx=u_w.jobs_ctx, core=u_w.core)
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_username, "new_user")
        aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert modal.line_edit_url.text()
        assert modal.line_edit_username.text() == "new_user"
        assert modal.line_edit_token.text()
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_cancel, QtCore.Qt.LeftButton)
        assert not modal.widget_registration.isVisible()

    await qt_thread_gateway.send_action(run_dialog)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_user_modal_offline(
    aqtbot, logged_gui_alice, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    u_w = logged_gui_alice.test_get_users_widget()
    assert u_w is not None

    def run_dialog():
        modal = RegisterUserDialog(parent=u_w, jobs_ctx=u_w.jobs_ctx, core=u_w.core)
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_username, "new_user")

        with running_backend.offline():
            with aqtbot.qtbot.waitSignal(modal.registration_error):
                aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)

    await qt_thread_gateway.send_action(run_dialog)

    assert autoclose_dialog.dialogs == [("Error", "Cannot reach the server.")]


@pytest.mark.gui
@pytest.mark.trio
async def test_register_user_modal_already_registered(
    aqtbot, logged_gui_alice, running_backend, qt_thread_gateway, alice, autoclose_dialog
):
    u_w = logged_gui_alice.test_get_users_widget()
    assert u_w is not None

    def run_dialog():
        modal = RegisterUserDialog(parent=u_w, jobs_ctx=u_w.jobs_ctx, core=u_w.core)
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_username, "alice")
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)

    await qt_thread_gateway.send_action(run_dialog)
    assert autoclose_dialog.dialogs == [("Error", "This user already exists.")]


@pytest.mark.skip(
    "Useless test, since in the GUI, users have no way to open the RegisterUserDialog if they are not admin"
)
@pytest.mark.gui
@pytest.mark.trio
async def test_register_user_modal_not_admin(
    aqtbot, logged_gui_bob, running_backend, qt_thread_gateway, bob, autoclose_dialog
):
    u_w = logged_gui_bob.test_get_users_widget()
    assert u_w is not None

    def run_dialog():
        modal = RegisterUserDialog(parent=u_w, jobs_ctx=u_w.jobs_ctx, core=u_w.core)
        modal.show()
        assert not modal.line_edit_token.text()
        assert not modal.line_edit_url.text()
        aqtbot.qtbot.keyClicks(modal.line_edit_username, "new_user")
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert autoclose_dialog.dialogs == [("Error", "Only administrators can invite new users.")]

    await qt_thread_gateway.send_action(run_dialog)


@pytest.mark.gui
@pytest.mark.trio
async def test_register_user_unknown_error(
    monkeypatch,
    aqtbot,
    logged_gui_alice,
    running_backend,
    qt_thread_gateway,
    alice,
    autoclose_dialog,
):
    async def _broken(*args, **kwargs):
        raise RuntimeError()

    monkeypatch.setattr("parsec.core.gui.register_user_dialog.core_invite_and_create_user", _broken)

    u_w = logged_gui_alice.test_get_users_widget()
    assert u_w is not None

    def run_dialog():
        modal = RegisterUserDialog(parent=u_w, jobs_ctx=u_w.jobs_ctx, core=u_w.core)
        modal.show()
        aqtbot.qtbot.keyClicks(modal.line_edit_username, "new_user")
        with aqtbot.qtbot.waitSignal(modal.registration_error):
            aqtbot.qtbot.mouseClick(modal.button_register, QtCore.Qt.LeftButton)
        assert autoclose_dialog.dialogs == [("Error", "Cannot register the user.")]

    await qt_thread_gateway.send_action(run_dialog)
    # TODO: Make sure a log is emitted


# TODO test with timeout
