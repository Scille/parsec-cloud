# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from PyQt5 import QtCore

from tests.fixtures import local_device_to_backend_user
from tests.common import customize_fixtures

from parsec.api.protocol import OrganizationID
from parsec.core.types import BackendOrganizationBootstrapAddr, BackendAddr

from parsec.core.gui.lang import translate


@pytest.fixture
def catch_create_org_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.create_org_widget.CreateOrgWidget")


@pytest.fixture
async def organization_bootstrap_addr(running_backend):
    org_id = OrganizationID("AnomalousMaterials")
    org_token = "123456"
    await running_backend.backend.organization.create(org_id, org_token)
    return BackendOrganizationBootstrapAddr.build(running_backend.addr, org_id, org_token)


async def _do_creation_process(aqtbot, co_w):
    await aqtbot.wait_until(co_w.user_widget.isVisible)
    await aqtbot.wait_until(co_w.device_widget.isHidden)
    await aqtbot.wait_until(co_w.button_previous.isHidden)
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.user_widget.line_edit_org_name, "AnomalousMaterials")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    assert co_w.button_validate.isEnabled()

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(co_w.user_widget.isHidden)
    await aqtbot.wait_until(co_w.device_widget.isVisible)
    await aqtbot.wait_until(co_w.button_previous.isVisible)
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.device_widget.widget_password.line_edit_password, "nihilanth")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(
        co_w.device_widget.widget_password.line_edit_password_check, "nihilanth"
    )
    assert co_w.button_validate.isEnabled()

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)


@pytest.mark.flaky(reruns=1)
@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_boostrap=True)
async def test_create_organization(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog
):
    # The org creation window is usually opened using a sub-menu.
    # Sub-menus can be a bit challenging to open in tests so we cheat
    # using the keyboard shortcut Ctrl+N that has the same effect.
    await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()

    assert co_w
    await _do_creation_process(aqtbot, co_w)

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


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_boostrap=True)
async def test_create_organization_offline(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog
):
    with running_backend.offline():
        await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

        co_w = await catch_create_org_widget()
        assert co_w

        await _do_creation_process(aqtbot, co_w)

        def _modal_shown():
            assert autoclose_dialog.dialogs == [("Error", "Cannot connect to the server.")]

        await aqtbot.wait_until(_modal_shown)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.flaky(reruns=1)
