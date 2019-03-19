# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from PyQt5 import QtCore

from parsec.types import BackendOrganizationAddr
from parsec.core.backend_connection import backend_cmds_factory
from parsec.core.invite_claim import invite_and_create_user
from parsec.core.gui.main_window import MainWindow


@pytest.fixture
def gui(qtbot, core_config):
    main_w = MainWindow(core_config)
    qtbot.addWidget(main_w)
    return main_w


@pytest.fixture
def gui_ready_for_claim(qtbot, gui, backend_service, coolorg, alice):
    backend_service.start(populated=True)

    token = "123456"
    user_id = "Zack"
    device_name = "pc1"
    password = "S3cr3tP@ss"
    org_addr = BackendOrganizationAddr.build(
        backend_service.get_url(), coolorg.organization_id, coolorg.root_verify_key
    )

    # Create invitation in the backend

    async def _invite_user(backend, ready):
        await backend.user.set_user_admin(alice.organization_id, alice.user_id, True)

        async def _wait_for_backend():
            # Make sure the backend is ready to receive user_claim requests
            with backend.event_bus.listen() as spy:
                await spy.wait("event.connected", kwargs={"event_name": "user.claimed"})
                ready()

        async with trio.open_nursery() as nursery:
            nursery.start_soon(_wait_for_backend)
            async with backend_cmds_factory(org_addr, alice.device_id, alice.signing_key) as cmds:
                await invite_and_create_user(alice, cmds, user_id, token, True)

    execution = backend_service.execute_in_thread(_invite_user)

    # Go do the bootstrap

    claim_w = gui.login_widget.claim_user_widget
    assert not claim_w.isVisible()
    qtbot.mouseClick(gui.login_widget.button_register_user_instead, QtCore.Qt.LeftButton)
    # assert claim_w.isVisible()

    qtbot.keyClicks(claim_w.line_edit_login, user_id)
    qtbot.keyClicks(claim_w.line_edit_device, device_name)
    qtbot.keyClicks(claim_w.line_edit_token, token)
    qtbot.keyClicks(claim_w.line_edit_url, org_addr)
    qtbot.keyClicks(claim_w.line_edit_password, password)
    qtbot.keyClicks(claim_w.line_edit_password_check, password)

    return execution


@pytest.mark.gui
def test_claim_user(qtbot, gui, gui_ready_for_claim, autoclose_dialog):
    claim_w = gui.login_widget.claim_user_widget
    with qtbot.waitSignal(claim_w.user_claimed):
        qtbot.mouseClick(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [
        ("Information", "The user has been registered. You can now login.")
    ]


@pytest.mark.gui
def test_claim_user_offline(qtbot, unused_tcp_addr, gui, coolorg, autoclose_dialog):
    token = "123456"
    user_id = "Zack"
    device_name = "pc1"
    password = "S3cr3tP@ss"
    org_addr = BackendOrganizationAddr.build(
        unused_tcp_addr, coolorg.organization_id, coolorg.root_verify_key
    )

    # Go do the bootstrap

    claim_w = gui.login_widget.claim_user_widget
    assert not claim_w.isVisible()
    qtbot.mouseClick(gui.login_widget.button_register_user_instead, QtCore.Qt.LeftButton)
    # assert claim_w.isVisible()

    qtbot.keyClicks(claim_w.line_edit_login, user_id)
    qtbot.keyClicks(claim_w.line_edit_device, device_name)
    qtbot.keyClicks(claim_w.line_edit_token, token)
    qtbot.keyClicks(claim_w.line_edit_url, org_addr)
    qtbot.keyClicks(claim_w.line_edit_password, password)
    qtbot.keyClicks(claim_w.line_edit_password_check, password)

    claim_w = gui.login_widget.claim_user_widget
    with qtbot.waitSignal(claim_w.on_claim_error):
        qtbot.mouseClick(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert len(autoclose_dialog.dialogs) == 1
    assert autoclose_dialog.dialogs[0][0] == "Error"
    assert autoclose_dialog.dialogs[0][1].startswith(
        "Can not claim this user (all attempts to connect to "
    )


@pytest.mark.gui
def test_claim_user_unknown_error(monkeypatch, qtbot, gui, gui_ready_for_claim, autoclose_dialog):
    claim_w = gui.login_widget.claim_user_widget

    async def _broken(*args, **kwargs):
        raise RuntimeError("Ooops...")

    monkeypatch.setattr("parsec.core.gui.claim_user_widget.core_claim_user", _broken)

    with qtbot.waitSignal(claim_w.on_claim_error):
        qtbot.mouseClick(claim_w.button_claim, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [("Error", "Can not claim this user (Ooops...).")]
    # TODO: Make a log is emitted
