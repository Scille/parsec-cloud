# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from unittest.mock import ANY
from PyQt5 import QtCore

from parsec.core.gui.users_widget import UserInvitationButton
from parsec.core.gui.lang import translate as _

from tests.common import customize_fixtures


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("online", (True, False))
@customize_fixtures(logged_gui_as_admin=True)
async def test_invite_user(
    aqtbot,
    logged_gui,
    bob,
    running_backend,
    monkeypatch,
    autoclose_dialog,
    email_letterbox,
    online,
    snackbar_catcher,
):
    u_w = await logged_gui.test_switch_to_users_widget()

    assert u_w.layout_users.count() == 3

    monkeypatch.setattr(
        "parsec.core.gui.users_widget.get_text_input",
        lambda *args, **kwargs: "hubert.farnsworth@pe.com",
    )

    if online:
        aqtbot.mouse_click(u_w.button_add_user, QtCore.Qt.LeftButton)

        def _new_invitation_displayed():
            assert u_w.layout_users.count() == 4
            inv_btn = u_w.layout_users.itemAt(0).widget()
            assert isinstance(inv_btn, UserInvitationButton)
            assert inv_btn.email == "hubert.farnsworth@pe.com"
            assert email_letterbox.emails == [(inv_btn.email, ANY)]
            snackbar_catcher.snackbars = [
                "The invitation to join your organization was successfully sent to : <b>hubert.farnsworth@pe.com</b>"
            ]

        await aqtbot.wait_until(_new_invitation_displayed)

        snackbar_catcher.reset()

        # Also checks that an invitation already exists for this user

        monkeypatch.setattr(
            "parsec.core.gui.users_widget.get_text_input",
            lambda *args, **kwargs: bob.human_handle.email,
        )
        aqtbot.mouse_click(u_w.button_add_user, QtCore.Qt.LeftButton)

        def _already_member():
            assert snackbar_catcher.snackbars == [_("TEXT_INVITE_USER_ALREADY_MEMBER_ERROR")]

        await aqtbot.wait_until(_already_member)

    else:
        with running_backend.offline():
            aqtbot.mouse_click(u_w.button_add_user, QtCore.Qt.LeftButton)

            def _email_send_failed():
                assert snackbar_catcher.snackbars == [
                    "The server is offline or you have no access to the internet."
                ]

            await aqtbot.wait_until(_email_send_failed)
            assert not email_letterbox.emails


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_invite_and_greet_user_whith_active_users_limit_reached(
    aqtbot, gui, alice, running_backend, monkeypatch, autoclose_dialog, snackbar_catcher
):
    # Set the active user limit before login to ensure no cache information has been kept
    await running_backend.backend.organization.update(alice.organization_id, active_users_limit=1)
    await gui.test_switch_to_logged_in(alice)
    u_w = await gui.test_switch_to_users_widget()

    assert u_w.layout_users.count() == 3

    monkeypatch.setattr(
        "parsec.core.gui.users_widget.get_text_input",
        lambda *args, **kwargs: "hubert.farnsworth@pe.com",
    )

    aqtbot.mouse_click(u_w.button_add_user, QtCore.Qt.LeftButton)

    # The active user limit doesn't prevent to send invitation (check is
    # enforced by the backend during claim finalization)

    def _new_invitation_displayed():
        assert u_w.layout_users.count() == 4
        inv_btn = u_w.layout_users.itemAt(0).widget()
        assert isinstance(inv_btn, UserInvitationButton)
        assert inv_btn.email == "hubert.farnsworth@pe.com"
        assert snackbar_catcher.snackbars == [
            "The invitation to join your organization was successfully sent to : <b>hubert.farnsworth@pe.com</b>"
        ]

    await aqtbot.wait_until(_new_invitation_displayed)


