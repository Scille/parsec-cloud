# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from PyQt5 import QtCore

from parsec.api.protocol import DeviceLabel
from parsec.core.pki import (
    PkiEnrollmentSubmitterInitialCtx,
    PkiEnrollementAccepterValidSubmittedCtx,
)
from parsec.core.types.backend_address import BackendPkiEnrollmentAddr
from parsec.core.gui.enrollment_widget import EnrollmentButton
from parsec.core.gui.login_widget import PendingEnrollment, EnrollmentPendingButton


@pytest.fixture
async def pki_org_addr(organization_factory, alice, running_backend):
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
    return BackendPkiEnrollmentAddr.build(running_backend.addr, org.organization_id)


@pytest.fixture
async def pki_request(core_config):
    context = await PkiEnrollmentSubmitterInitialCtx.new(pki_org_addr)
    # context = await context.submit(
    #     config_dir=core_config.config_dir,
    #     requested_device_label=DeviceLabel("device"),
    #     force=True,
    # )


@pytest.mark.gui
@pytest.mark.trio
async def test_list_enrollments(
    aqtbot, logged_gui, mocked_parsec_ext_smartcard, pki_org_addr, pki_request
):
    e_w = await logged_gui.test_switch_to_enrollment_widget()

    def _enrollment_shown():
        assert not e_w.label_empty_list.isVisible()
        assert e_w.main_layout.count() == 1
        item = e_w.main_layout.itemAt(0)
        assert item and item.widget() and isinstance(item.widget(), EnrollmentButton)
        w = item.widget()
        assert w.label_name.text() == "John Doe"
        assert w.label_email.text() == "john@example.com"
        assert w.label_issuer.text() == ""
        assert isinstance(w.pending, PkiEnrollementAccepterValidSubmittedCtx)
        assert w.button_accept.isVisible()
        assert w.button_reject.isVisible()

    await aqtbot.wait_until(_enrollment_shown)


@pytest.mark.xfail(reason="Enrollment list is hard coded right now")
@pytest.mark.gui
@pytest.mark.trio
async def test_list_enrollments_empty(aqtbot, logged_gui):
    e_w = await logged_gui.test_switch_to_enrollment_widget()

    def _enrollment_shown():
        assert e_w.label_empty_list.isVisible()
        assert e_w.main_layout.count() == 0
        assert e_w.label_empty_list.text() == "No enrollment to show."

    await aqtbot.wait_until(_enrollment_shown)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("should_succeed", [True, False])
async def test_enrollment_accept(aqtbot, logged_gui, snackbar_catcher, should_succeed):
    e_w = await logged_gui.test_switch_to_enrollment_widget()

    def _enrollment_shown():
        assert not e_w.label_empty_list.isVisible()
        assert e_w.main_layout.count() == 6

    await aqtbot.wait_until(_enrollment_shown)

    e_b = e_w.main_layout.itemAt(0).widget()
    assert e_b is not None

    if not should_succeed:

        def _raise_something(_):
            raise ValueError()

        e_w._fake_accept = _raise_something

    async with aqtbot.wait_signal(e_b.accept_clicked):
        aqtbot.mouse_click(e_b.button_accept, QtCore.Qt.LeftButton)

    if should_succeed:

        def _accept_succeeded():
            assert not e_w.label_empty_list.isVisible()
            assert e_w.main_layout.count() == 5
            user_list = [
                "john.zoidberg@planetexpress.com",
                "leela.turanga@planetexpress.com",
                "bender.rodriguez@planetexpress.com",
                "zapp.brannigan@doop.com",
                "philip.fry@planetexpress.com",
            ]
            for i in range(e_w.main_layout.count()):
                item = e_w.main_layout.itemAt(i)
                assert item and item.widget() and isinstance(item.widget(), EnrollmentButton)
                w = item.widget()
                assert w.enrollment_info.email in user_list
                user_list.remove(w.enrollment_info.email)
            assert not user_list
            assert snackbar_catcher.snackbars == ["This demand was successfully accepted."]

        await aqtbot.wait_until(_accept_succeeded)
    else:

        def _accept_failed():
            assert e_w.main_layout.count() == 6
            assert snackbar_catcher.snackbars == ["Could not accept this demand."]

        await aqtbot.wait_until(_accept_failed)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("should_succeed", [True, False])
