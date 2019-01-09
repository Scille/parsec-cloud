import threading
import queue

from PyQt5.QtCore import QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog

import trio

from parsec.core.invite_claim import generate_invitation_token, invite_and_create_user

from parsec.core.gui import desktop
from parsec.core.gui.custom_widgets import show_warning, show_info
from parsec.core.gui.ui.register_user_dialog import Ui_RegisterUserDialog


async def _handle_invite_and_create_user(queue, qt_on_done, core, username, token):
    with trio.open_cancel_scope() as cancel_scope:
        queue.put(cancel_scope)
        async with trio.open_nursery() as nursery:
            await invite_and_create_user(core.device, core.backend_cmds, username, token)
        qt_on_done.emit()


class RegisterUserDialog(QDialog, Ui_RegisterUserDialog):
    on_registered = pyqtSignal()

    def __init__(self, portal, core, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.portal = portal
        self.cancel_scope = None
        self.register_thread = None
        self.register_queue = queue.Queue(1)
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.clicked.connect(self.register_user)
        self.button_cancel.clicked.connect(self.cancel_register_user)
        self.button_copy_username.clicked.connect(self.copy_field(self.line_edit_user))
        self.button_copy_token.clicked.connect(self.copy_field(self.line_edit_token))
        self.button_copy_url.clicked.connect(self.copy_field(self.line_edit_url))
        self.on_registered.connect(self.user_registered)
        self.closing_allowed = True

    def copy_field(self, widget):
        def _inner_copy_field():
            desktop.copy_to_clipboard(widget.text())

        return _inner_copy_field

    def user_registered(self):
        self.register_thread.join()
        self.register_thread = None
        show_info(
            self,
            QCoreApplication.translate(
                "RegisterUserDialog", "User has been registered. You may now close this window."
            ),
        )
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_user.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_username.show()
        self.closing_allowed = True

    def cancel_register_user(self):
        self.portal.run_sync(self.cancel_scope.cancel)
        self.cancel_scope = None
        self.register_thread.join()
        self.register_thread = None
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_user.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_username.show()
        self.closing_allowed = True

    def closeEvent(self, event):
        if not self.closing_allowed:
            show_warning(
                self,
                QCoreApplication.translate(
                    "RegisterUserDialog",
                    "Can not close this window while waiting for the new user to register. Please cancel first.",
                ),
            )
            event.ignore()
        else:
            event.accept()

    def register_user(self):
        def _run_registration(username, token):
            self.portal.run(
                _handle_invite_and_create_user,
                self.register_queue,
                self.on_registered,
                self.core,
                username,
                token,
            )

        if not self.line_edit_username.text():
            show_warning(
                self, QCoreApplication.translate("RegisterUserDialog", "Please enter a username.")
            )
            return

        users = self.portal.run(self.core.fs.backend_cmds.user_find, self.line_edit_username.text())
        if len(users):
            show_warning(
                self,
                QCoreApplication.translate(
                    "RegisterUserDialog", "A user with the same name already exists."
                ),
            )
            return
        try:
            token = generate_invitation_token()
            self.line_edit_user.setText(self.line_edit_username.text())
            self.line_edit_user.setCursorPosition(0)
            self.line_edit_token.setText(token)
            self.line_edit_token.setCursorPosition(0)
            self.line_edit_url.setText(self.core.device.backend_addr)
            self.line_edit_url.setCursorPosition(0)
            self.button_cancel.setFocus()
            self.widget_registration.show()
            self.cancel_event = trio.Event()
            self.register_thread = threading.Thread(
                target=_run_registration, args=(self.line_edit_username.text(), token)
            )
            self.register_thread.start()
            self.cancel_scope = self.register_queue.get()
            self.button_cancel.show()
            self.line_edit_username.hide()
            self.button_register.hide()
            self.closing_allowed = False
        except:
            show_warning(
                self,
                QCoreApplication.translate("RegisterUserDialog", "Could not register the user."),
            )
