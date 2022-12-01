# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import multiprocessing
import multiprocessing.pool
import sys
import time
from concurrent.futures import ThreadPoolExecutor
from contextlib import contextmanager
from typing import Callable, Collection, Iterator, TypeVar, cast

from PyQt5.QtCore import QEventLoop, Qt, pyqtSignal
from PyQt5.QtGui import (
    QIcon,
    QKeyEvent,
    QMouseEvent,
    QPainter,
    QPaintEvent,
    QPixmap,
    QTextDocument,
    QValidator,
)
from PyQt5.QtPrintSupport import QPrintDialog, QPrinter
from PyQt5.QtWidgets import (
    QApplication,
    QCompleter,
    QDialog,
    QFileDialog,
    QLineEdit,
    QSizePolicy,
    QSpacerItem,
    QStyle,
    QStyleOption,
    QWidget,
)
from structlog import get_logger
from typing_extensions import Concatenate, ParamSpec

from parsec import _subprocess_dialog
from parsec.core.gui import desktop
from parsec.core.gui.custom_widgets import Button
from parsec.core.gui.lang import translate as _
from parsec.core.gui.parsec_application import ParsecApp
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.ui.error_widget import Ui_ErrorWidget
from parsec.core.gui.ui.greyed_dialog import Ui_GreyedDialog
from parsec.core.gui.ui.info_widget import Ui_InfoWidget
from parsec.core.gui.ui.input_widget import Ui_InputWidget
from parsec.core.gui.ui.question_widget import Ui_QuestionWidget


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

P = ParamSpec("P")
R = TypeVar("R")
T_CLASS = TypeVar("T_CLASS")


@contextmanager
def bring_process_window_to_top(
    pool: multiprocessing.pool.Pool, timeout: float = 3.0
) -> Iterator[None]:
    # This feature is windows only
    if sys.platform != "win32":
        yield
        return

    def _bring_to_top(pid_target: int) -> bool:
        import ctypes
        from ctypes import wintypes as win

        # Prepare ctypes
        enum_windows = ctypes.windll.user32.EnumWindows
        get_window_thread_process_id = ctypes.windll.user32.GetWindowThreadProcessId
        set_foreground_window = ctypes.windll.user32.SetForegroundWindow
        callback_type = ctypes.WINFUNCTYPE(win.BOOL, win.HWND, win.LPARAM)

        # Does the callback finished successfully?
        success = False

        # Callback enumerating the windows
        def enum_windows_callback(hwnd: win.HWND, param: object) -> bool:
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
        self,
        center_widget: QWidget,
        title: str | None,
        parent: QWidget | None,
        hide_close: bool = False,
        width: int | None = None,
        close_on_click: bool = False,
        on_close_requested: Callable[[], bool] | None = None,
    ) -> None:
        super().__init__(None)
        self.setupUi(self)
        self.setModal(True)
        self.setObjectName("GreyedDialog")
        self.setWindowModality(Qt.ApplicationModal)
        self.button_close.apply_style()
        self.button_close.clicked.connect(self._on_close_clicked)
        self.on_close_requested = on_close_requested
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
        if width and main_win:
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

    def _on_close_clicked(self) -> None:
        if not self.on_close_requested or self.on_close_requested():
            self.reject()

    def _get_spacer_top(self) -> QSpacerItem:
        return self.vertical_layout.itemAt(0).spacerItem()

    def _get_spacer_bottom(self) -> QSpacerItem:
        return self.vertical_layout.itemAt(2).spacerItem()

    def _get_spacer_left(self) -> QSpacerItem:
        return self.horizontal_layout.itemAt(0).spacerItem()

    def _get_spacer_right(self) -> QSpacerItem:
        return self.horizontal_layout.itemAt(2).spacerItem()

    def paintEvent(self, event: QPaintEvent) -> None:
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)

    def mousePressEvent(self, event: QMouseEvent) -> None:
        super().mousePressEvent(event)
        if not self.close_on_click:
            return
        if event.button() == Qt.LeftButton:
            self.accept()

    def on_finished(self) -> None:
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
            # Mypy This is the correct way to remove a parent from a widget but PyQt miss that overload
            self.setParent(None)  # type: ignore[call-overload]


def generate_dialog_method(
    sub_cls: T_CLASS, sub_method: Callable[Concatenate[QWidget | None, P], R], default_return: R
) -> Callable[Concatenate[QDialogInProcess, QWidget | None, P], R]:
    def wrapper(
        cls: QDialogInProcess, parent: QWidget | None, *args: P.args, **kwargs: P.kwargs
    ) -> R:
        try:
            return cls._exec_method(sub_cls, sub_method, parent, *args, **kwargs)
        except cls.DialogCancelled:
            return default_return

    return wrapper


