# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore

from parsec.api.protocol import DeviceID
from parsec.core.types import BackendOrganizationClaimDeviceAddr
from parsec.core.invite_claim import invite_and_create_device

from tests.common import addr_with_device_subdomain
from tests.open_tcp_stream_mock_wrapper import offline


@pytest.fixture
async def alice_invite(running_backend, backend, alice):
    device_id = DeviceID(f"{alice.user_id}@pc1")
    # Modify address subdomain to be able to switch it offline whithout
    # disconnecting the inviter
    organization_addr = addr_with_device_subdomain(alice.organization_addr, device_id)
    invitation = {
        "addr": BackendOrganizationClaimDeviceAddr.build(organization_addr, device_id, "123456"),
        "token": "123456",
        "user_id": device_id.user_id,
        "device_name": device_id.device_name,
        "password": "S3cr3tP@ss",
    }

    async def _invite():
        await invite_and_create_device(alice, invitation["device_name"], invitation["token"])

    async with trio.open_service_nursery() as nursery:
        with backend.event_bus.listen() as spy:
            nursery.start_soon(_invite)
            await spy.wait_with_timeout("event.connected", {"event_name": "device.claimed"})

            yield invitation

            nursery.cancel_scope.cancel()


async def _gui_ready_for_claim(aqtbot, gui, invitation, monkeypatch):
    login_w = gui.test_get_login_widget()
    claim_w = gui.test_get_claim_device_widget()
    assert login_w is not None
    assert claim_w is None

    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.TextInputDialog.get_text",
        classmethod(lambda *args, **kwargs: (invitation["addr"].to_url())),
    )

    await aqtbot.mouse_click(login_w.button_enter_url, QtCore.Qt.LeftButton)

    claim_w = gui.test_get_claim_device_widget()
    assert claim_w is not None

    await aqtbot.key_clicks(claim_w.line_edit_password, invitation.get("password", ""))
    await aqtbot.key_clicks(claim_w.line_edit_password_check, invitation.get("password", ""))


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_device(aqtbot, gui, autoclose_dialog, alice_invite, monkeypatch):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite, monkeypatch)
    claim_w = gui.test_get_claim_device_widget()
    async with aqtbot.wait_signal(claim_w.device_claimed):
        await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [
        ("Information", "The device has been created. You can now log in.")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_device_offline(
    aqtbot, gui, autoclose_dialog, running_backend, alice_invite, monkeypatch
):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite, monkeypatch)
    claim_w = gui.test_get_claim_device_widget()

    with offline(alice_invite["addr"]):
        async with aqtbot.wait_signal(claim_w.claim_error):
            await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [
        ("Error", "Cannot reach the server. Please check your internet connection.")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_device_unknown_error(monkeypatch, aqtbot, gui, autoclose_dialog, alice_invite):
    await _gui_ready_for_claim(aqtbot, gui, alice_invite, monkeypatch)
    claim_w = gui.test_get_claim_device_widget()

    async def _broken(*args, **kwargs):
        raise RuntimeError()

    monkeypatch.setattr("parsec.core.gui.claim_device_widget.core_claim_device", _broken)

    async with aqtbot.wait_signal(claim_w.claim_error):
        await aqtbot.mouse_click(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [("Error", "Cannot claim this device.")]
    # TODO: Make sure a log is emitted


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_device_with_start_arg(event_bus, core_config, gui_factory):
    start_arg = "parsec://parsec.example.com/my_org?action=claim_device&device_id=John%40pc&rvk=P25GRG3XPSZKBEKXYQFBOLERWQNEDY3AO43MVNZCLPXPKN63JRYQssss&token=1234ABCD"

    gui = await gui_factory(event_bus=event_bus, core_config=core_config, start_arg=start_arg)

    claim_w = gui.test_get_claim_device_widget()
    assert claim_w

    assert claim_w.line_edit_token.text() == "1234ABCD"


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_device_with_bad_start_arg(
    event_bus, core_config, gui_factory, autoclose_dialog
):
    bad_start_arg = "parsec://parsec.example.com/my_org?action=dummy&device_id=John%40pc&rvk=P25GRG3XPSZKBEKXYQFBOLERWQNEDY3AO43MVNZCLPXPKN63JRYQssss&token=1234ABCD"

    gui = await gui_factory(event_bus=event_bus, core_config=core_config, start_arg=bad_start_arg)

    claim_w = gui.test_get_claim_device_widget()
    assert not claim_w

    assert autoclose_dialog.dialogs == [("Error", "URL is invalid.")]