async def test_enrollment_reject(aqtbot, logged_gui, snackbar_catcher, should_succeed):
    e_w = await logged_gui.test_switch_to_enrollment_widget()

    def _enrollment_shown():
        assert not e_w.label_empty_list.isVisible()
        assert e_w.main_layout.count() == 6

    await aqtbot.wait_until(_enrollment_shown)

    e_b = e_w.main_layout.itemAt(0).widget()
    assert e_b is not None

    if not should_succeed:

        def _raise_something(_):
            raise ValueError()

        e_w._fake_reject = _raise_something

    async with aqtbot.wait_signal(e_b.reject_clicked):
        aqtbot.mouse_click(e_b.button_reject, QtCore.Qt.LeftButton)

    if should_succeed:

        def _reject_succeeded():
            assert not e_w.label_empty_list.isVisible()
            assert e_w.main_layout.count() == 5
            user_list = [
                "john.zoidberg@planetexpress.com",
                "leela.turanga@planetexpress.com",
                "bender.rodriguez@planetexpress.com",
                "zapp.brannigan@doop.com",
                "philip.fry@planetexpress.com",
            ]
            for i in range(e_w.main_layout.count()):
                item = e_w.main_layout.itemAt(i)
                assert item and item.widget() and isinstance(item.widget(), EnrollmentButton)
                w = item.widget()
                assert w.enrollment_info.email in user_list
                user_list.remove(w.enrollment_info.email)
            assert not user_list
            assert snackbar_catcher.snackbars == ["This demand was successfully rejected."]

        await aqtbot.wait_until(_reject_succeeded)
    else:

        def _reject_failed():
            assert e_w.main_layout.count() == 6
            assert snackbar_catcher.snackbars == ["Could not reject this demand."]

        await aqtbot.wait_until(_reject_failed)


@pytest.fixture
def catch_enrollment_query_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.enrollment_query_widget.EnrollmentQueryWidget")


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("cert_is_valid", [True, False])
async def test_query_enrollment(
    aqtbot, gui, monkeypatch, catch_enrollment_query_widget, cert_is_valid
):
    monkeypatch.setattr(
        "parsec.core.gui.main_window.get_text_input", lambda *args, **kwargs: ("enrollment")
    )

    aqtbot.key_click(gui, "o", QtCore.Qt.ControlModifier, 200)

    eq_w = await catch_enrollment_query_widget()
    assert eq_w

    assert not eq_w.label_cert_error.isVisible()
    assert not eq_w.widget_user_info.isVisible()
    assert not eq_w.button_ask_to_join.isEnabled()
    assert eq_w.button_select_cert.isEnabled()

    if cert_is_valid:
        aqtbot.mouse_click(eq_w.button_select_cert, QtCore.Qt.LeftButton)
        assert not eq_w.label_cert_error.isVisible()
        assert eq_w.widget_user_info.isVisible()
        assert eq_w.button_ask_to_join.isEnabled()
        assert eq_w.button_select_cert.isEnabled()
        assert eq_w.line_edit_user_name.text() == "Hubert Farnsworth"
        assert eq_w.line_edit_user_email.text() == "hubert.farnsworth@planetexpress.com"
    else:
        aqtbot.mouse_click(eq_w.button_select_cert, QtCore.Qt.LeftButton)
        assert eq_w.label_cert_error.isVisible()
        assert not eq_w.widget_user_info.isVisible()
        assert not eq_w.button_ask_to_join.isEnabled()
        assert eq_w.button_select_cert.isEnabled()


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("enrollment_status", ["pending", "rejected", "accepted"])
async def test_list_pending_enrollments(aqtbot, gui_factory, monkeypatch, enrollment_status):
    def list_enrollments(_):
        return [
            PendingEnrollment("1", "Kiff Kroker", "Planet_Express", enrollment_status, "16/02/3022")
        ]

    monkeypatch.setattr(
        "parsec.core.gui.login_widget.LoginWidget.list_pending_enrollments", list_enrollments
    )

    gui = await gui_factory()
    lw = gui.test_get_login_widget()
    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w.accounts_widget.layout().count() == 2
    epw = accounts_w.accounts_widget.layout().itemAt(0).widget()
    assert isinstance(epw, EnrollmentPendingButton)
    assert epw.label_org.text() == "Planet_Express"
    assert epw.label_name.text() == "Kiff Kroker"
    if enrollment_status == "pending":
        assert not epw.button_action.isVisible()
        assert epw.label_status.text() == "TEXT_ENROLLMENT_STATUS_PENDING"
    elif enrollment_status == "accepted":
        assert epw.button_action.isVisible()
        assert epw.button_action.text() == "ACTION_ENROLLMENT_FINALIZE"
        assert epw.label_status.text() == "TEXT_ENROLLMENT_STATUS_ACCEPTED"
    elif enrollment_status == "rejected":
        assert epw.button_action.isVisible()
        assert epw.button_action.text() == "ACTION_ENROLLMENT_CLEAR"
        assert epw.label_status.text() == "TEXT_ENROLLMENT_STATUS_REJECTED"
