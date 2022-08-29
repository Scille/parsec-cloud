# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from pathlib import Path, PurePath

from parsec.core.recovery import generate_recovery_device
from parsec.core.backend_connection import BackendConnectionError, BackendNotAvailable
from parsec.core.types import LocalDevice
from parsec.core.local_device import (
    load_device_with_password,
    load_device_with_smartcard,
    LocalDeviceError,
    get_recovery_device_file_name,
    save_recovery_device,
    get_available_device,
    LocalDeviceAlreadyExistsError,
    DeviceFileType,
)
from parsec.core.gui.trio_jobs import QtToTrioJob, JobResultError
from parsec.core.gui.lang import translate
from parsec.core.gui.desktop import open_files_job
from parsec.core.gui.custom_dialogs import (
    GreyedDialog,
    QDialogInProcess,
    show_error,
    get_text_input,
)

from parsec.core.gui.ui.device_recovery_export_widget import Ui_DeviceRecoveryExportWidget
from parsec.core.gui.ui.device_recovery_export_page1_widget import (
    Ui_DeviceRecoveryExportPage1Widget,
)
from parsec.core.gui.ui.device_recovery_export_page2_widget import (
    Ui_DeviceRecoveryExportPage2Widget,
)


class DeviceRecoveryExportPage2Widget(QWidget, Ui_DeviceRecoveryExportPage2Widget):
    def __init__(self, jobs_ctx, device, save_path, passphrase, parent=None):
        super().__init__(parent=parent)
        self.setupUi(self)
        self.button_print.clicked.connect(self._print_recovery_key)
        self.device = device
        self.jobs_ctx = jobs_ctx
        self.passphrase = passphrase
        self.label_file_path.setText(str(save_path))
        font = self.label_file_path.font()
        font.setUnderline(True)
        self.label_file_path.setFont(font)
        self.label_file_path.setStyleSheet("color: #0092FF;")
        self.label_file_path.clicked.connect(self._on_path_clicked)
        self.edit_passphrase.setText(passphrase)

    def _on_path_clicked(self, file_path):
        self.jobs_ctx.submit_job(None, None, open_files_job, [PurePath(file_path).parent])

    def _print_recovery_key(self):
        html = translate("TEXT_RECOVERY_HTML_EXPORT_user-organization-keyname-password").format(
            organization=self.device.organization_id,
            label=self.device.user_display,
            password=self.passphrase,
            keyname=PurePath(self.label_file_path.text()).name,
        )
        QDialogInProcess.print_html(self, html)


class DeviceRecoveryExportPage1Widget(QWidget, Ui_DeviceRecoveryExportPage1Widget):
    info_filled = pyqtSignal(bool)

    def __init__(self, devices, parent=None):
        super().__init__(parent=parent)
        self.setupUi(self)
        self.button_select_file.clicked.connect(self._on_select_file_clicked)
        self.devices = {device.slug: device for device in devices}
        for device in devices:
            # We consider it's unlikely to have multiple devices for the same user
            # so we don't show the device label for better readability
            self.combo_devices.addItem(
                f"{device.organization_id} - {device.user_display}", device.slug
            )

    def _on_select_file_clicked(self):
        key_dir = QDialogInProcess.getExistingDirectory(
            self, translate("TEXT_EXPORT_KEY"), str(Path.home())
        )
        if key_dir:
            self.label_file_path.setText(key_dir)
        self._check_infos()

    def _check_infos(self):
        self.info_filled.emit(len(self.label_file_path.text()) > 0)

    def get_selected_device(self):
        return self.devices[self.combo_devices.currentData()]

    def get_save_path(self):
        return PurePath(self.label_file_path.text())


