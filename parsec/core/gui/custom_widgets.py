# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pathlib

from PyQt5.QtCore import Qt, QTimer, pyqtSignal, QSize, QPoint
from PyQt5.QtGui import QPixmap, QIcon, QPainter, QColor, QPen
from PyQt5.QtWidgets import (
    QFileDialog,
    QAbstractItemView,
    QListView,
    QTreeView,
    QDialog,
    QCompleter,
    QLineEdit,
    QPushButton,
    QLabel,
)

from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.message_dialog import Ui_MessageDialog
from parsec.core.gui.ui.input_dialog import Ui_InputDialog
from parsec.core.gui.ui.question_dialog import Ui_QuestionDialog


class TextInputDialog(QDialog, Ui_InputDialog):
    def __init__(
        self, title, message, placeholder="", default_text="", completion=None, *args, **kwargs
    ):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.line_edit_text.setPlaceholderText(placeholder)
        self.setWindowFlags(Qt.SplashScreen)
        self.line_edit_text.setFocus()
        self.line_edit_text.setText(default_text)
        if completion:
            completer = QCompleter(completion)
            completer.setCaseSensitivity(Qt.CaseInsensitive)
            completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
            self.line_edit_text.setCompleter(completer)

    @property
    def text(self):
        return self.line_edit_text.text()


def get_text(parent, title, message, placeholder="", default_text="", completion=None):

    m = TextInputDialog(
        title=title,
        message=message,
        placeholder=placeholder,
        parent=parent,
        default_text=default_text,
        completion=completion,
    )
    status = m.exec_()
    if status == QDialog.Accepted:
        return m.text
    return None


