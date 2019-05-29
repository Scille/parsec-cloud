# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from unittest.mock import patch
from PyQt5 import QtCore, QtWidgets

from parsec.core.local_device import save_device_with_password
from parsec.core.gui.users_widget import UserButton


@pytest.fixture
async def logged_gui(
    aqtbot, gui_factory, running_backend, autoclose_dialog, core_config, alice, bob
):
    save_device_with_password(core_config.config_dir, alice, "P@ssw0rd")

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    llw = gui.test_get_login_login_widget()

    assert llw is not None

    await aqtbot.key_clicks(llw.line_edit_password, "P@ssw0rd")

    async with aqtbot.wait_signals([lw.login_with_password_clicked, gui.logged_in]):
        await aqtbot.mouse_click(llw.button_login, QtCore.Qt.LeftButton)

    central_widget = gui.test_get_central_widget()
    assert central_widget is not None

    save_device_with_password(core_config.config_dir, bob, "P@ssw0rd")
    await aqtbot.mouse_click(central_widget.menu.button_users, QtCore.Qt.LeftButton)

    yield gui


@pytest.mark.gui
@pytest.mark.trio
async def test_list_users(aqtbot, running_backend, logged_gui):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 2
    item = u_w.layout_users.itemAt(0)
    assert item.widget().label_user.text() == "alice\n(you)"
    item = u_w.layout_users.itemAt(1)
    assert item.widget().label_user.text() == "bob"


@pytest.mark.gui
@pytest.mark.trio
async def test_user_info(aqtbot, running_backend, autoclose_dialog, logged_gui):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 2
    item = u_w.layout_users.itemAt(0)
    item.widget().show_user_info()
    item = u_w.layout_users.itemAt(1)
    item.widget().show_user_info()
    assert autoclose_dialog.dialogs == [
        ("Information", "alice\n\nCreated on 01/01/00 00:00:00"),
        ("Information", "bob\n\nCreated on 01/01/00 00:00:00"),
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_revoke_user(
    aqtbot, running_backend, autoclose_dialog, monkeypatch, logged_gui, alice
):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    await running_backend.backend.user.set_user_admin(alice.organization_id, alice.user_id, True)
    assert u_w.layout_users.count() == 2
    bob_w = u_w.layout_users.itemAt(1).widget()
    assert bob_w.is_revoked is False
    monkeypatch.setattr(
        "parsec.core.gui.custom_widgets.QuestionDialog.ask", classmethod(lambda *args: True)
    )

    async with aqtbot.wait_signal(u_w.revoke_success):
        bob_w.revoke_clicked.emit(bob_w)
    assert autoclose_dialog.dialogs == [("Information", 'User "bob" has been revoked.')]
    assert bob_w.is_revoked is True


@pytest.mark.gui
@pytest.mark.trio
async def test_revoke_user_not_admin(
    aqtbot, running_backend, autoclose_dialog, monkeypatch, logged_gui
):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 2
    bob_w = u_w.layout_users.itemAt(1).widget()
    assert bob_w.is_revoked is False
    monkeypatch.setattr(
        "parsec.core.gui.custom_widgets.QuestionDialog.ask", classmethod(lambda *args: True)
    )

    async with aqtbot.wait_signal(u_w.revoke_error):
        bob_w.revoke_clicked.emit(bob_w)
    assert autoclose_dialog.dialogs == [
        ("Error", "You don't have the permission to revoke this user.")
    ]
    assert bob_w.is_revoked is False


@pytest.mark.gui
@pytest.mark.trio
async def test_filter_users(aqtbot, running_backend, logged_gui):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 2
    alice_w = u_w.layout_users.itemAt(0).widget()
    bob_w = u_w.layout_users.itemAt(1).widget()

    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is True

    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        aqtbot.qtbot.keyClicks(u_w.line_edit_search, "bo")
    assert alice_w.isVisible() is False
    assert bob_w.isVisible() is True
    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        u_w.line_edit_search.setText("")
    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is True
    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        aqtbot.qtbot.keyClicks(u_w.line_edit_search, "a")
    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is False
