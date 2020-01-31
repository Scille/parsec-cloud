# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.core.local_device import list_available_devices
from parsec.core.types import (
    BackendActionAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationClaimUserAddr,
    BackendOrganizationClaimDeviceAddr,
)
from parsec.core.gui.claim_user_widget import ClaimUserWidget
from parsec.core.gui.claim_device_widget import ClaimDeviceWidget
from parsec.core.gui.bootstrap_organization_widget import BootstrapOrganizationWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_dialogs import show_info, TextInputDialog, show_error
from parsec.core.gui.ui.login_widget import Ui_LoginWidget
from parsec.core.gui.ui.login_login_widget import Ui_LoginLoginWidget


class LoginLoginWidget(QWidget, Ui_LoginLoginWidget):
    login_with_password_clicked = pyqtSignal(object, str)

    def __init__(self, config, login_failed_sig, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        login_failed_sig.connect(self.on_login_failed)
        self.config = config
        self.button_login.clicked.connect(self.emit_login)
        devices = list_available_devices(self.config.config_dir)
        # Display devices in `<organization>:<device_id>` format
        self.devices = {}
        for o, d, t, kf in devices:
            self.combo_login.addItem(f"{o}:{d}")
            self.devices[f"{o}:{d}"] = (o, d, t, kf)
        self.combo_login.currentTextChanged.connect(self.line_edit_password.clear)
        self.button_login.setEnabled(self.combo_login.count() > 0)
        last_device = self.config.gui_last_device
        if last_device and last_device in self.devices:
            self.combo_login.setCurrentText(last_device)
        self.line_edit_password.setFocus()

    def on_login_failed(self):
        self.button_login.setEnabled(self.combo_login.count() > 0)
        self.button_login.setText(_("BUTTON_LOG_IN"))

    def keyPressEvent(self, event):
        if event.key() == Qt.Key_Return:
            self.emit_login()
        event.accept()

    def emit_login(self):
        if not self.combo_login.currentText():
            return
        *args, key_file = self.devices[self.combo_login.currentText()]
        self.button_login.setDisabled(True)
        self.button_login.setText(_("BUTTON_LOGGING_IN"))
        self.login_with_password_clicked.emit(key_file, self.line_edit_password.text())


class LoginWidget(QWidget, Ui_LoginWidget):
    login_with_password_clicked = pyqtSignal(object, str)
    state_changed = pyqtSignal(str)

    def __init__(self, jobs_ctx, event_bus, config, login_failed_sig, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config
        self.login_failed_sig = login_failed_sig

        self.button_login_instead.clicked.connect(self.show_login_widget)
        self.button_enter_url.clicked.connect(self.enter_url)

        self.show_login_widget()

    def enter_url(self):
        url = TextInputDialog.get_text(
            self,
            _("LABEL_ENTER_URL_TITLE"),
            _("LABEL_ENTER_URL_MESSAGE"),
            "parsec://host:port?arguments",
        )
        if not url:
            return
        try:
            action_addr = BackendActionAddr.from_url(url)
        except ValueError as exc:
            show_error(self, _("ERR_BAD_URL"), exception=exc)
            return

        if isinstance(action_addr, BackendOrganizationBootstrapAddr):
            self.show_bootstrap_widget(addr=action_addr)
        elif isinstance(action_addr, BackendOrganizationClaimUserAddr):
            self.show_claim_user_widget(addr=action_addr)
        elif isinstance(action_addr, BackendOrganizationClaimDeviceAddr):
            self.show_claim_device_widget(addr=action_addr)
        else:
            self.show_login_widget()

    def disconnect_all(self):
        pass

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

    def show_login_widget(self):
        self.clear_widgets()

        login_widget = LoginLoginWidget(self.config, self.login_failed_sig)
        self.layout.insertWidget(0, login_widget)
        login_widget.login_with_password_clicked.connect(self.emit_login_with_password)

        self.button_login_instead.hide()
        login_widget.show()
        self.state_changed.emit("login")

    def show_bootstrap_widget(self, addr: BackendOrganizationBootstrapAddr):
        self.clear_widgets()

        bootstrap_organization = BootstrapOrganizationWidget(self.jobs_ctx, self.config, addr=addr)
        self.layout.insertWidget(0, bootstrap_organization)
        bootstrap_organization.organization_bootstrapped.connect(self.organization_bootstrapped)
        self.button_login_instead.show()
        bootstrap_organization.show()
        self.state_changed.emit("bootstrap")

    def show_claim_user_widget(self, addr: BackendOrganizationClaimUserAddr):
        self.clear_widgets()

        claim_user_widget = ClaimUserWidget(self.jobs_ctx, self.config, addr=addr)
        self.layout.insertWidget(0, claim_user_widget)
        claim_user_widget.user_claimed.connect(self.user_claimed)

        self.button_login_instead.show()
        claim_user_widget.show()
        self.state_changed.emit("claim_user")

    def show_claim_device_widget(self, addr: BackendOrganizationClaimDeviceAddr):
        self.clear_widgets()

        claim_device_widget = ClaimDeviceWidget(self.jobs_ctx, self.config, addr=addr)
        self.layout.insertWidget(0, claim_device_widget)
        claim_device_widget.device_claimed.connect(self.show_login_widget)

        self.button_login_instead.show()
        claim_device_widget.show()
        self.state_changed.emit("claim_device")

    def clear_widgets(self):
        item = self.layout.takeAt(0)
        if item:
            item.widget().hide()
            item.widget().setParent(None)
