# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import pyqtSignal

from PyQt5.QtGui import QPixmap, QColor

from PyQt5.QtWidgets import QHBoxLayout, QWidget, QSpacerItem, QSizePolicy

from parsec.core.fs import FsPath
from parsec.api.data import EntryName

from parsec.core.gui.custom_widgets import ClickableLabel, IconLabel


class NavigationBarWidget(QWidget):
    route_clicked = pyqtSignal(FsPath)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setLayout(QHBoxLayout())
        self.layout().addItem(QSpacerItem(10, 0, QSizePolicy.Policy.Expanding))
        self.paths = []

    def _on_label_clicked(self, idx):
        def _internal_label_clicked(_title):
            complete_path = FsPath(
                [EntryName(p) for i, p in enumerate(self.paths) if i <= idx and i != 0]
            )
            self.route_clicked.emit(complete_path)

        return _internal_label_clicked

    def get_current_path(self):
        return FsPath(self.paths[1:])

    def from_path(self, workspace_name, path):
        self.clear()
        path = FsPath(path)
        parts = [workspace_name]
        parts.extend(path.parts)
        for idx, part in enumerate(parts):
            icon = IconLabel()
            icon.setScaledContents(True)
            icon.setProperty("color", QColor(153, 153, 153))
            if idx == 0:
                icon.setFixedSize(10, 10)
                icon.setPixmap(QPixmap(":/icons/images/material/fiber_manual_record.svg"))
            else:
                icon.setFixedSize(15, 15)
                icon.setPixmap(QPixmap(":/icons/images/material/chevron_right.svg"))
            icon.apply_style()
            self.layout().insertWidget(self.layout().count() - 1, icon)
            label = ClickableLabel(part)
            font = label.font()
            font.setBold(True)
            font.setUnderline(True)
            label.setFont(font)
            label.setStyleSheet("color: #0099FF;")
            label.clicked.connect(self._on_label_clicked(idx))
            self.layout().insertWidget(self.layout().count() - 1, label)
            self.paths.append(part)

    def clear(self):
        self.paths = []
        while self.layout().count() != 1:
            item = self.layout().takeAt(0)
            if item:
                w = item.widget()
                self.layout().removeWidget(w)
                w.hide()
