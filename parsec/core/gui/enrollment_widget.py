# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from collections import namedtuple

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QIcon
from PyQt5.QtWidgets import QWidget

from parsec.core.core_events import CoreEvent

from parsec.core.gui.lang import translate
from parsec.core.gui.snackbar_widget import SnackbarManager

from parsec.core.gui.ui.enrollment_widget import Ui_EnrollmentWidget
from parsec.core.gui.ui.recruitment_widget import Ui_RecruitmentWidget


RecruitmentInfo = namedtuple(
    "RecruitmentInfo", ["token", "name", "email", "date", "certif_is_valid"]
)


class RecruitmentWidget(QWidget, Ui_RecruitmentWidget):
    accept_clicked = pyqtSignal(QWidget)
    reject_clicked = pyqtSignal(QWidget)

    def __init__(self, recruitment_info):
        super().__init__()
        self.setupUi(self)
        self.recruitment_info = recruitment_info
        self.label_name.setText(self.recruitment_info.name)
        self.label_email.setText(self.recruitment_info.email)
        self.label_date.setText(self.recruitment_info.date)
        self.button_accept.setIcon(QIcon(":/icons/images/material/done.svg"))
        self.button_reject.setIcon(QIcon(":/icons/images/material/clear.svg"))

        if self.recruitment_info.certif_is_valid:
            self.button_accept.setVisible(True)
            self.label_certif.setText(translate("TEXT_ENROLLMENT_CERTIFICATE_IS_VALID"))
        else:
            self.button_accept.setVisible(False)
            self.label_certif.setText(translate("TEXT_ENROLLMENT_CERTIFICATE_IS_INVALID"))
        self.button_accept.clicked.connect(lambda: self.accept_clicked.emit(self))
        self.button_reject.clicked.connect(lambda: self.reject_clicked.emit(self))

    @property
    def token(self):
        return self.recruitment_info.token

    def set_buttons_enabled(self, enabled):
        self.button_accept.setEnabled(enabled)
        self.button_reject.setEnabled(enabled)


