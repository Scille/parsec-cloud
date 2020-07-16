# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore

from parsec.api.data import UserProfile
from parsec.api.protocol import InvitationType, HumanHandle
from parsec.core.types import BackendInvitationAddr
from parsec.core.invite import UserGreetInitialCtx
from parsec.core.gui.claim_user_widget import (
    ClaimUserFinalizeWidget,
    ClaimUserCodeExchangeWidget,
    ClaimUserProvideInfoWidget,
    ClaimUserInstructionsWidget,
    ClaimUserWidget,
)

from tests.common import customize_fixtures


@pytest.fixture
async def invitation_addr(backend_addr, backend, alice):
    invitation = await backend.invite.new_for_user(
        organization_id=alice.organization_id,
        greeter_user_id=alice.user_id,
        claimer_email="philip.j.fry@pe.com",
    )
    return BackendInvitationAddr.build(
        backend_addr=backend_addr,
        organization_id=alice.organization_id,
        invitation_type=InvitationType.USER,
        token=invitation.token,
    )


@pytest.fixture
def catch_claim_user_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.claim_user_widget.ClaimUserFinalizeWidget",
        "parsec.core.gui.claim_user_widget.ClaimUserCodeExchangeWidget",
        "parsec.core.gui.claim_user_widget.ClaimUserProvideInfoWidget",
        "parsec.core.gui.claim_user_widget.ClaimUserInstructionsWidget",
        "parsec.core.gui.claim_user_widget.ClaimUserWidget",
    )


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_claim_device(
    aqtbot,
    gui,
    running_backend,
    autoclose_dialog,
    catch_claim_user_widget,
    invitation_addr,
    alice,
    alice_backend_cmds,
):
    password = "P@ssw0rd."
    device_name = "PC1"
    human_email = "philip.j.fry@pe.com"
    human_label = "Philip J. Fry"

    await aqtbot.run(gui.add_instance, invitation_addr.to_url())

    # We should have a new tab with the invitation ready to be claimed now

    cu_w = await catch_claim_user_widget()
    assert isinstance(cu_w, ClaimUserWidget)
    cui_w = await catch_claim_user_widget()
    assert isinstance(cui_w, ClaimUserInstructionsWidget)

    def _register_user_displayed():
        tab = gui.test_get_tab()
        assert tab and tab.isVisible()
        assert cu_w.isVisible()
        assert cu_w.dialog.label_title.text() == "Register a user"
        assert cui_w.isVisible()

    await aqtbot.wait_until(_register_user_displayed)

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

        initial_ctx = UserGreetInitialCtx(cmds=alice_backend_cmds, token=invitation_addr.token)
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

        await in_progress_ctx.do_create_new_user(
            author=alice,
            device_label=in_progress_ctx.requested_device_label,
            human_handle=HumanHandle(email=human_email, label=human_label),
            profile=UserProfile.STANDARD,
        )
        greeter_done.set()

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_run_greeter)

        # Start the claim

        await aqtbot.mouse_click(cui_w.button_start, QtCore.Qt.LeftButton)

        def _claimer_started():
            assert not cui_w.button_start.isEnabled()
            assert cui_w.button_start.text() == "Waiting for the other user"

        await aqtbot.wait_until(_claimer_started)

        # Greeter also starts, page should switch to greet SAS code selection

        start_greeter.set()
        await greeter_sas_available.wait()

        cuce_w = await catch_claim_user_widget()
        assert isinstance(cuce_w, ClaimUserCodeExchangeWidget)

        def _greeter_sas_code_choices_displayed():
            assert not cui_w.isVisible()
            assert cuce_w.isVisible()
            assert cuce_w.widget_greeter_code.isVisible()
            assert cuce_w.code_input_widget.isVisible()
            assert cuce_w.code_input_widget.code_layout.count() == 4

        await aqtbot.wait_until(_greeter_sas_code_choices_displayed)

        # Pretend we have choosen the right code, page should switch to claimer SAS code display

        await aqtbot.run(cuce_w.code_input_widget.good_code_clicked.emit)

        def _claimer_sas_code_displayed():
            assert not cuce_w.widget_greeter_code.isVisible()
            assert not cuce_w.code_input_widget.isVisible()
            assert cuce_w.widget_claimer_code.isVisible()
            assert cuce_w.line_edit_claimer_code.isVisible()

        await aqtbot.wait_until(_claimer_sas_code_displayed)

        # Greeter received the right code, page should switch to claim info

        start_greeter_trust.set()

        cupi_w = await catch_claim_user_widget()
        assert isinstance(cupi_w, ClaimUserProvideInfoWidget)

        def _claim_info_displayed():
            assert not cuce_w.isVisible()
            assert cupi_w.isVisible()
            assert cupi_w.line_edit_device.text()  # Should have a default value

        await aqtbot.wait_until(_claim_info_displayed)

        # Fill claim info and submit them to the greeter

        await aqtbot.key_clicks(cupi_w.line_edit_user_email, human_email)
        await aqtbot.key_clicks(cupi_w.line_edit_user_full_name, human_label)
        await aqtbot.key_clicks(cupi_w.line_edit_device, device_name)
        await aqtbot.mouse_click(cupi_w.button_ok, QtCore.Qt.LeftButton)

        def _claim_info_submitted():
            assert not cupi_w.button_ok.isEnabled()
            assert cupi_w.label_wait.isVisible()

        await aqtbot.wait_until(_claim_info_submitted)

        # Greeter accept our claim info, page should switch to finalization

        start_greeter_create_device.set()

        # breakpoint()
        cuf_w = await catch_claim_user_widget()
        assert isinstance(cuf_w, ClaimUserFinalizeWidget)

        def _claim_finish_displayed():
            assert not cupi_w.isVisible()
            assert cuf_w.isVisible()

        await aqtbot.wait_until(_claim_finish_displayed)

        # Fill password and we're good to go ;-)

        await aqtbot.key_clicks(cuf_w.line_edit_password, password)
        await aqtbot.key_clicks(cuf_w.line_edit_password_check, password)
        await aqtbot.mouse_click(cuf_w.button_finalize, QtCore.Qt.LeftButton)

        def _claim_done():
            assert not cu_w.isVisible()
            assert not cuf_w.isVisible()
            # Should be logged in with the new device
            central_widget = gui.test_get_central_widget()
            assert central_widget and central_widget.isVisible()

        await aqtbot.wait_until(_claim_done)

        with trio.fail_after(1):
            await greeter_done.wait()

    assert autoclose_dialog.dialogs == [
        ("", "The user was successfully created. You will now be logged in.\nWelcome to Parsec!")
    ]
