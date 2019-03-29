# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.types import BackendOrganizationBootstrapAddr


async def _gui_ready_for_bootstrap(aqtbot, gui, running_backend):
    org_id = "NewOrg"
    org_token = "123456"
    user_id = "Zack"
    device_name = "pc1"
    password = "S3cr3tP@ss"
    organization_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, org_id, org_token
    )

    # Create organization in the backend
    await running_backend.backend.organization.create(org_id, org_token)

    # Go do the bootstrap

    login_w = gui.login_widget
    assert not login_w.bootstrap_organization.isVisible()
    await aqtbot.mouse_click(login_w.button_bootstrap_instead, QtCore.Qt.LeftButton)
    assert login_w.bootstrap_organization.isVisible()

    await aqtbot.key_clicks(login_w.bootstrap_organization.line_edit_login, user_id)
    await aqtbot.key_clicks(login_w.bootstrap_organization.line_edit_password, password)
    await aqtbot.key_clicks(login_w.bootstrap_organization.line_edit_password_check, password)
    await aqtbot.key_clicks(login_w.bootstrap_organization.line_edit_url, organization_addr)
    await aqtbot.key_clicks(login_w.bootstrap_organization.line_edit_device, device_name)


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_organization(aqtbot, running_backend, gui, autoclose_dialog, monitor):
    await _gui_ready_for_bootstrap(aqtbot, gui, running_backend)

    login_w = gui.login_widget
    async with aqtbot.wait_signal(login_w.bootstrap_organization.organization_bootstrapped):
        await aqtbot.mouse_click(
            login_w.bootstrap_organization.button_bootstrap, QtCore.Qt.LeftButton
        )
    assert autoclose_dialog.dialogs == [
        ("Information", "The organization and the user have been created. You can now login.")
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_organization_backend_offline(
    unused_tcp_addr, aqtbot, running_backend, gui, autoclose_dialog
):
    await _gui_ready_for_bootstrap(aqtbot, gui, running_backend)

    with running_backend.offline():
        login_w = gui.login_widget
        async with aqtbot.wait_signal(login_w.bootstrap_organization.bootstrap_error):
            await aqtbot.mouse_click(
                login_w.bootstrap_organization.button_bootstrap, QtCore.Qt.LeftButton
            )
        assert autoclose_dialog.dialogs == [
            ("Error", "Can not bootstrap this organization ([Errno 111] Connection refused).")
        ]


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_organization_unknown_error(
    monkeypatch, aqtbot, running_backend, gui, autoclose_dialog
):
    await _gui_ready_for_bootstrap(aqtbot, gui, running_backend)
    login_w = gui.login_widget

    def _broken(*args, **kwargs):
        raise RuntimeError("Ooops...")

    monkeypatch.setattr(
        "parsec.core.gui.bootstrap_organization_widget.build_user_certificate", _broken
    )

    async with aqtbot.wait_signal(login_w.bootstrap_organization.bootstrap_error):
        await aqtbot.mouse_click(
            login_w.bootstrap_organization.button_bootstrap, QtCore.Qt.LeftButton
        )
    assert autoclose_dialog.dialogs == [
        (
            "Error",
            "Can not bootstrap this organization (Unexpected error: RuntimeError('Ooops...',)).",
        )
    ]
    # TODO: Make a log is emitted