class DeviceRecoveryExportWidget(QWidget, Ui_DeviceRecoveryExportWidget):
    export_success = pyqtSignal(QtToTrioJob)
    export_failure = pyqtSignal(QtToTrioJob)

    def __init__(self, config, jobs_ctx, devices, parent=None):
        super().__init__(parent=parent)
        self.setupUi(self)
        self.dialog = None
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.button_validate.clicked.connect(self._on_validate_clicked)
        self.button_validate.setEnabled(False)
        self.current_page = DeviceRecoveryExportPage1Widget(devices, self)
        self.current_page.info_filled.connect(self._on_page1_info_filled)
        self.main_layout.addWidget(self.current_page)
        self.export_success.connect(self._on_export_success)
        self.export_failure.connect(self._on_export_failure)

    def _on_export_success(self, job):
        recovery_device, file_path, passphrase = job.ret
        self.main_layout.removeWidget(self.current_page)
        self.current_page = DeviceRecoveryExportPage2Widget(
            self.jobs_ctx,
            device=recovery_device,
            save_path=file_path,
            passphrase=passphrase,
            parent=self,
        )
        self.main_layout.addWidget(self.current_page)
        self.button_validate.setText(translate("ACTION_CLOSE"))
        self.button_validate.setEnabled(True)

    def _on_export_failure(self, job):
        self.button_validate.setEnabled(True)

    def _on_page1_info_filled(self, valid):
        self.button_validate.setEnabled(valid)

    async def _export_recovery_device(self, config_dir, device, export_path):
        try:
            recovery_device = await generate_recovery_device(device)
            file_name = get_recovery_device_file_name(recovery_device)
            file_path = export_path / file_name
            passphrase = await save_recovery_device(file_path, recovery_device)
            return recovery_device, file_path, passphrase
        except BackendNotAvailable as exc:
            show_error(self, translate("EXPORT_KEY_BACKEND_OFFLINE"), exception=exc)
            raise JobResultError("backend-error") from exc
        except BackendConnectionError as exc:
            show_error(self, translate("EXPORT_KEY_BACKEND_ERROR"), exception=exc)
            raise JobResultError("backend-error") from exc
        except LocalDeviceAlreadyExistsError as exc:
            show_error(self, translate("TEXT_RECOVERY_DEVICE_FILE_ALREADY_EXISTS"), exception=exc)
            raise JobResultError("already-exists") from exc
        except Exception as exc:
            show_error(self, translate("EXPORT_KEY_ERROR"), exception=exc)
            raise JobResultError("error") from exc
        self.button_validate.setEnabled(True)

    async def _on_validate_clicked(self):
        if isinstance(self.current_page, DeviceRecoveryExportPage1Widget):
            self.button_validate.setEnabled(False)
            selected_device = self.current_page.get_selected_device()
            save_path = self.current_page.get_save_path()
            device = None

            if isinstance(selected_device, LocalDevice):
                selected_device = get_available_device(self.config.config_dir, selected_device)

            if selected_device.type == DeviceFileType.PASSWORD:
                password = get_text_input(
                    self,
                    translate("TEXT_DEVICE_UNLOCK_TITLE"),
                    translate("TEXT_DEVICE_UNLOCK_FOR_RECOVERY_LABEL"),
                    placeholder="",
                    default_text="",
                    completion=None,
                    button_text=None,
                    validator=None,
                    hidden=True,
                )
                if password is None:
                    self.button_validate.setEnabled(True)
                    return
                try:
                    device = load_device_with_password(selected_device.key_file_path, password)
                except LocalDeviceError:
                    show_error(self, translate("TEXT_LOGIN_ERROR_AUTHENTICATION_FAILED"))
                    self.button_validate.setEnabled(True)
                    return
            elif selected_device.type == DeviceFileType.SMARTCARD:
                try:
                    device = await load_device_with_smartcard(selected_device.key_file_path)
                except LocalDeviceError as exc:
                    show_error(
                        self, translate("TEXT_LOGIN_ERROR_AUTHENTICATION_FAILED"), exception=exc
                    )
                    self.button_validate.setEnabled(True)
                    return
                except ModuleNotFoundError as exc:
                    show_error(
                        self, translate("TEXT_UNLOCK_ERROR_SMARTCARD_NOT_AVAILABLE"), exception=exc
                    )
                    self.button_validate.setEnabled(True)
                    return
            self.jobs_ctx.submit_job(
                (self, "export_success"),
                (self, "export_failure"),
                self._export_recovery_device,
                config_dir=self.config.config_dir,
                device=device,
                export_path=save_path,
            )
        else:
            self.dialog.accept()

    @classmethod
    def show_modal(cls, config, jobs_ctx, devices, parent):
        w = cls(config=config, jobs_ctx=jobs_ctx, devices=devices)
        d = GreyedDialog(
            w, translate("TEXT_DEVICE_RECOVERY_EXPORT_WIZARD_TITLE"), parent=parent, width=800
        )
        w.dialog = d

        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
