# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from PyQt5 import QtCore

from parsec.api.protocol import DeviceLabel
from parsec.core.pki import PkiEnrollmentSubmitterInitialCtx
from parsec.core.types.backend_address import BackendPkiEnrollmentAddr

# from parsec.core.gui.parsec_application import ParsecApp
# from parsec.core.gui.lang import switch_language
# from parsec.core.gui.enrollment_widget import EnrollmentButton
# from parsec.core.gui.login_widget import (
#     EnrollmentPendingButton,
#     AccountButton,
#     LoginPasswordInputWidget,
# )
from parsec.core.local_device import save_device_with_password_in_config

# from tests.fixtures import local_device_to_backend_user
# from tests.test_cli import mocked_parsec_ext_smartcard
from tests.test_cli import cli_with_running_backend_testbed, mocked_parsec_ext_smartcard

# Fixtures
(mocked_parsec_ext_smartcard,)


@pytest.mark.gui
@pytest.mark.trio
async def test_list_enrollments(
    aqtbot, mocked_parsec_ext_smartcard, alice, core_config, logged_gui, running_backend
):
    core = logged_gui.test_get_core()
    addr = core._backend_conn._cmds.addr

    pki_org_addr = BackendPkiEnrollmentAddr.build(addr.get_backend_addr(), addr.organization_id)
    context = await PkiEnrollmentSubmitterInitialCtx.new(pki_org_addr)
    context = await context.submit(
        config_dir=core_config.config_dir, requested_device_label=DeviceLabel("device"), force=True
    )

    # TODO: running backend is now working, keep implementing


#         event_bus = event_bus_factory()
#         ParsecApp.connected_devices = set()

#         # Language config rely on global var, must reset it for each test !
#         switch_language(core_config, "en")

#         # Pass minimize_on_close to avoid having test blocked by the closing confirmation prompt
#         main_w = testing_main_window_cls(
#             job_scheduler, job_scheduler.close, event_bus, core_config, minimize_on_close=True
#         )
#         aqtbot.add_widget(main_w)
#         main_w.show_window(skip_dialogs=True)
#         main_w.show_top()
#         main_w.add_instance()

#         def right_main_window():
#             assert ParsecApp.get_main_window() is main_w

#         # For some reasons, the main window from the previous test might
#         # still be around. Simply wait for things to settle down until
#         # our freshly created window is detected as the app main window.
#         await aqtbot.wait_until(right_main_window)

#         l_w = main_w.test_get_login_widget()

#         def _devices_and_pendings_listed():
#             assert l_w.widget.layout().count() == 1
#             a_w = l_w.widget.layout().itemAt(0).widget()
#             assert a_w.accounts_widget.layout().count() == 3
#             assert isinstance(a_w.accounts_widget.layout().itemAt(0).widget(), EnrollmentPendingButton)
#             assert isinstance(a_w.accounts_widget.layout().itemAt(1).widget(), AccountButton)

#         await aqtbot.wait_until(_devices_and_pendings_listed)

#         acc_w = l_w.widget.layout().itemAt(0).widget().accounts_widget.layout().itemAt(1).widget()
#         aqtbot.mouse_click(acc_w, QtCore.Qt.LeftButton)

#         assert False

#         def _password_widget_shown():
#             assert isinstance(l_w.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)

#         await aqtbot.wait_until(_password_widget_shown)
#         password_w = l_w.widget.layout().itemAt(0).widget()
#         aqtbot.key_clicks(password_w.line_edit_password, alice_password)

#         await aqtbot.wait_until(lambda: password_w.line_edit_password.text() == alice_password)

#         async with aqtbot.wait_signals([l_w.login_with_password_clicked, tabw.logged_in]):
#             aqtbot.mouse_click(password_w.button_login, QtCore.Qt.LeftButton)

#         def _wait_logged_in():
#             assert not l_w.isVisible()
#             c_w = self.test_get_central_widget()
#             assert c_w.isVisible()

