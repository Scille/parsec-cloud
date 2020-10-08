# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from tests.common import customize_fixtures
from PyQt5.QtWidgets import QLabel
from PyQt5 import QtCore
from PyQt5.Qt import Qt

from parsec.core.gui.lang import translate


@pytest.mark.gui
@pytest.mark.trio
async def test_list_users(aqtbot, running_backend, logged_gui, autoclose_dialog):
    u_w = await logged_gui.test_switch_to_users_widget()

    assert u_w.layout_users.count() == 3

    item = u_w.layout_users.itemAt(0)
    assert item.widget().label_username.text() == "Adamy McAda..."
    assert item.widget().label_email.text() == "adam@example.c..."
    assert item.widget().label_role.text() == "Administrator"
    assert item.widget().label_is_current.text() == ""

    item = u_w.layout_users.itemAt(1)
    assert item.widget().label_username.text() == "Alicey McAliceF..."
    assert item.widget().label_email.text() == "alice@example.c..."
    assert item.widget().label_is_current.text() == ""
    assert item.widget().label_role.text() == "Administrator"

    item = u_w.layout_users.itemAt(2)
    assert item.widget().label_username.text() == "Boby McBobFace"
    assert item.widget().label_email.text() == "bob@example.com"
    assert item.widget().label_is_current.text() == "(you)"
    assert item.widget().label_role.text() == "Standard"


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_list_users_and_invitations(
    aqtbot, running_backend, logged_gui, autoclose_dialog, alice
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
    assert item.widget().label_email.text() == "amy@pe.com"

    item = u_w.layout_users.itemAt(1)
    assert item.widget().label_email.text() == "fry@pe.com"

    item = u_w.layout_users.itemAt(2)
    assert item.widget().label_username.text() == "Adamy McAda..."
    assert item.widget().label_email.text() == "adam@example.c..."
    assert item.widget().label_role.text() == "Administrator"
    assert item.widget().label_is_current.text() == ""

    item = u_w.layout_users.itemAt(3)
    assert item.widget().label_username.text() == "Alicey McAliceF..."
    assert item.widget().label_email.text() == "alice@example.c..."
    assert item.widget().label_is_current.text() == "(you)"
    assert item.widget().label_role.text() == "Administrator"

    item = u_w.layout_users.itemAt(4)
    assert item.widget().label_username.text() == "Boby McBobFace"
    assert item.widget().label_email.text() == "bob@example.com"
    assert item.widget().label_is_current.text() == ""
    assert item.widget().label_role.text() == "Standard"


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
async def test_filter_users(aqtbot, running_backend, logged_gui):
    def _users_shown(count: int):
        assert u_w.layout_users.count() == count
        items = (u_w.layout_users.itemAt(i) for i in range(u_w.layout_users.count()))
        for item in items:
            widget = item.widget()
            assert widget.label_username.text() in [
                "Alicey McAliceFace",
                "Boby McBobFace",
                "Adamy McAdamFace",
            ]

    def _all_users_visible(u_w):
        assert u_w.layout_users.count() == 3
        adam_w = u_w.layout_users.itemAt(0).widget()
        assert adam_w.label_username.text() == "Adamy McAda..."
        assert adam_w.label_email.text() == "adam@example.c..."
        alice_w = u_w.layout_users.itemAt(1).widget()
        assert alice_w.label_username.text() == "Alicey McAliceF..."
        assert alice_w.label_email.text() == "alice@example.c..."

        bob_w = u_w.layout_users.itemAt(2).widget()
        assert bob_w.label_username.text() == "Boby McBobFace"
        assert bob_w.label_email.text() == "bob@example.com"

        assert alice_w.isVisible() is True
        assert bob_w.isVisible() is True
        assert adam_w.isVisible() is True

    u_w = await logged_gui.test_switch_to_users_widget()
    await aqtbot.wait_until(lambda: _all_users_visible(u_w=u_w))

    async with aqtbot.wait_signal(u_w.list_success):
        await aqtbot.key_clicks(u_w.line_edit_search, "bo")
        await aqtbot.mouse_click(u_w.button_users_filter, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(lambda: _users_shown(count=1))

    bob_w = u_w.layout_users.itemAt(0).widget()

    assert bob_w.isVisible() is True
    assert bob_w.label_username.text() == "Boby McBobFace"
    assert bob_w.label_email.text() == "bob@example.com"
    assert u_w.layout_users.count() == 1

    async with aqtbot.wait_signal(u_w.list_success):
        await aqtbot.wait_until(lambda: u_w.line_edit_search.setText(""))

    await aqtbot.wait_until(lambda: _all_users_visible(u_w=u_w))

    # Test find()
    async with aqtbot.wait_signal(u_w.list_success):
        await aqtbot.key_clicks(u_w.line_edit_search, "McA")
        await aqtbot.key_press(u_w.line_edit_search, Qt.Key_Enter)

    assert u_w.layout_users.count() == 2

    adam_w = u_w.layout_users.itemAt(0).widget()

    assert adam_w.isVisible() is True
    assert adam_w.label_username.text() == "Adamy McAda..."
    assert adam_w.label_email.text() == "adam@example.c..."

    alice_w = u_w.layout_users.itemAt(1).widget()
    assert alice_w.isVisible() is True
    assert alice_w.label_username.text() == "Alicey McAliceF..."
    assert alice_w.label_email.text() == "alice@example.c..."
    assert u_w.layout_users.count() == 2
