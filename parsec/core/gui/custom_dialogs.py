# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import sys
import time
import multiprocessing
from contextlib import contextmanager
from concurrent.futures import ThreadPoolExecutor

from PyQt5.QtCore import Qt, pyqtSignal, QEventLoop
from PyQt5.QtGui import QPainter, QValidator, QIcon, QPixmap, QTextDocument
from PyQt5.QtPrintSupport import QPrinter, QPrintDialog
from PyQt5.QtWidgets import (
    QWidget,
    QCompleter,
    QLineEdit,
    QDialog,
    QApplication,
    QStyleOption,
    QStyle,
    QSizePolicy,
    QFileDialog,
)

from structlog import get_logger

from parsec import _subprocess_dialog
from parsec.core.gui.lang import translate as _
from parsec.core.gui import desktop
from parsec.core.gui.custom_widgets import Button
from parsec.core.gui.parsec_application import ParsecApp
from parsec.core.gui.snackbar_widget import SnackbarManager

from parsec.core.gui.ui.error_widget import Ui_ErrorWidget
from parsec.core.gui.ui.info_widget import Ui_InfoWidget
from parsec.core.gui.ui.question_widget import Ui_QuestionWidget
from parsec.core.gui.ui.input_widget import Ui_InputWidget
from parsec.core.gui.ui.greyed_dialog import Ui_GreyedDialog


logger = get_logger()

# Qt classes used in the subprocesses dedicated to dialogs
# They're not used directly but imported in order to fail early if one of those is not available
# See https://github.com/Scille/parsec-cloud/issues/2161 for a related issue.
qt_classes_in_subprocess = (
    QIcon,
    QPixmap,
    QPrinter,
    QPrintDialog,
    QTextDocument,
    QApplication,
    QEventLoop,
)


@contextmanager
def bring_process_window_to_top(pool, timeout=3.0):
    # This feature is windows only
    if sys.platform != "win32":
        yield
        return

    def _bring_to_top(pid_target):
        import ctypes
        from ctypes import wintypes as win

        # Prepare ctypes
        enum_windows = ctypes.windll.user32.EnumWindows
        get_window_thread_process_id = ctypes.windll.user32.GetWindowThreadProcessId
        set_foreground_window = ctypes.windll.user32.SetForegroundWindow
        callback_type = ctypes.WINFUNCTYPE(win.BOOL, win.HWND, win.LPARAM)

        # Callback enumerating the windows
        def enum_windows_callback(hwnd, param):
            nonlocal success
            # Get pid of the current window
            pid = win.DWORD()
            get_window_thread_process_id(hwnd, ctypes.byref(pid))
            # Keep enumerating
            if pid.value != pid_target:
                return True
            # Stop enumerating
            success = set_foreground_window(hwnd)
            return False

        # Run the callback in a loop
        success = False
        deadline = time.monotonic() + timeout
        callback = callback_type(enum_windows_callback)
        while not success:
            enum_windows(callback)
            if time.monotonic() > deadline:
                raise TimeoutError

    # Run the call in a thread
    pid = pool._pool[0].pid
    executor = ThreadPoolExecutor()
    try:
        future = executor.submit(_bring_to_top, pid)
        yield
    # Safely shutdown the executor without waiting
    finally:
        executor.shutdown(wait=False)
        if future.done():
            try:
                future.result()
            except TimeoutError:
                logger.warning("The call to `bring_to_top` timed out")
            except Exception:
                logger.exception("The call to `bring_to_top` failed unexpectedly")


