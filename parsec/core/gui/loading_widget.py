# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.loading_widget import Ui_LoadingWidget


class LoadingWidget(QWidget, Ui_LoadingWidget):
    cancelled = pyqtSignal()

    def __init__(self, total_size):
        super().__init__()
        self.setupUi(self)
        self.progress_bar.setMaximum(total_size)
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
        self.progress_bar.setValue(size)
