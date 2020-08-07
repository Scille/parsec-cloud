# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from PyQt5 import QtCore

from tests.common import customize_fixtures

from parsec.api.protocol import OrganizationID
from parsec.core.types import BackendOrganizationBootstrapAddr


@pytest.fixture
def catch_create_org_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.create_org_widget.CreateOrgWidget")


@pytest.fixture
async def organization_bootstrap_addr(running_backend):
    org_id = OrganizationID("AnomalousMaterials")
    org_token = "123456"
    await running_backend.backend.organization.create(org_id, org_token)
    return BackendOrganizationBootstrapAddr.build(running_backend.addr, org_id, org_token)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("backend_status", [True, False])
@customize_fixtures(backend_spontaneous_organization_boostrap=True)
async def test_create_organization(
    gui,
    aqtbot,
    running_backend,
    qt_thread_gateway,
    catch_create_org_widget,
    autoclose_dialog,
    backend_status,
):

    if backend_status:
        gui.config = gui.config.evolve(default_backend_addr=str(running_backend.addr))
    else:
        gui.config = gui.config.evolve(default_backend_addr="parsec://nonexisting.com:6666")

    await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()

    assert co_w
    assert co_w.user_widget.isVisible() is True
    assert co_w.device_widget.isVisible() is False
    assert co_w.button_previous.isVisible() is False
    assert co_w.button_validate.isEnabled() is False

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.user_widget.line_edit_org_name, "AnomalousMaterials")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    assert co_w.button_validate.isEnabled() is True

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    assert co_w.user_widget.isVisible() is False
    assert co_w.device_widget.isVisible() is True
    assert co_w.button_previous.isVisible() is True
    assert co_w.button_validate.isEnabled() is False

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.device_widget.line_edit_password, "nihilanth")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.device_widget.line_edit_password_check, "nihilanth")
    assert co_w.button_validate.isEnabled() is True

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        if backend_status:
            assert autoclose_dialog.dialogs == [
                (
                    "",
                    "You organization <b>AnomalousMaterials</b> has been created!<br />\n<br />\n"
                    "You will now be automatically logged in.<br />\n<br />\n"
                    "To help you start with PARSEC, you can read the "
                    '<a href="https://docs.parsec.cloud/en/stable/" title="User guide">user guide</a>.',
                )
            ]
        else:
            assert autoclose_dialog.dialogs == [("Error", "Cannot connect to the server.")]

    await aqtbot.wait_until(_modal_shown)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_boostrap=True)
async def test_create_organization_previous_clicked(
    gui, aqtbot, running_backend, qt_thread_gateway, catch_create_org_widget, autoclose_dialog
):

    gui.config = gui.config.evolve(default_backend_addr=str(running_backend.addr))

    await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()

    assert co_w
    assert co_w.user_widget.isVisible() is True
    assert co_w.device_widget.isVisible() is False
    assert co_w.button_previous.isVisible() is False
    assert co_w.button_validate.isEnabled() is False

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.user_widget.line_edit_org_name, "AnomalousMaterials")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    assert co_w.button_validate.isEnabled() is True

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    assert co_w.user_widget.isVisible() is False
    assert co_w.device_widget.isVisible() is True
    assert co_w.button_previous.isVisible() is True
    assert co_w.button_validate.isEnabled() is False

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.device_widget.line_edit_password, "nihilanth")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.device_widget.line_edit_password_check, "nihilanth")
    assert co_w.button_validate.isEnabled() is True

    await aqtbot.mouse_click(co_w.button_previous, QtCore.Qt.LeftButton)

    assert co_w.user_widget.isVisible() is True
    assert co_w.device_widget.isVisible() is False
    assert co_w.button_previous.isVisible() is False
    assert co_w.button_validate.isEnabled() is True

    assert co_w.user_widget.line_edit_user_full_name.text() == "Gordon Freeman"
    assert co_w.user_widget.line_edit_user_email.text() == "gordon.freeman@blackmesa.com"
    assert co_w.user_widget.line_edit_org_name.text() == "AnomalousMaterials"
    assert co_w.user_widget.check_accept_contract.isChecked() is True
    assert co_w.button_validate.isEnabled() is True

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)
    assert co_w.device_widget.line_edit_device.text() == "HEV"
    assert co_w.device_widget.line_edit_password.text() == "nihilanth"
    assert co_w.device_widget.line_edit_password_check.text() == "nihilanth"


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_boostrap=True)
async def test_create_organization_bootstrap_only(
    aqtbot,
    running_backend,
    qt_thread_gateway,
    catch_create_org_widget,
    autoclose_dialog,
    gui_factory,
    organization_bootstrap_addr,
    monkeypatch,
):

    gui = await gui_factory(start_arg=organization_bootstrap_addr.to_url())

    co_w = await catch_create_org_widget()

    assert co_w
    assert co_w.user_widget.isVisible() is True
    assert co_w.device_widget.isVisible() is False
    assert co_w.button_previous.isVisible() is False
    assert co_w.button_validate.isEnabled() is False

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    assert co_w.button_validate.isEnabled() is False
    assert co_w.user_widget.line_edit_org_name.text() == "AnomalousMaterials"
    assert co_w.user_widget.line_edit_org_name.isReadOnly() is True
    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    assert co_w.button_validate.isEnabled() is True

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    assert co_w.user_widget.isVisible() is False
    assert co_w.device_widget.isVisible() is True
    assert co_w.button_previous.isVisible() is True
    assert co_w.button_validate.isEnabled() is False

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.device_widget.line_edit_password, "nihilanth")
    assert co_w.button_validate.isEnabled() is False
    await aqtbot.key_clicks(co_w.device_widget.line_edit_password_check, "nihilanth")
    assert co_w.button_validate.isEnabled() is True

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [
            (
                "",
                "You organization <b>AnomalousMaterials</b> has been created!<br />\n<br />\n"
                "You will now be automatically logged in.<br />\n<br />\n"
                "To help you start with PARSEC, you can read the "
                '<a href="https://docs.parsec.cloud/en/stable/" title="User guide">user guide</a>.',
            )
        ]

    await aqtbot.wait_until(_modal_shown)

    autoclose_dialog.reset()

    monkeypatch.setattr(
        "parsec.core.gui.main_window.get_text_input",
        lambda *args, **kwargs: (str(organization_bootstrap_addr)),
    )

    def _join_org():
        gui._on_join_org_clicked()

    await qt_thread_gateway.send_action(_join_org)

    await aqtbot.key_clicks(gui, "o", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    assert co_w.user_widget.line_edit_org_name.text() == "AnomalousMaterials"
    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)
    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    await aqtbot.key_clicks(co_w.device_widget.line_edit_password, "nihilanth")
    await aqtbot.key_clicks(co_w.device_widget.line_edit_password_check, "nihilanth")
    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [("Error", "This bootstrap link was already used.")]

    await aqtbot.wait_until(_modal_shown)
