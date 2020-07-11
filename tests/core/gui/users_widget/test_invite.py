# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from unittest.mock import ANY
from PyQt5 import QtCore

from parsec.core.gui.users_widget import UserInvitationButton
from parsec.core.gui.lang import translate as _

from tests.common import customize_fixtures


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("online", (True, False))
@customize_fixtures(backend_has_email=True, logged_gui_as_admin=True)
async def test_invite_user(
    aqtbot, logged_gui, running_backend, monkeypatch, autoclose_dialog, email_letterbox, online
):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3

    monkeypatch.setattr(
        "parsec.core.gui.users_widget.get_text_input",
        lambda *args, **kwargs: "hubert.farnsworth@pe.com",
    )

    if online:
        async with aqtbot.wait_signal(u_w.invite_user_success):
            await aqtbot.mouse_click(u_w.button_add_user, QtCore.Qt.LeftButton)

        def invitation_shown():
            assert u_w.layout_users.count() == 4

        await aqtbot.wait_until(invitation_shown)
        inv_btn = u_w.layout_users.itemAt(3).widget()
        assert isinstance(inv_btn, UserInvitationButton)
        assert inv_btn.email == "hubert.farnsworth@pe.com"

        assert email_letterbox == [(inv_btn.email, ANY)]

    else:
        with running_backend.offline():
            async with aqtbot.wait_signal(u_w.invite_user_error):
                await aqtbot.mouse_click(u_w.button_add_user, QtCore.Qt.LeftButton)

            assert autoclose_dialog.dialogs == [
                ("Error", "The server is offline or you have no access to the internet.")
            ]


@pytest.mark.gui
@pytest.mark.trio
async def test_invite_user_not_allowed(
    aqtbot, logged_gui, running_backend, monkeypatch, autoclose_dialog
):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3

    # Just make sure the button is not available
    assert u_w.button_add_user.isHidden()


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("online", (True, False))
@customize_fixtures(logged_gui_as_admin=True)
async def test_revoke_user(
    aqtbot, running_backend, autoclose_dialog, monkeypatch, logged_gui, online
):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3
    bob_w = u_w.layout_users.itemAt(2).widget()
    assert bob_w.label_username.text() == "Boby McBobFace"
    assert bob_w.label_email.text() == "bob@example.com"
    assert bob_w.user_info.is_revoked is False

    monkeypatch.setattr(
        "parsec.core.gui.users_widget.ask_question",
        lambda *args: _("ACTION_USER_REVOCATION_CONFIRM"),
    )

    def emit_revoke():
        bob_w.revoke_clicked.emit(bob_w.user_info)

    if online:
        async with aqtbot.wait_signal(u_w.revoke_success):
            await aqtbot.qt_thread_gateway.send_action(emit_revoke)

        assert autoclose_dialog.dialogs == [
            (
                "",
                "The user <b>Boby McBobFace</b> has been successfully revoked. Do no forget to reencrypt the workspaces that were shared with them.",
            )
        ]
        assert bob_w.user_info.is_revoked is True

    else:
        with running_backend.offline():
            async with aqtbot.wait_signal(u_w.revoke_error):
                await aqtbot.qt_thread_gateway.send_action(emit_revoke)

            assert autoclose_dialog.dialogs == [
                ("Error", "The server is offline or you have no access to the internet.")
            ]
            assert bob_w.user_info.is_revoked is False


@pytest.mark.gui
@pytest.mark.trio
async def test_revoke_user_not_allowed(
    aqtbot, running_backend, autoclose_dialog, monkeypatch, logged_gui
):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3
    alice_w = u_w.layout_users.itemAt(1).widget()
    assert alice_w.label_email.text() == "alice@example.com"
    assert alice_w.user_info.is_revoked is False

    # TODO: we should instead check that the menu giving access to revocation button is hidden...

    monkeypatch.setattr(
        "parsec.core.gui.users_widget.ask_question",
        lambda *args: _("ACTION_USER_REVOCATION_CONFIRM"),
    )

    def emit_revoke():
        alice_w.revoke_clicked.emit(alice_w.user_info)

    async with aqtbot.wait_signal(u_w.revoke_error):
        await aqtbot.qt_thread_gateway.send_action(emit_revoke)

    assert autoclose_dialog.dialogs == [("Error", "Could not revoke this user.")]
    assert alice_w.user_info.is_revoked is False
