# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/login_register_device_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_LoginRegisterDeviceWidget(object):
    def setupUi(self, LoginRegisterDeviceWidget):
        LoginRegisterDeviceWidget.setObjectName("LoginRegisterDeviceWidget")
        LoginRegisterDeviceWidget.resize(508, 397)
        self.verticalLayout = QtWidgets.QVBoxLayout(LoginRegisterDeviceWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.group_config_device = QtWidgets.QGroupBox(LoginRegisterDeviceWidget)
        self.group_config_device.setObjectName("group_config_device")
        self.verticalLayout_4 = QtWidgets.QVBoxLayout(self.group_config_device)
        self.verticalLayout_4.setObjectName("verticalLayout_4")
        self.device_label = QtWidgets.QLabel(self.group_config_device)
        self.device_label.setWordWrap(True)
        self.device_label.setObjectName("device_label")
        self.verticalLayout_4.addWidget(self.device_label)
        self.line_edit_login = QtWidgets.QLineEdit(self.group_config_device)
        self.line_edit_login.setObjectName("line_edit_login")
        self.verticalLayout_4.addWidget(self.line_edit_login)
        self.line_edit_password = QtWidgets.QLineEdit(self.group_config_device)
        self.line_edit_password.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_password.setObjectName("line_edit_password")
        self.verticalLayout_4.addWidget(self.line_edit_password)
        self.line_edit_password_check = QtWidgets.QLineEdit(self.group_config_device)
        self.line_edit_password_check.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_password_check.setObjectName("line_edit_password_check")
        self.verticalLayout_4.addWidget(self.line_edit_password_check)
        self.check_box_use_nitrokey = QtWidgets.QCheckBox(self.group_config_device)
        self.check_box_use_nitrokey.setObjectName("check_box_use_nitrokey")
        self.verticalLayout_4.addWidget(self.check_box_use_nitrokey)
        self.widget_nitrokey = QtWidgets.QWidget(self.group_config_device)
        self.widget_nitrokey.setObjectName("widget_nitrokey")
        self.formLayout = QtWidgets.QFormLayout(self.widget_nitrokey)
        self.formLayout.setObjectName("formLayout")
        self.label = QtWidgets.QLabel(self.widget_nitrokey)
        self.label.setObjectName("label")
        self.formLayout.setWidget(0, QtWidgets.QFormLayout.LabelRole, self.label)
        self.label_2 = QtWidgets.QLabel(self.widget_nitrokey)
        self.label_2.setObjectName("label_2")
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.LabelRole, self.label_2)
        self.combo_nitrokey_token = QtWidgets.QComboBox(self.widget_nitrokey)
        self.combo_nitrokey_token.setObjectName("combo_nitrokey_token")
        self.formLayout.setWidget(0, QtWidgets.QFormLayout.FieldRole, self.combo_nitrokey_token)
        self.combo_nitrokey_key = QtWidgets.QComboBox(self.widget_nitrokey)
        self.combo_nitrokey_key.setObjectName("combo_nitrokey_key")
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.FieldRole, self.combo_nitrokey_key)
        self.verticalLayout_4.addWidget(self.widget_nitrokey)
        self.line_edit_device = QtWidgets.QLineEdit(self.group_config_device)
        self.line_edit_device.setObjectName("line_edit_device")
        self.verticalLayout_4.addWidget(self.line_edit_device)
        self.line_edit_token = QtWidgets.QLineEdit(self.group_config_device)
        self.line_edit_token.setObjectName("line_edit_token")
        self.verticalLayout_4.addWidget(self.line_edit_token)
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_2.addItem(spacerItem)
        self.button_register = QtWidgets.QPushButton(self.group_config_device)
        self.button_register.setEnabled(False)
        self.button_register.setObjectName("button_register")
        self.horizontalLayout_2.addWidget(self.button_register)
        self.verticalLayout_4.addLayout(self.horizontalLayout_2)
        self.label_error = QtWidgets.QLabel(self.group_config_device)
        self.label_error.setText("")
        self.label_error.setObjectName("label_error")
        self.verticalLayout_4.addWidget(self.label_error)
        self.verticalLayout.addWidget(self.group_config_device)

        self.retranslateUi(LoginRegisterDeviceWidget)
        self.check_box_use_nitrokey.toggled["bool"].connect(self.widget_nitrokey.setVisible)
        self.check_box_use_nitrokey.toggled["bool"].connect(self.line_edit_password.setDisabled)
        self.check_box_use_nitrokey.toggled["bool"].connect(
            self.line_edit_password_check.setDisabled
        )
        QtCore.QMetaObject.connectSlotsByName(LoginRegisterDeviceWidget)

    def retranslateUi(self, LoginRegisterDeviceWidget):
        _translate = QtCore.QCoreApplication.translate
        LoginRegisterDeviceWidget.setWindowTitle(_translate("LoginRegisterDeviceWidget", "Form"))
        self.group_config_device.setTitle(
            _translate("LoginRegisterDeviceWidget", "Register a new device")
        )
        self.device_label.setText(
            _translate(
                "LoginRegisterDeviceWidget",
                "To register, you need an existing device to declare a new device and get the resulting token.",
            )
        )
        self.line_edit_login.setPlaceholderText(_translate("LoginRegisterDeviceWidget", "Login"))
        self.line_edit_password.setPlaceholderText(
            _translate("LoginRegisterDeviceWidget", "Password")
        )
        self.line_edit_password_check.setPlaceholderText(
            _translate("LoginRegisterDeviceWidget", "Password check")
        )
        self.check_box_use_nitrokey.setText(
            _translate(
                "LoginRegisterDeviceWidget", "Use NitroKey authentication instead of password"
            )
        )
        self.label.setText(_translate("LoginRegisterDeviceWidget", "Token ID"))
        self.label_2.setText(_translate("LoginRegisterDeviceWidget", "Key ID"))
        self.line_edit_device.setPlaceholderText(_translate("LoginRegisterDeviceWidget", "Device"))
        self.line_edit_token.setPlaceholderText(_translate("LoginRegisterDeviceWidget", "Token"))
        self.button_register.setText(_translate("LoginRegisterDeviceWidget", "Register"))