class GreyedDialog(QDialog, Ui_GreyedDialog):
    closing = pyqtSignal()

    def __init__(
        self, center_widget, title, parent, hide_close=False, width=None, close_on_click=False
    ):
        super().__init__(None)
        self.setupUi(self)
        self.setModal(True)
        self.setObjectName("GreyedDialog")
        self.setWindowModality(Qt.ApplicationModal)
        self.button_close.apply_style()
        self.close_on_click = close_on_click
        if sys.platform == "win32":
            # SplashScreen on Windows freezes the Window
            self.setWindowFlags(Qt.FramelessWindowHint)
        else:
            # FramelessWindowHint on Linux (at least xfce) is less pretty
            self.setWindowFlags(Qt.SplashScreen)
        self.setAttribute(Qt.WA_TranslucentBackground)
        self.center_widget = center_widget
        self.main_layout.addWidget(center_widget)
        if not title and hide_close:
            self.widget_title.hide()
        if title:
            self.label_title.setText(title)
        if hide_close:
            self.button_close.hide()
        main_win = ParsecApp.get_main_window()
        if width:
            if width < main_win.size().width():
                spacing = int((main_win.size().width() - width) / 2)
                self._get_spacer_right().changeSize(
                    spacing, 0, QSizePolicy.Preferred, QSizePolicy.Preferred
                )
                self._get_spacer_left().changeSize(
                    spacing, 0, QSizePolicy.Preferred, QSizePolicy.Preferred
                )
        if main_win:
            if main_win.isVisible():
                self.setParent(main_win)
                self.resize(main_win.size())
            else:
                self.showMaximized()
            self.move(0, 0)
        elif parent is not None:
            logger.error("GreyedDialog did not find the main window, this is probably a bug")
        self.setFocus()
        self.accepted.connect(self.on_finished)
        self.rejected.connect(self.on_finished)

    def _get_spacer_top(self):
        return self.vertical_layout.itemAt(0).spacerItem()

    def _get_spacer_bottom(self):
        return self.vertical_layout.itemAt(2).spacerItem()

    def _get_spacer_left(self):
        return self.horizontal_layout.itemAt(0).spacerItem()

    def _get_spacer_right(self):
        return self.horizontal_layout.itemAt(2).spacerItem()

    def paintEvent(self, event):
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)

    def mousePressEvent(self, event):
        super().mousePressEvent(event)
        if not self.close_on_click:
            return
        if event.button() == Qt.LeftButton:
            self.accept()

    def on_finished(self):
        if (
            self.result() == QDialog.Rejected
            and self.center_widget
            and getattr(self.center_widget, "on_close", None)
        ):
            getattr(self.center_widget, "on_close")()
        self.closing.emit()
        # On Windows, GreyedDialogs don't get cleared out if their parent
        # is not set to None. Linux seems to clear them automatically over time.
        # Resetting the parent on MacOS causes a crash.
        if sys.platform != "darwin":
            self.setParent(None)


class QDialogInProcess(GreyedDialog):

    pools = {}
    process_finished = pyqtSignal()

    class DialogCancelled(Exception):
        pass

    def __init__(self, parent=None):
        super().__init__(QWidget(), "", parent, hide_close=True)
        self.MainWidget.hide()

    @classmethod
    @contextmanager
    def manage_pools(cls):
        # No pool for you OSX
        if sys.platform == "darwin":
            yield
            return
        with multiprocessing.Pool(processes=1) as dialog_pool:
            with multiprocessing.Pool(processes=1) as printer_pool:
                cls.pools[QFileDialog] = dialog_pool
                dialog_pool.apply_async(_subprocess_dialog.load_resources, ())
                cls.pools[_subprocess_dialog.PrintHelper] = printer_pool
                printer_pool.apply_async(
                    _subprocess_dialog.load_resources, (), {"with_printer": True}
                )
                yield

    @classmethod
    def _exec_method(cls, target_cls, target_method, parent, *args, **kwargs):
        assert hasattr(target_cls, target_method)
        # Fire up the process pool is necessary
        pool = cls.pools.get(target_cls)
        if pool is None:
            cls.pools[target_cls] = pool = multiprocessing.Pool(processes=1)
        # Control the lifetime of the pool
        try:
            # Show the dialog
            dialog = cls(parent=parent)
            dialog.process_finished.connect(dialog.close)
            dialog.show()
            # Start the subprocess
            async_result = pool.apply_async(
                _subprocess_dialog.run_dialog,
                (target_cls, target_method, *args),
                kwargs,
                lambda *args: dialog.process_finished.emit(),
                lambda *args: dialog.process_finished.emit(),
            )
            # Run the dialog until either the process is finished or the dialog is closed
            with bring_process_window_to_top(pool):
                dialog.exec_()
            # Terminate the process and close the pool if the dialog exited first
            if not async_result.ready():
                pool.terminate()
                cls.pools.pop(target_cls, None)
                raise dialog.DialogCancelled
            # Return result
            return async_result.get()
        finally:
            # On OSX, using the same process for several dialogs messes up with the OS
            # In particular, the parsec icon corresponding to the dialog window persists between calls
            if sys.platform == "darwin":
                pool.close()
                cls.pools.pop(target_cls, None)

    @classmethod
    def getOpenFileName(cls, *args, **kwargs):
        try:
            return cls._exec_method(QFileDialog, "getOpenFileName", *args, **kwargs)
        except cls.DialogCancelled:
            return "", ""

    @classmethod
    def getOpenFileNames(cls, *args, **kwargs):
        try:
            return cls._exec_method(QFileDialog, "getOpenFileNames", *args, **kwargs)
        except cls.DialogCancelled:
            return [], ""

    @classmethod
    def getExistingDirectory(cls, *args, **kwargs):
        try:
            return cls._exec_method(QFileDialog, "getExistingDirectory", *args, **kwargs)
        except cls.DialogCancelled:
            return ""

    @classmethod
    def getSaveFileName(cls, *args, **kwargs):
        try:
            return cls._exec_method(QFileDialog, "getSaveFileName", *args, **kwargs)
        except cls.DialogCancelled:
            return "", ""

    @classmethod
    def print_html(cls, *args, **kwargs):
        try:
            return cls._exec_method(_subprocess_dialog.PrintHelper, "print_html", *args, **kwargs)
        except cls.DialogCancelled:
            return None


