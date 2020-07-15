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
    GreetUserCheckInfoWidget,
    GreetUserCodeExchangeWidget,
    GreetUserWidget,
)

from tests.common import customize_fixtures


@pytest.fixture
def catch_greet_user_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.greet_user_widget.GreetUserInstructionsWidget",
        "parsec.core.gui.greet_user_widget.GreetUserCheckInfoWidget",
        "parsec.core.gui.greet_user_widget.GreetUserCodeExchangeWidget",
        "parsec.core.gui.greet_user_widget.GreetUserWidget",
    )


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

    # First switch to users page, and click on the invitation listed there

    u_w = await logged_gui.test_switch_to_users_widget()

    assert u_w.layout_users.count() == 4

    inv_btn = u_w.layout_users.itemAt(3).widget()
    assert isinstance(inv_btn, UserInvitationButton)
    assert inv_btn.email == "fry@pe.com"

    await aqtbot.mouse_click(inv_btn.button_greet, QtCore.Qt.LeftButton)

    # User greet widget should show up now with welcome page

    gu_w = await catch_greet_user_widget()
    assert isinstance(gu_w, GreetUserWidget)

    gui_w = await catch_greet_user_widget()
    assert isinstance(gui_w, GreetUserInstructionsWidget)

    def _greet_user_displayed():
        assert gu_w.dialog.isVisible()
        assert gu_w.isVisible()
        assert gu_w.dialog.label_title.text() == "Greet a new user"
        assert gui_w.isVisible()

    await aqtbot.wait_until(_greet_user_displayed)

    # Now we can setup the boilerplates for the test

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

        # Start the greeting
        await aqtbot.mouse_click(gui_w.button_start, QtCore.Qt.LeftButton)

        def _greet_started():
            assert not gui_w.button_start.isEnabled()
            assert gui_w.button_start.text() == "Waiting for the other user..."

        await aqtbot.wait_until(_greet_started)

        # Start the claimer, this should change page to code exchange
        start_claimer.set()

        guce_w = await catch_greet_user_widget()
        assert isinstance(guce_w, GreetUserCodeExchangeWidget)
        await greeter_sas_available.wait()

        def _greeter_code_displayed():
            assert not gui_w.isVisible()
            assert guce_w.isVisible()
            # We should be displaying the greeter SAS code
            assert guce_w.widget_greeter_code.isVisible()
            assert not guce_w.widget_claimer_code.isVisible()
            assert not guce_w.code_input_widget.isVisible()
            assert guce_w.line_edit_greeter_code.text() == greeter_sas

        await aqtbot.wait_until(_greeter_code_displayed)

        # Now pretent the code was correctly transmitted to the claimer
        start_claimer_trust.set()

        def _claimer_code_choices_displayed():
            assert not guce_w.widget_greeter_code.isVisible()
            assert guce_w.widget_claimer_code.isVisible()
            assert guce_w.code_input_widget.isVisible()
            assert guce_w.code_input_widget.code_layout.count() == 4
            # TODO: better check on codes

        await aqtbot.wait_until(_claimer_code_choices_displayed)

        # Pretend we have choosen the right code
        # TODO: click on button instead of sending the corresponding event
        await aqtbot.run(guce_w.code_input_widget.good_code_clicked.emit)

        def _wait_claimer_info():
            # TODO: unlike with greet_device_widget, there is no
            # `guce_w.label_wait_info` to check for waiting message
            assert not guce_w.widget_greeter_code.isVisible()
            assert not guce_w.widget_claimer_code.isVisible()

        await aqtbot.wait_until(_wait_claimer_info)

        # Claimer info arrive, this should change the page to check info
        start_claimer_claim_user.set()

        guci_w = await catch_greet_user_widget()
        assert isinstance(guci_w, GreetUserCheckInfoWidget)

        def _check_info_displayed():
            assert not guce_w.isVisible()
            assert guci_w.isVisible()
            # assert guci_w.line_edit_user_full_name.text() == requested_human_handle.label
            # assert guci_w.line_edit_user_email.text() == requested_human_handle.email
            assert guci_w.line_edit_device.text() == requested_device_label

        await aqtbot.wait_until(_check_info_displayed)

        # Finally confirm the claimer info and finish the greeting !
        await aqtbot.mouse_click(guci_w.button_create_user, QtCore.Qt.LeftButton)

        def _greet_done():
            assert not gu_w.isVisible()
            assert autoclose_dialog.dialogs == [
                ("", "The user was successfully greeter in your organization.")
            ]

        await aqtbot.wait_until(_greet_done)

        with trio.fail_after(1):
            await claimer_done.wait()
