# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt, QDate, QTime
from PyQt5.QtWidgets import QDialog

import pendulum
from trio import run

from parsec.core.gui.ui.ts_ws_dialog import Ui_TsWsDialog


class TsWsDialog(QDialog, Ui_TsWsDialog):
    def __init__(self, workspace_fs, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.setWindowFlags(Qt.SplashScreen)
        self.workspace_fs = workspace_fs
        creation = run(workspace_fs.path_info, "/")["created"].in_timezone("local")
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
        self.button_show.clicked.connect(self.mount_it)

    def mount_it(self):
        date = self.calendar_widget.selectedDate()
        time = self.time_edit.time()

        datetime = pendulum.datetime(
            date.year(),
            date.month(),
            date.day(),
            time.hour(),
            time.minute(),
            time.second(),
            tzinfo="local",
        )
        self.parent.workspace_mount_timestamped_requested.emit(
            self.workspace_fs.workspace_id, datetime
        )
        self.accept()

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