#         await aqtbot.wait_until(_wait_logged_in)
#         c_w = self.test_get_central_widget()
#         #await main_w.test_switch_to_logged_in(alice, alice_password)

#         assert False

# await gui.test_switch_to_logged_in(alice)


# e_w = await logged_gui.test_switch_to_enrollment_widget()

# def _enrollment_shown():
#     assert not e_w.label_empty_list.isVisible()
#     assert e_w.main_layout.count() == 1
#     item = e_w.main_layout.itemAt(0)
#     assert item and item.widget() and isinstance(item.widget(), EnrollmentButton)
#     w = item.widget()
#     assert w.label_name.text() == "John Doe"
#     assert w.label_email.text() == "john@example.com"
#     assert w.label_issuer.text() == ""
#     assert isinstance(w.pending, PkiEnrollementAccepterValidSubmittedCtx)
#     assert w.button_accept.isVisible()
#     assert w.button_reject.isVisible()

# await aqtbot.wait_until(_enrollment_shown)


# @pytest.mark.xfail(reason="Enrollment list is hard coded right now")
# @pytest.mark.gui
# @pytest.mark.trio
# async def test_list_enrollments_empty(aqtbot, logged_gui):
#     e_w = await logged_gui.test_switch_to_enrollment_widget()

#     def _enrollment_shown():
#         assert e_w.label_empty_list.isVisible()
#         assert e_w.main_layout.count() == 0
#         assert e_w.label_empty_list.text() == "No enrollment to show."

#     await aqtbot.wait_until(_enrollment_shown)


# @pytest.mark.gui
# @pytest.mark.trio
# @pytest.mark.parametrize("should_succeed", [True, False])
# async def test_enrollment_accept(aqtbot, logged_gui, snackbar_catcher, should_succeed):
#     e_w = await logged_gui.test_switch_to_enrollment_widget()

#     def _enrollment_shown():
#         assert not e_w.label_empty_list.isVisible()
#         assert e_w.main_layout.count() == 6

#     await aqtbot.wait_until(_enrollment_shown)

#     e_b = e_w.main_layout.itemAt(0).widget()
#     assert e_b is not None

#     if not should_succeed:

#         def _raise_something(_):
#             raise ValueError()

#         e_w._fake_accept = _raise_something

#     async with aqtbot.wait_signal(e_b.accept_clicked):
#         aqtbot.mouse_click(e_b.button_accept, QtCore.Qt.LeftButton)

#     if should_succeed:

#         def _accept_succeeded():
#             assert not e_w.label_empty_list.isVisible()
#             assert e_w.main_layout.count() == 5
#             user_list = [
#                 "john.zoidberg@planetexpress.com",
#                 "leela.turanga@planetexpress.com",
#                 "bender.rodriguez@planetexpress.com",
#                 "zapp.brannigan@doop.com",
#                 "philip.fry@planetexpress.com",
#             ]
#             for i in range(e_w.main_layout.count()):
#                 item = e_w.main_layout.itemAt(i)
#                 assert item and item.widget() and isinstance(item.widget(), EnrollmentButton)
#                 w = item.widget()
#                 assert w.enrollment_info.email in user_list
#                 user_list.remove(w.enrollment_info.email)
#             assert not user_list
#             assert snackbar_catcher.snackbars == ["This demand was successfully accepted."]

#         await aqtbot.wait_until(_accept_succeeded)
#     else:

#         def _accept_failed():
#             assert e_w.main_layout.count() == 6
#             assert snackbar_catcher.snackbars == ["Could not accept this demand."]

#         await aqtbot.wait_until(_accept_failed)


# @pytest.mark.gui
# @pytest.mark.trio
# @pytest.mark.parametrize("should_succeed", [True, False])
# async def test_enrollment_reject(aqtbot, logged_gui, snackbar_catcher, should_succeed):
#     e_w = await logged_gui.test_switch_to_enrollment_widget()

