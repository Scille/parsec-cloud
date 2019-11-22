# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore

from parsec.core.types import BackendOrganizationClaimUserAddr
from parsec.core.invite_claim import invite_and_create_user


@pytest.fixture
async def alice_invite(running_backend, backend, alice):
    invitation = {
        "addr": BackendOrganizationClaimUserAddr.build(alice.organization_addr, "Zack", "123456"),
        "token": "123456",
        "user_id": "Zack",
        "device_name": "pc1",
        "password": "S3cr3tP@ss",
    }

    async def _invite():
        await invite_and_create_user(alice, invitation["user_id"], invitation["token"], True)

    async with trio.open_service_nursery() as nursery:
        with backend.event_bus.listen() as spy:
            nursery.start_soon(_invite)
            await spy.wait_with_timeout("event.connected", {"event_name": "user.claimed"})

            yield invitation

            nursery.cancel_scope.cancel()


async def _gui_ready_for_claim(aqtbot, gui, invitation, monkeypatch):
    lw = gui.test_get_login_widget()

    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.TextInputDialog.get_text",
        classmethod(lambda *args, **kwargs: (invitation["addr"].to_url())),
    )
    await aqtbot.mouse_click(lw.button_enter_url, QtCore.Qt.LeftButton)

    claim_w = gui.test_get_claim_user_widget()
    assert claim_w is not None

    await aqtbot.key_clicks(claim_w.line_edit_device, invitation.get("device_name", ""))
    await aqtbot.key_clicks(claim_w.line_edit_password, invitation.get("password", ""))
    await aqtbot.key_clicks(claim_w.line_edit_password_check, invitation.get("password", ""))


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user(aqtbot, gui, autoclose_dialog, alice_invite, monkeypatch):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite, monkeypatch)
    claim_w = gui.test_get_claim_user_widget()

    assert claim_w is not None

    autoclose_dialog.dialogs = []
    async with aqtbot.wait_signal(claim_w.user_claimed):
        await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [
        ("Information", "The user has been created.\nYou will now be logged in.")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user_offline(
    aqtbot, gui, autoclose_dialog, running_backend, alice_invite, monkeypatch
):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite, monkeypatch)
    claim_w = gui.test_get_claim_user_widget()

    assert claim_w is not None

    with running_backend.offline():
        async with aqtbot.wait_signal(claim_w.claim_error):
            await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [
        ("Error", "Cannot reach the server. Please check your internet connection.")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user_unknown_error(monkeypatch, aqtbot, gui, autoclose_dialog, alice_invite):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite, monkeypatch)
    claim_w = gui.test_get_claim_user_widget()

    assert claim_w is not None

    async def _broken(*args, **kwargs):
        raise RuntimeError()

    monkeypatch.setattr("parsec.core.gui.claim_user_widget.core_claim_user", _broken)

    async with aqtbot.wait_signal(claim_w.claim_error):
        await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [("Error", "Cannot register the user.")]
    # TODO: Make sure a log is emitted


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user_with_start_arg(event_bus, core_config, gui_factory):
    start_arg = "parsec://parsec.example.com/my_org?action=claim_user&rvk=P25GRG3XPSZKBEKXYQFBOLERWQNEDY3AO43MVNZCLPXPKN63JRYQssss&token=1234ABCD&user_id=John"

    gui = await gui_factory(event_bus=event_bus, core_config=core_config, start_arg=start_arg)

    claim_w = gui.test_get_claim_user_widget()
    assert claim_w

    assert claim_w.line_edit_token.text() == "1234ABCD"


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user_with_bad_start_arg(event_bus, core_config, gui_factory, autoclose_dialog):
    bad_start_arg = "parsec://parsec.example.com/my_org?action=dummy&rvk=P25GRG3XPSZKBEKXYQFBOLERWQNEDY3AO43MVNZCLPXPKN63JRYQssss&token=1234ABCD&user_id=John"

    await gui_factory(event_bus=event_bus, core_config=core_config, start_arg=bad_start_arg)

    assert autoclose_dialog.dialogs == [("Error", "URL is invalid.")]
