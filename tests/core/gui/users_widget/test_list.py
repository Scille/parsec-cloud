# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from tests.common import customize_fixtures


@pytest.mark.gui
@pytest.mark.trio
async def test_list_users(aqtbot, running_backend, logged_gui, autoclose_dialog):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3

    item = u_w.layout_users.itemAt(0)
    assert item.widget().label_username.text() == "Adamy McAdamFace"
    assert item.widget().label_email.text() == "adam@example.com"
    assert item.widget().label_role.text() == "Administrator"
    assert item.widget().label_is_current.text() == ""

    item = u_w.layout_users.itemAt(1)
    assert item.widget().label_username.text() == "Alicey McAliceFace"
    assert item.widget().label_email.text() == "alice@example.com"
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

    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 5

    item = u_w.layout_users.itemAt(0)
    assert item.widget().label_username.text() == "Adamy McAdamFace"
    assert item.widget().label_email.text() == "adam@example.com"
    assert item.widget().label_role.text() == "Administrator"
    assert item.widget().label_is_current.text() == ""

    item = u_w.layout_users.itemAt(1)
    assert item.widget().label_username.text() == "Alicey McAliceFace"
    assert item.widget().label_email.text() == "alice@example.com"
    assert item.widget().label_is_current.text() == "(you)"
    assert item.widget().label_role.text() == "Administrator"

    item = u_w.layout_users.itemAt(2)
    assert item.widget().label_username.text() == "Boby McBobFace"
    assert item.widget().label_email.text() == "bob@example.com"
    assert item.widget().label_is_current.text() == ""
    assert item.widget().label_role.text() == "Standard"

    item = u_w.layout_users.itemAt(3)
    assert item.widget().label_email.text() == "fry@pe.com"

    item = u_w.layout_users.itemAt(4)
    assert item.widget().label_email.text() == "amy@pe.com"


@pytest.mark.gui
@pytest.mark.trio
async def test_list_users_offline(aqtbot, logged_gui, autoclose_dialog):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_error):
        pass

    assert u_w.layout_users.count() == 0
    assert not autoclose_dialog.dialogs


@pytest.mark.gui
@pytest.mark.trio
async def test_filter_users(aqtbot, running_backend, logged_gui):
    u_w = logged_gui.test_get_users_widget()
    assert u_w is not None

    async with aqtbot.wait_signal(u_w.list_success):
        pass

    assert u_w.layout_users.count() == 3

    adam_w = u_w.layout_users.itemAt(0).widget()
    assert adam_w.label_username.text() == "Adamy McAdamFace"
    assert adam_w.label_email.text() == "adam@example.com"
    alice_w = u_w.layout_users.itemAt(1).widget()
    assert alice_w.label_username.text() == "Alicey McAliceFace"
    assert alice_w.label_email.text() == "alice@example.com"

    bob_w = u_w.layout_users.itemAt(2).widget()
    assert bob_w.label_username.text() == "Boby McBobFace"
    assert bob_w.label_email.text() == "bob@example.com"

    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is True
    assert adam_w.isVisible() is True

    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        await aqtbot.key_clicks(u_w.line_edit_search, "bo")
    assert alice_w.isVisible() is False
    assert bob_w.isVisible() is True
    assert adam_w.isVisible() is False

    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        await aqtbot.run(lambda: u_w.line_edit_search.setText(""))
    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is True
    assert adam_w.isVisible() is True

    async with aqtbot.wait_signal(u_w.filter_timer.timeout):
        await aqtbot.key_clicks(u_w.line_edit_search, "mca")
    assert alice_w.isVisible() is True
    assert bob_w.isVisible() is False
    assert adam_w.isVisible() is True