class QDialogInProcess(GreyedDialog):

    pools: dict[object, multiprocessing.pool.Pool] = {}
    process_finished = pyqtSignal()

    class DialogCancelled(Exception):
        pass

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(QWidget(), "", parent, hide_close=True)
        self.MainWidget.hide()

    @classmethod
    @contextmanager
    def manage_pools(cls) -> Iterator[None]:
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
    def _exec_method(
        cls,
        target_cls: T_CLASS,
        target_method: Callable[Concatenate[QWidget | None, P], R],
        parent: QWidget | None,
        *args: P.args,
        **kwargs: P.kwargs,
    ) -> R:
        assert hasattr(target_cls, target_method.__name__)
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
                (target_cls, target_method.__name__, *args),
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
            return cast(R, async_result.get())
        finally:
            # On OSX, using the same process for several dialogs messes up with the OS
            # In particular, the parsec icon corresponding to the dialog window persists between calls
            if sys.platform == "darwin":
                pool.close()
                cls.pools.pop(target_cls, None)

    getOpenFileName = classmethod(
        generate_dialog_method(QFileDialog, QFileDialog.getOpenFileName, ("", ""))
    )
    getOpenFileNames = classmethod(
        generate_dialog_method(QFileDialog, QFileDialog.getOpenFileNames, (cast(list[str], []), ""))
    )
    getExistingDirectory = classmethod(
        generate_dialog_method(QFileDialog, QFileDialog.getExistingDirectory, "")
    )
    getSaveFileName = classmethod(
        generate_dialog_method(QFileDialog, QFileDialog.getSaveFileName, ("", ""))
    )
    print_html = classmethod(
        generate_dialog_method(
            _subprocess_dialog.PrintHelper, _subprocess_dialog.PrintHelper.print_html, None
        )
    )


class TextInputWidget(QWidget, Ui_InputWidget):
    def __init__(
        self,
        message: str,
        placeholder: str = "",
        default_text: str = "",
        completion: None = None,
        button_text: str | None = None,
        validator: QValidator | None = None,
        hidden: bool = False,
        selection: tuple[int, int] | None = None,
    ):
        super().__init__()
        self.setupUi(self)
        self.dialog: GreyedDialog | None = None
        button_text = button_text or _("ACTION_OK")
        self.button_ok.setText(button_text)
        self.label_message.setText(message)
        self.line_edit_text.setPlaceholderText(placeholder)
        self.line_edit_text.setText(default_text)
        self.line_edit_text.set_validator(validator)
        if hidden:
            self.line_edit_text.setEchoMode(QLineEdit.Password)
        if selection:
            self.line_edit_text.setSelection(selection[0], selection[1])
        self.line_edit_text.validity_changed.connect(self._on_validity_changed)
        self.button_ok.setEnabled(self.line_edit_text.is_input_valid())
        if completion:
            completer = QCompleter(completion)
            completer.setCaseSensitivity(Qt.CaseInsensitive)
            completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
            self.line_edit_text.setCompleter(completer)
        self.button_ok.clicked.connect(self._on_button_clicked)
        if not selection:
            self.line_edit_text.setCursorPosition(0)
        self.line_edit_text.setFocus()

    @property
    def text(self) -> str:
        return self.line_edit_text.text()

    def _on_validity_changed(self, validity: QValidator.State) -> None:
        self.button_ok.setEnabled(validity == QValidator.Acceptable)

    def keyPressEvent(self, event: QKeyEvent) -> None:
        if self.button_ok.isEnabled() and event.key() in (Qt.Key_Return, Qt.Key_Enter):
            self._on_button_clicked()
        event.accept()

    def _on_button_clicked(self) -> None:
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            # Mypy: `activeModalWidget` return only a `QWidget` that don't define an `accept` method
            # So finger-crossed that the return object has it.
            QApplication.activeModalWidget().accept()  # type: ignore[attr-defined]
        else:
            logger.warning("Cannot close dialog when requesting user text input")


def get_text_input(
    parent: QWidget,
    title: str,
    message: str,
    placeholder: str = "",
    default_text: str = "",
    completion: None = None,
    button_text: str | None = None,
    validator: QValidator | None = None,
    hidden: bool = False,
    selection: tuple[int, int] | None = None,
) -> str | None:
    widget = TextInputWidget(
        message=message,
        placeholder=placeholder,
        default_text=default_text,
        completion=completion,
        button_text=button_text,
        validator=validator,
        hidden=hidden,
        selection=selection,
    )
    dialog = GreyedDialog(widget, title=title, parent=parent)
    widget.dialog = dialog
    widget.line_edit_text.setFocus()
    result = dialog.exec_()
    if result == QDialog.Accepted:
        return widget.text
    return None


