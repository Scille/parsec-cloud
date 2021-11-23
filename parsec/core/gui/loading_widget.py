# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.ui.loading_widget import Ui_LoadingWidget


class LoadingWidget(QWidget, Ui_LoadingWidget):
    cancelled = pyqtSignal()

    def __init__(self, total_size, message=None):
        super().__init__()
        self.setupUi(self)
        self.divider = 1
        # Check for int32 overflow
        while int(total_size / self.divider) > 0x7FFFFFFF:
            self.divider *= 1000
        self.progress_bar.setMaximum(int(total_size / self.divider))
        self.current_size_display = None
        self.current_file_display = None
        self.total_size_display = get_filesize(total_size)
        self.progress_bar.setMinimum(0)
        self.progress_bar.setValue(0)
        if message:
            self.label.setText(message)
        self.format_text()

    def format_text(self):
        if self.current_size_display:
            self.progress_bar.setFormat(
                f"{self.current_file_display or ''} %p% ({self.current_size_display} / {self.total_size_display})"
            )
        else:
            self.progress_bar.setFormat(f"{self.current_file_display or ''} %p%")

    def on_close(self):
        self.cancelled.emit()

    def set_current_file(self, f):
        if len(f) > 35:
            self.current_file_display = "{}...{}".format(f[:26], f[-6:])
        else:
            self.current_file_display = f"{f}"
        self.format_text()

    def set_progress(self, size):
        self.current_size_display = get_filesize(size)
        self.progress_bar.setValue(int(size / self.divider))
        self.format_text()