class TextInputWidget(QWidget, Ui_InputWidget):
    def __init__(
        self,
        message,
        placeholder="",
        default_text="",
        completion=None,
        button_text=None,
        validator=None,
        hidden=False,
    ):
        super().__init__()
        self.setupUi(self)
        self.dialog = None
        button_text = button_text or _("ACTION_OK")
        self.button_ok.setText(button_text)
        self.label_message.setText(message)
        self.line_edit_text.setPlaceholderText(placeholder)
        self.line_edit_text.setText(default_text)
        self.line_edit_text.set_validator(validator)
        if hidden:
            self.line_edit_text.setEchoMode(QLineEdit.Password)
        self.line_edit_text.validity_changed.connect(self._on_validity_changed)
        self.button_ok.setEnabled(self.line_edit_text.is_input_valid())
        if completion:
            completer = QCompleter(completion)
            completer.setCaseSensitivity(Qt.CaseInsensitive)
            completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
            self.line_edit_text.setCompleter(completer)
        self.button_ok.clicked.connect(self._on_button_clicked)
        self.setFocus()
        self.line_edit_text.setFocus()

    @property
    def text(self):
        return self.line_edit_text.text()

    def _on_validity_changed(self, validity):
        self.button_ok.setEnabled(validity == QValidator.Acceptable)

    def keyPressEvent(self, event):
        if self.button_ok.isEnabled() and event.key() in (Qt.Key_Return, Qt.Key_Enter):
            self._on_button_clicked()
        event.accept()

    def _on_button_clicked(self):
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            QApplication.activeModalWidget().accept()
        else:
            logger.warning("Cannot close dialog when requesting user text input")


def get_text_input(
    parent,
    title,
    message,
    placeholder="",
    default_text="",
    completion=None,
    button_text=None,
    validator=None,
    hidden=False,
):
    w = TextInputWidget(
        message=message,
        placeholder=placeholder,
        default_text=default_text,
        completion=completion,
        button_text=button_text,
        validator=validator,
        hidden=hidden,
    )
    d = GreyedDialog(w, title=title, parent=parent)
    w.dialog = d
    w.line_edit_text.setFocus()
    result = d.exec_()
    if result == QDialog.Accepted:
        return w.text
    return None


class QuestionWidget(QWidget, Ui_QuestionWidget):
    def __init__(
        self, message, button_texts, radio_mode=False, oriented_question=False, dangerous_yes=False
    ):
        super().__init__()
        self.setupUi(self)
        self.status = None
        self.dialog = None
        self.label_message.setText(message)

        if oriented_question:
            yes_text, no_text = button_texts
            assert not radio_mode

            # Add "no" button
            b = Button(no_text)
            b.clicked_self.connect(self._on_button_clicked)
            b.setCursor(Qt.PointingHandCursor)
            b.setStyleSheet(
                "QPushButton {background-color: darkgrey;} QPushButton:hover {background-color: grey;}"
            )
            self.layout_buttons.addWidget(b)

            # Add "yes" button
            b = Button(yes_text)
            b.clicked_self.connect(self._on_button_clicked)
            b.setCursor(Qt.PointingHandCursor)
            if dangerous_yes:
                b.setStyleSheet(
                    "QPushButton {background-color: red;} QPushButton:hover {background-color: darkred;}"
                )
            self.layout_buttons.addWidget(b)

        else:
            assert not dangerous_yes
            for text in button_texts:
                b = Button(text)
                b.clicked_self.connect(self._on_button_clicked)
                b.setCursor(Qt.PointingHandCursor)
                if radio_mode:
                    self.layout_radios.addWidget(b)
                else:
                    self.layout_buttons.insertWidget(1, b)

    def _on_button_clicked(self, button):
        self.status = button.text()
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            QApplication.activeModalWidget().accept()
        else:
            logger.warning("Cannot close dialog when asking question")


