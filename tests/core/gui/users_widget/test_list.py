# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from parsec.core.gui.users_widget import UserButton

from tests.common import customize_fixtures
from PyQt5.QtWidgets import QLabel
from PyQt5 import QtCore, QtGui

from parsec.core.gui.lang import translate


@pytest.fixture
def str_len_limiter(monkeypatch):
    monkeypatch.setattr(
        "parsec.core.gui.users_widget.ensure_string_size", lambda s, size, font: (s[:12] + "...")
    )


def _assert_all_users_visible(u_w, index=0):
    assert u_w.layout_users.count() == index + 3

    adam_w = u_w.layout_users.itemAt(index).widget()
    assert adam_w.label_username.text() == "Adamy McAdam..."
    assert adam_w.label_email.text() == "adam@example..."

    alice_w = u_w.layout_users.itemAt(index + 1).widget()
    assert alice_w.label_username.text() == "Alicey McAli..."
    assert alice_w.label_email.text() == "alice@exampl..."

    bob_w = u_w.layout_users.itemAt(index + 2).widget()
    assert bob_w.label_username.text() == "Boby McBobFa..."
    assert bob_w.label_email.text() == "bob@example...."

    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is True
    assert adam_w.isVisible() is True


@pytest.mark.gui
@pytest.mark.trio
async def test_list_users(aqtbot, running_backend, logged_gui, autoclose_dialog, str_len_limiter):
    u_w = await logged_gui.test_switch_to_users_widget()
    _assert_all_users_visible(u_w)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_list_users_and_invitations(
    aqtbot, running_backend, logged_gui, autoclose_dialog, alice, str_len_limiter, snackbar_catcher
):
    # Also populate some invitations
    await running_backend.backend.invite.new_for_user(
        alice.organization_id, greeter_user_id=alice.user_id, claimer_email="fry@pe.com"
    )
    await running_backend.backend.invite.new_for_user(
        alice.organization_id, greeter_user_id=alice.user_id, claimer_email="amy@pe.com"
    )
    # Device invitation should be ignored here
    await running_backend.backend.invite.new_for_device(
        alice.organization_id, greeter_user_id=alice.user_id
    )

    u_w = await logged_gui.test_switch_to_users_widget()

    assert u_w.layout_users.count() == 5

    item = u_w.layout_users.itemAt(0)
    assert item.widget().label_email.text() == "amy@pe.com..."

    item = u_w.layout_users.itemAt(1)
    assert item.widget().label_email.text() == "fry@pe.com..."

    _assert_all_users_visible(u_w, index=2)

    # Check if the option to copy the email is working properly
    clipboard = QtGui.QGuiApplication.clipboard()
    user_button = u_w.layout_users.itemAt(2).widget()
    assert isinstance(user_button, UserButton)
    user_button.copy_email()
    assert snackbar_catcher.snackbars == [
        ("INFO", translate("TEXT_GREET_USER_EMAIL_COPIED_TO_CLIPBOARD"))
    ]
    assert clipboard.text() == user_button.user_email


@pytest.mark.gui
@pytest.mark.trio
async def test_list_users_offline(aqtbot, logged_gui, autoclose_dialog):
    u_w = await logged_gui.test_switch_to_users_widget(error=True)
    assert u_w.layout_users.count() == 1
    error_msg = u_w.layout_users.itemAt(0).widget()
    assert isinstance(error_msg, QLabel)
    assert error_msg.text() == translate("TEXT_USER_LIST_RETRIEVABLE_FAILURE")
    assert not autoclose_dialog.dialogs


