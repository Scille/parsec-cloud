# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Callable, Optional, Tuple, TypeVar

from PyQt5.QtCore import Qt, QDate, QTime, pyqtSignal
from PyQt5.QtGui import QColor
from PyQt5.QtWidgets import QWidget, QDialog, QApplication

from structlog import get_logger

from parsec._parsec import DateTime
from parsec.core.fs.workspacefs.workspacefs import WorkspaceFS

from parsec.core.gui.trio_jobs import QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.lang import get_qlocale, translate as _, format_datetime
from parsec.core.gui.custom_dialogs import show_error, GreyedDialog
from parsec.core.gui.ui.timestamped_workspace_widget import Ui_TimestampedWorkspaceWidget


logger = get_logger()

R = TypeVar("R")


async def _do_workspace_get_creation_timestamp(workspace_fs: WorkspaceFS) -> DateTime:
    # Add 1 second as we want a timestamp safe to ask for a manifest to the backend from the GUI
    return (await workspace_fs.get_earliest_timestamp()).add(seconds=1)


class TimestampedWorkspaceWidget(QWidget, Ui_TimestampedWorkspaceWidget):
    get_creation_timestamp_success = pyqtSignal(QtToTrioJob)
    get_creation_timestamp_error = pyqtSignal(QtToTrioJob)

    def __init__(self, workspace_fs: WorkspaceFS, jobs_ctx: QtToTrioJobScheduler) -> None:
        super().__init__()
        self.setupUi(self)
        self.dialog: Optional[GreyedDialog] = None
        self.workspace_fs = workspace_fs
        self.jobs_ctx = jobs_ctx
        self.creation_date: Optional[Tuple[int, int, int]] = None
        self.creation_time: Optional[Tuple[int, int, int]] = None
        self.calendar_widget.setLocale(get_qlocale())
        for d in (Qt.Saturday, Qt.Sunday):
            fmt = self.calendar_widget.weekdayTextFormat(d)
            fmt.setForeground(QColor(0, 0, 0))
            self.calendar_widget.setWeekdayTextFormat(d, fmt)
        self.get_creation_timestamp_success.connect(self.on_success)
        self.get_creation_timestamp_error.connect(self.on_error)
        self.limits_job: Optional[QtToTrioJob[DateTime]] = self.jobs_ctx.submit_job(
            (self, "get_creation_timestamp_success"),
            (self, "get_creation_timestamp_error"),
            _do_workspace_get_creation_timestamp,
            workspace_fs=workspace_fs,
        )
        self.button_show.clicked.connect(self._on_show_clicked)
        # Only enable widget once limits_job has finished
        self.setEnabled(False)

    @property
    def date(self) -> QDate:
        return self.calendar_widget.selectedDate()

    @property
    def time(self) -> QTime:
        return self.time_edit.time()

    def _on_show_clicked(self) -> None:
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            QApplication.activeModalWidget().accept()  # type: ignore[attr-defined]
        else:
            logger.warning("Cannot close dialog when displaying info")

    def cancel(self) -> None:
        if self.limits_job:
            self.limits_job.cancel()

    def set_time_limits(self) -> None:
        selected_date = self.calendar_widget.selectedDate()
        if selected_date == QDate(*self.creation_date):
            self.time_edit.setMinimumTime(QTime(*self.creation_time))
        else:
            self.time_edit.clearMinimumTime()
        if selected_date == QDate(*self.now_date):
            self.time_edit.setMaximumTime(QTime(*self.now_time))
        else:
            self.time_edit.clearMaximumTime()

    def on_error(self, job: QtToTrioJob[DateTime]) -> None:
        assert self.limits_job is not None and self.limits_job is job
        if self.limits_job.status != "cancelled":
            show_error(
                self,
                _("TEXT_WORKSPACE_TIMESTAMPED_VERSION_RETRIEVAL_FAILED"),
                exception=self.limits_job.exc,
            )
        self.limits_job = None
        assert self.dialog is not None
        self.dialog.reject()

    def on_success(self, job: QtToTrioJob[DateTime]) -> None:
        assert self.limits_job is not None and self.limits_job is job
        assert self.limits_job.ret is not None
        creation = self.limits_job.ret.to_local()
        self.limits_job = None
        self.creation_date = (creation.year, creation.month, creation.day)
        self.creation_time = (creation.hour, creation.minute, creation.second)
        self.label_info.setText(
            _("TEXT_WORKSPACE_TIMESTAMPED_INSTRUCTIONS_created").format(
                created=format_datetime(creation, full=True)
            )
        )
        now = DateTime.now().to_local()
        self.now_date = (now.year, now.month, now.day)
        self.now_time = (now.hour, now.minute, now.second)
        self.calendar_widget.setMinimumDate(QDate(*self.creation_date))
        self.calendar_widget.setMaximumDate(QDate(*self.now_date))
        self.calendar_widget.selectionChanged.connect(self.set_time_limits)
        self.set_time_limits()
        self.time_edit.setDisplayFormat("h:mm:ss")
        # All set, now user can pick date&time !
        self.setEnabled(True)

    def on_close(self) -> None:
        self.cancel()

    @classmethod
    def show_modal(  # type: ignore[misc]
        cls,
        workspace_fs: WorkspaceFS,
        jobs_ctx: QtToTrioJobScheduler,
        parent: QWidget,
        on_finished: Callable[..., R],
    ) -> TimestampedWorkspaceWidget:
        w = cls(workspace_fs=workspace_fs, jobs_ctx=jobs_ctx)
        d = GreyedDialog(
            center_widget=w, title=_("TEXT_WORKSPACE_TIMESTAMPED_TITLE"), parent=parent, width=600
        )
        w.dialog = d

        def _on_finished(result: QDialog.DialogCode) -> R:
            if result == QDialog.Rejected:
                return on_finished(None, None)
            return on_finished(w.date, w.time)

        d.finished.connect(_on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
