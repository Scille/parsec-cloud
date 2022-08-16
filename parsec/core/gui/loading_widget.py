# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.loading_widget import Ui_LoadingWidget


class LoadingWidget(QWidget, Ui_LoadingWidget):
    cancelled = pyqtSignal()

    def __init__(self, total_size):
        super().__init__()
        self.setupUi(self)
        self.divider = 1
        # Check for int32 overflow
        while int(total_size / self.divider) > 0x7FFFFFFF:
            self.divider *= 1000
        self.progress_bar.setMaximum(int(total_size / self.divider))
        self.progress_bar.setMinimum(0)
        self.progress_bar.setValue(0)

    def on_close(self):
        self.cancelled.emit()

    def set_current_file(self, f):
        if len(f) > 35:
            self.label.setText('"{}...{}"'.format(f[:26], f[-6:]))
        else:
            self.label.setText(f'"{f}"')

    def set_progress(self, size):
        self.progress_bar.setValue(int(size / self.divider))
