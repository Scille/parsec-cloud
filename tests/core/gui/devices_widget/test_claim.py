# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore

from parsec.api.protocol import InvitationType
from parsec.core.types import BackendInvitationAddr
from parsec.core.invite import DeviceGreetInitialCtx
from parsec.core.gui.claim_device_widget import (
    ClaimDeviceCodeExchangeWidget,
    ClaimDeviceProvideInfoWidget,
    ClaimDeviceInstructionsWidget,
    ClaimDeviceWidget,
)


@pytest.fixture
async def invitation_addr(backend_addr, backend, bob):
    invitation = await backend.invite.new_for_device(
        bob.organization_id, greeter_user_id=bob.user_id
    )
    return BackendInvitationAddr.build(
        backend_addr=backend_addr,
        organization_id=bob.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=invitation.token,
    )


@pytest.fixture
def catch_claim_device_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.claim_device_widget.ClaimDeviceCodeExchangeWidget",
        "parsec.core.gui.claim_device_widget.ClaimDeviceProvideInfoWidget",
        "parsec.core.gui.claim_device_widget.ClaimDeviceInstructionsWidget",
        "parsec.core.gui.claim_device_widget.ClaimDeviceWidget",
    )


@pytest.mark.gui
@pytest.mark.trio
async def test_claim_device(
    aqtbot,
    gui,
    running_backend,
    autoclose_dialog,
    catch_claim_device_widget,
    invitation_addr,
    bob,
    bob_backend_cmds,
):
    password = "P@ssw0rd."
    device_name = "PC1"

    await aqtbot.run(gui.add_instance, invitation_addr.to_url())

    # We should have a new tab with the invitation ready to be claimed now

    cd_w = await catch_claim_device_widget()
    assert isinstance(cd_w, ClaimDeviceWidget)

    cdi_w = await catch_claim_device_widget()
    assert isinstance(cdi_w, ClaimDeviceInstructionsWidget)

    def _register_display_displayed():
        tab = gui.test_get_tab()
        assert tab and tab.isVisible()
        assert cd_w.isVisible()
        assert cd_w.dialog.label_title.text() == "Register a device"
        assert cdi_w.isVisible()

    await aqtbot.wait_until(_register_display_displayed)

    # Now start greeter

    start_greeter = trio.Event()
    start_greeter_trust = trio.Event()
    start_greeter_create_device = trio.Event()

    greeter_sas = None
    greeter_sas_available = trio.Event()
    claimer_sas = None
    claimer_sas_available = trio.Event()
    greeter_done = trio.Event()

    async def _run_greeter():
        nonlocal greeter_sas
        nonlocal claimer_sas
        await start_greeter.wait()

        initial_ctx = DeviceGreetInitialCtx(cmds=bob_backend_cmds, token=invitation_addr.token)
        in_progress_ctx = await initial_ctx.do_wait_peer()
        greeter_sas = in_progress_ctx.greeter_sas
        greeter_sas_available.set()

        in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()
        claimer_sas = in_progress_ctx.claimer_sas
        claimer_sas_available.set()

        await start_greeter_trust.wait()
        in_progress_ctx = await in_progress_ctx.do_signify_trust()

        await start_greeter_create_device.wait()

        in_progress_ctx = await in_progress_ctx.do_get_claim_requests()

        await in_progress_ctx.do_create_new_device(
            author=bob, device_label=in_progress_ctx.requested_device_label
        )
        greeter_done.set()

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_run_greeter)

        # Start the claim

        await aqtbot.mouse_click(cdi_w.button_start, QtCore.Qt.LeftButton)

        def _claimer_started():
            assert not cdi_w.button_start.isEnabled()
            assert cdi_w.button_start.text() == "Waiting..."

        await aqtbot.wait_until(_claimer_started)

        # Greeter also starts, page should switch to greet SAS code selection

        start_greeter.set()
        await greeter_sas_available.wait()

        cdce_w = await catch_claim_device_widget()
        assert isinstance(cdce_w, ClaimDeviceCodeExchangeWidget)

        def _greeter_sas_code_choices_displayed():
            assert not cdi_w.isVisible()
            assert cdce_w.isVisible()
            assert cdce_w.widget_greeter_code.isVisible()
            assert cdce_w.code_input_widget.isVisible()
            assert cdce_w.code_input_widget.code_layout.count() == 4

        await aqtbot.wait_until(_greeter_sas_code_choices_displayed)

        # Pretend we have choosen the right code, page should switch to claimer SAS code display

        await aqtbot.run(cdce_w.code_input_widget.good_code_clicked.emit)

        def _claimer_sas_code_displayed():
            assert not cdce_w.widget_greeter_code.isVisible()
            assert not cdce_w.code_input_widget.isVisible()
            assert cdce_w.widget_claimer_code.isVisible()
            assert cdce_w.line_edit_claimer_code.isVisible()

        await aqtbot.wait_until(_claimer_sas_code_displayed)

        # Greeter received the right code, page should switch to claim info

        start_greeter_trust.set()

        cdpi_w = await catch_claim_device_widget()
        assert isinstance(cdpi_w, ClaimDeviceProvideInfoWidget)

        def _claim_info_displayed():
            assert not cdce_w.isVisible()
            assert cdpi_w.isVisible()
            assert cdpi_w.line_edit_device.text()  # Should have a default value

        await aqtbot.wait_until(_claim_info_displayed)

        # Fill claim info and submit them to the greeter

        await aqtbot.key_clicks(cdpi_w.line_edit_device, device_name)
        await aqtbot.key_clicks(cdpi_w.line_edit_password, password)
        await aqtbot.key_clicks(cdpi_w.line_edit_password_check, password)
        await aqtbot.mouse_click(cdpi_w.button_ok, QtCore.Qt.LeftButton)

        def _claim_info_submitted():
            assert not cdpi_w.button_ok.isEnabled()
            assert cdpi_w.label_wait.isVisible()

        await aqtbot.wait_until(_claim_info_submitted)

        # Greeter accept our claim info

        start_greeter_create_device.set()

        def _claim_done():
            assert not cd_w.isVisible()
            assert not cdpi_w.isVisible()
            # Should be logged in with the new device
            central_widget = gui.test_get_central_widget()
            assert central_widget and central_widget.isVisible()

        await aqtbot.wait_until(_claim_done)

        with trio.fail_after(1):
            await greeter_done.wait()

    assert autoclose_dialog.dialogs == [("", "The device was successfully created!")]
