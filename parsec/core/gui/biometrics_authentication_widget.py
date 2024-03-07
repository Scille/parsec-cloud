# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Literal

from PyQt5 import QtCore, QtGui, QtWidgets
from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget


class Ui_BiometricsAuthenticationWidget(object):
    def setupUi(self, BiometricsAuthenticationWidget: BiometricsAuthenticationWidget) -> None:
        BiometricsAuthenticationWidget.setObjectName("BiometricsAuthenticationWidget")
        BiometricsAuthenticationWidget.resize(389, 125)
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(BiometricsAuthenticationWidget)
        self.verticalLayout_2.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_2.setSpacing(5)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.verticalLayout = QtWidgets.QVBoxLayout()
        self.verticalLayout.setContentsMargins(-1, -1, -1, 0)
        self.verticalLayout.setSpacing(5)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label = QtWidgets.QLabel(BiometricsAuthenticationWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Minimum
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label.sizePolicy().hasHeightForWidth())
        self.label.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(False)
        font.setWeight(50)
        self.label.setFont(font)
        self.label.setWordWrap(True)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        spacerItem = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem)
        self.verticalLayout_2.addLayout(self.verticalLayout)

        self.retranslateUi(BiometricsAuthenticationWidget)
        QtCore.QMetaObject.connectSlotsByName(BiometricsAuthenticationWidget)

    def retranslateUi(self, BiometricsAuthenticationWidget: BiometricsAuthenticationWidget) -> None:
        _translate = QtCore.QCoreApplication.translate
        BiometricsAuthenticationWidget.setWindowTitle(
            _translate("BiometricsAuthenticationWidget", "Form")
        )
        self.label.setText(_translate("BiometricsAuthenticationWidget", "TEXT_EXPLAIN_BIOMETRICS"))


class BiometricsAuthenticationWidget(QWidget, Ui_BiometricsAuthenticationWidget):
    authentication_state_changed = pyqtSignal(bool)

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setupUi(self)

    def is_auth_valid(self) -> Literal[True]:
        return True
