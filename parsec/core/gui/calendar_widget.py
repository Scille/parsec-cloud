# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from PyQt5.QtCore import Qt
from PyQt5.QtGui import QColor, QBrush, QFont
from PyQt5.QtWidgets import QCalendarWidget


class CalendarWidget(QCalendarWidget):
    def __init__(self, parent=None):
        super().__init__(parent)

        self.setDateEditEnabled(True)
        self.setNavigationBarVisible(True)
        self.setHorizontalHeaderFormat(QCalendarWidget.ShortDayNames)
        self.setVerticalHeaderFormat(QCalendarWidget.NoVerticalHeader)
        self.setFirstDayOfWeek(Qt.Monday)

        self.normal_background = QColor(0xFFFFFF)
        self.normal_foreground = QColor(0x000000)
        self.invalid_background = QColor(0xFFFFFF)
        self.invalid_foreground = QColor(0x999999)
        self.highlighted_background = QColor(0xFFFFFF)
        self.highlighted_foreground = QColor(0x0092FF)

        cell_text_format = self.headerTextFormat()
        cell_text_format.setFontPointSize(10)
        cell_text_format.setBackground(QBrush(Qt.white))
        cell_text_format.setForeground(self.palette().windowText())

        header_text_format = cell_text_format
        header_text_format.setFontWeight(QFont.Bold)

        self.setHeaderTextFormat(header_text_format)
        self.setWeekdayTextFormat(Qt.Monday, cell_text_format)
        self.setWeekdayTextFormat(Qt.Tuesday, cell_text_format)
        self.setWeekdayTextFormat(Qt.Wednesday, cell_text_format)
        self.setWeekdayTextFormat(Qt.Thursday, cell_text_format)
        self.setWeekdayTextFormat(Qt.Friday, cell_text_format)
        self.setWeekdayTextFormat(Qt.Saturday, cell_text_format)
        self.setWeekdayTextFormat(Qt.Sunday, cell_text_format)

    def paintCell(self, painter, rect, date):
        painter.save()

        is_invalid_date = (
            date < self.minimumDate()
            or date > self.maximumDate()
            or date.month() != self.monthShown()
            or date.year() != self.yearShown()
        )
        is_highlighted_date = date == self.selectedDate()

        background_color = (
            self.invalid_background
            if is_invalid_date
            else (self.highlighted_background if is_highlighted_date else self.normal_background)
        )
        foreground_color = (
            self.invalid_foreground
            if is_invalid_date
            else (self.highlighted_foreground if is_highlighted_date else self.normal_foreground)
        )

        painter.fillRect(rect, background_color)

        painter.setPen(foreground_color)
        painter.drawText(rect, Qt.AlignCenter, str(date.day()))
        painter.restore()
