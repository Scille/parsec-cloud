# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio
from parsec._parsec import DateTime
from PyQt5 import QtCore

from parsec.api.protocol import OrganizationID, HumanHandle, DeviceLabel
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.invite import bootstrap_organization
from parsec.core.gui.create_org_widget import CreateOrgUserInfoWidget
from parsec.core.gui.authentication_choice_widget import AuthenticationChoiceWidget
from parsec.core.gui.lang import translate

from tests.common import customize_fixtures, freeze_time, local_device_to_backend_user


@pytest.fixture
def catch_create_org_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.create_org_widget.CreateOrgWidget")


@pytest.fixture
async def organization_bootstrap_addr(running_backend):
    org_id = OrganizationID("AnomalousMaterials")
    org_token = "123456"
    await running_backend.backend.organization.create(id=org_id, bootstrap_token=org_token)
    return BackendOrganizationBootstrapAddr.build(running_backend.addr, org_id, org_token)


async def _do_creation_process(aqtbot, co_w):
    def _user_widget_ready():
        assert isinstance(co_w.current_widget, CreateOrgUserInfoWidget)

    await aqtbot.wait_until(_user_widget_ready)

    # Adding a few spaces to the name
    await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "  Gordon     Freeman   ")
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
    )
    await aqtbot.key_clicks(co_w.current_widget.line_edit_org_name, "AnomalousMaterials")
    await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")
    assert not co_w.button_validate.isEnabled()

    aqtbot.mouse_click(co_w.current_widget.check_accept_contract, QtCore.Qt.LeftButton)

    def _user_widget_button_validate_ready():
        assert co_w.button_validate.isEnabled()

    await aqtbot.wait_until(_user_widget_button_validate_ready)

    async with aqtbot.wait_signal(co_w.req_success):
        aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _device_widget_ready():
        assert isinstance(co_w.current_widget, AuthenticationChoiceWidget)

    await aqtbot.wait_until(_device_widget_ready)

    assert co_w.current_widget.main_layout.itemAt(
        0
    ).widget().label_password_warning.text() == translate("TEXT_PASSWORD_WARNING")
    await aqtbot.key_clicks(
        co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password, "nihilanth"
    )
    assert not co_w.button_validate.isEnabled()

    await aqtbot.key_clicks(
        co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password_check, "nihilanth"
    )

    def _device_widget_button_validate_ready():
        assert co_w.button_validate.isEnabled()

    await aqtbot.wait_until(_device_widget_button_validate_ready)
    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True)
async def test_create_organization(
    monkeypatch, gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog
):
    # Disable the sync monitor to avoid concurrent sync right when the claim finish
    monkeypatch.setattr(
        "parsec.core.sync_monitor.freeze_sync_monitor_mockpoint", trio.sleep_forever
    )

    # The org creation window is usually opened using a sub-menu.
    # Sub-menus can be a bit challenging to open in tests so we cheat
    # using the keyboard shortcut Ctrl+N that has the same effect.
    aqtbot.key_click(gui, "n", QtCore.Qt.ControlModifier, 200)

    co_w = await catch_create_org_widget()

    assert co_w
    await _do_creation_process(aqtbot, co_w)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [
            (
                translate("TEXT_BOOTSTRAP_ORG_SUCCESS_TITLE"),
                translate("TEXT_BOOTSTRAP_ORG_SUCCESS_organization").format(
                    organization="AnomalousMaterials"
                ),
            )
        ]

    await aqtbot.wait_until(_modal_shown)

    def _logged_in():
        c_w = gui.test_get_central_widget()
        assert c_w
        assert c_w.button_user.text() == "AnomalousMaterials\nGordon Freeman"

        # Claimed user should start with a non-speculative user manifest
        um = c_w.core.user_fs.get_user_manifest()
        assert not um.speculative

    await aqtbot.wait_until(_logged_in)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True)
