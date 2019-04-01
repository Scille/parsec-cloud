# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore

from parsec.core.invite_claim import invite_and_create_user


@pytest.fixture
async def alice_invite(running_backend, backend, alice):
    invitation = {
        "addr": alice.organization_addr,
        "token": "123456",
        "user_id": "Zack",
        "device_name": "pc1",
        "password": "S3cr3tP@ss",
    }

    await backend.user.set_user_admin(alice.organization_id, alice.user_id, True)

    async def _invite():
        await invite_and_create_user(alice, invitation["user_id"], invitation["token"], True)

    async with trio.open_nursery() as nursery:
        with backend.event_bus.listen() as spy:
            nursery.start_soon(_invite)
            await spy.wait("event.connected", kwargs={"event_name": "user.claimed"})

            yield invitation

            nursery.cancel_scope.cancel()


async def _gui_ready_for_claim(aqtbot, gui, invitation):
    claim_w = gui.login_widget.claim_user_widget
    # Claim user page is the default if there is no devices available
    assert claim_w.isVisible()

    await aqtbot.key_clicks(claim_w.line_edit_login, invitation.get("user_id", ""))
    await aqtbot.key_clicks(claim_w.line_edit_device, invitation.get("device_name", ""))
    await aqtbot.key_clicks(claim_w.line_edit_token, invitation.get("token", ""))
    await aqtbot.key_clicks(claim_w.line_edit_url, invitation.get("addr", ""))
    await aqtbot.key_clicks(claim_w.line_edit_password, invitation.get("password", ""))
    await aqtbot.key_clicks(claim_w.line_edit_password_check, invitation.get("password", ""))


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user(aqtbot, gui, autoclose_dialog, alice_invite):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite)
    claim_w = gui.login_widget.claim_user_widget
    async with aqtbot.wait_signal(claim_w.user_claimed):
        await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [
        ("Information", "The user has been registered. You can now login.")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user_offline(aqtbot, gui, autoclose_dialog, running_backend, alice_invite):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite)
    claim_w = gui.login_widget.claim_user_widget

    with running_backend.offline():
        async with aqtbot.wait_signal(claim_w.claim_error):
            await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [
        ("Error", "Can not claim this user ([Errno 111] Connection refused).")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_user_unknown_error(monkeypatch, aqtbot, gui, autoclose_dialog, alice_invite):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite)
    claim_w = gui.login_widget.claim_user_widget

    async def _broken(*args, **kwargs):
        raise RuntimeError()

    monkeypatch.setattr("parsec.core.gui.claim_user_widget.core_claim_user", _broken)

    async with aqtbot.wait_signal(claim_w.claim_error):
        await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [
        ("Error", "Can not claim this user (Unexpected error: RuntimeError()).")
    ]
    # TODO: Make sure a log is emitted
