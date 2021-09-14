# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget, QFileDialog

from pathlib import Path

from parsec.core.local_device import load_device_file, load_device_with_password, LocalDeviceError

from parsec.core.recovery import (
    create_new_device_from_original,
    RecoveryLocalDeviceError,
    RecoveryBackendError,
    is_valid_passphrase,
    get_recovery_password_from_passphrase,
)

from parsec.core.gui.trio_jobs import QtToTrioJob
from parsec.core.gui.lang import translate
from parsec.core.gui.custom_dialogs import GreyedDialog, show_error, show_info
from parsec.core.gui import validators
from parsec.core.gui.desktop import get_default_device

from parsec.core.gui.ui.device_recovery_import_widget import Ui_DeviceRecoveryImportWidget
from parsec.core.gui.ui.device_recovery_import_page1_widget import (
    Ui_DeviceRecoveryImportPage1Widget,
)
from parsec.core.gui.ui.device_recovery_import_page2_widget import (
    Ui_DeviceRecoveryImportPage2Widget,
)


class DeviceRecoveryImportPage2Widget(QWidget, Ui_DeviceRecoveryImportPage2Widget):
    info_filled = pyqtSignal(bool)

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        self.widget_password.info_changed.connect(self.check_infos)
        self.line_edit_device.setText(get_default_device())
        self.line_edit_device.validity_changed.connect(self.check_infos)
        self.line_edit_device.set_validator(validators.DeviceNameValidator())

    def check_infos(self, _=None):
        self.info_filled.emit(
            bool(self.line_edit_device.is_input_valid() and self.widget_password.is_valid())
        )

    def get_password(self):
        return self.widget_password.password

    def get_device_name(self):
        return self.line_edit_device.text()


class DeviceRecoveryImportPage1Widget(QWidget, Ui_DeviceRecoveryImportPage1Widget):
    info_filled = pyqtSignal(bool)

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        self.button_import_key.clicked.connect(self._on_import_key_clicked)
        self.label_passphrase_error.hide()
        self.edit_passphrase.textChanged.connect(self._on_passphrase_text_changed)

    def _on_import_key_clicked(self):
        key_file, _ = QFileDialog.getOpenFileName(
            self,
            translate("ACTION_IMPORT_KEY"),
            str(Path.home()),
            filter=translate("RECOVERY_KEY_FILTERS"),
            initialFilter=translate("RECOVERY_KEY_INITIAL_FILTER"),
        )
        if key_file:
            self.label_key_file.setText(key_file)
        self._check_infos()

    def _on_passphrase_text_changed(self):
        if not is_valid_passphrase(self.edit_passphrase.toPlainText()):
            self.label_passphrase_error.setText(translate("TEXT_RECOVERY_INVALID_PASSPHRASE"))
            self.label_passphrase_error.show()
        else:
            self.label_passphrase_error.hide()
        self._check_infos()

    def _check_infos(self):
        self.info_filled.emit(
            is_valid_passphrase(self.edit_passphrase.toPlainText())
            and len(self.label_key_file.text()) > 0
        )

    def get_recovery_password(self):
        return get_recovery_password_from_passphrase(self.edit_passphrase.toPlainText())

    def get_recovery_key(self):
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
        self.initial_device = None
        self.button_validate.clicked.connect(self._on_validate_clicked)
        self.button_validate.setEnabled(False)
        self.current_page = DeviceRecoveryImportPage1Widget(self)
        self.current_page.info_filled.connect(self._on_page1_info_filled)
        self.main_layout.addWidget(self.current_page)
        self.create_new_device_success.connect(self._on_create_new_device_success)
        self.create_new_device_failure.connect(self._on_create_new_device_failure)

    def _on_page1_info_filled(self, valid):
        self.button_validate.setEnabled(valid)

    def _on_page2_info_filled(self, valid):
        self.button_validate.setEnabled(valid)

    async def _create_new_device(self, config_dir, initial_device, device_label, password):
        try:
            await create_new_device_from_original(
                initial_device, device_label, password, config_dir=config_dir
            )
            show_info(self, translate("TEXT_RECOVERY_IMPORT_SUCCESS"))
        except RecoveryLocalDeviceError as exc:
            show_error(self, translate("IMPORT_KEY_LOCAL_DEVICE_ERROR"), exception=exc)
        except RecoveryBackendError as exc:
            show_error(self, translate("IMPORT_KEY_BACKEND_ERROR"), exception=exc)
        except Exception as exc:
            show_error(self, translate("IMPORT_KEY_ERROR"), exception=exc)

    def _on_validate_clicked(self):
        if isinstance(self.current_page, DeviceRecoveryImportPage1Widget):
            try:
                self.button_validate.setEnabled(False)
                device_file = load_device_file(Path(self.current_page.get_recovery_key()))
                self.initial_device = load_device_with_password(
                    device_file.key_file_path, password=self.current_page.get_recovery_password()
                )
                assert self.initial_device
            except AssertionError as exc:
                show_error(self, translate("TEXT_INVALID_DEVICE_KEY"), exception=exc)
            except LocalDeviceError as exc:
                if "Decryption failed" in str(exc):
                    show_error(self, translate("TEXT_IMPORT_KEY_WRONG_PASSPHRASE"), exception=exc)
                else:
                    show_error(self, translate("IMPORT_KEY_LOCAL_DEVICE_ERROR"), exception=exc)
            except Exception as exc:
                show_error(self, translate("IMPORT_KEY_ERROR"), exception=exc)
            else:
                self.main_layout.removeWidget(self.current_page)
                self.current_page = DeviceRecoveryImportPage2Widget(parent=self)
                self.current_page.info_filled.connect(self._on_page2_info_filled)
                self.main_layout.addWidget(self.current_page)
                self.button_validate.setText(translate("ACTION_CREATE_DEVICE"))
        else:
            self.jobs_ctx.submit_job(
                self.create_new_device_success,
                self.create_new_device_failure,
                self._create_new_device,
                config_dir=self.config.config_dir,
                initial_device=self.initial_device,
                device_label=self.current_page.get_device_name(),
                password=self.current_page.get_password(),
            )

    def _on_create_new_device_success(self, job):
        self.dialog.accept()

    def _on_create_new_device_failure(self, job):
        pass

    @classmethod
    def show_modal(cls, config, jobs_ctx, parent, on_finished):
        w = cls(config=config, jobs_ctx=jobs_ctx)
        d = GreyedDialog(
            w, translate("TEXT_DEVICE_RECOVERY_EXPORT_WIZARD_TITLE"), parent=parent, width=800
        )
        w.dialog = d

        def _on_finished(result):
            return on_finished()

        d.finished.connect(_on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
