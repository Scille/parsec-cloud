import threading
import queue

from PyQt5.QtCore import QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog

import trio

from parsec.core.backend_connection import BackendCmdsBadResponse
from parsec.core.invite_claim import generate_invitation_token, invite_and_create_user

from parsec.core.gui import desktop
from parsec.core.gui.custom_widgets import show_warning, show_info
from parsec.core.gui.ui.register_user_dialog import Ui_RegisterUserDialog


async def _handle_invite_and_create_user(
    queue, qt_on_done, qt_on_error, core, username, token, is_admin
):
    try:
        with trio.open_cancel_scope() as cancel_scope:
            queue.put(cancel_scope)
            await invite_and_create_user(core.device, core.backend_cmds, username, token, is_admin)
            qt_on_done.emit()
    except BackendCmdsBadResponse as exc:
        qt_on_error.emit(exc.status)
    except:
        qt_on_error.emit(None)


class RegisterUserDialog(QDialog, Ui_RegisterUserDialog):
    on_registered = pyqtSignal()
    on_register_error = pyqtSignal(str)

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
        self.on_register_error.connect(self.registration_error)
        self.closing_allowed = True

    def copy_field(self, widget):
        def _inner_copy_field():
            desktop.copy_to_clipboard(widget.text())

        return _inner_copy_field

    def registration_error(self, status):
        self.register_thread.join()
        self.register_thread = None
        self.cancel_scope = None
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_user.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.checkbox_is_admin.show()
        self.button_register.show()
        self.line_edit_username.show()
        self.closing_allowed = True
        if status is None:
            show_warning(self, QCoreApplication.translate("RegisterUserDialog", "Unknown error."))
        elif status == "invalid_role":
            show_warning(
                self,
                QCoreApplication.translate(
                    "RegisterUserDialog", "Only admins can invite a new user."
                ),
            )
        else:
            show_warning(
                self,
                QCoreApplication.translate(
                    "RegisterUserDialog", "Unhandled response {}".format(status)
                ),
            )

    def user_registered(self):
        self.register_thread.join()
        self.register_thread = None
        self.cancel_scope = None
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
        self.checkbox_is_admin.show()
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
        self.checkbox_is_admin.show()
        self.closing_allowed = True

    def closeEvent(self, event):
        if not self.closing_allowed:
            show_warning(
                self,
                QCoreApplication.translate(
                    "RegisterUserDialog",
                    "Can not close this window while waiting for the new user to register. "
                    "Please cancel first.",
                ),
            )
            event.ignore()
        else:
            event.accept()

    def register_user(self):
        def _run_registration(username, token, is_admin):
            self.portal.run(
                _handle_invite_and_create_user,
                self.register_queue,
                self.on_registered,
                self.on_register_error,
                self.core,
                username,
                token,
                is_admin,
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
            self.line_edit_url.setText(self.core.device.organization_addr)
            self.line_edit_url.setCursorPosition(0)
            self.button_cancel.setFocus()
            self.widget_registration.show()
            self.register_thread = threading.Thread(
                target=_run_registration,
                args=(self.line_edit_username.text(), token, self.checkbox_is_admin.isChecked()),
            )
            self.register_thread.start()
            self.cancel_scope = self.register_queue.get()
            self.button_cancel.show()
            self.line_edit_username.hide()
            self.checkbox_is_admin.hide()
            self.button_register.hide()
            self.closing_allowed = False
        except:
            import traceback

            traceback.print_exc()
            show_warning(
                self,
                QCoreApplication.translate("RegisterUserDialog", "Could not register the user."),
            )