async def test_create_organization_offline(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog
):
    with running_backend.offline():
        aqtbot.key_click(gui, "n", QtCore.Qt.ControlModifier, 200)

        co_w = await catch_create_org_widget()
        assert co_w

        def _user_widget_ready():
            assert isinstance(co_w.current_widget, CreateOrgUserInfoWidget)

        await aqtbot.wait_until(_user_widget_ready)

        # Adding a few spaces to the name
        await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "Gordon Freeman")
        await aqtbot.key_clicks(
            co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
        )
        await aqtbot.key_clicks(co_w.current_widget.line_edit_org_name, "AnomalousMaterials")
        await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")
        assert not co_w.button_validate.isEnabled()

        aqtbot.mouse_click(co_w.current_widget.check_accept_contract, QtCore.Qt.LeftButton)

        def _user_widget_button_validate_ready():
            assert co_w.button_validate.isEnabled()

        await aqtbot.wait_until(_user_widget_button_validate_ready)

        async with aqtbot.wait_signal(co_w.req_error):
            aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

        def _modal_shown():
            assert autoclose_dialog.dialogs == [("Error", "Cannot connect to the server.")]

        await aqtbot.wait_until(_modal_shown)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True)
async def test_create_organization_same_name(
    gui,
    aqtbot,
    running_backend,
    catch_create_org_widget,
    autoclose_dialog,
    organization_bootstrap_addr,
):
    # Create an org
    human_handle = HumanHandle(email="zack@example.com", label="Zack")

    await bootstrap_organization(
        organization_bootstrap_addr, human_handle=human_handle, device_label=DeviceLabel("PC1")
    )

    # Now create an org with the same name
    aqtbot.key_click(gui, "n", QtCore.Qt.ControlModifier, 200)

    co_w = await catch_create_org_widget()
    assert co_w

    def _user_widget_ready():
        assert isinstance(co_w.current_widget, CreateOrgUserInfoWidget)

    await aqtbot.wait_until(_user_widget_ready)

    # Adding a few spaces to the name
    await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
    )
    await aqtbot.key_clicks(co_w.current_widget.line_edit_org_name, "AnomalousMaterials")
    await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")
    assert not co_w.button_validate.isEnabled()

    aqtbot.mouse_click(co_w.current_widget.check_accept_contract, QtCore.Qt.LeftButton)

    def _user_widget_button_validate_ready():
        assert co_w.button_validate.isEnabled()

    await aqtbot.wait_until(_user_widget_button_validate_ready)

    async with aqtbot.wait_signal(co_w.req_error):
        aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [("Error", "This organization name is already used.")]

    await aqtbot.wait_until(_modal_shown)


@pytest.mark.skip("No previous button in new process")
@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True)
async def test_create_organization_previous_clicked(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog
):
    aqtbot.key_click(gui, "n", QtCore.Qt.ControlModifier, 200)

    co_w = await catch_create_org_widget()

    assert co_w
    await aqtbot.wait_until(co_w.user_widget.isVisible)

    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(co_w.user_widget.line_edit_user_email, "gordon.freeman@blackmesa.com")
    await aqtbot.key_clicks(co_w.user_widget.line_edit_org_name, "AnomalousMaterials")
    aqtbot.mouse_click(co_w.user_widget.check_accept_contract, QtCore.Qt.LeftButton)
    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(co_w.device_widget.isVisible)

    await aqtbot.key_clicks(co_w.device_widget.line_edit_device, "HEV")
    await aqtbot.key_clicks(
        co_w.device_widget.widget_auth.main_layout.itemAt(0).widget().line_edit_password,
        "nihilanth",
    )
    await aqtbot.key_clicks(
        co_w.device_widget.widget_auth.main_layout.itemAt(0).widget().line_edit_password_check,
        "nihilanth",
    )

    aqtbot.mouse_click(co_w.button_previous, QtCore.Qt.LeftButton)

    def _previous_page_ready():
        assert co_w.user_widget.isVisible()
        assert not co_w.device_widget.isVisible()
        assert not co_w.button_previous.isVisible()
        assert co_w.button_validate.isEnabled()

        assert co_w.user_widget.line_edit_user_full_name.text() == "Gordon Freeman"
        assert co_w.user_widget.line_edit_user_email.text() == "gordon.freeman@blackmesa.com"
        assert co_w.user_widget.line_edit_org_name.text() == "AnomalousMaterials"
        assert co_w.user_widget.check_accept_contract.isChecked()

        assert co_w.button_validate.isEnabled()

    await aqtbot.wait_until(_previous_page_ready)

    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _next_page_ready():
        assert co_w.device_widget.isVisible()
        assert co_w.device_widget.line_edit_device.text() == "HEV"
        assert (
            co_w.device_widget.widget_auth.main_layout.itemAt(0).widget().line_edit_password.text()
            == "nihilanth"
        )
        assert (
            co_w.device_widget.widget_auth.main_layout.itemAt(0)
            .widget()
            .line_edit_password_check.text()
            == "nihilanth"
        )

    await aqtbot.wait_until(_next_page_ready)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True)
