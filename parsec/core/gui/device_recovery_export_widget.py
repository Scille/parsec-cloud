# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path, PurePath
from typing import Sequence

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core import CoreConfig
from parsec.core.backend_connection import BackendConnectionError, BackendNotAvailable
from parsec.core.gui.custom_dialogs import (
    GreyedDialog,
    QDialogInProcess,
    get_text_input,
    show_error,
)
from parsec.core.gui.desktop import open_files_job
from parsec.core.gui.lang import translate
from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.ui.device_recovery_export_page1_widget import (
    Ui_DeviceRecoveryExportPage1Widget,
)
from parsec.core.gui.ui.device_recovery_export_page2_widget import (
    Ui_DeviceRecoveryExportPage2Widget,
)
from parsec.core.gui.ui.device_recovery_export_widget import Ui_DeviceRecoveryExportWidget
from parsec.core.local_device import (
    RECOVERY_DEVICE_FILE_SUFFIX,
    AvailableDevice,
    DeviceFileType,
    LocalDeviceError,
    get_available_device,
    get_recovery_device_file_name,
    load_device_with_password,
    load_device_with_smartcard,
    save_recovery_device,
)
from parsec.core.recovery import generate_recovery_device
from parsec.core.types import LocalDevice


class DeviceRecoveryExportPage2Widget(QWidget, Ui_DeviceRecoveryExportPage2Widget):
    def __init__(
        self,
        jobs_ctx: QtToTrioJobScheduler,
        device: LocalDevice,
        save_path: PurePath,
        passphrase: str,
        parent: QWidget | None = None,
    ) -> None:
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

    def _on_path_clicked(self, file_path: str) -> None:
        _ = self.jobs_ctx.submit_job(None, None, open_files_job, [PurePath(file_path).parent])

    def _print_recovery_key(self) -> None:
        html = translate("TEXT_RECOVERY_HTML_EXPORT_user-organization-keyname-password").format(
            organization=self.device.organization_id.str,
            label=self.device.user_display,
            password=self.passphrase,
            keyname=PurePath(self.label_file_path.text()).name,
        )
        QDialogInProcess.print_html(self, html)


class DeviceRecoveryExportPage1Widget(QWidget, Ui_DeviceRecoveryExportPage1Widget):
    info_filled = pyqtSignal(bool)

    def __init__(self, devices: Sequence[AvailableDevice], parent: QWidget | None = None) -> None:
        super().__init__(parent=parent)
        self.setupUi(self)
        self.button_select_file.clicked.connect(self._on_select_file_clicked)
        self.devices = {device.slug: device for device in devices}
        for device in devices:
            # We consider it's unlikely to have multiple devices for the same user
            # so we don't show the device label for better readability
            self.combo_devices.addItem(
                f"{device.organization_id.str} - {device.user_display}", device.slug
            )

    def _on_select_file_clicked(self) -> None:
        key_file, _ = QDialogInProcess.getSaveFileName(
            self,
            translate("TEXT_EXPORT_KEY"),
            str(Path.home() / get_recovery_device_file_name(self.get_selected_device())),
            f"*{RECOVERY_DEVICE_FILE_SUFFIX}",
        )
        if key_file:
            self.label_file_path.setText(key_file)
        self._check_infos()

    def _check_infos(self) -> None:
        self.info_filled.emit(len(self.label_file_path.text()) > 0)

    def get_selected_device(self) -> AvailableDevice:
        return self.devices[self.combo_devices.currentData()]

    def get_save_path(self) -> PurePath:
        return PurePath(self.label_file_path.text())


class DeviceRecoveryExportWidget(QWidget, Ui_DeviceRecoveryExportWidget):
    export_success = pyqtSignal(QtToTrioJob)
    export_failure = pyqtSignal(QtToTrioJob)

    def __init__(
        self,
        config: CoreConfig,
        jobs_ctx: QtToTrioJobScheduler,
        devices: Sequence[AvailableDevice],
        parent: QWidget | None = None,
    ) -> None:
        super().__init__(parent=parent)
        self.setupUi(self)
        self.dialog: GreyedDialog | None = None
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.button_validate.clicked.connect(self._on_validate_clicked)
        self.button_validate.setEnabled(False)
        self.current_page: DeviceRecoveryExportPage1Widget | DeviceRecoveryExportPage2Widget = (
            DeviceRecoveryExportPage1Widget(devices, self)
        )
        self.current_page.info_filled.connect(self._on_page1_info_filled)
        self.main_layout.addWidget(self.current_page)
        self.export_success.connect(self._on_export_success)
        self.export_failure.connect(self._on_export_failure)

    def _on_export_success(self, job: QtToTrioJob[tuple[LocalDevice, PurePath, str]]) -> None:
        assert job.ret is not None
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

    def _on_export_failure(self, job: QtToTrioJob[tuple[LocalDevice, PurePath, str]]) -> None:
        self.button_validate.setEnabled(True)

    def _on_page1_info_filled(self, valid: bool) -> None:
        self.button_validate.setEnabled(valid)

    async def _export_recovery_device(
        self, config_dir: PurePath, device: LocalDevice, export_path: PurePath
    ) -> tuple[LocalDevice, PurePath, str]:
        try:
            recovery_device = await generate_recovery_device(device)
            passphrase = await save_recovery_device(export_path, recovery_device, force=True)
            return recovery_device, export_path, passphrase
        except BackendNotAvailable as exc:
            show_error(self, translate("EXPORT_KEY_BACKEND_OFFLINE"), exception=exc)
            raise JobResultError("backend-error") from exc
        except BackendConnectionError as exc:
            show_error(self, translate("EXPORT_KEY_BACKEND_ERROR"), exception=exc)
            raise JobResultError("backend-error") from exc
        except Exception as exc:
            show_error(self, translate("EXPORT_KEY_ERROR"), exception=exc)
            raise JobResultError("error") from exc

    async def _on_validate_clicked(self) -> None:
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
            assert device is not None
            _ = self.jobs_ctx.submit_job(
                (self, "export_success"),
                (self, "export_failure"),
                self._export_recovery_device,
                config_dir=self.config.config_dir,
                device=device,
                export_path=save_path,
            )
        else:
            assert self.dialog is not None
            self.dialog.accept()

    @classmethod
    def show_modal(
        cls,
        config: CoreConfig,
        jobs_ctx: QtToTrioJobScheduler,
        devices: Sequence[AvailableDevice],
        parent: QWidget,
    ) -> DeviceRecoveryExportWidget:
        w = cls(config=config, jobs_ctx=jobs_ctx, devices=devices)
        d = GreyedDialog(
            w, translate("TEXT_DEVICE_RECOVERY_EXPORT_WIZARD_TITLE"), parent=parent, width=800
        )
        w.dialog = d

        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
