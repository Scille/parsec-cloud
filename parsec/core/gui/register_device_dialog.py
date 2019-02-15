import threading
import queue

from PyQt5.QtCore import QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog

import trio

from parsec.core.backend_connection import BackendCmdsBadResponse
from parsec.core.invite_claim import generate_invitation_token, invite_and_create_device

from parsec.core.gui import desktop
from parsec.core.gui import validators
from parsec.core.gui.custom_widgets import show_warning, show_info
from parsec.core.gui.ui.register_device_dialog import Ui_RegisterDeviceDialog


async def _handle_invite_and_create_device(
    queue, qt_on_done, qt_on_error, core, device_name, token
):
    try:
        with trio.CancelScope() as cancel_scope:
            queue.put(cancel_scope)
            await invite_and_create_device(core.device, core.backend_cmds, device_name, token)
            qt_on_done.emit()
    except BackendCmdsBadResponse as exc:
        qt_on_error.emit(exc.status)
    except:
        qt_on_error.emit(None)


class RegisterDeviceDialog(QDialog, Ui_RegisterDeviceDialog):
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
        self.button_register.clicked.connect(self.register_device)
        self.button_cancel.clicked.connect(self.cancel_register_device)
        self.button_copy_device.clicked.connect(self.copy_field(self.line_edit_device))
        self.button_copy_token.clicked.connect(self.copy_field(self.line_edit_token))
        self.button_copy_url.clicked.connect(self.copy_field(self.line_edit_url))
        self.on_registered.connect(self.device_registered)
        self.on_register_error.connect(self.register_error)
        self.line_edit_device_name.setValidator(validators.DeviceNameValidator())
        self.closing_allowed = True

    def copy_field(self, widget):
        def _inner_copy_field():
            desktop.copy_to_clipboard(widget.text())

        return _inner_copy_field

    def register_error(self, status):
        self.register_thread.join()
        self.register_thread = None
        self.cancel_scope = None
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_device_name.show()
        self.closing_allowed = True
        if status is None:
            show_warning(self, QCoreApplication.translate("RegisterUserDialog", "Unknown error."))
        elif status == "already_exists":
            show_warning(
                self,
                QCoreApplication.translate("RegisterDeviceDialog", "This device already exists."),
            )
        else:
            show_warning(
                self,
                QCoreApplication.translate(
                    "RegisterDeviceDialog", "Can not register this device ({})."
                ).format(status),
            )

    def device_registered(self):
        self.register_thread.join()
        self.register_thread = None
        show_info(
            self,
            QCoreApplication.translate(
                "RegisterDeviceDialog", "Device has been registered. You may now close this window."
            ),
        )
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_device_name.show()
        self.closing_allowed = True

    def cancel_register_device(self):
        self.portal.run_sync(self.cancel_scope.cancel)
        self.cancel_scope = None
        self.register_thread.join()
        self.register_thread = None
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_device_name.show()
        self.closing_allowed = True

    def closeEvent(self, event):
        if not self.closing_allowed:
            show_warning(
                self,
                QCoreApplication.translate(
                    "RegisterDeviceDialog",
                    "Can not close this window while waiting for the new device to register. "
                    "Please cancel first.",
                ),
            )
            event.ignore()
        else:
            event.accept()

    def register_device(self):
        def _run_registration(device_name, token):
            self.portal.run(
                _handle_invite_and_create_device,
                self.register_queue,
                self.on_registered,
                self.on_register_error,
                self.core,
                device_name,
                token,
            )

        if not self.line_edit_device_name.text():
            show_warning(
                self,
                QCoreApplication.translate("RegisterDeviceDialog", "Please enter a device name."),
            )
            return

        try:
            token = generate_invitation_token()
            self.line_edit_device.setText(self.line_edit_device_name.text())
            self.line_edit_device.setCursorPosition(0)
            self.line_edit_token.setText(token)
            self.line_edit_token.setCursorPosition(0)
            self.line_edit_url.setText(self.core.device.organization_addr)
            self.line_edit_url.setCursorPosition(0)
            self.button_cancel.setFocus()
            self.widget_registration.show()
            self.register_thread = threading.Thread(
                target=_run_registration, args=(self.line_edit_device_name.text(), token)
            )
            self.register_thread.start()
            self.cancel_scope = self.register_queue.get()
            self.button_cancel.show()
            self.line_edit_device_name.hide()
            self.button_register.hide()
            self.closing_allowed = False
        except:
            show_warning(
                self,
                QCoreApplication.translate(
                    "RegisterDeviceDialog", "Could not register the device."
                ),
            )
