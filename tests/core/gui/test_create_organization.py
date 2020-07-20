# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from PyQt5 import QtCore

from parsec.core.gui.create_org_widget import CreateOrgChoseActionWidget, CreateOrgCustomWidget


@pytest.fixture
def catch_create_org_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.create_org_widget.CreateOrgWidget")


@pytest.fixture
def catch_bootstrap_org_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.bootstrap_organization_widget.BootstrapOrganizationWidget"
    )


@pytest.mark.gui
@pytest.mark.trio
async def test_create_organization(
    gui,
    aqtbot,
    running_backend,
    qt_thread_gateway,
    catch_create_org_widget,
    autoclose_dialog,
    catch_bootstrap_org_widget,
):

    await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()
    assert co_w
    choice_w = co_w.current_widget
    assert isinstance(choice_w, CreateOrgChoseActionWidget)

    await aqtbot.mouse_click(choice_w.radio_create_custom.label, QtCore.Qt.LeftButton)
    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    custom_w = co_w.current_widget

    assert isinstance(custom_w, CreateOrgCustomWidget)

    await aqtbot.key_clicks(custom_w.line_edit_org_name, "MyOrg")
    await aqtbot.key_clicks(custom_w.line_edit_server_url, str(running_backend.addr))
    await aqtbot.key_clicks(
        custom_w.line_edit_admin_token, running_backend.backend.config.administration_token
    )

    async with aqtbot.wait_signal(co_w.create_custom_success):
        await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)


@pytest.mark.gui
@pytest.mark.trio
async def test_create_organization_wrong_token(
    gui, aqtbot, running_backend, qt_thread_gateway, catch_create_org_widget, autoclose_dialog
):

    await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()
    assert co_w
    choice_w = co_w.current_widget
    assert isinstance(choice_w, CreateOrgChoseActionWidget)

    await aqtbot.mouse_click(choice_w.radio_create_custom.label, QtCore.Qt.LeftButton)
    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    custom_w = co_w.current_widget

    assert isinstance(custom_w, CreateOrgCustomWidget)

    await aqtbot.key_clicks(custom_w.line_edit_org_name, "MyOrg")
    await aqtbot.key_clicks(custom_w.line_edit_server_url, str(running_backend.addr))
    await aqtbot.key_clicks(custom_w.line_edit_admin_token, "WrongToken")

    async with aqtbot.wait_signal(co_w.create_custom_error):
        await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs[1] == ("Error", "TEXT_ORG_WIZARD_INVALID_ADMIN_TOKEN")


@pytest.mark.gui
@pytest.mark.trio
async def test_create_organization_invalid_org_id(
    gui, aqtbot, running_backend, qt_thread_gateway, catch_create_org_widget, autoclose_dialog
):

    await aqtbot.key_clicks(gui, "n", QtCore.Qt.ControlModifier)

    co_w = await catch_create_org_widget()
    assert co_w
    choice_w = co_w.current_widget
    assert isinstance(choice_w, CreateOrgChoseActionWidget)

    await aqtbot.mouse_click(choice_w.radio_create_custom.label, QtCore.Qt.LeftButton)
    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    custom_w = co_w.current_widget

    assert isinstance(custom_w, CreateOrgCustomWidget)

    await aqtbot.key_clicks(custom_w.line_edit_org_name, "My Invalid Org Id")
    await aqtbot.key_clicks(custom_w.line_edit_server_url, str(running_backend.addr))
    await aqtbot.key_clicks(
        custom_w.line_edit_admin_token, running_backend.backend.config.administration_token
    )

    await aqtbot.mouse_click(co_w.button_validate, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs[1] == ("Error", "The organization name is invalid.")
