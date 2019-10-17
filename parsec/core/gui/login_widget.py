# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional
from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.core.local_device import list_available_devices
from parsec.core.types import (
    BackendOrganizationBootstrapAddr,
    BackendOrganizationClaimUserAddr,
    BackendOrganizationClaimDeviceAddr,
)
from parsec.core.gui.claim_user_widget import ClaimUserWidget
from parsec.core.gui.claim_device_widget import ClaimDeviceWidget
from parsec.core.gui.bootstrap_organization_widget import BootstrapOrganizationWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.settings_dialog import SettingsDialog
from parsec.core.gui.custom_dialogs import show_info
from parsec.core.gui.ui.login_widget import Ui_LoginWidget
from parsec.core.gui.ui.login_login_widget import Ui_LoginLoginWidget


class LoginLoginWidget(QWidget, Ui_LoginLoginWidget):
    login_with_password_clicked = pyqtSignal(object, str)
    login_with_pkcs11_clicked = pyqtSignal(object, str, int, int)

    def __init__(self, config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.config = config
        self.button_login.clicked.connect(self.emit_login)
        self.check_box_use_pkcs11.hide()
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.addItem("0")
        self.widget_pkcs11.hide()
        devices = list_available_devices(self.config.config_dir)
        # Display devices in `<organization>:<device_id>` format
        self.devices = {}
        for o, d, t, kf in devices:
            self.combo_login.addItem(f"{o}:{d}")
            self.devices[f"{o}:{d}"] = (o, d, t, kf)
        last_device = self.config.gui_last_device
        if last_device and last_device in self.devices:
            self.combo_login.setCurrentText(last_device)
        self.line_edit_password.setFocus()

    def keyPressEvent(self, event):
        if event.key() == Qt.Key_Return:
            self.emit_login()
        event.accept()

    def emit_login(self):
        *_, key_file = self.devices[self.combo_login.currentText()]
        if self.check_box_use_pkcs11.checkState() == Qt.Unchecked:
            self.login_with_password_clicked.emit(key_file, self.line_edit_password.text())
        else:
            self.login_with_pkcs11_clicked.emit(
                key_file,
                self.line_edit_pkcs11_pin.text(),
                int(self.combo_pkcs11_key.currentText()),
                int(self.combo_pkcs11_token.currentText()),
            )


class LoginWidget(QWidget, Ui_LoginWidget):
    login_with_password_clicked = pyqtSignal(object, str)
    login_with_pkcs11_clicked = pyqtSignal(object, str, int, int)
    state_changed = pyqtSignal(str)

    def __init__(self, jobs_ctx, event_bus, config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config

        self.button_login_instead.clicked.connect(self.show_login_widget)
        self.button_register_user_instead.clicked.connect(self.show_claim_user_widget)
        self.button_register_device_instead.clicked.connect(self.show_claim_device_widget)
        self.button_bootstrap_instead.clicked.connect(self.show_bootstrap_widget)
        self.button_settings.clicked.connect(self.show_settings)

        self.event_bus.connect("gui.config.changed", self.on_config_updated)

        if len(list_available_devices(self.config.config_dir)) == 0:
            self.show_claim_user_widget()
        else:
            self.show_login_widget()

    def disconnect_all(self):
        self.event_bus.disconnect("gui.config.changed", self.on_config_updated)

    def on_config_updated(self, event, **kwargs):
        self.config = self.config.evolve(**kwargs)

    def organization_bootstrapped(self, organization, device, password):
        devices = list_available_devices(self.config.config_dir)
        if len(devices) == 1:
            if devices[0][0] == organization and devices[0][1] == device:
                show_info(self, _("INFO_BOOTSTRAP_ORG_SUCCESS_MONO_DEVICE"))
                self.emit_login_with_password(devices[0][3], password)
        else:
            show_info(self, _("INFO_BOOTRSTRAP_ORG_SUCCESS_MULTI_DEVICE"))
            self.show_login_widget()

    def user_claimed(self, organization, device, password):
        devices = list_available_devices(self.config.config_dir)
        if len(devices) == 1:
            if devices[0][0] == organization and devices[0][1] == device:
                show_info(self, _("INFO_USER_CREATED_SUCCESS_MONO_DEVICE"))
                self.emit_login_with_password(devices[0][3], password)
        else:
            show_info(self, _("INFO_USER_CREATED_SUCCESS_MULTI_DEVICE"))
            self.show_login_widget()

    def emit_login_with_password(self, key_file, password):
        self.login_with_password_clicked.emit(key_file, password)

    def emit_login_with_pkcs11(self, key_file, pkcs11_pin, pkcs11_key, pkcs11_token):
        self.login_with_pkcs11_clicked.emit(key_file, pkcs11_pin, pkcs11_key, pkcs11_token)

    def show_settings(self):
        settings_dialog = SettingsDialog(self.config, self.jobs_ctx, self.event_bus, parent=self)
        settings_dialog.exec_()

    def show_login_widget(self):
        self.clear_widgets()

        login_widget = LoginLoginWidget(self.config)
        self.layout.insertWidget(0, login_widget)
        login_widget.login_with_password_clicked.connect(self.emit_login_with_password)
        login_widget.login_with_pkcs11_clicked.connect(self.emit_login_with_pkcs11)

        self.button_login_instead.hide()
        self.button_register_user_instead.show()
        self.button_register_device_instead.show()
        self.button_bootstrap_instead.show()
        login_widget.show()
        self.state_changed.emit("login")

    def show_bootstrap_widget(self, addr: Optional[BackendOrganizationBootstrapAddr] = None):
        self.clear_widgets()

        bootstrap_organization = BootstrapOrganizationWidget(self.jobs_ctx, self.config, addr=addr)
        self.layout.insertWidget(0, bootstrap_organization)
        bootstrap_organization.organization_bootstrapped.connect(self.organization_bootstrapped)
        self.button_bootstrap_instead.hide()
        if len(list_available_devices(self.config.config_dir)) == 0:
            self.button_login_instead.hide()
        else:
            self.button_login_instead.show()
        self.button_register_user_instead.show()
        self.button_register_device_instead.show()
        bootstrap_organization.show()
        self.state_changed.emit("bootstrap")

    def show_claim_user_widget(self, addr: Optional[BackendOrganizationClaimUserAddr] = None):
        self.clear_widgets()

        claim_user_widget = ClaimUserWidget(self.jobs_ctx, self.config, addr=addr)
        self.layout.insertWidget(0, claim_user_widget)
        claim_user_widget.user_claimed.connect(self.user_claimed)

        if len(list_available_devices(self.config.config_dir)) == 0:
            self.button_login_instead.hide()
        else:
            self.button_login_instead.show()
        self.button_register_user_instead.hide()
        self.button_register_device_instead.show()
        self.button_bootstrap_instead.show()
        claim_user_widget.show()
        self.state_changed.emit("claim_user")

    def show_claim_device_widget(self, addr: Optional[BackendOrganizationClaimDeviceAddr] = None):
        self.clear_widgets()

        claim_device_widget = ClaimDeviceWidget(self.jobs_ctx, self.config, addr=addr)
        self.layout.insertWidget(0, claim_device_widget)
        claim_device_widget.device_claimed.connect(self.show_login_widget)

        if len(list_available_devices(self.config.config_dir)) == 0:
            self.button_login_instead.hide()
        else:
            self.button_login_instead.show()
        self.button_register_user_instead.show()
        self.button_register_device_instead.hide()
        self.button_bootstrap_instead.show()
        claim_device_widget.show()
        self.state_changed.emit("claim_device")

    def clear_widgets(self):
        item = self.layout.takeAt(0)
        if item:
            item.widget().hide()
            item.widget().setParent(None)
