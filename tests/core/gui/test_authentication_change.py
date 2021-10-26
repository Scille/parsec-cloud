# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.gui.authentication_change_widget import (
    AuthenticationChangePage1Widget,
    AuthenticationChangePage2Widget,
)


@pytest.fixture
def catch_auth_change_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.authentication_change_widget.AuthenticationChangeWidget", timeout=2000
    )


def get_page_widget(auth_widget):
    return auth_widget.main_layout.itemAt(0).widget()


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_invalid_old_password(
    aqtbot, running_backend, logged_gui, catch_auth_change_widget, autoclose_dialog
):
    c_w = logged_gui.test_get_central_widget()
    assert c_w is not None

    # Trigger password change
    c_w.button_user.menu().actions()[0].trigger()

    pc_w = await catch_auth_change_widget()

    page1 = get_page_widget(pc_w)

    assert isinstance(page1, AuthenticationChangePage1Widget)
    assert not pc_w.button_validate.isEnabled()
    aqtbot.key_clicks(page1.widget, "0123456789")
    assert pc_w.button_validate.isEnabled()

    aqtbot.mouse_click(pc_w.button_validate, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [
        ("Error", "You did not provide the right password for this device.")
    ]
    assert pc_w.button_validate.isEnabled()
    assert isinstance(pc_w.main_layout.itemAt(0).widget, AuthenticationChangePage1Widget)


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_invalid_password_check(
    aqtbot, running_backend, logged_gui, catch_auth_change_widget, autoclose_dialog
):
    c_w = logged_gui.test_get_central_widget()

    assert c_w is not None

    # Trigger password change
    c_w.button_user.menu().actions()[0].trigger()

    pc_w = await catch_auth_change_widget()

    page1 = get_page_widget(pc_w)

    assert isinstance(page1, AuthenticationChangePage1Widget)
    assert not pc_w.button_validate.isEnabled()
    aqtbot.key_clicks(page1.widget, "P@ssw0rd")
    assert pc_w.button_validate.isEnabled()

    def _page2_shown():
        page2 = get_page_widget(pc_w)
        assert isinstance(page2, AuthenticationChangePage2Widget)

    await aqtbot.wait_until(_page2_shown)
    page2 = get_page_widget(pc_w)
    assert not pc_w.button_validate.isEnabled()
    aqtbot.key_clicks(
        page2.widget_new_auth.main_layout.itemAt(0).widget().line_edit_password, "P@ssw0rd2"
    )
    aqtbot.key_clicks(
        page2.widget_new_auth.main_layout.itemAt(0).widget().line_edit_password_check, "P@ssw0rd3"
    )
    assert not pc_w.button_validate.isEnabled()


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_success(
    aqtbot, running_backend, logged_gui, bob, catch_auth_change_widget, autoclose_dialog
):
    c_w = logged_gui.test_get_central_widget()

    assert c_w is not None

    c_w.button_user.menu().actions()[0].trigger()

    pc_w = await catch_auth_change_widget()

    page1 = get_page_widget(pc_w)

    assert isinstance(page1, AuthenticationChangePage1Widget)
    assert not pc_w.button_validate.isEnabled()
    aqtbot.key_clicks(page1.widget, "P@ssw0rd")
    assert pc_w.button_validate.isEnabled()

    aqtbot.mouse_click(pc_w.button_validate, QtCore.Qt.LeftButton)

    def _page2_shown():
        page2 = get_page_widget(pc_w)
        assert isinstance(page2, AuthenticationChangePage2Widget)

    await aqtbot.wait_until(_page2_shown)
    page2 = get_page_widget(pc_w)
    assert not pc_w.button_validate.isEnabled()
    aqtbot.key_clicks(
        page2.widget_new_auth.main_layout.itemAt(0).widget().line_edit_password, "P@ssw0rd2"
    )
    aqtbot.key_clicks(
        page2.widget_new_auth.main_layout.itemAt(0).widget().line_edit_password_check, "P@ssw0rd2"
    )
    assert pc_w.button_validate.isEnabled()
    aqtbot.mouse_click(pc_w.button_change, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [("", "The password has been successfully changed.")]
    autoclose_dialog.reset()

    # Retry to login...
    await logged_gui.test_logout_and_switch_to_login_widget()

    # ...with old password...
    await logged_gui.test_proceed_to_login(bob, "P@ssw0rd", error=True)
    assert autoclose_dialog.dialogs == [("Error", "The password is incorrect.")]
    autoclose_dialog.reset()

    # ...and new password
    await logged_gui.test_proceed_to_login(bob, "P@ssw0rd2")
