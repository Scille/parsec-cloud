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


@pytest.fixture
def ClaimUserTestBed(
    aqtbot,
    catch_claim_user_widget,
    autoclose_dialog,
    backend,
    running_backend,
    gui,
    alice,
    alice_backend_cmds,
):
    class _ClaimUserTestBed:
        def __init__(self):
            self.requested_human_handle = HumanHandle(email="pfry@pe.com", label="Philip J. Fry")
            self.requested_device_label = "PC1"
            self.password = "P@ssw0rd."
            self.steps_done = []

            self.author = alice
            self.cmds = alice_backend_cmds

            # Set during bootstrap
            self.invitation_addr = None
            self.claim_user_widget = None
            self.claim_user_instructions_widget = None

            # Set by step 2
            self.claimer_user_code_exchange_widget = None

            # Set by step 5
            self.claimer_claim_task = None
            self.greet_user_check_informations_widget = None

        async def run(self):
            await self.bootstrap()
            async with trio.open_nursery() as self.nursery:
                next_step = "step_1_start_claim"
                while True:
                    current_step = next_step
                    next_step = await getattr(self, current_step)()
                    self.steps_done.append(current_step)
                    if next_step is None:
                        break
                if self.claimer_claim_task:
                    await self.claimer_claim_task.cancel_and_join()

        async def bootstrap(self):
            claimer_email = self.requested_human_handle.email

            # Create new invitation

            invitation = await backend.invite.new_for_user(
                organization_id=self.author.organization_id,
                greeter_user_id=self.author.user_id,
                claimer_email=claimer_email,
            )
            invitation_addr = BackendInvitationAddr.build(
                backend_addr=self.author.organization_addr,
                organization_id=self.author.organization_id,
                invitation_type=InvitationType.USER,
                token=invitation.token,
            )

            # Switch to users claim page

            await aqtbot.run(gui.add_instance, invitation_addr.to_url())

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

            self.invitation_addr = invitation_addr
            self.claim_user_widget = cu_w
            self.claim_user_instructions_widget = cui_w

        async def step_1_start_claim(self):
            cui_w = self.claim_user_instructions_widget
            await aqtbot.mouse_click(cui_w.button_start, QtCore.Qt.LeftButton)

            def _claimer_started():
                assert not cui_w.button_start.isEnabled()
                assert cui_w.button_start.text() == "Waiting for the other user"

            await aqtbot.wait_until(_claimer_started)

            return "step_2_start_greeter"

        async def step_2_start_greeter(self):
            cui_w = self.claim_user_instructions_widget

            self.greeter_initial_ctx = UserGreetInitialCtx(
                cmds=self.cmds, token=self.invitation_addr.token
            )
            self.greeter_in_progress_ctx = await self.greeter_initial_ctx.do_wait_peer()

            cuce_w = await catch_claim_user_widget()
            assert isinstance(cuce_w, ClaimUserCodeExchangeWidget)

            def _greeter_sas_code_choices_displayed():
                assert not cui_w.isVisible()
                assert cuce_w.isVisible()
                assert cuce_w.widget_greeter_code.isVisible()
                assert cuce_w.code_input_widget.isVisible()
                assert cuce_w.code_input_widget.code_layout.count() == 4
                # TODO: better check on codes

            await aqtbot.wait_until(_greeter_sas_code_choices_displayed)

            self.claimer_user_code_exchange_widget = cuce_w

            return "step_3_exchange_greeter_sas"

        async def step_3_exchange_greeter_sas(self):
            cuce_w = self.claimer_user_code_exchange_widget

            # Pretend we have choosen the right code
            await aqtbot.run(cuce_w.code_input_widget.good_code_clicked.emit)

            self.greeter_in_progress_ctx = await self.greeter_in_progress_ctx.do_wait_peer_trust()
            claimer_sas = self.greeter_in_progress_ctx.claimer_sas

            def _claimer_sas_code_displayed():
                assert not cuce_w.widget_greeter_code.isVisible()
                assert not cuce_w.code_input_widget.isVisible()
                assert cuce_w.widget_claimer_code.isVisible()
                assert cuce_w.line_edit_claimer_code.isVisible()
                assert cuce_w.line_edit_claimer_code.text() == claimer_sas

            await aqtbot.wait_until(_claimer_sas_code_displayed)

            return "step_4_exchange_claimer_sas"

        async def step_4_exchange_claimer_sas(self):
            cuce_w = self.claimer_user_code_exchange_widget

            self.greeter_in_progress_ctx = await self.greeter_in_progress_ctx.do_signify_trust()

            cupi_w = await catch_claim_user_widget()
            assert isinstance(cupi_w, ClaimUserProvideInfoWidget)

            def _claim_info_displayed():
                assert not cuce_w.isVisible()
                assert cupi_w.isVisible()
                assert cupi_w.line_edit_device.text()  # Should have a default value

            await aqtbot.wait_until(_claim_info_displayed)

            self.greet_user_provide_info_widget = cupi_w

            return "step_5_provide_claim_info"

        async def step_5_provide_claim_info(self):
            cupi_w = self.greet_user_provide_info_widget
            human_email = self.requested_human_handle.email
            human_label = self.requested_human_handle.label
            device_label = self.requested_device_label

            await aqtbot.key_clicks(cupi_w.line_edit_user_email, human_email)
            await aqtbot.key_clicks(cupi_w.line_edit_user_full_name, human_label)
            await aqtbot.run(cupi_w.line_edit_device.clear)
            await aqtbot.key_clicks(cupi_w.line_edit_device, device_label)
            await aqtbot.mouse_click(cupi_w.button_ok, QtCore.Qt.LeftButton)

            def _claim_info_submitted():
                assert not cupi_w.button_ok.isEnabled()
                assert cupi_w.label_wait.isVisible()

            await aqtbot.wait_until(_claim_info_submitted)

            self.greeter_in_progress_ctx = (
                await self.greeter_in_progress_ctx.do_get_claim_requests()
            )
            assert (
                self.greeter_in_progress_ctx.requested_device_label == self.requested_device_label
            )
            assert (
                self.greeter_in_progress_ctx.requested_human_handle == self.requested_human_handle
            )

            return "step_6_validate_claim_info"

        async def step_6_validate_claim_info(self):
            cupi_w = self.greet_user_provide_info_widget

            await self.greeter_in_progress_ctx.do_create_new_user(
                author=self.author,
                device_label=self.greeter_in_progress_ctx.requested_device_label,
                human_handle=self.requested_human_handle,
                profile=UserProfile.STANDARD,
            )

            cuf_w = await catch_claim_user_widget()
            assert isinstance(cuf_w, ClaimUserFinalizeWidget)

            def _claim_finish_displayed():
                assert not cupi_w.isVisible()
                assert cuf_w.isVisible()

            await aqtbot.wait_until(_claim_finish_displayed)

            self.claim_user_finalize = cuf_w

            return "step_7_finalize"

        async def step_7_finalize(self):
            cu_w = self.claim_user_widget
            cuf_w = self.claim_user_finalize

            # Fill password and we're good to go ;-)

            await aqtbot.key_clicks(cuf_w.line_edit_password, self.password)
            await aqtbot.key_clicks(cuf_w.line_edit_password_check, self.password)
            await aqtbot.mouse_click(cuf_w.button_finalize, QtCore.Qt.LeftButton)

            def _claim_done():
                assert not cu_w.isVisible()
                assert not cuf_w.isVisible()
                # Should be logged in with the new device
                central_widget = gui.test_get_central_widget()
                assert central_widget and central_widget.isVisible()

            await aqtbot.wait_until(_claim_done)

            assert autoclose_dialog.dialogs == [
                (
                    "",
                    "The user was successfully created. You will now be logged in.\nWelcome to Parsec!",
                )
            ]

            return None  # Test is done \o/

    return _ClaimUserTestBed


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_claim_user(ClaimUserTestBed):
    await ClaimUserTestBed().run()
