# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/register_device.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_RegisterDevice(object):
    def setupUi(self, RegisterDevice):
        RegisterDevice.setObjectName("RegisterDevice")
        RegisterDevice.resize(400, 300)
        self.verticalLayoutWidget = QtWidgets.QWidget(RegisterDevice)
        self.verticalLayoutWidget.setGeometry(QtCore.QRect(9, 9, 381, 221))
        self.verticalLayoutWidget.setObjectName("verticalLayoutWidget")
        self.verticalLayout = QtWidgets.QVBoxLayout(self.verticalLayoutWidget)
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.device_name = QtWidgets.QLineEdit(self.verticalLayoutWidget)
        self.device_name.setObjectName("device_name")
        self.horizontalLayout_2.addWidget(self.device_name)
        self.verticalLayout.addLayout(self.horizontalLayout_2)
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.password = QtWidgets.QLineEdit(self.verticalLayoutWidget)
        self.password.setEchoMode(QtWidgets.QLineEdit.Password)
        self.password.setObjectName("password")
        self.horizontalLayout.addWidget(self.password)
        self.verticalLayout.addLayout(self.horizontalLayout)
        spacerItem = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem)
        self.outcome_panel = QtWidgets.QWidget(self.verticalLayoutWidget)
        self.outcome_panel.setObjectName("outcome_panel")
        self.outcome_panel_layout = QtWidgets.QHBoxLayout(self.outcome_panel)
        self.outcome_panel_layout.setObjectName("outcome_panel_layout")
        self.outcome_status = QtWidgets.QLabel(self.outcome_panel)
        self.outcome_status.setWordWrap(True)
        self.outcome_status.setObjectName("outcome_status")
        self.outcome_panel_layout.addWidget(self.outcome_status)
        self.verticalLayout.addWidget(self.outcome_panel)
        self.config_waiter_panel = QtWidgets.QWidget(self.verticalLayoutWidget)
        self.config_waiter_panel.setObjectName("config_waiter_panel")
        self.config_waiter_panel_layout = QtWidgets.QVBoxLayout(self.config_waiter_panel)
        self.config_waiter_panel_layout.setObjectName("config_waiter_panel_layout")
        self.horizontalLayout_4 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_4.setObjectName("horizontalLayout_4")
        self.label = QtWidgets.QLabel(self.config_waiter_panel)
        self.label.setObjectName("label")
        self.horizontalLayout_4.addWidget(self.label)
        self.device_token = QtWidgets.QLineEdit(self.config_waiter_panel)
        self.device_token.setObjectName("device_token")
        self.horizontalLayout_4.addWidget(self.device_token)
        self.config_waiter_panel_layout.addLayout(self.horizontalLayout_4)
        self.config_waiter_label = QtWidgets.QLabel(self.config_waiter_panel)
        self.config_waiter_label.setWordWrap(True)
        self.config_waiter_label.setObjectName("config_waiter_label")
        self.config_waiter_panel_layout.addWidget(self.config_waiter_label)
        self.config_waiter_progress = QtWidgets.QProgressBar(self.config_waiter_panel)
        self.config_waiter_progress.setEnabled(True)
        self.config_waiter_progress.setLayoutDirection(QtCore.Qt.LeftToRight)
        self.config_waiter_progress.setMaximum(0)
        self.config_waiter_progress.setProperty("value", 0)
        self.config_waiter_progress.setTextVisible(False)
        self.config_waiter_progress.setOrientation(QtCore.Qt.Horizontal)
        self.config_waiter_progress.setInvertedAppearance(False)
        self.config_waiter_progress.setObjectName("config_waiter_progress")
        self.config_waiter_panel_layout.addWidget(self.config_waiter_progress)
        self.verticalLayout.addWidget(self.config_waiter_panel)
        self.button_register_device = QtWidgets.QPushButton(RegisterDevice)
        self.button_register_device.setGeometry(QtCore.QRect(270, 250, 99, 27))
        self.button_register_device.setObjectName("button_register_device")

        self.retranslateUi(RegisterDevice)
        QtCore.QMetaObject.connectSlotsByName(RegisterDevice)

    def retranslateUi(self, RegisterDevice):
        _translate = QtCore.QCoreApplication.translate
        RegisterDevice.setWindowTitle(_translate("RegisterDevice", "Register new device"))
        self.device_name.setPlaceholderText(_translate("RegisterDevice", "Device name"))
        self.password.setPlaceholderText(_translate("RegisterDevice", "Password"))
        self.outcome_status.setText(_translate("RegisterDevice", "TextLabel"))
        self.label.setText(_translate("RegisterDevice", "Device's token"))
        self.device_token.setPlaceholderText(_translate("RegisterDevice", "Token"))
        self.config_waiter_label.setText(
            _translate("RegisterDevice", "Waiting for the new device...")
        )
        self.button_register_device.setText(_translate("RegisterDevice", "OK"))
