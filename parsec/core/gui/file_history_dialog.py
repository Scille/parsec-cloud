# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QDialog, QWidget, QListWidgetItem

from parsec.core.gui.ui.file_history_dialog import Ui_FileHistoryDialog
from parsec.core.gui.ui.file_history_widget import Ui_FileHistoryWidget


class FileHistoryWidget(QWidget, Ui_FileHistoryWidget):
    def __init__(self, date_time, user, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_datetime.setText(date_time.isoformat())
        self.label_user.setText(user)


class FileHistoryDialog(QDialog, Ui_FileHistoryDialog):
    def __init__(self, file_name, created_on, updated_on, history, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.setWindowFlags(Qt.SplashScreen)
        self.label_file_name.setText(file_name)
        self.label_created_on.setText(created_on.isoformat())
        self.label_updated_on.setText(updated_on.isoformat())
        for h in history:
            self.add_history()

    def add_history(self):
        import datetime

        item = QListWidgetItem()
        w = FileHistoryWidget(datetime.datetime.utcnow(), "Max")
        item.setSizeHint(w.size())
        self.history_list.addItem(item)
        self.history_list.setItemWidget(item, w)
