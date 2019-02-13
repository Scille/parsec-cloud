# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import threading
import queue

from PyQt5.QtCore import pyqtSignal, QCoreApplication, Qt
from PyQt5.QtWidgets import QWidget

import trio

from parsec.core import devices_manager
from parsec.types import BackendOrganizationAddr, DeviceID
from parsec.core.backend_connection import backend_anonymous_cmds_factory, BackendCmdsBadResponse
from parsec.core.invite_claim import claim_device as core_claim_device
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.custom_widgets import show_error, show_info
from parsec.core.gui import validators
from parsec.core.gui.password_validation import (
    get_password_strength,
    PASSWORD_STRENGTH_TEXTS,
    PASSWORD_CSS,
)
from parsec.core.gui.ui.claim_device_widget import Ui_ClaimDeviceWidget


async def _trio_claim_device(
    queue,
    qt_on_done,
    qt_on_error,
    config,
    addr,
    device_id,
    token,
    use_pkcs11,
    password=None,
    pkcs11_token=None,
    pkcs11_key=None,
):
    portal = trio.BlockingTrioPortal()
    queue.put(portal)
    with trio.CancelScope() as cancel_scope:
        queue.put(cancel_scope)
        try:
            async with backend_anonymous_cmds_factory(addr) as cmds:
                device = await core_claim_device(cmds, device_id, token)
            if use_pkcs11:
                devices_manager.save_device_with_pkcs11(
                    config.config_dir, device, pkcs11_token, pkcs11_key
                )
            else:
                devices_manager.save_device_with_password(config.config_dir, device, password)
            qt_on_done.emit()
        except BackendCmdsBadResponse as e:
            qt_on_error.emit(e.status)


class ClaimDeviceWidget(QWidget, Ui_ClaimDeviceWidget):
    device_claimed = pyqtSignal()
    claim_successful = pyqtSignal()
    on_claim_error = pyqtSignal(str)

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core_config = core_config
        self.button_cancel.hide()
        self.button_claim.clicked.connect(self.claim_clicked)
        self.button_cancel.clicked.connect(self.cancel_claim)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_url.textChanged.connect(self.check_infos)
        self.line_edit_token.textChanged.connect(self.check_infos)
        self.line_edit_password.textChanged.connect(self.password_changed)
        self.claim_successful.connect(self.claim_finished)
        self.on_claim_error.connect(self.claim_error)
        self.line_edit_login.setValidator(validators.UserIDValidator())
        self.line_edit_device.setValidator(validators.DeviceNameValidator())
        self.line_edit_url.setValidator(validators.BackendOrganizationAddrValidator())
        self.claim_thread = None
        self.cancel_scope = None
        self.trio_portal = None
        self.claim_queue = queue.Queue(2)

    def claim_error(self, status):
        self.claim_thread.join()
        self.claim_thread = None
        self.cancel_scope = None
        self.trio_portal = None
        self.button_cancel.hide()
        self.button_claim.setDisabled(False)
        self.check_infos("")
        if status == "not_found":
            show_error(
                self,
                QCoreApplication.translate(
                    "ClaimUserWidget", "No invitation found for this device."
                ),
            )
        else:
            show_error(
                self,
                QCoreApplication.translate(
                    "ClaimUserWidget", "Can not claim this user ({})."
                ).format(status),
            )

    def cancel_claim(self):
        self.trio_portal.run_sync(self.cancel_scope.cancel)
        self.claim_thread.join()
        self.trio_portal = None
        self.cancel_scope = None
        self.claim_thread = None
        self.button_cancel.hide()
        self.button_claim.setDisabled(False)
        self.check_infos("")

    def claim_finished(self):
        self.claim_thread.join()
        self.claim_thread = None
        self.cancel_scope = None
        self.trio_portal = None
        show_info(
            self,
            QCoreApplication.translate(
                "ClaimDeviceWidget", "The device has been registered. You can now login."
            ),
        )
        self.device_claimed.emit()

    def password_changed(self, text):
        if len(text):
            self.label_password_strength.show()
            score = get_password_strength(text)
            self.label_password_strength.setText(
                QCoreApplication.translate("ClaimDeviceWidget", "Password strength: {}").format(
                    PASSWORD_STRENGTH_TEXTS[score]
                )
            )
            self.label_password_strength.setStyleSheet(PASSWORD_CSS[score])
        else:
            self.label_password_strength.hide()

    def check_infos(self, _):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_token.text())
            and len(self.line_edit_device.text())
            and len(self.line_edit_url.text())
        ):
            self.button_claim.setDisabled(False)
        else:
            self.button_claim.setDisabled(True)

    def _thread_claim_device(
        self, addr, device_id, token, use_pkcs11, password=None, pkcs11_token=None, pkcs11_key=None
    ):
        trio.run(
            _trio_claim_device,
            self.claim_queue,
            self.claim_successful,
            self.on_claim_error,
            self.core_config,
            addr,
            device_id,
            token,
            use_pkcs11,
            password,
            pkcs11_token,
            pkcs11_key,
        )

    def claim_clicked(self):
        use_pkcs11 = True
        if self.check_box_use_pkcs11.checkState() == Qt.Unchecked:
            use_pkcs11 = False
            if (
                len(self.line_edit_password.text()) > 0
                or len(self.line_edit_password_check.text()) > 0
            ):
                if self.line_edit_password.text() != self.line_edit_password_check.text():
                    show_error(
                        self,
                        QCoreApplication.translate("ClaimDeviceWidget", "Passwords don't match"),
                    )
                    return
        backend_addr = None
        device_id = None
        try:
            backend_addr = BackendOrganizationAddr(self.line_edit_url.text())
            device_id = DeviceID(
                "{}@{}".format(self.line_edit_login.text(), self.line_edit_device.text())
            )
        except:
            show_error(
                self, QCoreApplication.translate("ClaimDeviceWidget", "URL or device is invalid.")
            )
            return
        try:
            args = None
            if use_pkcs11:
                args = (
                    backend_addr,
                    device_id,
                    self.line_edit_token.text(),
                    True,
                    None,
                    int(self.line_edit_pkcs11_token.text()),
                    int(self.line_edit_pkcs11.key.text()),
                )
            else:
                args = (
                    backend_addr,
                    device_id,
                    self.line_edit_token.text(),
                    False,
                    self.line_edit_password.text(),
                )
            self.button_cancel.show()
            self.claim_thread = threading.Thread(target=self._thread_claim_device, args=args)
            self.claim_thread.start()
            self.trio_portal = self.claim_queue.get()
            self.cancel_scope = self.claim_queue.get()
            self.button_claim.setDisabled(True)
        except:
            import traceback

            traceback.print_exc()

            show_error(
                self,
                QCoreApplication.translate("ClaimDeviceWidget", "Can not register the new device."),
            )

    def reset(self):
        self.line_edit_login.setText("")
        self.line_edit_password.setText("")
        self.line_edit_password_check.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText(get_default_device())
        self.line_edit_token.setText("")
        self.line_edit_password.setEnabled(True)
        self.line_edit_password_check.setEnabled(True)
        self.label_password_strength.hide()
        self.check_box_use_pkcs11.setCheckState(Qt.Unchecked)
        self.combo_pkcs11_key.clear()
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.clear()
        self.combo_pkcs11_token.addItem("0")
        self.button_cancel.hide()
        self.widget_pkcs11.hide()