class QuestionWidget(QWidget, Ui_QuestionWidget):
    def __init__(
        self,
        message: str,
        button_texts: Collection[str],
        radio_mode: bool = False,
        oriented_question: bool = False,
        dangerous_yes: bool = False,
    ):
        super().__init__()
        self.setupUi(self)
        self.status: None | str = None
        self.dialog: None | GreyedDialog = None
        self.label_message.setText(message)

        if oriented_question:
            assert len(button_texts) == 2
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

    def _on_button_clicked(self, button: Button) -> None:
        self.status = button.text()
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            # Mypy: `activeModalWidget` return only a `QWidget` that don't define an `accept` method
            # So finger-crossed that the return object has it.
            QApplication.activeModalWidget().accept()  # type: ignore[attr-defined]
        else:
            logger.warning("Cannot close dialog when asking question")


def ask_question(
    parent: QWidget | None,
    title: str,
    message: str,
    button_texts: Collection[str],
    radio_mode: bool = False,
    oriented_question: bool = False,
    dangerous_yes: bool = False,
) -> str | None:
    widget = QuestionWidget(
        message=message,
        button_texts=button_texts,
        radio_mode=radio_mode,
        oriented_question=oriented_question,
        dangerous_yes=dangerous_yes,
    )
    dialog = GreyedDialog(widget, title=title, parent=parent)
    widget.dialog = dialog
    status = dialog.exec_()
    if status == QDialog.Accepted:
        return widget.status
    return None


class ErrorWidget(QWidget, Ui_ErrorWidget):
    def __init__(self, message: str, exception: BaseException | None = None) -> None:
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

    def copy_to_clipboard(self) -> None:
        desktop.copy_to_clipboard(self.text_details.toPlainText())
        SnackbarManager.inform(_("TEXT_STACKTRACE_COPIED_TO_CLIPBOARD"))

    def toggle_details(self, checked: bool) -> None:
        if not checked:
            self.text_details.hide()
            self.button_copy.hide()
        else:
            self.text_details.show()
            self.button_copy.show()


def show_error(parent: QWidget, message: str, exception: BaseException | None = None) -> None:
    widget = ErrorWidget(message, exception)
    dialog = GreyedDialog(widget, title=_("TEXT_ERR_DIALOG_TITLE"), parent=parent)
    return dialog.open()


class InfoWidget(QWidget, Ui_InfoWidget):
    def __init__(
        self, message: str, dialog: GreyedDialog | None = None, button_text: str | None = None
    ):
        super().__init__()
        self.setupUi(self)
        self.dialog = dialog
        self.label_message.setText(message)
        self.label_icon.apply_style()
        self.button_ok.setText(button_text or _("ACTION_CONTINUE"))
        self.button_ok.clicked.connect(self._on_button_clicked)
        self.button_ok.setFocus()

    def _on_button_clicked(self, button: Button) -> None:
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            # Mypy: `activeModalWidget` return only a `QWidget` that don't define an `accept` method
            # So finger-crossed that the return object has it.
            QApplication.activeModalWidget().accept()  # type: ignore[attr-defined]
        else:
            logger.warning("Cannot close dialog when displaying info")


def show_info(parent: QWidget, message: str, button_text: str | None = None) -> None:
    widget = InfoWidget(message, button_text=button_text)
    dialog = GreyedDialog(widget, title=None, parent=parent, hide_close=True)
    widget.dialog = dialog
    widget.button_ok.setFocus()
    return dialog.open()


class InfoLinkWidget(QWidget, Ui_InfoWidget):
    def __init__(
        self, message: str, url: str, button_text: str, dialog: GreyedDialog | None = None
    ) -> None:
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

    def _on_button_clicked(self, button: Button) -> None:
        desktop.open_url(self.url)


def show_info_link(parent: QWidget, title: str, message: str, button_text: str, url: str) -> None:
    widget = InfoLinkWidget(message, url, button_text)
    dialog = GreyedDialog(widget, title=title, parent=parent)
    widget.dialog = dialog
    widget.button_ok.setFocus()
    return dialog.open()


class InfoCopyLinkWidget(InfoLinkWidget):
    def _on_button_clicked(self, button: Button) -> None:
        desktop.copy_to_clipboard(self.url)
        SnackbarManager.inform("TEXT_ENROLLMENT_ADDR_COPIED_TO_CLIPBOARD")


def show_info_copy_link(
    parent: QWidget, title: str, message: str, button_text: str, url: str
) -> None:
    widget = InfoCopyLinkWidget(message, url, button_text)
    dialog = GreyedDialog(widget, title=title, parent=parent)
    widget.dialog = dialog
    widget.button_ok.setFocus()
    return dialog.open()