# TODO: If this ever gets used again, it needs to transition to the new job system
class UserInputDialog(QDialog, Ui_InputDialog):
    def __init__(self, portal, core, title, message, exclude=None, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.portal = portal
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.line_edit_text.setPlaceholderText(_("User name"))
        self.setWindowFlags(Qt.SplashScreen)
        self.exclude = exclude or []
        self.timer = QTimer()
        self.timer.timeout.connect(self.show_auto_complete)
        self.line_edit_text.setFocus()
        self.line_edit_text.textChanged.connect(self.text_changed)

    def text_changed(self, text):
        # In order to avoid a segfault by making too many requests,
        # we wait a little bit after the user has stopped pressing keys
        # to make the query.
        if len(text):
            self.timer.start(500)

    def show_auto_complete(self):
        self.timer.stop()
        if len(self.line_edit_text.text()):
            users = self.portal.run(self.core.backend_cmds.user_find, self.line_edit_text.text())
            if self.exclude:
                users = [u for u in users if u not in self.exclude]
            completer = QCompleter(users)
            completer.setCaseSensitivity(Qt.CaseInsensitive)
            completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
            self.line_edit_text.setCompleter(completer)
            self.line_edit_text.completer().complete()

    @property
    def text(self):
        return self.line_edit_text.text()


def get_user_name(parent, portal, core, title, message, exclude=None):
    m = UserInputDialog(
        core=core, portal=portal, title=title, message=message, exclude=exclude, parent=parent
    )
    status = m.exec_()
    if status == QDialog.Accepted:
        return m.text
    return None


class QuestionDialog(QDialog, Ui_QuestionDialog):
    def __init__(self, title, message, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.setWindowFlags(Qt.SplashScreen)


def ask_question(parent, title, message):
    m = QuestionDialog(title=title, message=message, parent=parent)
    status = m.exec_()
    if status == QDialog.Accepted:
        return True
    return False


class MessageDialog(QDialog, Ui_MessageDialog):
    def __init__(self, icon, title, message, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.label_icon.setPixmap(icon)
        self.setWindowFlags(Qt.SplashScreen)


def show_info(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/info.png"), _("Information"), text, parent=parent
    )
    return m.exec_()


def show_warning(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/warning.png"), _("Warning"), text, parent=parent
    )
    return m.exec_()


def show_error(parent, text):
    m = MessageDialog(QPixmap(":/icons/images/icons/error.png"), _("Error"), text, parent=parent)
    return m.exec_()


def get_open_files(parent):
    class FileDialog(QFileDialog):
        def __init__(self, parent):
            super().__init__(
                parent=parent, caption="Select files", directory=str(pathlib.Path.home())
            )
            self.setFileMode(QFileDialog.AnyFile)
            self.setOption(QFileDialog.DontUseNativeDialog, True)
            list_view = self.findChild(QListView, "listView")
            if list_view:
                list_view.setSelectionMode(QAbstractItemView.MultiSelection)
            tree_view = self.findChild(QTreeView)
            if tree_view:
                tree_view.setSelectionMode(QAbstractItemView.MultiSelection)

    file_dialog = FileDialog(parent=parent)
    result = file_dialog.exec_()
    return bool(result), file_dialog.selectedFiles()


class FileLineEdit(QLineEdit):
    clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.full_text = ""

    def setText(self, text):
        self.full_text = text
        if len(text) > 30:
            text = text[:30] + "..."
        super().setText(text)

    def enterEvent(self, _):
        f = self.font()
        f.setUnderline(True)
        self.setFont(f)

    def leaveEvent(self, _):
        f = self.font()
        f.setUnderline(False)
        self.setFont(f)

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.full_text)


class TaskbarButton(QPushButton):
    def __init__(self, icon_path, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setMinimumSize(64, 64)
        self.setMaximumSize(64, 64)
        self.setFixedSize(64, 64)
        self.setIcon(QIcon(icon_path))
        self.setIconSize(QSize(50, 50))
        self.setStyleSheet("background-color: rgb(12, 65, 157); border: 0;")


class NotificationTaskbarButton(TaskbarButton):
    def __init__(self, *args, **kwargs):
        super().__init__(icon_path=":/icons/images/icons/notification_off.png", *args, **kwargs)
        self.notif_count = 0

    def inc_notif_count(self):
        self.notif_count += 1

    def reset_notif_count(self):
        self.notif_count = 0

    def setChecked(self, val):
        super().setChecked(val)
        if val:
            self.setIcon(QIcon(":/icons/images/icons/notification_on.png"))
            self.setIconSize(QSize(50, 50))
        else:
            self.setIcon(QIcon(":/icons/images/icons/notification_off.png"))
            self.setIconSize(QSize(50, 50))

    def paintEvent(self, event):
        super().paintEvent(event)
        if self.notif_count == 0:
            return
        text = str(self.notif_count)
        if self.notif_count >= 100:
            text = "99+"
        rect = event.rect()
        painter = QPainter(self)
        painter.setPen(QColor(220, 54, 66))
        painter.setBrush(QColor(220, 54, 66))
        painter.drawEllipse(rect.right() - 35, 0, 35, 35)
        painter.setPen(QColor(255, 255, 255))
        if len(text) == 1:
            painter.drawText(QPoint(rect.right() - 22, 23), text)
        elif len(text) == 2:
            painter.drawText(QPoint(rect.right() - 27, 23), text)
        elif len(text) == 3:
            painter.drawText(QPoint(rect.right() - 30, 23), text)
        painter.end()


class PageLabel(QLabel):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.page = 0
        self.max_page = 0

    def paintEvent(self, event):
        super().paintEvent(event)
        painter = QPainter(self)
        grey = QColor(215, 215, 215)
        blue = QColor(38, 142, 212)

        x = 0
        for p in range(self.max_page):
            if p == self.page:
                painter.setPen(blue)
                painter.setBrush(blue)
            else:
                painter.setPen(grey)
                painter.setBrush(grey)
            x = p * 16 + 22 * p
            painter.drawEllipse(x, 20, 16, 16)
        painter.end()


class DeviceLabel(QLabel):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.is_revoked = False

    def paintEvent(self, event):
        super().paintEvent(event)
        if not self.is_revoked:
            return
        rect = event.rect()
        painter = QPainter(self)
        painter.setRenderHints(QPainter.HighQualityAntialiasing)
        pen = QPen(QColor(218, 53, 69))
        pen.setWidth(5)
        pen.setJoinStyle(Qt.RoundJoin)
        pen.setCapStyle(Qt.RoundCap)
        painter.setPen(pen)
        painter.drawEllipse(rect.right() - 53, 3, 50, 53)
        painter.drawLine(rect.right() - 44, 44, rect.right() - 12, 12)
        painter.end()
