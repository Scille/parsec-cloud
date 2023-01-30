# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

"""
Switch button implementation in PyQt5

Taken from:
- https://stackoverflow.com/a/51825815/2846140
"""

from PyQt5.QtCore import QEvent, QPropertyAnimation, QRectF, QSize, Qt, pyqtProperty
from PyQt5.QtGui import QMouseEvent, QPainter, QPaintEvent, QResizeEvent
from PyQt5.QtWidgets import QAbstractButton, QSizePolicy, QWidget


class SwitchButton(QAbstractButton):
    def __init__(
        self, parent: QWidget | None = None, track_radius: int = 10, thumb_radius: int = 8
    ):
        super().__init__(parent=parent)
        self.setCheckable(True)
        self.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

        self._track_radius: int = track_radius
        self._thumb_radius: int = thumb_radius

        self._margin: int = max(0, self._thumb_radius - self._track_radius)
        self._base_offset: int = max(self._thumb_radius, self._track_radius)
        self._end_offset = {
            True: lambda: self.width() - self._base_offset,
            False: lambda: self._base_offset,
        }
        self._offset: int = self._base_offset

        palette = self.palette()
        if self._thumb_radius > self._track_radius:
            self._track_color = {True: palette.highlight(), False: palette.dark()}
            self._thumb_color = {True: palette.highlight(), False: palette.light()}
            self._text_color = {
                True: palette.highlightedText().color(),
                False: palette.dark().color(),
            }
            self._thumb_text = {True: "", False: ""}
            self._track_opacity = 0.5
        else:
            self._thumb_color = {True: palette.highlightedText(), False: palette.light()}
            self._track_color = {True: palette.highlight(), False: palette.dark()}
            self._text_color = {True: palette.highlight().color(), False: palette.dark().color()}
            self._thumb_text = {True: "✔", False: "✕"}
            self._track_opacity = 1

    def getOffset(self) -> int:
        return self._offset

    def setOffset(self, value: int) -> None:
        self._offset = value
        self.update()

    offset = pyqtProperty(int, fget=getOffset, fset=setOffset)

    def sizeHint(self) -> QSize:
        return QSize(
            4 * self._track_radius + 2 * self._margin, 2 * self._track_radius + 2 * self._margin
        )

    def setChecked(self, checked: bool) -> None:
        super().setChecked(checked)
        self.setOffset(self._end_offset[checked]())

    def resizeEvent(self, event: QResizeEvent) -> None:
        super().resizeEvent(event)
        self.setOffset(self._end_offset[self.isChecked()]())

    def paintEvent(self, event: QPaintEvent) -> None:
        p = QPainter(self)
        p.setRenderHint(QPainter.Antialiasing, True)
        p.setPen(Qt.NoPen)
        track_opacity = self._track_opacity
        thumb_opacity = 1.0
        text_opacity = 1.0
        if self.isEnabled():
            track_brush = self._track_color[self.isChecked()]
            thumb_brush = self._thumb_color[self.isChecked()]
            text_color = self._text_color[self.isChecked()]
        else:
            track_opacity *= 0.8
            track_brush = self.palette().shadow()
            thumb_brush = self.palette().mid()
            text_color = self.palette().shadow().color()

        p.setBrush(track_brush)
        p.setOpacity(track_opacity)
        p.drawRoundedRect(
            self._margin,
            self._margin,
            self.width() - 2 * self._margin,
            self.height() - 2 * self._margin,
            self._track_radius,
            self._track_radius,
        )
        p.setBrush(thumb_brush)
        p.setOpacity(thumb_opacity)
        p.drawEllipse(
            self.getOffset() - self._thumb_radius,
            self._base_offset - self._thumb_radius,
            2 * self._thumb_radius,
            2 * self._thumb_radius,
        )
        p.setPen(text_color)
        p.setOpacity(text_opacity)
        font = p.font()
        font.setPixelSize(int(round(1.5 * self._thumb_radius)))
        p.setFont(font)
        p.drawText(
            QRectF(
                self.getOffset() - self._thumb_radius,
                self._base_offset - self._thumb_radius,
                2 * self._thumb_radius,
                2 * self._thumb_radius,
            ),
            Qt.AlignCenter,
            self._thumb_text[self.isChecked()],
        )

    def mouseReleaseEvent(self, event: QMouseEvent) -> None:
        super().mouseReleaseEvent(event)
        if event.button() == Qt.LeftButton:
            anim = QPropertyAnimation(self, b"offset", self)
            anim.setDuration(120)
            anim.setStartValue(self.offset)
            anim.setEndValue(self._end_offset[self.isChecked()]())
            anim.start()

    def enterEvent(self, event: QEvent) -> None:
        self.setCursor(Qt.PointingHandCursor)
        super().enterEvent(event)
