# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from PyQt5 import QtCore

from parsec.core.types.backend_address import BackendPkiEnrollmentAddr

from parsec.core.local_device import save_device_with_password_in_config
from parsec.core.gui.enrollment_widget import EnrollmentButton
from parsec.core.gui.login_widget import EnrollmentPendingButton, AccountButton
from parsec.core.gui.central_widget import CentralWidget
from parsec.core.gui.lang import translate


@pytest.fixture
def catch_enrollment_query_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.enrollment_query_widget.EnrollmentQueryWidget")


@pytest.fixture
def catch_enrollment_accept_check_infos_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.enrollment_widget.AcceptCheckInfoWidget")


@pytest.mark.gui
@pytest.mark.trio
async def test_full_enrollment(
    aqtbot,
    mocked_parsec_ext_smartcard,
    core_config,
    gui,
    alice,
    running_backend,
    catch_enrollment_accept_check_infos_widget,
    snackbar_catcher,
    monkeypatch,
    autoclose_dialog,
    catch_enrollment_query_widget,
):
    # Add trust root to the configuration
    gui.config = gui.config.evolve(
        pki_extra_trust_roots={mocked_parsec_ext_smartcard.default_trust_root_path}
    )
    config_dir = gui.config.config_dir
    alice_password = "S3cr3t"
    save_device_with_password_in_config(config_dir, alice, alice_password)

    pki_org_addr = BackendPkiEnrollmentAddr.build(
        alice.organization_addr.get_backend_addr(), alice.organization_addr.organization_id
    )
    monkeypatch.setattr(
        "parsec.core.gui.main_window.get_text_input",
        lambda *args, **kwargs: (pki_org_addr.to_url()),
    )

    # Open the PKI enrollment dialog
    aqtbot.key_click(gui, "o", QtCore.Qt.ControlModifier, 200)

    eq_w = await catch_enrollment_query_widget()
    assert eq_w

    aqtbot.mouse_click(eq_w.button_select_cert, QtCore.Qt.LeftButton)

    def _cert_loaded():
        assert not eq_w.label_cert_error.isVisible()
        assert eq_w.widget_user_info.isVisible()
        assert eq_w.button_ask_to_join.isEnabled()
        assert eq_w.button_select_cert.isEnabled()
        assert eq_w.line_edit_user_name.text() == "John Doe"
        assert eq_w.line_edit_user_email.text() == "john@example.com"
        assert len(eq_w.line_edit_device.text())

    await aqtbot.wait_until(_cert_loaded)

    aqtbot.mouse_click(eq_w.button_ask_to_join, QtCore.Qt.LeftButton)

    def _request_made():
        assert autoclose_dialog.dialogs == [("", "Your request has been sent")]

    await aqtbot.wait_until(_request_made)

    # Check if the enrollment appears among the list of devices with a PENDING status

    lw = gui.test_get_login_widget()

    def _devices_listed():
        assert lw.widget.layout().count() == 1
        assert lw.widget.layout().itemAt(0) is not None
        assert lw.widget.layout().itemAt(0).widget().accounts_widget.layout().count() == 3
        pending = lw.widget.layout().itemAt(0).widget().accounts_widget.layout().itemAt(0).widget()
        assert isinstance(pending, EnrollmentPendingButton)
        assert pending.label_org.text() == alice.organization_addr.organization_id.str
        assert pending.label_name.text() == "John Doe"
        assert pending.label_status.text() == "Pending request"
        assert pending.button_action.isVisible() is False

    await aqtbot.wait_until(_devices_listed)

    # Log in with an admin device

    cw = await gui.test_switch_to_logged_in(alice, alice_password)
    cw.button_user.text() == "Alice"

    assert cw.menu.button_enrollment.isVisible() is True

    # Check if the enrollment request we made appears

    e_w = await gui.test_switch_to_enrollment_widget()

    def _enrollments_listed():
        assert e_w.main_layout.count() == 1
        assert e_w.main_layout.itemAt(0) is not None
        assert isinstance(e_w.main_layout.itemAt(0).widget(), EnrollmentButton)

    await aqtbot.wait_until(_enrollments_listed)

    button = e_w.main_layout.itemAt(0).widget()
    assert isinstance(button, EnrollmentButton)
    assert button.button_accept.isEnabled()
    assert button.button_reject.isEnabled()
    assert button.widget_cert_infos.isVisible()
    assert not button.widget_cert_error.isVisible()
    assert button.label_name.text() == "John Doe"
    assert button.label_email.text() == "john@example.com"
    assert button.label_issuer.text() == "My CA"
    assert button.label_cert_validity.text() == "✔ Valid certificate"

    # Accept the enrollment request

    aqtbot.mouse_click(button.button_accept, QtCore.Qt.LeftButton)

    acc_w = await catch_enrollment_accept_check_infos_widget()
    assert acc_w

    assert acc_w.line_edit_user_full_name.text() == "John Doe"
    assert acc_w.line_edit_user_email.text() == "john@example.com"
    assert len(acc_w.line_edit_device.text()) > 0

    assert acc_w.combo_profile.currentData() is None
    assert acc_w.button_create_user.isEnabled() is False

    acc_w.combo_profile.setCurrentIndex(2)

    def _button_create_user_enabled():
        acc_w.button_create_user.isEnabled() is True

    await aqtbot.wait_until(_button_create_user_enabled)

    aqtbot.mouse_click(acc_w.button_create_user, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(
        lambda: snackbar_catcher.snackbars
        == [("INFO", translate("TEXT_ENROLLMENT_ACCEPT_SUCCESS"))]
    )
    snackbar_catcher.reset()

    monkeypatch.setattr(
        "parsec.core.gui.users_widget.ensure_string_size", lambda s, size, font: (s[:12] + "...")
    )

    # Check if the new user appears

    u_w = await gui.test_switch_to_users_widget()
    assert u_w.layout_users.count() == 4

    a_w = u_w.layout_users.itemAt(0).widget()
    assert a_w.label_username.text() == "Adamy McAdam..."
    assert a_w.label_email.text() == "adam@example..."

    j_w = u_w.layout_users.itemAt(3).widget()
    assert j_w.label_username.text() == "John Doe..."
    assert j_w.label_email.text() == "john@example..."

    # Logout and check the status of the enrollment request

    lw = await gui.test_logout_and_switch_to_login_widget()

    def _devices_listed():
        assert lw.widget.layout().count() == 1
        assert lw.widget.layout().itemAt(0) is not None
        assert lw.widget.layout().itemAt(0).widget().accounts_widget.layout().count() == 3
        pending = lw.widget.layout().itemAt(0).widget().accounts_widget.layout().itemAt(0).widget()
        assert isinstance(pending, EnrollmentPendingButton)
        assert pending.label_org.text() == alice.organization_addr.organization_id.str
        assert pending.label_name.text() == "John Doe"
        assert pending.label_status.text() == "Request approved"
        assert pending.button_action.isVisible() is True

    await aqtbot.wait_until(_devices_listed)

    # Finalize the enrollment

    pending = lw.widget.layout().itemAt(0).widget().accounts_widget.layout().itemAt(0).widget()
    aqtbot.mouse_click(pending.button_action, QtCore.Qt.LeftButton)

    # Check if the new device appears

    def _new_device_listed():
        org_name = alice.organization_addr.organization_id.str
        devices = ["John Doe", "Alicey McAliceFace"]

        assert lw.widget.layout().itemAt(0) is not None
        assert lw.widget.layout().itemAt(0).widget().accounts_widget.layout().count() == 3
        for i in [0, 1]:
            device = (
                lw.widget.layout().itemAt(0).widget().accounts_widget.layout().itemAt(i).widget()
            )
            assert isinstance(device, AccountButton)
            assert device.label_name.text() in devices
            devices.remove(device.label_name.text())
            assert device.label_organization.text() == org_name

    await aqtbot.wait_until(_new_device_listed)

    # Log in with the new device

    new_device = None
    for i in [0, 1]:
        device_button = (
            lw.widget.layout().itemAt(0).widget().accounts_widget.layout().itemAt(i).widget()
        )
        if device_button.label_name.text() == "John Doe":
            new_device = device_button.device
            break

    cw = await gui.test_proceed_to_login(new_device)

    assert isinstance(cw, CentralWidget)
    assert cw.button_user.text() == f"{alice.organization_addr.organization_id.str}\nJohn Doe"