def ask_question(
    parent,
    title,
    message,
    button_texts,
    radio_mode=False,
    oriented_question=False,
    dangerous_yes=False,
):
    w = QuestionWidget(
        message=message,
        button_texts=button_texts,
        radio_mode=radio_mode,
        oriented_question=oriented_question,
        dangerous_yes=dangerous_yes,
    )
    d = GreyedDialog(w, title=title, parent=parent)
    w.dialog = d
    status = d.exec_()
    if status == QDialog.Accepted:
        return w.status
    return None


class ErrorWidget(QWidget, Ui_ErrorWidget):
    def __init__(self, message, exception=None):
        super().__init__()
        self.setupUi(self)
        self.label_message.setText(message)
        self.label_message.setOpenExternalLinks(True)
        self.label_icon.apply_style()
        self.text_details.hide()
        if not exception:
            self.button_details.hide()
        else:
            import traceback

            stack = traceback.format_exception(
                None, exception, getattr(exception, "__traceback__", None)
            )
            if not stack:
                self.button_details.hide()
            else:
                self.text_details.insertPlainText(type(exception).__name__ + "\n\n")
                self.text_details.insertPlainText("".join(stack))
                cursor = self.text_details.textCursor()
                cursor.setPosition(0)
                self.text_details.setTextCursor(cursor)
        self.button_details.clicked.connect(self.toggle_details)
        self.button_details.apply_style()
        self.button_copy.clicked.connect(self.copy_to_clipboard)
        self.button_copy.hide()
        self.button_copy.apply_style()

    def copy_to_clipboard(self):
        desktop.copy_to_clipboard(self.text_details.toPlainText())
        SnackbarManager.inform(_("TEXT_STACKTRACE_COPIED_TO_CLIPBOARD"))

    def toggle_details(self, checked):
        if not checked:
            self.text_details.hide()
            self.button_copy.hide()
        else:
            self.text_details.show()
            self.button_copy.show()


def show_error(parent, message, exception=None):
    w = ErrorWidget(message, exception)
    d = GreyedDialog(w, title=_("TEXT_ERR_DIALOG_TITLE"), parent=parent)
    return d.open()


class InfoWidget(QWidget, Ui_InfoWidget):
    def __init__(self, message, dialog=None, button_text=None):
        super().__init__()
        self.setupUi(self)
        self.dialog = dialog
        self.label_message.setText(message)
        self.label_icon.apply_style()
        self.button_ok.setText(button_text or _("ACTION_CONTINUE"))
        self.button_ok.clicked.connect(self._on_button_clicked)
        self.button_ok.setFocus()

    def _on_button_clicked(self, button):
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            QApplication.activeModalWidget().accept()
        else:
            logger.warning("Cannot close dialog when displaying info")


def show_info(parent, message, button_text=None):
    w = InfoWidget(message, button_text=button_text)
    d = GreyedDialog(w, title=None, parent=parent, hide_close=True)
    w.dialog = d
    w.button_ok.setFocus()
    return d.open()


class InfoLinkWidget(QWidget, Ui_InfoWidget):
    def __init__(self, message, url, button_text, dialog=None):
        super().__init__()
        self.setupUi(self)
        self.dialog = dialog
        self.url = url
        self.label_message.setText(message)
        self.label_icon.apply_style()
        self.label_message.setOpenExternalLinks(True)
        self.button_ok.setText(button_text)
        self.button_ok.clicked.connect(self._on_button_clicked)
        self.button_ok.setFocus()

    def _on_button_clicked(self, button):
        desktop.open_url(self.url)


def show_info_link(parent, title, message, button_text, url):
    w = InfoLinkWidget(message, url, button_text)
    d = GreyedDialog(w, title=title, parent=parent)
    w.dialog = d
    w.button_ok.setFocus()
    return d.open()


class InfoCopyLinkWidget(InfoLinkWidget):
    def _on_button_clicked(self, button):
        desktop.copy_to_clipboard(self.url)


def show_info_copy_link(parent, title, message, button_text, url):
    w = InfoCopyLinkWidget(message, url, button_text)
    d = GreyedDialog(w, title=title, parent=parent)
    w.dialog = d
    w.button_ok.setFocus()
    return d.open()
