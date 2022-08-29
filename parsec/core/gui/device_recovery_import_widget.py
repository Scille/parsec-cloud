# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget
from pathlib import Path, PurePath

from parsec.api.protocol import DeviceLabel
from parsec.crypto import derivate_secret_key_from_recovery_passphrase
from parsec.core.backend_connection import BackendConnectionError, BackendNotAvailable
from parsec.core.recovery import generate_new_device_from_recovery
from parsec.core.local_device import (
    save_device_with_password_in_config,
    save_device_with_smartcard_in_config,
    DeviceFileType,
    load_recovery_device,
    LocalDeviceError,
    LocalDeviceCryptoError,
    LocalDeviceNotFoundError,
)
from parsec.core.gui.authentication_choice_widget import AuthenticationChoiceWidget
from parsec.core.gui.trio_jobs import QtToTrioJob, JobResultError
from parsec.core.gui.lang import translate
from parsec.core.gui.custom_dialogs import GreyedDialog, QDialogInProcess, show_error, show_info
from parsec.core.gui import validators
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.ui.device_recovery_import_widget import Ui_DeviceRecoveryImportWidget
from parsec.core.gui.ui.device_recovery_import_page1_widget import (
    Ui_DeviceRecoveryImportPage1Widget,
)


class DeviceRecoveryImportPage1Widget(QWidget, Ui_DeviceRecoveryImportPage1Widget):
    info_filled = pyqtSignal(bool)

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        self.button_import_key.clicked.connect(self._on_import_key_clicked)
        self.label_passphrase_error.hide()
        self.line_edit_device.setText(get_default_device())
        self.line_edit_device.validity_changed.connect(self._check_infos)
        self.line_edit_device.set_validator(validators.DeviceLabelValidator())
        self.edit_passphrase.textChanged.connect(self._on_passphrase_text_changed)

    def _on_import_key_clicked(self):
        key_file, _ = QDialogInProcess.getOpenFileName(
            self,
            translate("ACTION_IMPORT_KEY"),
            str(Path.home()),
            filter=translate("RECOVERY_KEY_FILTERS"),
            initialFilter=translate("RECOVERY_KEY_INITIAL_FILTER"),
        )
        if key_file:
            self.label_key_file.setText(key_file)
        self._check_infos()

    def _is_valid_passphrase(self, passphrase):
        try:
            derivate_secret_key_from_recovery_passphrase(passphrase)
            return True
        except ValueError:
            return False

    def _on_passphrase_text_changed(self):
        if not self._is_valid_passphrase(self.edit_passphrase.toPlainText()):
            self.label_passphrase_error.setText(translate("TEXT_RECOVERY_INVALID_PASSPHRASE"))
            self.label_passphrase_error.show()
        else:
            self.label_passphrase_error.hide()
        self._check_infos()

    def _check_infos(self):
        self.info_filled.emit(
            bool(
                self._is_valid_passphrase(self.edit_passphrase.toPlainText())
                and self.line_edit_device.is_input_valid()
                and len(self.label_key_file.text()) > 0
            )
        )

    def get_passphrase(self):
        return self.edit_passphrase.toPlainText()

    def get_device_label(self):
        return DeviceLabel(self.line_edit_device.text())

    def get_recovery_key_file(self):
        return self.label_key_file.text()