@pytest.mark.gui
@pytest.mark.trio
async def test_filter_users(aqtbot, running_backend, logged_gui, str_len_limiter):
    def _users_shown(count: int):
        assert u_w.layout_users.count() == count
        items = (u_w.layout_users.itemAt(i) for i in range(u_w.layout_users.count()))
        for item in items:
            widget = item.widget()
            assert widget.label_username.text() in [
                "Alicey McAliceFace",
                "Boby McBobFa...",
                "Adamy McAdamFace",
            ]

    u_w = await logged_gui.test_switch_to_users_widget()
    await aqtbot.wait_until(lambda: _assert_all_users_visible(u_w=u_w))

    async with aqtbot.wait_signal(u_w.list_success):
        await aqtbot.key_clicks(u_w.line_edit_search, "bo")

    await aqtbot.wait_until(lambda: _users_shown(count=1))

    bob_w = u_w.layout_users.itemAt(0).widget()

    assert bob_w.isVisible() is True
    assert bob_w.label_username.text() == "Boby McBobFa..."
    assert bob_w.label_email.text() == "bob@example...."
    assert u_w.layout_users.count() == 1

    async with aqtbot.wait_signal(u_w.list_success):
        aqtbot.mouse_click(u_w.line_edit_search.button_clear, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(lambda: _assert_all_users_visible(u_w=u_w))

    # Test find()
    async with aqtbot.wait_signal(u_w.list_success):
        await aqtbot.key_clicks(u_w.line_edit_search, "McA")

    assert u_w.layout_users.count() == 2

    adam_w = u_w.layout_users.itemAt(0).widget()

    assert adam_w.isVisible() is True
    assert adam_w.label_username.text() == "Adamy McAdam..."
    assert adam_w.label_email.text() == "adam@example..."

    alice_w = u_w.layout_users.itemAt(1).widget()
    assert alice_w.isVisible() is True
    assert alice_w.label_username.text() == "Alicey McAli..."
    assert alice_w.label_email.text() == "alice@exampl..."
    assert u_w.layout_users.count() == 2


@pytest.mark.gui
@pytest.mark.trio
async def test_filter_revoked_user(
    aqtbot, alice, adam, running_backend, backend, logged_gui, autoclose_dialog, str_len_limiter
):
    u_w = await logged_gui.test_switch_to_users_widget()

    def _assert_adam_not_visible(u_w):
        assert u_w.layout_users.count() == 2

        item = u_w.layout_users.itemAt(0)
        assert item.widget().label_username.text() == "Alicey McAli..."
        assert item.widget().label_email.text() == "alice@exampl..."
        assert item.widget().label_is_current.text() == ""
        assert item.widget().label_role.text() == "Administrator"

        item = u_w.layout_users.itemAt(1)
        assert item.widget().label_username.text() == "Boby McBobFa..."
        assert item.widget().label_email.text() == "bob@example...."
        assert item.widget().label_is_current.text() == "(you)"
        assert item.widget().label_role.text() == "Standard"

    assert not u_w.checkbox_filter_revoked.isChecked()
    _assert_all_users_visible(u_w)

    async with aqtbot.wait_signal(u_w.list_success):
        aqtbot.mouse_click(u_w.checkbox_filter_revoked, QtCore.Qt.LeftButton)
    _assert_all_users_visible(u_w)

    # Revoke Adam
    await backend.user.revoke_user(
        organization_id=alice.organization_id,
        user_id=adam.user_id,
        revoked_user_certificate=b"dummy",
        revoked_user_certifier=alice.device_id,
    )

    # Remove filter
    async with aqtbot.wait_signal(u_w.list_success):
        aqtbot.mouse_click(u_w.checkbox_filter_revoked, QtCore.Qt.LeftButton)
    _assert_all_users_visible(u_w)

    # Apply filter
    async with aqtbot.wait_signal(u_w.list_success):
        aqtbot.mouse_click(u_w.checkbox_filter_revoked, QtCore.Qt.LeftButton)
    _assert_adam_not_visible(u_w)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_filter_invitation_user(
    aqtbot, alice, adam, running_backend, backend, logged_gui, autoclose_dialog
):
    # Add invitation
    await running_backend.backend.invite.new_for_user(
        alice.organization_id, greeter_user_id=alice.user_id, claimer_email="amy@pe.com"
    )

    u_w = await logged_gui.test_switch_to_users_widget()
    assert u_w.layout_users.count() == 4

    # Apply filter
    async with aqtbot.wait_signal(u_w.list_success):
        aqtbot.mouse_click(u_w.checkbox_filter_invitation, QtCore.Qt.LeftButton)

    assert u_w.layout_users.count() == 3

    # Remove filter
    async with aqtbot.wait_signal(u_w.list_success):
        aqtbot.mouse_click(u_w.checkbox_filter_invitation, QtCore.Qt.LeftButton)

    assert u_w.layout_users.count() == 4
