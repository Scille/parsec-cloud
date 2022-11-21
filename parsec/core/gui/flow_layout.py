# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Any, Literal, Union, cast

from PyQt5.QtCore import Qt, QRect, QPoint, QSize
from PyQt5.QtWidgets import QLayout, QStyle, QSizePolicy, QWidget, QLayoutItem


class FlowLayout(QLayout):
    def __init__(self, spacing: int = 10, parent: QWidget | None = None) -> None:
        if parent:
            super().__init__(parent)
        else:
            super().__init__()
        self.setSpacing(spacing)
        self.items: list[QLayoutItem] = []

    def addItem(self, item: QLayoutItem) -> None:
        self.items.append(item)

    def horizontalSpacing(self) -> int:
        if self.spacing() >= 0:
            return self.spacing()
        return self._smart_spacing(QStyle.PM_LayoutHorizontalSpacing)

    def verticalSpacing(self) -> int:
        if self.spacing() >= 0:
            return self.spacing()
        return self._smart_spacing(QStyle.PM_LayoutVerticalSpacing)

    def count(self) -> int:
        return len(self.items)

    def itemAt(self, index: int) -> QLayoutItem:
        if index >= 0 and index < len(self.items):
            return self.items[index]
        # MyPy: `QLayout::itemAt` should return `0` if there is not such item at the provided index.
        # https://doc.qt.io/qt-5/qlayout.html#itemAt
        return cast(QLayoutItem, None)

    def takeAt(self, index: int) -> QLayoutItem:
        if index >= 0 and index < len(self.items):
            return self.items.pop(index)
        # MyPy: `QLayout::takeAt` should return `0` if there is not such item at the provided index.
        # https://doc.qt.io/qt-5/qlayout.html#takeAt
        return cast(QLayoutItem, None)

    def clear(self) -> None:
        for widget in self.pop_all():
            # Mypy: This is the correct way to remove a parent from a widget.
            widget.setParent(None)  # type: ignore[call-overload]

    def pop_all(self) -> list[QWidget]:
        results = []
        while self.count() != 0:
            try:
                item = self.takeAt(0)
            except IndexError:
                pass
            else:
                widget = item.widget()
                self.removeWidget(widget)
                widget.hide()
                results.append(widget)
        return results

    def expandingDirections(self) -> Qt.Orientations:
        return Qt.Orientations(Qt.Horizontal)

    def hasHeightForWidth(self) -> Literal[True]:
        return True

    def heightForWidth(self, width: int) -> int:
        height = self._do_layout(QRect(0, 0, width, 0), True)
        return height

    def setGeometry(self, rect: QRect) -> None:
        super().setGeometry(rect)
        self._do_layout(rect, False)

    def sizeHint(self) -> QSize:
        return self.minimumSize()

    def minimumSize(self) -> QSize:
        size = QSize()
        for item in self.items:
            size = size.expandedTo(item.minimumSize())
        return size

    def _do_layout(self, rect: QRect, test_only: bool) -> int:
        x = rect.x()
        y = rect.y()
        line_height = 0

        for item in self.items:
            w: QWidget = item.widget()
            space_x = self.horizontalSpacing()
            if space_x == -1:
                space_x = w.style().layoutSpacing(
                    QSizePolicy.PushButton, QSizePolicy.PushButton, Qt.Horizontal
                )
            space_y = self.verticalSpacing()
            if space_y == -1:
                space_y = w.style().layoutSpacing(
                    QSizePolicy.PushButton, QSizePolicy.PushButton, Qt.Vertical
                )
            next_x = x + item.sizeHint().width() + space_x
            if next_x - space_x > rect.right() and line_height > 0:
                x = rect.x()
                y = y + line_height + space_y
                next_x = x + item.sizeHint().width() + space_x
                line_height = 0
            if not test_only:
                item.setGeometry(QRect(QPoint(x, y), item.sizeHint()))
            x = next_x
            line_height = max(line_height, item.sizeHint().height())
        return y + line_height - rect.x()

    def _smart_spacing(self, pixel_metric: QStyle.PixelMetric) -> int:
        parent = cast(Union[QLayout, QWidget, None], self.parent())
        if not parent:
            return -1
        elif parent.isWidgetType():
            assert isinstance(parent, QWidget)
            # FIXME: what does the `0` mean here ?
            # The second argument is `QStyleOption | None`
            return parent.style().pixelMetric(pixel_metric, cast(Any, 0), parent)
        else:
            assert isinstance(parent, QLayout)
            return parent.spacing()