class EnrollmentWidget(QWidget, Ui_EnrollmentWidget):
    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.label_empty_list.hide()
        self.event_bus = event_bus

    def showEvent(self, event):
        self.reset()
        return
        self.event_bus.connect(CoreEvent.NEW_RECRUIT, self._on_new_recruit)
        self.event_bus.connect(CoreEvent.RECRUIT_UPDATED, self._on_recruit_updated)
        self.event_bus.connect(CoreEvent.RECRUIT_ACCEPTED, self._on_recruit_accepted)
        self.event_bus.connect(CoreEvent.RECRUIT_REJECTED, self._on_recruit_rejected)

    def hideEvent(self, event):
        return
        try:
            self.event_bus.disconnect(CoreEvent.NEW_RECRUIT, self._on_new_recruit)
            self.event_bus.disconnect(CoreEvent.RECRUIT_UPDATED, self._on_recruit_updated)
            self.event_bus.disconnect(CoreEvent.RECRUIT_ACCEPTED, self._on_recruit_accepted)
            self.event_bus.disconnect(CoreEvent.RECRUIT_REJECTED, self._on_recruit_rejected)
        except ValueError:
            pass

    def reset(self):
        self.jobs_ctx.submit_job(None, None, self.list_pending_recruitments)

    def clear_layout(self):
        while self.main_layout.count() != 0:
            item = self.main_layout.takeAt(0)
            if item and item.widget():
                w = item.widget()
                self.main_layout.removeWidget(w)
                w.hide()
                w.setParent(None)

    async def list_pending_recruitments(self):
        try:
            self.clear_layout()
            # recruitments = await self.core.list_pending_recruitments()
            recruitments = [
                RecruitmentInfo(
                    "1",
                    "Hubert Farnsworth",
                    "hubert.farnsworth@planetexpress.com",
                    "16/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "2", "John A. Zoidberg", "john.zoidberg@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "3", "Leela Turanga", "leela.turanga@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "4",
                    "Bender B. Rodriguez",
                    "bender.rodriguez@planetexpress.com",
                    "15/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "5", "Zapp Brannigan", "zapp.brannigan@doop.com", "15/02/3022", False
                ),
                RecruitmentInfo(
                    "6", "Philip J. Fry", "philip.fry@planetexpress.com", "16/02/3022", True
                ),
                RecruitmentInfo(
                    "1",
                    "Hubert Farnsworth",
                    "hubert.farnsworth@planetexpress.com",
                    "16/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "2", "John A. Zoidberg", "john.zoidberg@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "3", "Leela Turanga", "leela.turanga@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "4",
                    "Bender B. Rodriguez",
                    "bender.rodriguez@planetexpress.com",
                    "15/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "5", "Zapp Brannigan", "zapp.brannigan@doop.com", "15/02/3022", False
                ),
                RecruitmentInfo(
                    "6", "Philip J. Fry", "philip.fry@planetexpress.com", "16/02/3022", True
                ),
                RecruitmentInfo(
                    "1",
                    "Hubert Farnsworth",
                    "hubert.farnsworth@planetexpress.com",
                    "16/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "2", "John A. Zoidberg", "john.zoidberg@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "3", "Leela Turanga", "leela.turanga@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "4",
                    "Bender B. Rodriguez",
                    "bender.rodriguez@planetexpress.com",
                    "15/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "5", "Zapp Brannigan", "zapp.brannigan@doop.com", "15/02/3022", False
                ),
                RecruitmentInfo(
                    "6", "Philip J. Fry", "philip.fry@planetexpress.com", "16/02/3022", True
                ),
                RecruitmentInfo(
                    "1",
                    "Hubert Farnsworth",
                    "hubert.farnsworth@planetexpress.com",
                    "16/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "2", "John A. Zoidberg", "john.zoidberg@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "3", "Leela Turanga", "leela.turanga@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "4",
                    "Bender B. Rodriguez",
                    "bender.rodriguez@planetexpress.com",
                    "15/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "5", "Zapp Brannigan", "zapp.brannigan@doop.com", "15/02/3022", False
                ),
                RecruitmentInfo(
                    "6", "Philip J. Fry", "philip.fry@planetexpress.com", "16/02/3022", True
                ),
                RecruitmentInfo(
                    "1",
                    "Hubert Farnsworth",
                    "hubert.farnsworth@planetexpress.com",
                    "16/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "2", "John A. Zoidberg", "john.zoidberg@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "3", "Leela Turanga", "leela.turanga@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "4",
                    "Bender B. Rodriguez",
                    "bender.rodriguez@planetexpress.com",
                    "15/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "5", "Zapp Brannigan", "zapp.brannigan@doop.com", "15/02/3022", False
                ),
                RecruitmentInfo(
                    "6", "Philip J. Fry", "philip.fry@planetexpress.com", "16/02/3022", True
                ),
                RecruitmentInfo(
                    "1",
                    "Hubert Farnsworth",
                    "hubert.farnsworth@planetexpress.com",
                    "16/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "2", "John A. Zoidberg", "john.zoidberg@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "3", "Leela Turanga", "leela.turanga@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "4",
                    "Bender B. Rodriguez",
                    "bender.rodriguez@planetexpress.com",
                    "15/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "5", "Zapp Brannigan", "zapp.brannigan@doop.com", "15/02/3022", False
                ),
                RecruitmentInfo(
                    "6", "Philip J. Fry", "philip.fry@planetexpress.com", "16/02/3022", True
                ),
                RecruitmentInfo(
                    "1",
                    "Hubert Farnsworth",
                    "hubert.farnsworth@planetexpress.com",
                    "16/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "2", "John A. Zoidberg", "john.zoidberg@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "3", "Leela Turanga", "leela.turanga@planetexpress.com", "14/02/3022", True
                ),
                RecruitmentInfo(
                    "4",
                    "Bender B. Rodriguez",
                    "bender.rodriguez@planetexpress.com",
                    "15/02/3022",
                    True,
                ),
                RecruitmentInfo(
                    "5", "Zapp Brannigan", "zapp.brannigan@doop.com", "15/02/3022", False
                ),
                RecruitmentInfo(
                    "6", "Philip J. Fry", "philip.fry@planetexpress.com", "16/02/3022", True
                ),
            ]
            if len(recruitments) == 0:
                self.label_empty_list.setText(translate("TEXT_ENROLLMENT_NO_PENDING_RECRUITMENT"))
                return
            for recruit in recruitments:
                rw = RecruitmentWidget(recruit)
                rw.accept_clicked.connect(self._on_accept_clicked)
                rw.reject_clicked.connect(self._on_reject_clicked)
                self.main_layout.addWidget(rw)
        except:
            self.label_empty_list.setText(translate("TEXT_ENROLLMENT_FAILED_TO_RETRIEVE_LIST"))
            self.label_empty_list.show()

    def _on_accept_clicked(self, rw):
        rw.set_buttons_enabled(False)
        self.jobs_ctx.submit_job(None, None, self.accept_recruit, recruitment_widget=rw)

    def _on_reject_clicked(self, rw):
        rw.set_buttons_enabled(False)
        self.jobs_ctx.submit_job(None, None, self.reject_recruit, recruitment_widget=rw)

    async def accept_recruit(self, recruitment_widget):
        try:
            # await self.core.accept_recruit(token)
            SnackbarManager.inform(translate("TEXT_ENROLLMENT_ACCEPT_SUCCESS"))
            self.main_layout.removeWidget(recruitment_widget)
        except:
            SnackbarManager.warn(translate("TEXT_ENROLLMENT_ACCEPT_FAILURE"))
            recruitment_widget.set_buttons_enabled(True)

    async def reject_recruit(self, recruitment_widget):
        try:
            # await self.core.reject_recruit(token)
            SnackbarManager.inform(translate("TEXT_ENROLLMENT_REJECT_SUCCESS"))
            self.main_layout.remoteWidget(recruitment_widget)
        except:
            SnackbarManager.warn(translate("TEXT_ENROLLMENT_REJECT_FAILURE"))
            recruitment_widget.set_buttons_enabled(True)