#     def _enrollment_shown():
#         assert not e_w.label_empty_list.isVisible()
#         assert e_w.main_layout.count() == 6

#     await aqtbot.wait_until(_enrollment_shown)

#     e_b = e_w.main_layout.itemAt(0).widget()
#     assert e_b is not None

#     if not should_succeed:

#         def _raise_something(_):
#             raise ValueError()

#         e_w._fake_reject = _raise_something

#     async with aqtbot.wait_signal(e_b.reject_clicked):
#         aqtbot.mouse_click(e_b.button_reject, QtCore.Qt.LeftButton)

#     if should_succeed:

#         def _reject_succeeded():
#             assert not e_w.label_empty_list.isVisible()
#             assert e_w.main_layout.count() == 5
#             user_list = [
#                 "john.zoidberg@planetexpress.com",
#                 "leela.turanga@planetexpress.com",
#                 "bender.rodriguez@planetexpress.com",
#                 "zapp.brannigan@doop.com",
#                 "philip.fry@planetexpress.com",
#             ]
#             for i in range(e_w.main_layout.count()):
#                 item = e_w.main_layout.itemAt(i)
#                 assert item and item.widget() and isinstance(item.widget(), EnrollmentButton)
#                 w = item.widget()
#                 assert w.enrollment_info.email in user_list
#                 user_list.remove(w.enrollment_info.email)
#             assert not user_list
#             assert snackbar_catcher.snackbars == ["This demand was successfully rejected."]

#         await aqtbot.wait_until(_reject_succeeded)
#     else:

#         def _reject_failed():
#             assert e_w.main_layout.count() == 6
#             assert snackbar_catcher.snackbars == ["Could not reject this demand."]

#         await aqtbot.wait_until(_reject_failed)


@pytest.fixture
def catch_enrollment_query_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.enrollment_query_widget.EnrollmentQueryWidget")


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.real_tcp
async def test_query_enrollment(
    aqtbot,
    gui,
    monkeypatch,
    catch_enrollment_query_widget,
    mocked_parsec_ext_smartcard,
    autoclose_dialog,
    backend,
    alice,
):
    async with cli_with_running_backend_testbed(backend, alice) as (backend_addr, alice):
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


# @pytest.mark.gui
# @pytest.mark.trio
# @pytest.mark.parametrize("enrollment_status", ["pending", "rejected", "accepted"])
# async def test_list_pending_enrollments(aqtbot, gui_factory, monkeypatch, enrollment_status):
#     def list_enrollments(_):
#         return [
#             PendingEnrollment("1", "Kiff Kroker", "Planet_Express", enrollment_status, "16/02/3022")
#         ]

#     monkeypatch.setattr(
#         "parsec.core.gui.login_widget.LoginWidget.list_pending_enrollments", list_enrollments
#     )

#     gui = await gui_factory()
#     lw = gui.test_get_login_widget()
#     accounts_w = lw.widget.layout().itemAt(0).widget()
#     assert accounts_w.accounts_widget.layout().count() == 2
#     epw = accounts_w.accounts_widget.layout().itemAt(0).widget()
#     assert isinstance(epw, EnrollmentPendingButton)
#     assert epw.label_org.text() == "Planet_Express"
#     assert epw.label_name.text() == "Kiff Kroker"
#     if enrollment_status == "pending":
#         assert not epw.button_action.isVisible()
#         assert epw.label_status.text() == "TEXT_ENROLLMENT_STATUS_PENDING"
#     elif enrollment_status == "accepted":
#         assert epw.button_action.isVisible()
#         assert epw.button_action.text() == "ACTION_ENROLLMENT_FINALIZE"
#         assert epw.label_status.text() == "TEXT_ENROLLMENT_STATUS_ACCEPTED"
#     elif enrollment_status == "rejected":
#         assert epw.button_action.isVisible()
#         assert epw.button_action.text() == "ACTION_ENROLLMENT_CLEAR"
#         assert epw.label_status.text() == "TEXT_ENROLLMENT_STATUS_REJECTED"
