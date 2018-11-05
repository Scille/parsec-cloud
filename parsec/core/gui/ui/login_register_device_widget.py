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
        LoginRegisterDeviceWidget.resize(607, 720)
        LoginRegisterDeviceWidget.setStyleSheet(
            "QWidget#LoginRegisterDeviceWidget\n" "{\n" "background-color: rgb(12, 65, 156);\n" "}"
        )
        self.verticalLayout_4 = QtWidgets.QVBoxLayout(LoginRegisterDeviceWidget)
        self.verticalLayout_4.setObjectName("verticalLayout_4")
        self.label_9 = QtWidgets.QLabel(LoginRegisterDeviceWidget)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_9.setFont(font)
        self.label_9.setStyleSheet("color: rgb(255, 255, 255);\n" "padding-bottom: 20px;")
        self.label_9.setScaledContents(False)
        self.label_9.setAlignment(QtCore.Qt.AlignJustify | QtCore.Qt.AlignVCenter)
        self.label_9.setWordWrap(True)
        self.label_9.setObjectName("label_9")
        self.verticalLayout_4.addWidget(self.label_9)
        self.widget_2 = QtWidgets.QWidget(LoginRegisterDeviceWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.widget_2.sizePolicy().hasHeightForWidth())
        self.widget_2.setSizePolicy(sizePolicy)
        self.widget_2.setStyleSheet(
            "QLineEdit\n"
            "{\n"
            "border: 2px solid rgb(46, 145, 208);\n"
            "background-color: white;\n"
            "color: black;\n"
            "}\n"
            "\n"
            "QComboBox\n"
            "{\n"
            "border: 2px solid rgb(46, 145, 208);\n"
            "background-color: white;\n"
            "color: black;\n"
            "}\n"
            "\n"
            "QComboBox::drop-down\n"
            "{\n"
            "border: 0px;\n"
            "}\n"
            "\n"
            "QComboBox::down-arrow\n"
            "{\n"
            "image: url(:/icons/images/icons/down-arrow.png);\n"
            "width: 16px;\n"
            "height: 16px;\n"
            "padding-right: 5px;\n"
            "}\n"
            "\n"
            "QCheckBox\n"
            "{\n"
            "color: rgb(255, 255, 255);\n"
            "}\n"
            "\n"
            "QCheckBox::indicator\n"
            "{\n"
            "width: 15px;\n"
            "height: 15px;\n"
            "background-color: rgb(255, 255, 255);\n"
            "border: 2px solid rgb(46, 145, 208);\n"
            "color: black;\n"
            "}\n"
            "\n"
            "QCheckBox::indicator:checked\n"
            "{\n"
            "image: url(:/icons/images/icons/checked.png)\n"
            "}"
        )
        self.widget_2.setObjectName("widget_2")
        self.verticalLayout = QtWidgets.QVBoxLayout(self.widget_2)
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout.setSpacing(10)
        self.verticalLayout.setObjectName("verticalLayout")
        self.verticalLayout_5 = QtWidgets.QVBoxLayout()
        self.verticalLayout_5.setSpacing(0)
        self.verticalLayout_5.setObjectName("verticalLayout_5")
        self.label = QtWidgets.QLabel(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label.setFont(font)
        self.label.setStyleSheet("color: rgb(255, 255, 255);")
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setObjectName("label")
        self.verticalLayout_5.addWidget(self.label)
        self.line_edit_login = QtWidgets.QLineEdit(self.widget_2)
        self.line_edit_login.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_login.setFont(font)
        self.line_edit_login.setObjectName("line_edit_login")
        self.verticalLayout_5.addWidget(self.line_edit_login)
        self.verticalLayout.addLayout(self.verticalLayout_5)
        self.verticalLayout_6 = QtWidgets.QVBoxLayout()
        self.verticalLayout_6.setSpacing(0)
        self.verticalLayout_6.setObjectName("verticalLayout_6")
        self.label_7 = QtWidgets.QLabel(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_7.setFont(font)
        self.label_7.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_7.setAlignment(QtCore.Qt.AlignCenter)
        self.label_7.setObjectName("label_7")
        self.verticalLayout_6.addWidget(self.label_7)
        self.line_edit_device = QtWidgets.QLineEdit(self.widget_2)
        self.line_edit_device.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_device.setFont(font)
        self.line_edit_device.setObjectName("line_edit_device")
        self.verticalLayout_6.addWidget(self.line_edit_device)
        self.verticalLayout.addLayout(self.verticalLayout_6)
        self.verticalLayout_7 = QtWidgets.QVBoxLayout()
        self.verticalLayout_7.setSpacing(0)
        self.verticalLayout_7.setObjectName("verticalLayout_7")
        self.label_8 = QtWidgets.QLabel(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_8.setFont(font)
        self.label_8.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_8.setAlignment(QtCore.Qt.AlignCenter)
        self.label_8.setObjectName("label_8")
        self.verticalLayout_7.addWidget(self.label_8)
        self.line_edit_token = QtWidgets.QLineEdit(self.widget_2)
        self.line_edit_token.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_token.setFont(font)
        self.line_edit_token.setObjectName("line_edit_token")
        self.verticalLayout_7.addWidget(self.line_edit_token)
        self.verticalLayout.addLayout(self.verticalLayout_7)
        self.widget_password = QtWidgets.QWidget(self.widget_2)
        self.widget_password.setObjectName("widget_password")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.widget_password)
        self.verticalLayout_2.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_2.setSpacing(15)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.verticalLayout_8 = QtWidgets.QVBoxLayout()
        self.verticalLayout_8.setSpacing(0)
        self.verticalLayout_8.setObjectName("verticalLayout_8")
        self.label_2 = QtWidgets.QLabel(self.widget_password)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_2.setFont(font)
        self.label_2.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_2.setAlignment(QtCore.Qt.AlignCenter)
        self.label_2.setObjectName("label_2")
        self.verticalLayout_8.addWidget(self.label_2)
        self.line_edit_password = QtWidgets.QLineEdit(self.widget_password)
        self.line_edit_password.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_password.setFont(font)
        self.line_edit_password.setStyleSheet("")
        self.line_edit_password.setText("")
        self.line_edit_password.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_password.setObjectName("line_edit_password")
        self.verticalLayout_8.addWidget(self.line_edit_password)
        self.verticalLayout_2.addLayout(self.verticalLayout_8)
        self.verticalLayout_9 = QtWidgets.QVBoxLayout()
        self.verticalLayout_9.setSpacing(0)
        self.verticalLayout_9.setObjectName("verticalLayout_9")
        self.label_6 = QtWidgets.QLabel(self.widget_password)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_6.setFont(font)
        self.label_6.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_6.setAlignment(QtCore.Qt.AlignCenter)
        self.label_6.setObjectName("label_6")
        self.verticalLayout_9.addWidget(self.label_6)
        self.line_edit_password_check = QtWidgets.QLineEdit(self.widget_password)
        self.line_edit_password_check.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_password_check.setFont(font)
        self.line_edit_password_check.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_password_check.setObjectName("line_edit_password_check")
        self.verticalLayout_9.addWidget(self.line_edit_password_check)
        self.verticalLayout_2.addLayout(self.verticalLayout_9)
        self.verticalLayout.addWidget(self.widget_password)
        self.widget_nitrokey = QtWidgets.QWidget(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.widget_nitrokey.setFont(font)
        self.widget_nitrokey.setStyleSheet("color: rgb(255, 255, 255);")
        self.widget_nitrokey.setObjectName("widget_nitrokey")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.widget_nitrokey)
        self.verticalLayout_3.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_3.setSpacing(15)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.verticalLayout_10 = QtWidgets.QVBoxLayout()
        self.verticalLayout_10.setSpacing(0)
        self.verticalLayout_10.setObjectName("verticalLayout_10")
        self.label_3 = QtWidgets.QLabel(self.widget_nitrokey)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_3.setFont(font)
        self.label_3.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_3.setAlignment(QtCore.Qt.AlignCenter)
        self.label_3.setObjectName("label_3")
        self.verticalLayout_10.addWidget(self.label_3)
        self.line_edit_nitrokey_pin = QtWidgets.QLineEdit(self.widget_nitrokey)
        self.line_edit_nitrokey_pin.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_nitrokey_pin.setFont(font)
        self.line_edit_nitrokey_pin.setStyleSheet("color: rgb(0, 0, 0);")
        self.line_edit_nitrokey_pin.setText("")
        self.line_edit_nitrokey_pin.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_nitrokey_pin.setObjectName("line_edit_nitrokey_pin")
        self.verticalLayout_10.addWidget(self.line_edit_nitrokey_pin)
        self.verticalLayout_3.addLayout(self.verticalLayout_10)
        self.verticalLayout_11 = QtWidgets.QVBoxLayout()
        self.verticalLayout_11.setSpacing(0)
        self.verticalLayout_11.setObjectName("verticalLayout_11")
        self.label_4 = QtWidgets.QLabel(self.widget_nitrokey)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_4.setFont(font)
        self.label_4.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_4.setAlignment(QtCore.Qt.AlignCenter)
        self.label_4.setObjectName("label_4")
        self.verticalLayout_11.addWidget(self.label_4)
        self.combo_nitrokey_token = QtWidgets.QComboBox(self.widget_nitrokey)
        self.combo_nitrokey_token.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.combo_nitrokey_token.setFont(font)
        self.combo_nitrokey_token.setStyleSheet("color: black;")
        self.combo_nitrokey_token.setObjectName("combo_nitrokey_token")
        self.verticalLayout_11.addWidget(self.combo_nitrokey_token)
        self.verticalLayout_3.addLayout(self.verticalLayout_11)
        self.verticalLayout_12 = QtWidgets.QVBoxLayout()
        self.verticalLayout_12.setSpacing(0)
        self.verticalLayout_12.setObjectName("verticalLayout_12")
        self.label_5 = QtWidgets.QLabel(self.widget_nitrokey)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_5.setFont(font)
        self.label_5.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_5.setAlignment(QtCore.Qt.AlignCenter)
        self.label_5.setObjectName("label_5")
        self.verticalLayout_12.addWidget(self.label_5)
        self.combo_nitrokey_key = QtWidgets.QComboBox(self.widget_nitrokey)
        self.combo_nitrokey_key.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.combo_nitrokey_key.setFont(font)
        self.combo_nitrokey_key.setStyleSheet("color: black;")
        self.combo_nitrokey_key.setObjectName("combo_nitrokey_key")
        self.verticalLayout_12.addWidget(self.combo_nitrokey_key)
        self.verticalLayout_3.addLayout(self.verticalLayout_12)
        self.verticalLayout.addWidget(self.widget_nitrokey)
        self.check_box_use_nitrokey = QtWidgets.QCheckBox(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.check_box_use_nitrokey.setFont(font)
        self.check_box_use_nitrokey.setStyleSheet("")
        self.check_box_use_nitrokey.setCheckable(True)
        self.check_box_use_nitrokey.setChecked(False)
        self.check_box_use_nitrokey.setObjectName("check_box_use_nitrokey")
        self.verticalLayout.addWidget(self.check_box_use_nitrokey)
        self.label_error = QtWidgets.QLabel(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_error.setFont(font)
        self.label_error.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_error.setText("")
        self.label_error.setWordWrap(True)
        self.label_error.setObjectName("label_error")
        self.verticalLayout.addWidget(self.label_error)
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_3.setSpacing(0)
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_3.addItem(spacerItem)
        self.button_register = QtWidgets.QPushButton(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(13)
        self.button_register.setFont(font)
        self.button_register.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_register.setStyleSheet(
            "border: 1px solid rgb(255, 255, 255);\n" "color: rgb(255, 255, 255);\n" "padding: 7px;"
        )
        self.button_register.setIconSize(QtCore.QSize(28, 28))
        self.button_register.setFlat(True)
        self.button_register.setObjectName("button_register")
        self.horizontalLayout_3.addWidget(self.button_register)
        self.verticalLayout.addLayout(self.horizontalLayout_3)
        self.verticalLayout_4.addWidget(self.widget_2)

        self.retranslateUi(LoginRegisterDeviceWidget)
        self.check_box_use_nitrokey.toggled["bool"].connect(self.widget_nitrokey.setVisible)
        self.check_box_use_nitrokey.toggled["bool"].connect(self.widget_password.setHidden)
        QtCore.QMetaObject.connectSlotsByName(LoginRegisterDeviceWidget)

    def retranslateUi(self, LoginRegisterDeviceWidget):
        _translate = QtCore.QCoreApplication.translate
        LoginRegisterDeviceWidget.setWindowTitle(_translate("LoginRegisterDeviceWidget", "Form"))
        self.label_9.setText(
            _translate(
                "LoginRegisterDeviceWidget",
                "To register a new device, you need an existing device to declare a new device and get the resulting token.",
            )
        )
        self.label.setText(_translate("LoginRegisterDeviceWidget", "User name"))
        self.label_7.setText(_translate("LoginRegisterDeviceWidget", "Device name"))
        self.label_8.setText(_translate("LoginRegisterDeviceWidget", "Token"))
        self.label_2.setText(_translate("LoginRegisterDeviceWidget", "Password"))
        self.label_6.setText(_translate("LoginRegisterDeviceWidget", "Password check"))
        self.label_3.setText(_translate("LoginRegisterDeviceWidget", "NitroKey PIN"))
        self.label_4.setText(_translate("LoginRegisterDeviceWidget", "NitroKey Token"))
        self.label_5.setText(_translate("LoginRegisterDeviceWidget", "NitroKey Key"))
        self.check_box_use_nitrokey.setText(
            _translate("LoginRegisterDeviceWidget", "Authenticate using NitroKey")
        )
        self.button_register.setText(_translate("LoginRegisterDeviceWidget", "Register"))
