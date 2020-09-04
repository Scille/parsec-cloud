# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget, QMenu, QGraphicsDropShadowEffect, QLabel
from PyQt5.QtGui import QColor

from parsec.core.backend_connection import BackendNotAvailable, BackendConnectionError
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui.greet_device_widget import GreetDeviceWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import ensure_string_size
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.password_change_widget import PasswordChangeWidget
from parsec.core.gui.flow_layout import FlowLayout
from parsec.core.gui.ui.devices_widget import Ui_DevicesWidget
from parsec.core.gui.ui.device_button import Ui_DeviceButton

DEVICES_PER_PAGE = 100


class DeviceButton(QWidget, Ui_DeviceButton):
    change_password_clicked = pyqtSignal()

    def __init__(self, device_info, is_current_device):
        super().__init__()
        self.setupUi(self)
        self.is_current_device = is_current_device
        self.device_info = device_info
        self.label_icon.apply_style()

        self.label_device_name.setText(
            ensure_string_size(self.device_info.device_display, 260, self.label_device_name.font())
        )
        self.label_device_name.setToolTip(self.device_info.device_display)
        if self.is_current_device:
            self.label_is_current.setText("({})".format(_("TEXT_DEVICE_IS_CURRENT")))

        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)
        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)

    def show_context_menu(self, pos):
        if not self.is_current_device:
            return
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)
        action = menu.addAction(_("ACTION_DEVICE_MENU_CHANGE_PASSWORD"))
        action.triggered.connect(self.change_password)
        menu.exec_(global_pos)

    def change_password(self):
        self.change_password_clicked.emit()


async def _do_invite_device(core):
    try:
        return await core.new_device_invitation(send_email=False)
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


async def _do_list_devices(core, page, pattern=None):
    try:
        if pattern is None:
            return await core.get_user_devices_info(per_page=DEVICES_PER_PAGE, page=page)
        else:
            devices, total = await core.get_user_devices_info(per_page=DEVICES_PER_PAGE, page=page)
            for device in devices:
                if not device.device_display.startswith(pattern):
                    devices.remove(device)
        return devices, total
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


class DevicesWidget(QWidget, Ui_DevicesWidget):
    list_success = pyqtSignal(QtToTrioJob)
    list_error = pyqtSignal(QtToTrioJob)

    invite_success = pyqtSignal(QtToTrioJob)
    invite_error = pyqtSignal(QtToTrioJob)

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.core = core
        self.event_bus = event_bus
        self.layout_devices = FlowLayout(spacing=40)
        self.layout_content.addLayout(self.layout_devices)
        self.button_add_device.clicked.connect(self.invite_device)
        self.button_add_device.apply_style()
        self.button_previous_page.clicked.connect(self.show_previous_page)
        self.button_next_page.clicked.connect(self.show_next_page)
        self.list_success.connect(self._on_list_success)
        self.list_error.connect(self._on_list_error)
        self.invite_success.connect(self._on_invite_success)
        self.invite_error.connect(self._on_invite_error)
        self.button_devices_filter.clicked.connect(self.on_filter)
        self.line_edit_search.editingFinished.connect(lambda: self.on_filter(editing_finished=True))
        self.line_edit_search.textChanged.connect(lambda: self.on_filter(text_changed=True))

    def show(self):
        self._page = 1
        self.reset()
        super().show()

    def show_next_page(self):
        self._page += 1
        self.reset()

    def show_previous_page(self):
        if self._page > 1:
            self._page -= 1
        self.reset()

    def on_filter(self, editing_finished=False, text_changed=False):
        self._page = 1
        pattern = self.line_edit_search.text()
        if text_changed and len(pattern) <= 0:
            return self.reset()
        elif text_changed:
            return
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "list_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "list_error", QtToTrioJob),
            _do_list_devices,
            core=self.core,
            page=self._page,
            pattern=pattern,
        )

    def change_password(self):
        PasswordChangeWidget.show_modal(core=self.core, parent=self, on_finished=None)

    def invite_device(self):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "invite_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "invite_error", QtToTrioJob),
            _do_invite_device,
            core=self.core,
        )

    def _on_invite_success(self, job):
        assert job.is_finished()
        assert job.status == "ok"

        GreetDeviceWidget.show_modal(
            core=self.core,
            jobs_ctx=self.jobs_ctx,
            invite_addr=job.ret,
            parent=self,
            on_finished=self.reset,
        )

    def _on_invite_error(self, job):
        assert job.is_finished()
        assert job.status != "ok"

        status = job.status
        if status == "offline":
            errmsg = _("TEXT_INVITE_DEVICE_INVITE_OFFLINE")
        else:
            errmsg = _("TEXT_INVITE_DEVICE_INVITE_ERROR")

        show_error(self, errmsg, exception=job.exc)

    def add_device(self, device_info, is_current_device):
        button = DeviceButton(device_info, is_current_device)
        self.layout_devices.addWidget(button)
        button.change_password_clicked.connect(self.change_password)
        button.show()

    def pagination(self, total: int):
        """Show/activate or hide/deactivate previous and next page button"""
        if total > DEVICES_PER_PAGE:
            self.button_previous_page.show()
            self.button_next_page.show()
            if self._page * DEVICES_PER_PAGE >= total:
                self.button_next_page.setEnabled(False)
            else:
                self.button_next_page.setEnabled(True)
            if self._page <= 1:
                self.button_previous_page.setEnabled(False)
            else:
                self.button_previous_page.setEnabled(True)
        else:
            self.button_previous_page.hide()
            self.button_next_page.hide()

    def _on_list_success(self, job):
        assert job.is_finished()
        assert job.status == "ok"

        devices, total = job.ret
        # Securing if page go to far
        if total == 0 and self._page > 1:
            self._page -= 1
            self.reset()
        current_device = self.core.device
        self.layout_devices.clear()
        for device in devices:
            self.add_device(device, is_current_device=current_device.device_id == device.device_id)
        self.spinner.hide()
        self.pagination(total=total)
        # Show/activate or hide/deactivate previous and next page button
        self.pagination(total=total)

    def _on_list_error(self, job):
        assert job.is_finished()
        assert job.status != "ok"

        status = job.status
        if status in ["error", "offline"]:
            self.layout_devices.clear()
            label = QLabel(_("TEXT_DEVICE_LIST_RETRIEVABLE_FAILURE"))
            label.setAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
            self.layout_devices.addWidget(label)
        self.spinner.hide()

    def reset(self):
        self.layout_devices.clear()
        self.button_previous_page.hide()
        self.button_next_page.hide()
        self.spinner.show()
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "list_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "list_error", QtToTrioJob),
            _do_list_devices,
            core=self.core,
            page=self._page,
        )
