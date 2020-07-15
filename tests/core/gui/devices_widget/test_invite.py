# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore
from unittest.mock import ANY

from parsec.core.backend_connection import backend_invited_cmds_factory
from parsec.core.invite import claimer_retrieve_info
from parsec.core.gui.greet_device_widget import GreetDeviceWidget

from tests.common import customize_fixtures


@pytest.fixture
def catch_greet_device_widget(monkeypatch):
    widgets = []

    vanilla_exec_modal = GreetDeviceWidget.exec_modal

    def _patched_exec_modal(*args, **kwargs):
        widget = vanilla_exec_modal(*args, **kwargs)
        widgets.append(widget)
        return widget

    monkeypatch.setattr(
        "parsec.core.gui.greet_device_widget.GreetDeviceWidget.exec_modal",
        _patched_exec_modal,
        raising=False,
    )

    return widgets


@pytest.mark.gui
@pytest.mark.trio
async def test_invite_device_offline(aqtbot, logged_gui, autoclose_dialog, running_backend):
    d_w = await logged_gui.test_switch_to_devices_widget()

    # TODO: not sure why but this timeout without running_backend fixture...
    with running_backend.offline():
        async with aqtbot.wait_signal(d_w.invite_error):
            await aqtbot.mouse_click(d_w.button_add_device, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [
        ("Error", "The server is offline or you have no access to the internet.")
    ]


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("online", (True, False))
@customize_fixtures(backend_has_email=True)
async def test_invite_device_send_email(
    aqtbot,
    logged_gui,
    running_backend,
    autoclose_dialog,
    email_letterbox,
    catch_greet_device_widget,
    bob,
    online,
):
    d_w = await logged_gui.test_switch_to_devices_widget()

    async with aqtbot.wait_signal(d_w.invite_success):
        await aqtbot.mouse_click(d_w.button_add_device, QtCore.Qt.LeftButton)

    # Device invitation widget should show up now

    def invitation_shown():
        assert len(catch_greet_device_widget) == 1

    await aqtbot.wait_until(invitation_shown)

    gd_w = catch_greet_device_widget[0]
    assert isinstance(gd_w, GreetDeviceWidget)
    assert gd_w.dialog.label_title.text() == "Greet a new device"

    assert not gd_w.greet_device_instructions_widget.isHidden()
    assert gd_w.greet_device_code_exchange_widget.isHidden()

    gdi_w = gd_w.greet_device_instructions_widget
    if online:
        async with aqtbot.wait_signal(gdi_w.send_email_success):
            await aqtbot.mouse_click(gdi_w.button_send_email, QtCore.Qt.LeftButton)
        assert email_letterbox == [(bob.human_handle.email, ANY)]

    else:
        with running_backend.offline():
            async with aqtbot.wait_signal(gdi_w.send_email_error):
                await aqtbot.mouse_click(gdi_w.button_send_email, QtCore.Qt.LeftButton)
        assert not email_letterbox


# TODO: test copy invitation link


@pytest.mark.gui
@pytest.mark.trio
async def test_invite_and_greet_device(
    aqtbot, logged_gui, running_backend, autoclose_dialog, catch_greet_device_widget, bob
):
    requested_device_label = "PC1"

    d_w = await logged_gui.test_switch_to_devices_widget()

    async with aqtbot.wait_signal(d_w.invite_success):
        await aqtbot.mouse_click(d_w.button_add_device, QtCore.Qt.LeftButton)

    # Device invitation widget should show up now

    def invitation_shown():
        assert len(catch_greet_device_widget) == 1

    await aqtbot.wait_until(invitation_shown)

    gd_w = catch_greet_device_widget[0]
    gdi_w = gd_w.greet_device_instructions_widget
    gdce_w = gd_w.greet_device_code_exchange_widget

    assert isinstance(gd_w, GreetDeviceWidget)
    assert gd_w.dialog.label_title.text() == "Greet a new device"

    # Welcome page should be displayed
    assert gdi_w.isVisible()
    assert gdce_w.isHidden()

    await aqtbot.mouse_click(gdi_w.button_start, QtCore.Qt.LeftButton)

    assert not gdi_w.button_start.isEnabled()
    assert gdi_w.button_start.text() == "Waiting for the new device..."

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

        async with backend_invited_cmds_factory(addr=gdi_w.invite_addr) as cmds:
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

            await in_progress_ctx.do_claim_device(requested_device_label=requested_device_label)
            claimer_done.set()

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_run_claimer)
        async with aqtbot.wait_signal(gdce_w.get_greeter_sas_success):
            async with aqtbot.wait_signal(gdi_w.succeeded):
                start_claimer.set()

        # Next page shows up (SAS code exchange)
        assert gdi_w.isHidden()
        assert gdce_w.isVisible()

        # We should be displaying the greeter SAS code
        await greeter_sas_available.wait()
        assert gdce_w.widget_greeter_code.isVisible()
        assert gdce_w.widget_claimer_code.isHidden()
        assert gdce_w.line_edit_greeter_code.text() == greeter_sas

        # Now pretent the code was correctly transmitted to the claimer
        async with aqtbot.wait_signal(gdce_w.get_claimer_sas_success):
            async with aqtbot.wait_signal(gdce_w.wait_peer_trust_success):
                start_claimer_trust.set()

        # We now should be displaying the possible claimer SAS codes
        assert gdce_w.widget_greeter_code.isHidden()
        assert not gdce_w.widget_claimer_code.isHidden()
        # TODO: better check on codes

        # Finally we are waiting for claimer informations
        async with aqtbot.wait_signal(gd_w.dialog.finished):
            async with aqtbot.wait_signal(gdce_w.succeeded):
                # Pretent we choose the right code
                # TODO: click on button instead of sending the corresponding event
                await aqtbot.run(gdce_w.code_input_widget.good_code_clicked.emit)
                start_claimer_claim_user.set()

        with trio.fail_after(1):
            await claimer_done.wait()

    assert autoclose_dialog.dialogs == [("", "The device was successfully created.")]


# TODO: test offline during the claim steps
# TODO: test other-than offline errors (e.g. invitation used by another client in our back)
