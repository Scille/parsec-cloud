# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.local_device import save_device_with_password
from parsec.core.gui.lang import switch_language


@pytest.fixture
async def logged_gui(
    aqtbot, gui_factory, running_backend, autoclose_dialog, core_config, alice, bob
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

    assert u_w.layout_users.count() == 3
    item = u_w.layout_users.itemAt(0)
    assert item.widget().label_user.text() == "adam"
    item = u_w.layout_users.itemAt(1)
    assert item.widget().label_user.text() == "alice\n(you)"
    item = u_w.layout_users.itemAt(2)
    assert item.widget().label_user.text() == "bob"


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("custom_locale", (False, True))
async def test_user_info(aqtbot, running_backend, autoclose_dialog, logged_gui, custom_locale):
    if custom_locale:
        switch_language(None, "fr")
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3
    item = u_w.layout_users.itemAt(0)
    item.widget().show_user_info()
    item = u_w.layout_users.itemAt(1)
    item.widget().show_user_info()
    item = u_w.layout_users.itemAt(2)
    item.widget().show_user_info()
    if custom_locale:
        assert autoclose_dialog.dialogs == [
            ("Information", "adam\n\nCréé le samedi 1 janvier 2000 00:00\n\nAdministrateur"),
            ("Information", "alice\n\nCréé le samedi 1 janvier 2000 00:00\n\nAdministrateur"),
            ("Information", "bob\n\nCréé le samedi 1 janvier 2000 00:00"),
        ]
    else:
        assert autoclose_dialog.dialogs == [
            (
                "Information",
                "adam\n\nCreated on Saturday, January 1, 2000 12:00 AM\n\nAdministrator",
            ),
            (
                "Information",
                "alice\n\nCreated on Saturday, January 1, 2000 12:00 AM\n\nAdministrator",
            ),
            ("Information", "bob\n\nCreated on Saturday, January 1, 2000 12:00 AM"),
        ]


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("online", (True, False))
async def test_revoke_user(
    aqtbot, running_backend, autoclose_dialog, monkeypatch, logged_gui, alice, online
):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3
    bob_w = u_w.layout_users.itemAt(2).widget()
    assert bob_w.user_name == "bob"
    assert bob_w.is_revoked is False
    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.QuestionDialog.ask", classmethod(lambda *args: True)
    )

    if online:
        async with aqtbot.wait_signal(u_w.revoke_success):
            bob_w.revoke_clicked.emit(bob_w)
        assert autoclose_dialog.dialogs == [
            ("Information", 'User "bob" has been successfully revoked.')
        ]
        assert bob_w.is_revoked is True

    else:
        with running_backend.offline():
            async with aqtbot.wait_signal(u_w.revoke_error):
                bob_w.revoke_clicked.emit(bob_w)
            assert autoclose_dialog.dialogs == [
                ("Error", "Cannot reach the server. Please check your internet connection.")
            ]
            assert bob_w.is_revoked is False


@pytest.mark.gui
@pytest.mark.trio
async def test_filter_users(aqtbot, running_backend, logged_gui):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3

    adam_w = u_w.layout_users.itemAt(0).widget()
    assert adam_w.user_name == "adam"
    alice_w = u_w.layout_users.itemAt(1).widget()
    assert alice_w.user_name == "alice"
    bob_w = u_w.layout_users.itemAt(2).widget()
    assert bob_w.user_name == "bob"

    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is True
    assert adam_w.isVisible() is True

    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        aqtbot.qtbot.keyClicks(u_w.line_edit_search, "bo")
    assert alice_w.isVisible() is False
    assert bob_w.isVisible() is True
    assert adam_w.isVisible() is False

    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        u_w.line_edit_search.setText("")
    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is True
    assert adam_w.isVisible() is True

    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        aqtbot.qtbot.keyClicks(u_w.line_edit_search, "a")
    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is False
    assert adam_w.isVisible() is True
