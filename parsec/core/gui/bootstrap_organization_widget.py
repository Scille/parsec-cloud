# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import threading
import queue
import pendulum

from PyQt5.QtCore import QCoreApplication, pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.crypto import SigningKey
from parsec.trustchain import certify_user, certify_device
from parsec.types import BackendOrganizationBootstrapAddr, DeviceID
from parsec.core.backend_connection import (
    BackendCmdsBadResponse,
    backend_anonymous_cmds_factory,
    BackendHandshakeError,
)
from parsec.core.devices_manager import (
    generate_new_device,
    save_device_with_password,
    DeviceConfigAleadyExists,
)
from parsec.core.gui.custom_widgets import show_error, show_info
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui import validators
from parsec.core.gui.password_validation import (
    get_password_strength,
    PASSWORD_STRENGTH_TEXTS,
    PASSWORD_CSS,
)
from parsec.core.gui.ui.bootstrap_organization_widget import Ui_BootstrapOrganizationWidget


async def _trio_bootstrap_organization(
    queue,
    qt_on_done,
    qt_on_error,
    config,
    bootstrap_addr,
    device_id,
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
            root_signing_key = SigningKey.generate()
            root_verify_key = root_signing_key.verify_key
            organization_addr = bootstrap_addr.generate_organization_addr(root_verify_key)

            device = generate_new_device(device_id, organization_addr)

            save_device_with_password(config.config_dir, device, password)

            now = pendulum.now()
            certified_user = certify_user(
                None, root_signing_key, device.user_id, device.public_key, now
            )
            certified_device = certify_device(
                None, root_signing_key, device_id, device.verify_key, now
            )

            async with backend_anonymous_cmds_factory(bootstrap_addr) as cmds:
                await cmds.organization_bootstrap(
                    bootstrap_addr.organization_id,
                    bootstrap_addr.bootstrap_token,
                    root_verify_key,
                    certified_user,
                    certified_device,
                )
            qt_on_done.emit()
        except DeviceConfigAleadyExists:
            qt_on_error.emit("already-exists")
        except BackendHandshakeError:
            qt_on_error.emit("invalid-url")
        except BackendCmdsBadResponse as e:
            qt_on_error.emit(e.status)
        except:
            qt_on_error.emit("")


class BootstrapOrganizationWidget(QWidget, Ui_BootstrapOrganizationWidget):
    bootstrap_successful = pyqtSignal()
    bootstrap_error = pyqtSignal(str)
    organization_bootstrapped = pyqtSignal()

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core_config = core_config
        self.button_cancel.hide()
        self.button_bootstrap.clicked.connect(self.bootstrap_clicked)
        self.button_cancel.clicked.connect(self.cancel_bootstrap)
        self.line_edit_password.textChanged.connect(self.password_changed)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_url.textChanged.connect(self.check_infos)
        self.line_edit_login.setValidator(validators.UserIDValidator())
        self.line_edit_device.setValidator(validators.DeviceNameValidator())
        self.line_edit_url.setValidator(validators.BackendOrganizationAddrValidator())
        self.bootstrap_successful.connect(self.on_bootstrap_finished)
        self.bootstrap_error.connect(self.on_bootstrap_error)

        self.thread = None
        self.cancel_scope = None
        self.trio_portal = None
        self.queue = queue.Queue(2)

    def on_bootstrap_error(self, status):
        self.thread.join()
        self.thread = None
        self.cancel_scope = None
        self.trio_portal = None
        self.button_cancel.hide()
        self.check_infos("")
        if status == "invalid-url":
            show_error(
                self,
                QCoreApplication.translate(
                    "BootstrapOrganizationWidget",
                    "This organization does not exist (is the URL correct ?).",
                ).format(status),
            )
        elif status == "already-exists":
            show_error(
                self,
                QCoreApplication.translate(
                    "BootstrapOrganizationWidget", "This user already exists."
                ).format(status),
            )
        else:
            show_error(
                self,
                QCoreApplication.translate(
                    "BootstrapOrganizationWidget", "Can not bootstrap this organization."
                ).format(status),
            )

    def on_bootstrap_finished(self):
        self.thread.join()
        self.thread = None
        self.cancel_scope = None
        self.trio_portal = None
        self.button_bootstrap.setDisabled(False)
        self.button_cancel.hide()
        show_info(
            self,
            QCoreApplication.translate(
                "BootstrapOrganizationWidget",
                "The organization and the user have been created. You can now login.",
            ),
        )
        self.organization_bootstrapped.emit()

    def _thread_bootstrap_organization(
        self, addr, device_id, use_pkcs11, password=None, pkcs11_token=None, pkcs11_key=None
    ):
        trio.run(
            _trio_bootstrap_organization,
            self.queue,
            self.bootstrap_successful,
            self.bootstrap_error,
            self.core_config,
            addr,
            device_id,
            use_pkcs11,
            password,
            pkcs11_token,
            pkcs11_key,
        )

    def bootstrap_clicked(self):
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
                        QCoreApplication.translate(
                            "BootstrapOrganizationWidget", "Passwords don't match"
                        ),
                    )
                    return
            if len(self.line_edit_password.text()) < 8:
                show_error(
                    self,
                    QCoreApplication.translate(
                        "BootstrapOrganizationWidget", "Password must be at least 8 caracters long."
                    ),
                )
                return
        try:
            backend_addr = BackendOrganizationBootstrapAddr(self.line_edit_url.text())
            device_id = DeviceID(
                "{}@{}".format(self.line_edit_login.text(), self.line_edit_device.text())
            )
        except:
            show_error(
                self,
                QCoreApplication.translate(
                    "BootstrapOrganizationWidget", "URL or device is invalid."
                ),
            )
            return
        try:
            args = None
            if use_pkcs11:
                args = (
                    backend_addr,
                    device_id,
                    True,
                    None,
                    int(self.line_edit_pkcs11_token.text()),
                    int(self.line_edit_pkcs11.key.text()),
                )
            else:
                args = (backend_addr, device_id, False, self.line_edit_password.text())
            self.button_cancel.show()
            self.thread = threading.Thread(target=self._thread_bootstrap_organization, args=args)
            self.thread.start()
            self.trio_portal = self.queue.get()
            self.cancel_scope = self.queue.get()
            self.button_bootstrap.setDisabled(True)
        except:
            import traceback

            traceback.print_exc()

            show_error(
                self,
                QCoreApplication.translate(
                    "BootstrapOrganizationWidget", "Can not register the new user."
                ),
            )

    def cancel_bootstrap(self):
        self.trio_portal.run_sync(self.cancel_scope.cancel)
        self.thread.join()
        self.thread = None
        self.cancel_scope = None
        self.trio_portal = None
        self.button_cancel.hide()
        self.button_bootstrap.setDisabled(False)
        self.check_infos("")

    def check_infos(self, _):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_device.text())
            and len(self.line_edit_url.text())
        ):
            self.button_bootstrap.setDisabled(False)
        else:
            self.button_bootstrap.setDisabled(True)

    def password_changed(self, text):
        if len(text):
            self.label_password_strength.show()
            score = get_password_strength(text)
            self.label_password_strength.setText(
                QCoreApplication.translate(
                    "BootstrapOrganizationWidget", "Password strength: {}"
                ).format(PASSWORD_STRENGTH_TEXTS[score])
            )
            self.label_password_strength.setStyleSheet(PASSWORD_CSS[score])
        else:
            self.label_password_strength.hide()

    def reset(self):
        self.line_edit_login.setText("")
        self.line_edit_password.setText("")
        self.line_edit_password_check.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText(get_default_device())
        self.check_box_use_pkcs11.setCheckState(Qt.Unchecked)
        self.combo_pkcs11_key.clear()
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.clear()
        self.combo_pkcs11_token.addItem("0")
        self.button_cancel.hide()
        self.widget_pkcs11.hide()
        self.label_password_strength.hide()