async def test_create_organization_bootstrap_only(
    aqtbot,
    running_backend,
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

    await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
    )
    await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")

    def _user_widget_button_validate_ready():
        assert co_w.button_validate.isEnabled()

    assert co_w.current_widget.line_edit_org_name.text() == "AnomalousMaterials"
    assert co_w.current_widget.line_edit_org_name.isReadOnly() is True
    assert not co_w.current_widget.radio_use_custom.isChecked()
    assert co_w.current_widget.radio_use_commercial.isChecked()
    assert not co_w.current_widget.radio_use_custom.isEnabled()
    aqtbot.mouse_click(co_w.current_widget.check_accept_contract, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(_user_widget_button_validate_ready)

    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _device_widget_ready():
        assert isinstance(co_w.current_widget, AuthenticationChoiceWidget)

    await aqtbot.wait_until(_device_widget_ready)

    await aqtbot.key_clicks(
        co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password, "nihilanth"
    )
    await aqtbot.key_clicks(
        co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password_check, "nihilanth"
    )

    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [
            (
                translate("TEXT_BOOTSTRAP_ORG_SUCCESS_TITLE"),
                translate("TEXT_BOOTSTRAP_ORG_SUCCESS_organization").format(
                    organization="AnomalousMaterials"
                ),
            )
        ]

    await aqtbot.wait_until(_modal_shown)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True)
@customize_fixtures(fake_preferred_org_creation_backend_addr=True)
async def test_create_organization_bootstrap_only_custom_server(
    aqtbot,
    running_backend,
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

    await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
    )
    await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")

    def _user_widget_ready():
        assert co_w.current_widget.line_edit_org_name.text() == "AnomalousMaterials"
        assert co_w.current_widget.line_edit_org_name.isReadOnly() is True
        assert co_w.current_widget.radio_use_custom.isChecked()
        assert not co_w.current_widget.radio_use_commercial.isChecked()
        assert not co_w.current_widget.radio_use_commercial.isEnabled()
        assert len(co_w.current_widget.line_edit_backend_addr.text())
        assert not co_w.current_widget.line_edit_backend_addr.isEnabled()
        assert co_w.button_validate.isEnabled()

    await aqtbot.wait_until(_user_widget_ready)

    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _device_widget_ready():
        assert isinstance(co_w.current_widget, AuthenticationChoiceWidget)

    await aqtbot.wait_until(_device_widget_ready)

    await aqtbot.key_clicks(
        co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password, "nihilanth"
    )
    await aqtbot.key_clicks(
        co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password_check, "nihilanth"
    )

    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [
            (
                translate("TEXT_BOOTSTRAP_ORG_SUCCESS_TITLE"),
                translate("TEXT_BOOTSTRAP_ORG_SUCCESS_organization").format(
                    organization="AnomalousMaterials"
                ),
            )
        ]

    await aqtbot.wait_until(_modal_shown)


@pytest.mark.gui
@pytest.mark.trio
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
    await running_backend.backend.organization.create(
        id=org.organization_id,
        bootstrap_token=bootstrap_token,
    )
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
        "parsec.core.gui.main_window.get_text_input", lambda *args, **kwargs: (org_bs_addr.to_url())
    )

    # The org bootstrap window is usually opened using a sub-menu.
    # Sub-menus can be a bit challenging to open in tests so we cheat
    # using the keyboard shortcut Ctrl+O that has the same effect.
    aqtbot.key_click(gui, "o", QtCore.Qt.ControlModifier, 200)

    co_w = await catch_create_org_widget()
    await aqtbot.wait_until(lambda: isinstance(co_w.current_widget, CreateOrgUserInfoWidget))

    await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
    )
    await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")

    def _user_widget_ready():
        assert co_w.current_widget.line_edit_org_name.text() == org.organization_id.str
        assert co_w.current_widget.line_edit_org_name.isReadOnly() is True

    await aqtbot.wait_until(_user_widget_ready)

    aqtbot.mouse_click(co_w.current_widget.check_accept_contract, QtCore.Qt.LeftButton)
    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [("Error", "This bootstrap link was already used.")]

    await aqtbot.wait_until(_modal_shown)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True)