@pytest.mark.gui
@pytest.mark.trio
async def test_invite_user_not_allowed(logged_gui, running_backend):
    u_w = await logged_gui.test_switch_to_users_widget()

    # Just make sure the button is not available
    assert not u_w.button_add_user.isVisible()


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("online", (True, False))
@customize_fixtures(logged_gui_as_admin=True)
async def test_revoke_user(
    aqtbot, running_backend, autoclose_dialog, monkeypatch, logged_gui, online, snackbar_catcher
):
    u_w = await logged_gui.test_switch_to_users_widget()

    assert u_w.layout_users.count() == 3
    bob_w = u_w.layout_users.itemAt(2).widget()
    assert bob_w.label_username.text() == "Boby McBobFace"
    assert bob_w.label_email.text() == "bob@example.com"
    assert bob_w.user_info.is_revoked is False

    monkeypatch.setattr(
        "parsec.core.gui.users_widget.ask_question",
        lambda *args, **kwargs: _("ACTION_USER_REVOCATION_CONFIRM"),
    )

    if online:
        bob_w.revoke_clicked.emit(bob_w.user_info)

        def _revocation_done():
            assert bob_w.user_info.is_revoked is True
            assert snackbar_catcher.snackbars == [
                "The user <b>Boby McBobFace</b> has been successfully revoked. Do no forget to reencrypt the workspaces that were shared with them."
            ]

        await aqtbot.wait_until(_revocation_done)

    else:
        with running_backend.offline():
            bob_w.revoke_clicked.emit(bob_w.user_info)

            def _revocation_error():
                assert bob_w.user_info.is_revoked is False
                assert snackbar_catcher.snackbars == [
                    "The server is offline or you have no access to the internet."
                ]

            await aqtbot.wait_until(_revocation_error)


@pytest.mark.gui
@pytest.mark.trio
async def test_revoke_user_not_allowed(
    aqtbot, running_backend, autoclose_dialog, monkeypatch, logged_gui, snackbar_catcher
):
    u_w = await logged_gui.test_switch_to_users_widget()

    assert u_w.layout_users.count() == 3
    alice_w = u_w.layout_users.itemAt(0).widget()
    assert alice_w.label_email.text() == "adam@example.com"
    assert alice_w.user_info.is_revoked is False

    # TODO: we should instead check that the menu giving access to revocation button is hidden...

    monkeypatch.setattr(
        "parsec.core.gui.users_widget.ask_question",
        lambda *args, **kwargs: _("ACTION_USER_REVOCATION_CONFIRM"),
    )

    alice_w.revoke_clicked.emit(alice_w.user_info)

    def _revocation_error():
        assert alice_w.user_info.is_revoked is False
        assert snackbar_catcher.snackbars == ["Could not revoke this user."]

    await aqtbot.wait_until(_revocation_error)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_cancel_user_invitation(
    aqtbot,
    logged_gui,
    running_backend,
    backend,
    autoclose_dialog,
    monkeypatch,
    alice,
    email_letterbox,
    snackbar_catcher,
):

    email = "i@like.coffee"

    # Patch dialogs
    monkeypatch.setattr(
        "parsec.core.gui.users_widget.get_text_input", lambda *args, **kwargs: email
    )
    monkeypatch.setattr(
        "parsec.core.gui.users_widget.ask_question",
        lambda *args, **kwargs: _("TEXT_USER_INVITE_CANCEL_INVITE_ACCEPT"),
    )
    u_w = await logged_gui.test_switch_to_users_widget()

    # Invite new user
    aqtbot.mouse_click(u_w.button_add_user, QtCore.Qt.LeftButton)

    def _new_invitation_displayed():
        assert u_w.layout_users.count() == 4
        assert snackbar_catcher.snackbars == [
            f"The invitation to join your organization was successfully sent to : <b>{ email }</b>"
        ]

    await aqtbot.wait_until(_new_invitation_displayed)
    snackbar_catcher.reset()
    user_invitation_w = u_w.layout_users.itemAt(0).widget()
    assert user_invitation_w.email == email

    # Cancel invitation
    aqtbot.mouse_click(user_invitation_w.button_cancel, QtCore.Qt.LeftButton)

    def _new_invitation_removed():
        assert u_w.layout_users.count() == 3

    await aqtbot.wait_until(_new_invitation_removed)
    assert snackbar_catcher.snackbars == ["TEXT_INVITE_USER_CANCELLATION_SUCCESS"]
