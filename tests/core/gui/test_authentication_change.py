# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from PyQt5 import QtCore

from parsec.core.gui.lang import translate


@pytest.fixture
def catch_auth_change_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.authentication_change_widget.AuthenticationChangeWidget"
    )


def get_page_widget(auth_widget):
    return auth_widget.main_layout.itemAt(0).widget()


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_invalid_old_password(
    aqtbot, running_backend, logged_gui, catch_auth_change_widget, autoclose_dialog, monkeypatch
):
    c_w = logged_gui.test_get_central_widget()
    assert c_w is not None

    # Trigger password change
    with monkeypatch.context() as m:
        m.setattr(
            "parsec.core.gui.authentication_change_widget.get_text_input",
            lambda *args, **kwargs: "INVALID_PASSWORD",
        )
        # 0 is "Org info"
        c_w.button_user.menu().actions()[1].trigger()

        await aqtbot.wait_until(
            lambda: autoclose_dialog.dialogs
            == [
                (
                    translate("TEXT_ERR_DIALOG_TITLE"),
                    translate("TEXT_LOGIN_ERROR_AUTHENTICATION_FAILED"),
                )
            ]
        )


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_invalid_password_check(
    aqtbot, running_backend, logged_gui, catch_auth_change_widget, autoclose_dialog, monkeypatch
):
    c_w = logged_gui.test_get_central_widget()

    assert c_w is not None

    # Trigger password change
    with monkeypatch.context() as m:
        m.setattr(
            "parsec.core.gui.authentication_change_widget.get_text_input",
            lambda *args, **kwargs: "P@ssw0rd",
        )
        c_w.button_user.menu().actions()[1].trigger()

        pc_w = await catch_auth_change_widget()

    assert not pc_w.button_validate.isEnabled()
    await aqtbot.key_clicks(
        pc_w.widget_auth.main_layout.itemAt(0).widget().line_edit_password, "P@ssw0rd2"
    )
    await aqtbot.key_clicks(
        pc_w.widget_auth.main_layout.itemAt(0).widget().line_edit_password_check, "P@ssw0rd3"
    )
    assert not pc_w.button_validate.isEnabled()


@pytest.mark.gui
@pytest.mark.trio
async def test_change_password_success(
    aqtbot,
    running_backend,
    logged_gui,
    bob,
    catch_auth_change_widget,
    autoclose_dialog,
    monkeypatch,
):
    c_w = logged_gui.test_get_central_widget()

    assert c_w is not None

    with monkeypatch.context() as m:
        m.setattr(
            "parsec.core.gui.authentication_change_widget.get_text_input",
            lambda *args, **kwargs: "P@ssw0rd",
        )
        c_w.button_user.menu().actions()[1].trigger()

        pc_w = await catch_auth_change_widget()

    assert not pc_w.button_validate.isEnabled()
    await aqtbot.key_clicks(
        pc_w.widget_auth.main_layout.itemAt(0).widget().line_edit_password, "P@ssw0rd2"
    )
    await aqtbot.key_clicks(
        pc_w.widget_auth.main_layout.itemAt(0).widget().line_edit_password_check, "P@ssw0rd2"
    )

    await aqtbot.wait_until(pc_w.button_validate.isEnabled)
    aqtbot.mouse_click(pc_w.button_validate, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(
        lambda: autoclose_dialog.dialogs == [("", translate("TEXT_AUTH_CHANGE_SUCCESS"))]
    )
    autoclose_dialog.reset()

    # Retry to login...
    await logged_gui.test_logout_and_switch_to_login_widget()

    # ...with old password...
    await logged_gui.test_proceed_to_login(bob, "P@ssw0rd", error=True)
    assert autoclose_dialog.dialogs == [("Error", "The password is incorrect.")]
    autoclose_dialog.reset()

    # ...and new password
    await logged_gui.test_proceed_to_login(bob, "P@ssw0rd2")
