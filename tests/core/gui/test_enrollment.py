# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from PyQt5 import QtCore

from parsec.core.gui.enrollment_widget import RecruitmentWidget


@pytest.mark.gui
@pytest.mark.trio
async def test_list_enrollements(aqtbot, logged_gui):
    e_w = await logged_gui.test_switch_to_enrollment_widget()

    def _enrollment_shown():
        assert not e_w.label_empty_list.isVisible()
        assert e_w.main_layout.count() == 6
        user_list = [
            "hubert.farnsworth@planetexpress.com",
            "john.zoidberg@planetexpress.com",
            "leela.turanga@planetexpress.com",
            "bender.rodriguez@planetexpress.com",
            "zapp.brannigan@doop.com",
            "philip.fry@planetexpress.com",
        ]
        for i in range(e_w.main_layout.count()):
            item = e_w.main_layout.itemAt(i)
            assert item and item.widget() and isinstance(item.widget(), RecruitmentWidget)
            w = item.widget()
            assert w.recruitment_info.email in user_list
            assert w.label_email.text() == w.recruitment_info.email
            assert w.label_name.text() == w.recruitment_info.name
            if w.recruitment_info.certif_is_valid:
                assert w.button_accept.isVisible()
                assert w.button_reject.isVisible()
            else:
                assert not w.button_accept.isVisible()
                assert w.button_reject.isVisible()
            user_list.remove(w.recruitment_info.email)

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
async def test_enrollment_accept(aqtbot, logged_gui, snackbar_catcher):
    e_w = await logged_gui.test_switch_to_enrollment_widget()

    def _enrollment_shown():
        assert not e_w.label_empty_list.isVisible()
        assert e_w.main_layout.count() == 6

    await aqtbot.wait_until(_enrollment_shown)

    e_b = e_w.main_layout.itemAt(0).widget()
    assert e_b is not None
    async with aqtbot.wait_signal(e_b.accept_clicked):
        aqtbot.mouse_click(e_b.button_accept, QtCore.Qt.LeftButton)

    def _enrollment_removed():
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
            assert item and item.widget() and isinstance(item.widget(), RecruitmentWidget)
            w = item.widget()
            assert w.recruitment_info.email in user_list
            user_list.remove(w.recruitment_info.email)

    await aqtbot.wait_until(_enrollment_removed)

    assert snackbar_catcher.snackbars == ["This demand was successfully accepted."]


@pytest.mark.gui
@pytest.mark.trio
async def test_enrollment_reject(aqtbot, logged_gui, snackbar_catcher):
    e_w = await logged_gui.test_switch_to_enrollment_widget()

    def _enrollment_shown():
        assert not e_w.label_empty_list.isVisible()
        assert e_w.main_layout.count() == 6

    await aqtbot.wait_until(_enrollment_shown)

    e_b = e_w.main_layout.itemAt(0).widget()
    assert e_b is not None
    async with aqtbot.wait_signal(e_b.reject_clicked):
        aqtbot.mouse_click(e_b.button_reject, QtCore.Qt.LeftButton)

    def _enrollment_removed():
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
            assert item and item.widget() and isinstance(item.widget(), RecruitmentWidget)
            w = item.widget()
            assert w.recruitment_info.email in user_list
            user_list.remove(w.recruitment_info.email)

    await aqtbot.wait_until(_enrollment_removed)

    assert snackbar_catcher.snackbars == ["This demand was successfully rejected."]