@customize_fixtures(fake_preferred_org_creation_backend_addr=True)
async def test_create_organization_custom_backend(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog, unused_tcp_port
):
    # The org creation window is usually opened using a sub-menu.
    # Sub-menus can be a bit challenging to open in tests so we cheat
    # using the keyboard shortcut Ctrl+N that has the same effect.
    aqtbot.key_click(gui, "n", QtCore.Qt.ControlModifier, 200)

    co_w = await catch_create_org_widget()

    assert co_w

    def _user_widget_ready():
        assert isinstance(co_w.current_widget, CreateOrgUserInfoWidget)

    await aqtbot.wait_until(_user_widget_ready)

    await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
    )
    await aqtbot.key_clicks(co_w.current_widget.line_edit_org_name, "AnomalousMaterials")
    await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")

    # Mouse click doesn't work on Windows in this case for some reason so we check the radio programatically
    co_w.current_widget.radio_use_custom.setChecked(True)

    def _use_custom_checked():
        assert co_w.current_widget.radio_use_custom.isChecked()
        assert not co_w.current_widget.radio_use_commercial.isChecked()

    await aqtbot.wait_until(_use_custom_checked)

    def _user_widget_button_validate_ready(state, addr):
        assert co_w.current_widget.line_edit_backend_addr.text() == addr
        assert co_w.current_widget._are_inputs_valid() is state
        assert co_w.button_validate.isEnabled() is state

    # Space at the end, should be fine
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_backend_addr, running_backend.addr.to_url() + " "
    )
    assert co_w.current_widget.line_edit_backend_addr.is_input_valid()
    await aqtbot.wait_until(
        lambda: _user_widget_button_validate_ready(True, running_backend.addr.to_url() + " ")
    )

    # Empty, should not be valid
    co_w.current_widget.line_edit_backend_addr.setText("")
    assert not co_w.current_widget.line_edit_backend_addr.is_input_valid()
    await aqtbot.wait_until(lambda: _user_widget_button_validate_ready(False, ""))

    # Space at the beginning, should be fine
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_backend_addr, " " + running_backend.addr.to_url()
    )
    assert co_w.current_widget.line_edit_backend_addr.is_input_valid()
    await aqtbot.wait_until(
        lambda: _user_widget_button_validate_ready(True, " " + running_backend.addr.to_url())
    )

    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _device_widget_ready():
        assert isinstance(co_w.current_widget, AuthenticationChoiceWidget)

    await aqtbot.wait_until(_device_widget_ready)

    await aqtbot.key_clicks(
        co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password, "nihilanth"
    )
    await aqtbot.key_clicks(
        co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password_check, "nihilanth"
    )

    def _device_widget_button_validate_ready():
        assert co_w.button_validate.isEnabled()

    await aqtbot.wait_until(_device_widget_button_validate_ready)

    aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _modal_shown():
        assert autoclose_dialog.dialogs == [
            (
                translate("TEXT_BOOTSTRAP_ORG_SUCCESS_TITLE"),
                translate("TEXT_BOOTSTRAP_ORG_SUCCESS_organization").format(
                    organization="AnomalousMaterials"
                ),
            )
        ]

    await aqtbot.wait_until(_modal_shown)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True)
async def test_create_organization_wrong_timestamp(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog, monkeypatch
):
    aqtbot.key_click(gui, "n", QtCore.Qt.ControlModifier, 200)
    co_w = await catch_create_org_widget()
    assert co_w

    # Patch the DateTime.now() just for the organization creation in the core so we have
    # a different date than the server
    def _timestamp(device):
        with freeze_time("2000-01-01"):
            return DateTime.now()

    monkeypatch.setattr("parsec.core.types.LocalDevice.timestamp", _timestamp)

    await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "Gordon Freeman")
    await aqtbot.key_clicks(
        co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
    )
    await aqtbot.key_clicks(co_w.current_widget.line_edit_org_name, "AnomalousMaterials")
    await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")
    assert not co_w.button_validate.isEnabled()

    aqtbot.mouse_click(co_w.current_widget.check_accept_contract, QtCore.Qt.LeftButton)

    def _user_widget_button_validate_ready():
        assert co_w.button_validate.isEnabled()

    await aqtbot.wait_until(_user_widget_button_validate_ready)

    async with aqtbot.wait_signal(co_w.req_error):
        aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    def _error_shown():
        assert autoclose_dialog.dialogs == [("Error", translate("TEXT_BACKEND_STATE_DESYNC"))]

    await aqtbot.wait_until(_error_shown)


