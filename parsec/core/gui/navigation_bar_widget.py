# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt

from PyQt5.QtGui import QPixmap, QColor

from PyQt5.QtWidgets import (
    QHBoxLayout,
    QWidget,
    QSpacerItem,
    QSizePolicy,
    QLabel,
    QScrollArea,
    QFrame,
    QScroller,
)

from parsec.core.fs import FsPath
from parsec.api.data import EntryName

from parsec.core.gui.custom_widgets import ClickableLabel, IconLabel


class NavigationBarWidget(QWidget):
    route_clicked = pyqtSignal(FsPath)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        # Path can be quite long and they would force a window resize,
        # so we put them inside a ScrollArea.
        # Having a scrollbar there would be horrendous, so we use a
        # QScroller to be able to scroll with the mouse.
        self.setFixedHeight(25)
        self.setSizePolicy(QSizePolicy.Policy.MinimumExpanding, QSizePolicy.Policy.Fixed)
        self.setLayout(QHBoxLayout())
        self.layout().setContentsMargins(0, 0, 0, 0)
        self.scroll_area = QScrollArea()

        QScroller.grabGesture(self.scroll_area, QScroller.LeftMouseButtonGesture)
        self.scroll_area.setFrameShape(QFrame.Shape.NoFrame)
        self.scroll_area.setFrameShadow(QFrame.Shadow.Plain)
        self.scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        self.scroll_area.setLineWidth(0)
        self.scroll_area.setFixedHeight(25)
        self.scroll_area.setWidgetResizable(True)
        self.scroll_area.horizontalScrollBar().rangeChanged.connect(self._on_range_changed)
        self.layout().addWidget(self.scroll_area)
        self.inner_widget = QWidget()
        self.inner_widget.setFixedHeight(25)
        self.inner_widget.setStyleSheet("background-color: #EEEEEE;")
        self.inner_widget.setLayout(QHBoxLayout())
        self.scroll_area.setWidget(self.inner_widget)
        self.inner_widget.layout().setContentsMargins(0, 3, 0, 0)
        self.inner_widget.layout().addItem(QSpacerItem(10, 0, QSizePolicy.Policy.MinimumExpanding))
        self.paths = []
        self.workspace_name = None

    def _on_range_changed(self, min, max):
        self.scroll_area.horizontalScrollBar().setSliderPosition(max)

    def _on_label_clicked(self, idx):
        def _internal_label_clicked(_title):
            complete_path = FsPath([EntryName(p) for i, p in enumerate(self.paths) if i < idx])
            self.route_clicked.emit(complete_path)

        return _internal_label_clicked

    def get_current_path(self):
        return FsPath(self.paths)

    def from_path(self, workspace_name, path):
        self.clear()
        path = FsPath(path)
        parts = [workspace_name]
        self.workspace_name = workspace_name
        parts.extend(path.parts)
        self.paths = parts[1:]
        for idx, part in enumerate(parts):
            icon = IconLabel()
            icon.setScaledContents(True)
            icon.setProperty("color", QColor(153, 153, 153))
            if idx == 0:
                # Adding a new spacer item creates problems so we use the
                # icon to add a margin
                icon.setFixedSize(15, 10)
                icon.setPixmap(QPixmap(":/icons/images/material/fiber_manual_record.svg"))
                icon.apply_style()
                icon.setStyleSheet("margin-right: 5px;")
                self.layout().insertWidget(0, icon)
            else:
                icon.setFixedSize(15, 15)
                icon.setPixmap(QPixmap(":/icons/images/material/chevron_right.svg"))
                icon.apply_style()
                self.inner_widget.layout().insertWidget(
                    self.inner_widget.layout().count() - 1, icon
                )
            label = None
            if idx != len(parts) - 1:
                obj_name = str(idx)
                label = ClickableLabel(part.str)
                label.clicked.connect(self._on_label_clicked(idx))
                label.setObjectName(f"label_{obj_name}")
                label.setStyleSheet(
                    f"#label_{obj_name} {{ color: #999999; }} #label_{obj_name}:hover {{ color: #0092FF; }}"
                )
            else:
                label = QLabel(part.str)
                label.setStyleSheet("color: #999999;")
            font = label.font()
            font.setBold(True)
            font.setPixelSize(16)
            label.setFont(font)
            self.inner_widget.layout().insertWidget(self.inner_widget.layout().count() - 1, label)

    def clear(self):
        self.paths = []
        self.workspace_name = None
        while self.layout().count() != 1:
            item = self.layout().takeAt(0)
            if item and item.widget():
                w = item.widget()
                self.layout().removeWidget(w)
                w.hide()
        while self.inner_widget.layout().count() != 1:
            item = self.inner_widget.layout().takeAt(0)
            if item and item.widget():
                w = item.widget()
                self.inner_widget.layout().removeWidget(w)
                w.hide()
