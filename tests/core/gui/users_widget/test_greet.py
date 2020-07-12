# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore

from parsec.api.protocol import InvitationType, HumanHandle
from parsec.core.types import BackendInvitationAddr
from parsec.core.backend_connection import backend_invited_cmds_factory
from parsec.core.invite import claimer_retrieve_info
from parsec.core.gui.users_widget import UserInvitationButton
from parsec.core.gui.greet_user_widget import (
    GreetUserInstructionsWidget,
    GreetUserCodeExchangeWidget,
    GreetUserCheckInfoWidget,
)

from tests.common import customize_fixtures


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
    alice,
    monitor,
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

    start_claimer = trio.Event()
    start_claimer_trust = trio.Event()
    start_claimer_claim_user = trio.Event()

    greeter_sas = None
    greeter_sas_available = trio.Event()
    claimer_sas = None
    claimer_sas_available = trio.Event()

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

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_run_claimer)

        u_w = await logged_gui.test_switch_to_users_widget()

        assert u_w.layout_users.count() == 4

        inv_btn = u_w.layout_users.itemAt(3).widget()
        assert isinstance(inv_btn, UserInvitationButton)
        assert inv_btn.email == "fry@pe.com"

        # Click on greet button and capture the dialog
        widget = None

        from parsec.core.gui.greet_user_widget import GreetUserWidget

        vanilla_exec_modal = GreetUserWidget.exec_modal

        def _patched_exec_modal(*args, **kwargs):
            nonlocal widget
            widget = vanilla_exec_modal(*args, **kwargs)
            return widget

        monkeypatch.setattr(
            "parsec.core.gui.greet_user_widget.GreetUserWidget.exec_modal",
            _patched_exec_modal,
            raising=False,
        )

        await aqtbot.mouse_click(inv_btn.button_greet, QtCore.Qt.LeftButton)
        dialog = widget.dialog

        # Welcome page should be displayed
        assert dialog.label_title.text() == "Greet a new user"
        current_page = widget._get_current_page()
        assert isinstance(current_page, GreetUserInstructionsWidget)

        # Connect to peer
        async with aqtbot.wait_signal(current_page.wait_peer_success):
            await aqtbot.mouse_click(current_page.button_start, QtCore.Qt.LeftButton)
            # We should be waiting for greeter now
            assert not current_page.button_start.isEnabled()
            assert current_page.button_start.text() == "Waiting for the other user..."

            # Now tell claimer to start
            start_claimer.set()
        await aqtbot.run(qapp.processEvents)

        # Greeter SAS code display page
        current_page = widget._get_current_page()
        assert isinstance(current_page, GreetUserCodeExchangeWidget)
        assert current_page.widget_claimer_code.isHidden()
        assert not current_page.widget_greeter_code.isHidden()

        await greeter_sas_available.wait()

        # Cannot wait on current_page.get_greeter_sas_success signal given
        # we don't have access to this signal when the when get_greeter_sas_job
        # is triggered (which completion leads to get_greeter_sas_success)
        for _ in range(10):
            if current_page.line_edit_greeter_code.text():
                break
            await trio.sleep(0.1)
        else:
            raise AssertionError("Greeter SAS code still not available")
        assert current_page.line_edit_greeter_code.text() == greeter_sas
        await aqtbot.run(qapp.processEvents)

        # Claimer enter the greeter SAS, we should jump to next step
        async with aqtbot.wait_signal(current_page.wait_peer_trust_success):
            start_claimer_trust.set()
        await aqtbot.run(qapp.processEvents)

        # Claimer SAS code selection page
        assert not current_page.widget_claimer_code.isHidden()
        assert current_page.widget_greeter_code.isHidden()
        await aqtbot.run(qapp.processEvents)

        # TODO: check SAS code selection buttons
        async with aqtbot.wait_signal(current_page.signify_trust_success):
            await aqtbot.run(current_page.code_input_widget.good_code_clicked.emit)

        # Claimer information check page
        current_page = widget._get_current_page()
        assert isinstance(current_page, GreetUserCheckInfoWidget)

        async with aqtbot.wait_signal(current_page.succeeded):
            start_claimer_claim_user.set()

        # # Finally terminate the dialog, this should trigger a refresh on the
        # # user list and show the new user
        # async with aqtbot.wait_signal(u_w.list_success):
        #     dialog.done.set()

        nursery.cancel_scope.cancel()
