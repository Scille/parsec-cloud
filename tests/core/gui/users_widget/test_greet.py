# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore

from parsec.api.protocol import InvitationType, HumanHandle
from parsec.core.types import BackendInvitationAddr
from parsec.core.backend_connection import backend_invited_cmds_factory
from parsec.core.invite import claimer_retrieve_info
from parsec.core.gui.users_widget import UserInvitationButton
from parsec.core.gui.greet_user_widget import GreetUserWidget

from tests.common import customize_fixtures


@pytest.fixture
def catch_greet_user_widget(monkeypatch):
    widgets = []

    vanilla_exec_modal = GreetUserWidget.exec_modal

    def _patched_exec_modal(*args, **kwargs):
        widget = vanilla_exec_modal(*args, **kwargs)
        widgets.append(widget)
        return widget

    monkeypatch.setattr(
        "parsec.core.gui.greet_user_widget.GreetUserWidget.exec_modal",
        _patched_exec_modal,
        raising=False,
    )

    return widgets


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_greet_user(
    qapp,
    aqtbot,
    logged_gui,
    running_backend,
    backend,
    monkeypatch,
    autoclose_dialog,
    catch_greet_user_widget,
    alice,
):
    claimer_email = "fry@pe.com"
    requested_device_label = "PC1"
    requested_human_handle = HumanHandle(email=claimer_email, label="Philip J. Fry")

    invitation = await backend.invite.new_for_user(
        organization_id=alice.organization_id,
        greeter_user_id=alice.user_id,
        claimer_email=claimer_email,
    )
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=alice.organization_addr,
        organization_id=alice.organization_id,
        invitation_type=InvitationType.USER,
        token=invitation.token,
    )

    u_w = await logged_gui.test_switch_to_users_widget()

    assert u_w.layout_users.count() == 4

    inv_btn = u_w.layout_users.itemAt(3).widget()
    assert isinstance(inv_btn, UserInvitationButton)
    assert inv_btn.email == "fry@pe.com"

    # Click on greet button and capture the dialog

    await aqtbot.mouse_click(inv_btn.button_greet, QtCore.Qt.LeftButton)

    def invitation_shown():
        assert len(catch_greet_user_widget) == 1

    await aqtbot.wait_until(invitation_shown)

    gu_w = catch_greet_user_widget[0]
    gui_w = gu_w.greet_user_instructions_widget
    guce_w = gu_w.greet_user_code_exchange_widget
    guci_w = gu_w.greet_user_check_info_widget

    assert isinstance(gu_w, GreetUserWidget)
    assert gu_w.dialog.label_title.text() == "Greet a new user"

    # Welcome page should be displayed
    assert gui_w.isVisible()
    assert guce_w.isHidden()
    assert guci_w.isHidden()

    await aqtbot.mouse_click(gui_w.button_start, QtCore.Qt.LeftButton)

    assert not gui_w.button_start.isEnabled()
    assert gui_w.button_start.text() == "Waiting for the other user..."

    # Now start claimer

    start_claimer = trio.Event()
    start_claimer_trust = trio.Event()
    start_claimer_claim_user = trio.Event()

    greeter_sas = None
    greeter_sas_available = trio.Event()
    claimer_sas = None
    claimer_sas_available = trio.Event()
    claimer_done = trio.Event()

    async def _run_claimer():
        nonlocal greeter_sas
        nonlocal claimer_sas

        async with backend_invited_cmds_factory(addr=invitation_addr) as cmds:
            await start_claimer.wait()

            initial_ctx = await claimer_retrieve_info(cmds)
            in_progress_ctx = await initial_ctx.do_wait_peer()
            greeter_sas = in_progress_ctx.greeter_sas
            greeter_sas_available.set()

            await start_claimer_trust.wait()

            in_progress_ctx = await in_progress_ctx.do_signify_trust()
            claimer_sas = in_progress_ctx.claimer_sas
            claimer_sas_available.set()
            in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()

            await start_claimer_claim_user.wait()

            await in_progress_ctx.do_claim_user(
                requested_device_label=requested_device_label,
                requested_human_handle=requested_human_handle,
            )
            claimer_done.set()

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_run_claimer)

        async with aqtbot.wait_signal(guce_w.get_greeter_sas_success):
            async with aqtbot.wait_signal(gui_w.wait_peer_success):
                start_claimer.set()

        # Next page shows up (SAS code exchange)
        assert gui_w.isHidden()
        assert guce_w.isVisible()
        assert guci_w.isHidden()

        # We should be displaying the greeter SAS code
        await greeter_sas_available.wait()
        assert guce_w.widget_greeter_code.isVisible()
        assert guce_w.widget_claimer_code.isHidden()
        assert guce_w.line_edit_greeter_code.text() == greeter_sas

        # Now pretent the code was correctly transmitted to the claimer
        async with aqtbot.wait_signal(guce_w.get_claimer_sas_success):
            async with aqtbot.wait_signal(guce_w.wait_peer_trust_success):
                start_claimer_trust.set()

        # We now should be displaying the possible claimer SAS codes
        assert guce_w.widget_greeter_code.isHidden()
        assert not guce_w.widget_claimer_code.isHidden()
        # TODO: better check on codes

        async with aqtbot.wait_signal(guci_w.get_requests_success):
            async with aqtbot.wait_signal(guce_w.succeeded):
                # Pretent we choose the right code
                # TODO: click on button instead of sending the corresponding event
                await aqtbot.run(guce_w.code_input_widget.good_code_clicked.emit)
                start_claimer_claim_user.set()

        # Next step is retrieving the claimer informations
        assert gui_w.isHidden()
        assert guce_w.isHidden()
        assert guci_w.isVisible()

        assert guci_w.line_edit_user_full_name.text() == requested_human_handle.label
        assert guci_w.line_edit_user_email.text() == requested_human_handle.email
        assert guci_w.line_edit_device.text() == requested_device_label

        async with aqtbot.wait_signal(gu_w.dialog.finished):
            async with aqtbot.wait_signal(guci_w.succeeded):
                await aqtbot.mouse_click(guci_w.button_create_user, QtCore.Qt.LeftButton)

        with trio.fail_after(1):
            await claimer_done.wait()

    assert autoclose_dialog.dialogs == [
        ("", "The user was successfully greeter in your organization.")
    ]
