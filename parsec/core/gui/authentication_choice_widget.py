# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, List, cast

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec._parsec import DeviceFileType
from parsec.core.gui.lang import translate
from parsec.core.gui.password_authentication_widget import PasswordAuthenticationWidget
from parsec.core.gui.smartcard_authentication_widget import SmartCardAuthenticationWidget
from parsec.core.gui.ui.authentication_choice_widget import Ui_AuthenticationChoiceWidget
from parsec.core.local_device import is_smartcard_extension_available


class AuthenticationChoiceWidget(QWidget, Ui_AuthenticationChoiceWidget):
    authentication_state_changed: pyqtSignal = pyqtSignal(DeviceFileType, bool)

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setupUi(self)
        self.combo_auth_method.addItem(
            translate("TEXT_AUTH_METHOD_PASSWORD"), DeviceFileType.PASSWORD
        )
        if is_smartcard_extension_available():
            self.combo_auth_method.addItem(
                translate("TEXT_AUTH_METHOD_SMARTCARD"), DeviceFileType.SMARTCARD
            )
        self.combo_auth_method.setCurrentIndex(0)
        if self.combo_auth_method.count() == 1:
            self.combo_auth_method.setEnabled(False)
            self.combo_auth_method.setToolTip(translate("TEXT_ONLY_ONE_AUTH_METHOD_AVAILABLE"))
        self.auth_widgets: dict[
            DeviceFileType,
            PasswordAuthenticationWidget | SmartCardAuthenticationWidget,
        ] = {
            DeviceFileType.PASSWORD: PasswordAuthenticationWidget(),
            DeviceFileType.SMARTCARD: SmartCardAuthenticationWidget(),
        }
        self.current_auth_method: DeviceFileType = DeviceFileType.PASSWORD

        PasswordAuthenticationWidget, self.auth_widgets[
            DeviceFileType.PASSWORD
        ].authentication_state_changed.connect(self._on_password_state_changed)
        self.auth_widgets[DeviceFileType.SMARTCARD].authentication_state_changed.connect(
            self._on_smartcard_state_changed
        )
        self.main_layout.addWidget(self.auth_widgets[self.current_auth_method])
        self.combo_auth_method.currentIndexChanged.connect(self._on_auth_method_changed)

    def _on_auth_method_changed(self, idx: int) -> None:
        self.current_auth_method = self.combo_auth_method.itemData(idx)
        item = self.main_layout.takeAt(0)
        if item and item.widget():
            item.widget().setParent(None)
        self.main_layout.addWidget(self.auth_widgets[self.current_auth_method])
        self.authentication_state_changed.emit(
            self.current_auth_method, self.auth_widgets[self.current_auth_method].is_auth_valid()
        )

    def _on_password_state_changed(self, state: Any) -> None:
        self.authentication_state_changed.emit(self.current_auth_method, state)

    def _on_smartcard_state_changed(self, state: Any) -> None:
        self.authentication_state_changed.emit(self.current_auth_method, state)

    def exclude_strings(self, strings: List[str]) -> None:
        cast(
            PasswordAuthenticationWidget, self.auth_widgets[DeviceFileType.PASSWORD]
        ).set_excluded_strings(strings)

    def get_auth_method(self) -> DeviceFileType:
        return self.current_auth_method

    def get_auth(self) -> str:
        if self.current_auth_method == DeviceFileType.PASSWORD:
            return cast(
                PasswordAuthenticationWidget, self.auth_widgets[DeviceFileType.PASSWORD]
            ).password
        return ""

    def is_auth_valid(self) -> bool:
        return self.auth_widgets[self.current_auth_method].is_auth_valid()
