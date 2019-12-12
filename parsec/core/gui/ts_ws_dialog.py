# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt, QDate, QTime, pyqtSignal
from PyQt5.QtWidgets import QDialog

import pendulum

from parsec.core.gui.lang import get_qlocale, translate as _
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.trio_thread import ThreadSafeQtSignal
from parsec.core.gui.ui.ts_ws_dialog import Ui_TsWsDialog


async def _do_workspace_get_creation_timestamp(workspace_fs):
    # Add 1 second as we want a timestamp safe to ask for a manifest to the backend from the GUI
    return (await workspace_fs.get_earliest_timestamp()).add(seconds=1)


class TsWsDialog(QDialog, Ui_TsWsDialog):
    get_creation_timestamp_success = pyqtSignal()
    get_creation_timestamp_error = pyqtSignal()

    def __init__(self, workspace_fs, jobs_ctx, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.setWindowFlags(Qt.SplashScreen)
        self.workspace_fs = workspace_fs
        self.jobs_ctx = jobs_ctx
        self._disable()
        self.calendar_widget.setLocale(get_qlocale())
        self.get_creation_timestamp_success.connect(self.enable_with_timestamp)
        self.get_creation_timestamp_error.connect(self.show_error)
        self.button_close.clicked.connect(self.cancel)
        self.limits_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "get_creation_timestamp_success"),
            ThreadSafeQtSignal(self, "get_creation_timestamp_error"),
            _do_workspace_get_creation_timestamp,
            workspace_fs=workspace_fs,
        )

    @property
    def date(self):
        return self.calendar_widget.selectedDate()

    @property
    def time(self):
        return self.time_edit.time()

    def cancel(self):
        if self.limits_job:
            self.limits_job.cancel_and_join()
        self.reject()

    def set_time_limits(self):
        selected_date = self.calendar_widget.selectedDate()
        if selected_date == QDate(*self.creation_date):
            self.time_edit.setMinimumTime(QTime(*self.creation_time))
        else:
            self.time_edit.clearMinimumTime()
        if selected_date == QDate(*self.now_date):
            self.time_edit.setMaximumTime(QTime(*self.now_time))
        else:
            self.time_edit.clearMaximumTime()

    def _disable(self):
        self.label_info.setText(_("WORKSPACE_REENCRYPTION_WAIT"))
        self.widget_reencrypt.setEnabled(False)

    def _enable(self):
        self.label_info.setText(_("WORKSPACE_REENCRYPTION_INFO"))
        self.widget_reencrypt.setEnabled(True)

    def show_error(self):
        if self.limits_job and self.limits_job.status != "cancelled":
            show_error(self, _("ERR_WORKSPACE_VERSIONS_ACCESS"), exception=self.limits_job.exc)
        self.limits_job = None
        self.reject()

    def enable_with_timestamp(self):
        creation = self.limits_job.ret.in_timezone("local")
        self.limits_job = None
        self.creation_date = (creation.year, creation.month, creation.day)
        self.creation_time = (creation.hour, creation.minute, creation.second)
        now = pendulum.now().in_timezone("local")
        self.now_date = (now.year, now.month, now.day)
        self.now_time = (now.hour, now.minute, now.second)
        self.calendar_widget.setMinimumDate(QDate(*self.creation_date))
        self.calendar_widget.setMaximumDate(QDate(*self.now_date))
        self.calendar_widget.selectionChanged.connect(self.set_time_limits)
        self.set_time_limits()
        self.time_edit.setDisplayFormat("h:mm:ss")
        self._enable()