@customize_fixtures(backend_spontaneous_organization_boostrap=True)
async def test_create_organization_previous_clicked(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog
):
    await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()

    assert co_w
    await aqtbot.wait_until(co_w.user_widget.isVisible)

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    await aqtbot.key_clicks(co_w.user_widget.line_edit_org_name, "AnomalousMaterials")
    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(co_w.device_widget.isVisible)

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    await aqtbot.key_clicks(co_w.device_widget.widget_password.line_edit_password, "nihilanth")
    await aqtbot.key_clicks(
        co_w.device_widget.widget_password.line_edit_password_check, "nihilanth"
    )

    await aqtbot.mouse_click(co_w.button_previous, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(co_w.user_widget.isVisible)
    await aqtbot.wait_until(co_w.device_widget.isHidden)
    await aqtbot.wait_until(co_w.button_previous.isHidden)
    assert co_w.button_validate.isEnabled()

    assert co_w.user_widget.line_edit_user_full_name.text() == "Gordon Freeman"
    assert co_w.user_widget.line_edit_user_email.text() == "gordon.freeman@blackmesa.com"
    assert co_w.user_widget.line_edit_org_name.text() == "AnomalousMaterials"
    assert co_w.user_widget.check_accept_contract.isChecked()

    assert co_w.button_validate.isEnabled()

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)
    await aqtbot.wait_until(co_w.device_widget.isVisible)

    assert co_w.device_widget.line_edit_device.text() == "HEV"
    assert co_w.device_widget.widget_password.line_edit_password.text() == "nihilanth"
    assert co_w.device_widget.widget_password.line_edit_password_check.text() == "nihilanth"


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

    await gui_factory(start_arg=organization_bootstrap_addr.to_url())

    co_w = await catch_create_org_widget()

    assert co_w

    assert co_w.label_instructions.text() == translate(
        "TEXT_BOOTSTRAP_ORGANIZATION_INSTRUCTIONS_organization"
    ).format(organization="AnomalousMaterials")

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    assert co_w.user_widget.line_edit_org_name.text() == "AnomalousMaterials"
    assert co_w.user_widget.line_edit_org_name.isReadOnly() is True
    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(co_w.device_widget.isVisible)

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    await aqtbot.key_clicks(co_w.device_widget.widget_password.line_edit_password, "nihilanth")
    await aqtbot.key_clicks(
        co_w.device_widget.widget_password.line_edit_password_check, "nihilanth"
    )

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


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_boostrap=True)
async def test_create_organization_already_bootstrapped(
    aqtbot,
    running_backend,
    catch_create_org_widget,
    autoclose_dialog,
    gui,
    monkeypatch,
    organization_factory,
    local_device_factory,
    alice,
):
    org = organization_factory()
    backend_user, backend_first_device = local_device_to_backend_user(alice, org)
    bootstrap_token = "123456"
    await running_backend.backend.organization.create(org.organization_id, bootstrap_token, None)
    await running_backend.backend.organization.bootstrap(
        org.organization_id,
        backend_user,
        backend_first_device,
        bootstrap_token,
        org.root_verify_key,
    )

    org_bs_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, org.organization_id, bootstrap_token
    )

    monkeypatch.setattr(
        "parsec.core.gui.main_window.get_text_input", lambda *args, **kwargs: (str(org_bs_addr))
    )

    # The org bootstrap window is usually opened using a sub-menu.
    # Sub-menus can be a bit challenging to open in tests so we cheat
    # using the keyboard shortcut Ctrl+O that has the same effect.
    await aqtbot.key_clicks(gui, "o", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()
    await aqtbot.wait_until(co_w.user_widget.isVisible)

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    assert co_w.user_widget.line_edit_org_name.text() == org.organization_id
    assert co_w.user_widget.line_edit_org_name.isReadOnly() is True
    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(co_w.device_widget.isVisible)

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    await aqtbot.key_clicks(co_w.device_widget.widget_password.line_edit_password, "nihilanth")
    await aqtbot.key_clicks(
        co_w.device_widget.widget_password.line_edit_password_check, "nihilanth"
    )
    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [("Error", "This bootstrap link was already used.")]

    await aqtbot.wait_until(_modal_shown)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_boostrap=True)
async def test_create_organization_custom_backend(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog
):
    # The org creation window is usually opened using a sub-menu.
    # Sub-menus can be a bit challenging to open in tests so we cheat
    # using the keyboard shortcut Ctrl+N that has the same effect.
    await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()
    co_w.config = co_w.config.evolve(
        preferred_org_creation_backend_addr=BackendAddr.from_url("parsec://localhost:1337")
    )

    assert co_w
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.user_widget.line_edit_org_name, "AnomalousMaterials")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    assert co_w.button_validate.isEnabled()

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(co_w.user_widget.isHidden)
    await aqtbot.wait_until(co_w.device_widget.isVisible)
    await aqtbot.wait_until(co_w.button_previous.isVisible)
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.device_widget.widget_password.line_edit_password, "nihilanth")
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(
        co_w.device_widget.widget_password.line_edit_password_check, "nihilanth"
    )
    assert co_w.button_validate.isEnabled()

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    # Should fail because it will use an invalid backend addr
    def _error_modal_shown():
        assert autoclose_dialog.dialogs == [("Error", "Cannot connect to the server.")]

    await aqtbot.wait_until(_error_modal_shown)

    autoclose_dialog.reset()

    # Let's go back and provide a custom address
    await aqtbot.mouse_click(co_w.button_previous, QtCore.Qt.LeftButton)

    await aqtbot.mouse_click(co_w.user_widget.check_use_custom_backend, QtCore.Qt.LeftButton)
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(co_w.user_widget.line_edit_backend_addr, running_backend.addr.to_url())
    assert co_w.button_validate.isEnabled()

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)
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
