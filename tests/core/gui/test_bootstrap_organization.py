# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.types import BackendOrganizationBootstrapAddr


async def _gui_ready_for_bootstrap(aqtbot, gui, running_backend, monkeypatch):
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

    login_w = gui.test_get_login_widget()
    assert login_w is not None
    bootstrap_w = gui.test_get_bootstrap_organization_widget()
    assert bootstrap_w is None

    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.TextInputDialog.get_text",
        classmethod(lambda *args, **kwargs: (organization_addr.to_url())),
    )
    await aqtbot.mouse_click(login_w.button_enter_url, QtCore.Qt.LeftButton)

    bootstrap_w = gui.test_get_bootstrap_organization_widget()
    assert bootstrap_w is not None

    await aqtbot.key_clicks(bootstrap_w.line_edit_login, user_id)
    await aqtbot.key_clicks(bootstrap_w.line_edit_password, password)
    await aqtbot.key_clicks(bootstrap_w.line_edit_password_check, password)
    await aqtbot.key_clicks(bootstrap_w.line_edit_device, device_name)


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_organization(aqtbot, running_backend, gui, autoclose_dialog, monkeypatch):
    await _gui_ready_for_bootstrap(aqtbot, gui, running_backend, monkeypatch)

    bootstrap_w = gui.test_get_bootstrap_organization_widget()
    async with aqtbot.wait_signal(bootstrap_w.organization_bootstrapped):
        await aqtbot.mouse_click(bootstrap_w.button_bootstrap, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [
        (
            "Information",
            "The organization and the user have been created.\nYou will now be logged in.",
        )
    ]


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_organization_backend_offline(
    aqtbot, running_backend, gui, autoclose_dialog, monkeypatch
):
    await _gui_ready_for_bootstrap(aqtbot, gui, running_backend, monkeypatch)

    with running_backend.offline():
        bootstrap_w = gui.test_get_bootstrap_organization_widget()

        async with aqtbot.wait_signal(bootstrap_w.bootstrap_error):
            await aqtbot.mouse_click(bootstrap_w.button_bootstrap, QtCore.Qt.LeftButton)
        assert autoclose_dialog.dialogs == [
            ("Error", "Cannot reach the server. Please check your internet connection.")
        ]


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_organization_unknown_error(
    monkeypatch, aqtbot, running_backend, gui, autoclose_dialog
):
    await _gui_ready_for_bootstrap(aqtbot, gui, running_backend, monkeypatch)
    bootstrap_w = gui.test_get_bootstrap_organization_widget()

    def _broken(*args, **kwargs):
        raise RuntimeError()

    monkeypatch.setattr(
        "parsec.core.gui.bootstrap_organization_widget.UserCertificateContent", _broken
    )

    async with aqtbot.wait_signal(bootstrap_w.bootstrap_error):
        await aqtbot.mouse_click(bootstrap_w.button_bootstrap, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [("Error", "Cannot bootstrap this organization.")]
    # TODO: Make a log is emitted


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_organization_with_start_arg(event_bus, core_config, gui_factory):
    start_arg = "parsec://parsec.example.com/my_org?action=bootstrap_organization&token=1234ABCD"

    gui = await gui_factory(event_bus=event_bus, core_config=core_config, start_arg=start_arg)

    bootstrap_w = gui.test_get_bootstrap_organization_widget()
    assert bootstrap_w


@pytest.mark.gui
@pytest.mark.trio
async def test_bootstrap_organization_with_bad_start_arg(
    event_bus, core_config, gui_factory, autoclose_dialog
):
    bad_start_arg = "parsec://parsec.example.com/my_org?action=dummy&token=1234ABCD"

    gui = await gui_factory(event_bus=event_bus, core_config=core_config, start_arg=bad_start_arg)

    bootstrap_w = gui.test_get_bootstrap_organization_widget()
    assert not bootstrap_w
    assert autoclose_dialog.dialogs == [("Error", "URL is invalid.")]
