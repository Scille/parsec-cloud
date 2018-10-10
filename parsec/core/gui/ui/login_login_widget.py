# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/login_login_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_LoginLoginWidget(object):
    def setupUi(self, LoginLoginWidget):
        LoginLoginWidget.setObjectName("LoginLoginWidget")
        LoginLoginWidget.resize(400, 293)
        self.verticalLayout = QtWidgets.QVBoxLayout(LoginLoginWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.group_login = QtWidgets.QGroupBox(LoginLoginWidget)
        self.group_login.setObjectName("group_login")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.group_login)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.combo_devices = QtWidgets.QComboBox(self.group_login)
        self.combo_devices.setObjectName("combo_devices")
        self.verticalLayout_2.addWidget(self.combo_devices)
        self.line_edit_password = QtWidgets.QLineEdit(self.group_login)
        self.line_edit_password.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_password.setObjectName("line_edit_password")
        self.verticalLayout_2.addWidget(self.line_edit_password)
        self.check_box_use_nitrokey = QtWidgets.QCheckBox(self.group_login)
        self.check_box_use_nitrokey.setObjectName("check_box_use_nitrokey")
        self.verticalLayout_2.addWidget(self.check_box_use_nitrokey)
        self.widget_nitrokey = QtWidgets.QWidget(self.group_login)
        self.widget_nitrokey.setObjectName("widget_nitrokey")
        self.verticalLayout_5 = QtWidgets.QVBoxLayout(self.widget_nitrokey)
        self.verticalLayout_5.setObjectName("verticalLayout_5")
        self.formLayout = QtWidgets.QFormLayout()
        self.formLayout.setObjectName("formLayout")
        self.label_2 = QtWidgets.QLabel(self.widget_nitrokey)
        self.label_2.setObjectName("label_2")
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.LabelRole, self.label_2)
        self.label_3 = QtWidgets.QLabel(self.widget_nitrokey)
        self.label_3.setObjectName("label_3")
        self.formLayout.setWidget(2, QtWidgets.QFormLayout.LabelRole, self.label_3)
        self.combo_nitrokey_token = QtWidgets.QComboBox(self.widget_nitrokey)
        self.combo_nitrokey_token.setObjectName("combo_nitrokey_token")
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.FieldRole, self.combo_nitrokey_token)
        self.combo_nitrokey_key = QtWidgets.QComboBox(self.widget_nitrokey)
        self.combo_nitrokey_key.setObjectName("combo_nitrokey_key")
        self.formLayout.setWidget(2, QtWidgets.QFormLayout.FieldRole, self.combo_nitrokey_key)
        self.label_4 = QtWidgets.QLabel(self.widget_nitrokey)
        self.label_4.setObjectName("label_4")
        self.formLayout.setWidget(0, QtWidgets.QFormLayout.LabelRole, self.label_4)
        self.line_edit_nitrokey_pin = QtWidgets.QLineEdit(self.widget_nitrokey)
        self.line_edit_nitrokey_pin.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_nitrokey_pin.setObjectName("line_edit_nitrokey_pin")
        self.formLayout.setWidget(0, QtWidgets.QFormLayout.FieldRole, self.line_edit_nitrokey_pin)
        self.verticalLayout_5.addLayout(self.formLayout)
        self.verticalLayout_2.addWidget(self.widget_nitrokey)
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_3.addItem(spacerItem)
        self.button_login = QtWidgets.QPushButton(self.group_login)
        self.button_login.setEnabled(True)
        self.button_login.setObjectName("button_login")
        self.horizontalLayout_3.addWidget(self.button_login)
        self.verticalLayout_2.addLayout(self.horizontalLayout_3)
        self.verticalLayout.addWidget(self.group_login)

        self.retranslateUi(LoginLoginWidget)
        self.check_box_use_nitrokey.toggled["bool"].connect(self.line_edit_password.setDisabled)
        self.check_box_use_nitrokey.toggled["bool"].connect(self.widget_nitrokey.setVisible)
        QtCore.QMetaObject.connectSlotsByName(LoginLoginWidget)

    def retranslateUi(self, LoginLoginWidget):
        _translate = QtCore.QCoreApplication.translate
        LoginLoginWidget.setWindowTitle(_translate("LoginLoginWidget", "Form"))
        self.group_login.setTitle(_translate("LoginLoginWidget", "Log In"))
        self.line_edit_password.setPlaceholderText(_translate("LoginLoginWidget", "Password"))
        self.check_box_use_nitrokey.setText(
            _translate("LoginLoginWidget", "Use NitroKey authentication instead of password")
        )
        self.label_2.setText(_translate("LoginLoginWidget", "Token ID"))
        self.label_3.setText(_translate("LoginLoginWidget", "Key ID"))
        self.label_4.setText(_translate("LoginLoginWidget", "NitroKey PIN"))
        self.button_login.setText(_translate("LoginLoginWidget", "Log In"))
