import pathlib

from PyQt5.QtCore import QCoreApplication, Qt, QTimer, pyqtSignal, QSize, QPoint
from PyQt5.QtGui import QPixmap, QIcon, QPainter, QColor
from PyQt5.QtWidgets import (
    QFileDialog,
    QAbstractItemView,
    QListView,
    QTreeView,
    QDialog,
    QCompleter,
    QLineEdit,
    QPushButton,
)

from parsec.core.gui.ui.message_dialog import Ui_MessageDialog
from parsec.core.gui.ui.input_dialog import Ui_InputDialog
from parsec.core.gui.ui.question_dialog import Ui_QuestionDialog


def get_text(parent, title, message, placeholder="", default_text="", completion=None):
    class InputDialog(QDialog, Ui_InputDialog):
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

    m = InputDialog(
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


def get_user_name(parent, portal, core, title, message, exclude=None):
    class InputDialog(QDialog, Ui_InputDialog):
        def __init__(self, portal, core, title, message, exclude=None, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self.setupUi(self)
            self.core = core
            self.portal = portal
            self.label_title.setText(title)
            self.label_message.setText(message)
            self.line_edit_text.setPlaceholderText(
                QCoreApplication.translate("GetUserName", "User name")
            )
            self.setWindowFlags(Qt.SplashScreen)
            self.exclude = exclude or []
            self.timer = QTimer()
            self.timer.timeout.connect(self.show_auto_complete)
            self.line_edit_text.setFocus()
            self.line_edit_text.textChanged.connect(self.text_changed)

        def text_changed(self, text):
            # In order to avoid a segfault by making to many requests,
            # we wait a little bit after the user has stopped pressing keys
            # to make the query.
            if len(text):
                self.timer.start(500)

        def show_auto_complete(self):
            self.timer.stop()
            if len(self.line_edit_text.text()):
                users = self.portal.run(
                    self.core.fs.backend_cmds.user_find, self.line_edit_text.text()
                )
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

    m = InputDialog(
        core=core, portal=portal, title=title, message=message, exclude=exclude, parent=parent
    )
    status = m.exec_()
    if status == QDialog.Accepted:
        return m.text
    return None


def ask_question(parent, title, message):
    class QuestionDialog(QDialog, Ui_QuestionDialog):
        def __init__(self, title, message, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self.setupUi(self)
            self.label_title.setText(title)
            self.label_message.setText(message)
            self.setWindowFlags(Qt.SplashScreen)

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
        QPixmap(":/icons/images/icons/info.png"),
        QCoreApplication.translate("Message", "Information"),
        text,
        parent=parent,
    )
    return m.exec_()


def show_warning(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/warning.png"),
        QCoreApplication.translate("Message", "Warning"),
        text,
        parent=parent,
    )
    return m.exec_()


def show_error(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/error.png"),
        QCoreApplication.translate("Message", "Error"),
        text,
        parent=parent,
    )
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
    clicked = pyqtSignal(str, bool)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.is_dir = False
        self.text = ""

    def setText(self, text):
        self.text = text
        if len(text) > 30:
            text = text[:30] + "..."
        super().setText(text)

    def setIsDir(self, val):
        self.is_dir = val

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
            self.clicked.emit(self.text, self.is_dir)


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
        super().__init__(icon_path=":/icons/images/icons/menu_settings.png", *args, **kwargs)
        self.notif_count = 0

    def inc_notif_count(self):
        self.notif_count += 1

    def reset_notif_count(self):
        self.notif_count = 0

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