@pytest.mark.gui
@pytest.mark.trio
async def test_create_organization_with_bootstrap_token(
    gui, aqtbot, running_backend, catch_create_org_widget, autoclose_dialog
):
    # Firt create the organization
    bootstrap_token = "T0k3n"
    organization_id = OrganizationID("AnomalousMaterials")
    await running_backend.backend.organization.create(
        id=organization_id, bootstrap_token=bootstrap_token
    )

    good_bootstrap_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, organization_id, bootstrap_token
    )
    bad_bootstrap_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, organization_id, "B@dT0k3n"
    )

    # Now try to use the wrong bootstrap link
    for bootstrap_addr in (bad_bootstrap_addr, good_bootstrap_addr):
        autoclose_dialog.reset()

        gui.add_instance(bootstrap_addr.to_url())

        co_w = await catch_create_org_widget()

        assert co_w

        def _user_widget_ready():
            assert isinstance(co_w.current_widget, CreateOrgUserInfoWidget)
            # Organization name and server address should be already provided
            assert co_w.current_widget.line_edit_org_name.isReadOnly()
            assert not co_w.current_widget.radio_use_custom.isEnabled()

        await aqtbot.wait_until(_user_widget_ready)

        await aqtbot.key_clicks(co_w.current_widget.line_edit_user_full_name, "Gordon Freeman")
        await aqtbot.key_clicks(
            co_w.current_widget.line_edit_user_email, "gordon.freeman@blackmesa.com"
        )
        await aqtbot.key_clicks(co_w.current_widget.line_edit_device, "HEV")
        aqtbot.mouse_click(co_w.current_widget.check_accept_contract, QtCore.Qt.LeftButton)

        def _user_widget_button_validate_ready():
            assert co_w.button_validate.isEnabled()

        await aqtbot.wait_until(_user_widget_button_validate_ready)
        aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

        if bootstrap_addr is bad_bootstrap_addr:

            def _error_modal_shown():
                assert autoclose_dialog.dialogs == [("Error", "This bootstrap link is invalid.")]

            await aqtbot.wait_until(_error_modal_shown)

        else:

            def _device_widget_ready():
                assert isinstance(co_w.current_widget, AuthenticationChoiceWidget)

            await aqtbot.wait_until(_device_widget_ready)

            await aqtbot.key_clicks(
                co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password, "nihilanth"
            )
            await aqtbot.key_clicks(
                co_w.current_widget.main_layout.itemAt(0).widget().line_edit_password_check,
                "nihilanth",
            )

            def _device_widget_button_validate_ready():
                assert co_w.button_validate.isEnabled()

            await aqtbot.wait_until(_device_widget_button_validate_ready)

            aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

            def _modal_shown():
                assert autoclose_dialog.dialogs == [
                    (
                        translate("TEXT_BOOTSTRAP_ORG_SUCCESS_TITLE"),
                        translate("TEXT_BOOTSTRAP_ORG_SUCCESS_organization").format(
                            organization=organization_id.str
                        ),
                    )
                ]

            await aqtbot.wait_until(_modal_shown)


@pytest.fixture
def catch_text_input_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.custom_dialogs.TextInputWidget")


@pytest.mark.gui
@pytest.mark.trio
async def test_join_org_addr_input(
    gui, aqtbot, running_backend, autoclose_dialog, catch_text_input_widget
):
    # The org bootstrap window is usually opened using a sub-menu.
    # Sub-menus can be a bit challenging to open in tests so we cheat
    # using the keyboard shortcut Ctrl+O that has the same effect.
    aqtbot.key_click(gui, "o", QtCore.Qt.ControlModifier, 200)

    ti_w = await catch_text_input_widget()

    assert ti_w

    def _check_button_state(expected_state):
        assert ti_w.button_ok.isEnabled() is expected_state

    await aqtbot.key_clicks(ti_w.line_edit_text, "invalid backend addr")
    assert not ti_w.line_edit_text.is_input_valid()
    await aqtbot.wait_until(lambda: _check_button_state(False))

    ti_w.line_edit_text.setText("")
    assert not ti_w.line_edit_text.is_input_valid()
    await aqtbot.wait_until(lambda: _check_button_state(False))

    await aqtbot.key_clicks(
        ti_w.line_edit_text,
        "   parsec://example.com/org?action=claim_user&token=3a50b191122b480ebb113b10216ef343   ",
    )
    assert ti_w.line_edit_text.is_input_valid()
    await aqtbot.wait_until(lambda: _check_button_state(True))
