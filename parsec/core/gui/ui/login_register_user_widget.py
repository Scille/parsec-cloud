# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/login_register_user_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_LoginRegisterUserWidget(object):
    def setupUi(self, LoginRegisterUserWidget):
        LoginRegisterUserWidget.setObjectName("LoginRegisterUserWidget")
        LoginRegisterUserWidget.resize(519, 401)
        self.verticalLayout = QtWidgets.QVBoxLayout(LoginRegisterUserWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.group_claim = QtWidgets.QGroupBox(LoginRegisterUserWidget)
        self.group_claim.setEnabled(True)
        self.group_claim.setObjectName("group_claim")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.group_claim)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.label = QtWidgets.QLabel(self.group_claim)
        self.label.setObjectName("label")
        self.verticalLayout_3.addWidget(self.label)
        self.line_edit_login = QtWidgets.QLineEdit(self.group_claim)
        self.line_edit_login.setText("")
        self.line_edit_login.setObjectName("line_edit_login")
        self.verticalLayout_3.addWidget(self.line_edit_login)
        self.line_edit_password = QtWidgets.QLineEdit(self.group_claim)
        self.line_edit_password.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_password.setObjectName("line_edit_password")
        self.verticalLayout_3.addWidget(self.line_edit_password)
        self.line_edit_password_check = QtWidgets.QLineEdit(self.group_claim)
        self.line_edit_password_check.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_password_check.setObjectName("line_edit_password_check")
        self.verticalLayout_3.addWidget(self.line_edit_password_check)
        self.check_box_use_nitrokey = QtWidgets.QCheckBox(self.group_claim)
        self.check_box_use_nitrokey.setObjectName("check_box_use_nitrokey")
        self.verticalLayout_3.addWidget(self.check_box_use_nitrokey)
        self.widget_nitrokey = QtWidgets.QWidget(self.group_claim)
        self.widget_nitrokey.setObjectName("widget_nitrokey")
        self.formLayout = QtWidgets.QFormLayout(self.widget_nitrokey)
        self.formLayout.setObjectName("formLayout")
        self.label_2 = QtWidgets.QLabel(self.widget_nitrokey)
        self.label_2.setObjectName("label_2")
        self.formLayout.setWidget(0, QtWidgets.QFormLayout.LabelRole, self.label_2)
        self.combo_nitrokey_token = QtWidgets.QComboBox(self.widget_nitrokey)
        self.combo_nitrokey_token.setObjectName("combo_nitrokey_token")
        self.formLayout.setWidget(0, QtWidgets.QFormLayout.FieldRole, self.combo_nitrokey_token)
        self.label_3 = QtWidgets.QLabel(self.widget_nitrokey)
        self.label_3.setObjectName("label_3")
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.LabelRole, self.label_3)
        self.combo_nitrokey_key = QtWidgets.QComboBox(self.widget_nitrokey)
        self.combo_nitrokey_key.setObjectName("combo_nitrokey_key")
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.FieldRole, self.combo_nitrokey_key)
        self.verticalLayout_3.addWidget(self.widget_nitrokey)
        self.line_edit_device = QtWidgets.QLineEdit(self.group_claim)
        self.line_edit_device.setObjectName("line_edit_device")
        self.verticalLayout_3.addWidget(self.line_edit_device)
        self.line_edit_token = QtWidgets.QLineEdit(self.group_claim)
        self.line_edit_token.setObjectName("line_edit_token")
        self.verticalLayout_3.addWidget(self.line_edit_token)
        self.horizontalLayout_4 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_4.setObjectName("horizontalLayout_4")
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_4.addItem(spacerItem)
        self.button_register = QtWidgets.QPushButton(self.group_claim)
        self.button_register.setEnabled(False)
        self.button_register.setObjectName("button_register")
        self.horizontalLayout_4.addWidget(self.button_register)
        self.verticalLayout_3.addLayout(self.horizontalLayout_4)
        self.label_error = QtWidgets.QLabel(self.group_claim)
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.label_error.setFont(font)
        self.label_error.setText("")
        self.label_error.setAlignment(QtCore.Qt.AlignCenter)
        self.label_error.setObjectName("label_error")
        self.verticalLayout_3.addWidget(self.label_error)
        self.verticalLayout.addWidget(self.group_claim)

        self.retranslateUi(LoginRegisterUserWidget)
        self.check_box_use_nitrokey.toggled["bool"].connect(self.line_edit_password.setDisabled)
        self.check_box_use_nitrokey.toggled["bool"].connect(
            self.line_edit_password_check.setDisabled
        )
        self.check_box_use_nitrokey.toggled["bool"].connect(self.widget_nitrokey.setVisible)
        QtCore.QMetaObject.connectSlotsByName(LoginRegisterUserWidget)

    def retranslateUi(self, LoginRegisterUserWidget):
        _translate = QtCore.QCoreApplication.translate
        LoginRegisterUserWidget.setWindowTitle(_translate("LoginRegisterUserWidget", "Form"))
        self.group_claim.setTitle(_translate("LoginRegisterUserWidget", "Register a new account"))
        self.label.setText(
            _translate(
                "LoginRegisterUserWidget",
                "To register, you need another user to create an account and get a token.",
            )
        )
        self.line_edit_login.setPlaceholderText(_translate("LoginRegisterUserWidget", "Login"))
        self.line_edit_password.setPlaceholderText(
            _translate("LoginRegisterUserWidget", "Password")
        )
        self.line_edit_password_check.setPlaceholderText(
            _translate("LoginRegisterUserWidget", "Password check")
        )
        self.check_box_use_nitrokey.setText(
            _translate("LoginRegisterUserWidget", "Use NitroKey authentication instead of password")
        )
        self.label_2.setText(_translate("LoginRegisterUserWidget", "Token ID"))
        self.label_3.setText(_translate("LoginRegisterUserWidget", "Key ID"))
        self.line_edit_device.setPlaceholderText(_translate("LoginRegisterUserWidget", "Device"))
        self.line_edit_token.setPlaceholderText(_translate("LoginRegisterUserWidget", "Token"))
        self.button_register.setText(_translate("LoginRegisterUserWidget", "Register"))