class DeviceRecoveryImportWidget(QWidget, Ui_DeviceRecoveryImportWidget):
    create_new_device_success = pyqtSignal(QtToTrioJob)
    create_new_device_failure = pyqtSignal(QtToTrioJob)

    def __init__(self, config, jobs_ctx, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        self.dialog = None
        self.config = config
        self.jobs_ctx = jobs_ctx
        self.recovery_device = None
        self.button_validate.clicked.connect(self._on_validate_clicked)
        self.button_validate.setEnabled(False)
        self.current_page = DeviceRecoveryImportPage1Widget(self)
        self.current_page.info_filled.connect(self._on_page1_info_filled)
        self.main_layout.addWidget(self.current_page)
        self.create_new_device_success.connect(self._on_create_new_device_success)
        self.create_new_device_failure.connect(self._on_create_new_device_failure)

    def _on_page1_info_filled(self, valid):
        self.button_validate.setEnabled(valid)

    def _on_page2_info_filled(self, auth_method, valid):
        self.button_validate.setEnabled(valid)

    async def _create_new_device(self, device_label, file_path, passphrase):
        try:
            recovery_device = await load_recovery_device(file_path, passphrase)
            new_device = await generate_new_device_from_recovery(recovery_device, device_label)
            return new_device
        except LocalDeviceError as exc:
            self.button_validate.setEnabled(True)
            if "Decryption failed" in str(exc):
                show_error(self, translate("TEXT_IMPORT_KEY_WRONG_PASSPHRASE"), exception=exc)
            else:
                show_error(self, translate("IMPORT_KEY_LOCAL_DEVICE_ERROR"), exception=exc)
            raise JobResultError("error") from exc
        except BackendNotAvailable as exc:
            show_error(self, translate("IMPORT_KEY_BACKEND_OFFLINE"), exception=exc)
            raise JobResultError("backend-error") from exc
        except BackendConnectionError as exc:
            show_error(self, translate("IMPORT_KEY_BACKEND_ERROR"), exception=exc)
            raise JobResultError("backend-error") from exc
        except Exception as exc:
            show_error(self, translate("IMPORT_KEY_ERROR"), exception=exc)
            raise JobResultError("error") from exc

    async def _on_validate_clicked(self):
        if isinstance(self.current_page, DeviceRecoveryImportPage1Widget):
            # No try/except given `self.line_edit_device` has already been validated against `DeviceLabel`
            device_label = DeviceLabel(self.current_page.line_edit_device.clean_text())
            self.jobs_ctx.submit_job(
                (self, "create_new_device_success"),
                (self, "create_new_device_failure"),
                self._create_new_device,
                device_label=device_label,
                file_path=PurePath(self.current_page.get_recovery_key_file()),
                passphrase=self.current_page.get_passphrase(),
            )
        else:
            try:
                self.button_validate.setEnabled(False)
                auth_method = self.current_page.get_auth_method()
                if auth_method == DeviceFileType.PASSWORD:
                    save_device_with_password_in_config(
                        config_dir=self.config.config_dir,
                        device=self.new_device,
                        password=self.current_page.get_auth(),
                    )
                else:
                    await save_device_with_smartcard_in_config(
                        config_dir=self.config.config_dir, device=self.new_device
                    )
                show_info(self, translate("TEXT_RECOVERY_IMPORT_SUCCESS"))
                self.dialog.accept()
            except LocalDeviceCryptoError as exc:
                self.button_validate.setEnabled(True)
                if auth_method == DeviceFileType.SMARTCARD:
                    show_error(self, translate("TEXT_INVALID_SMARTCARD"), exception=exc)
            except LocalDeviceNotFoundError as exc:
                if auth_method == DeviceFileType.PASSWORD:
                    show_error(self, translate("TEXT_CANNOT_SAVE_DEVICE"), exception=exc)
                self.button_validate.setEnabled(True)
            except LocalDeviceError as exc:
                show_error(self, translate("TEXT_CANNOT_SAVE_DEVICE"), exception=exc)
                self.button_validate.setEnabled(True)

    def _on_create_new_device_success(self, job):
        self.new_device = job.ret
        self.main_layout.removeWidget(self.current_page)
        self.current_page = AuthenticationChoiceWidget(parent=self)
        self.current_page.authentication_state_changed.connect(self._on_page2_info_filled)
        self.main_layout.addWidget(self.current_page)
        self.button_validate.setEnabled(self.current_page.is_auth_valid())
        self.button_validate.setText(translate("ACTION_CREATE_DEVICE"))

    def _on_create_new_device_failure(self, job):
        self.button_validate.setEnabled(True)

    @classmethod
    def show_modal(cls, config, jobs_ctx, parent, on_finished):
        w = cls(config=config, jobs_ctx=jobs_ctx)
        d = GreyedDialog(
            w, translate("TEXT_DEVICE_RECOVERY_IMPORT_WIZARD_TITLE"), parent=parent, width=800
        )
        w.dialog = d

        def _on_finished(result):
            return on_finished()

        d.finished.connect(_on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
