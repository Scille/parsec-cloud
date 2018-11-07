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
        LoginLoginWidget.resize(695, 391)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(LoginLoginWidget.sizePolicy().hasHeightForWidth())
        LoginLoginWidget.setSizePolicy(sizePolicy)
        LoginLoginWidget.setMinimumSize(QtCore.QSize(550, 0))
        LoginLoginWidget.setAutoFillBackground(False)
        LoginLoginWidget.setStyleSheet(
            "QWidget#LoginLoginWidget\n" "{\n" "background-color: rgb(12, 65, 156);\n" "}"
        )
        self.horizontalLayout = QtWidgets.QHBoxLayout(LoginLoginWidget)
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout.setSpacing(0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.widget = QtWidgets.QWidget(LoginLoginWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.widget.sizePolicy().hasHeightForWidth())
        self.widget.setSizePolicy(sizePolicy)
        self.widget.setMinimumSize(QtCore.QSize(0, 0))
        self.widget.setStyleSheet("")
        self.widget.setObjectName("widget")
        self.verticalLayout_6 = QtWidgets.QVBoxLayout(self.widget)
        self.verticalLayout_6.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_6.setSpacing(0)
        self.verticalLayout_6.setObjectName("verticalLayout_6")
        self.widget_2 = QtWidgets.QWidget(self.widget)
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
        self.verticalLayout_4 = QtWidgets.QVBoxLayout()
        self.verticalLayout_4.setSpacing(0)
        self.verticalLayout_4.setObjectName("verticalLayout_4")
        self.label = QtWidgets.QLabel(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label.setFont(font)
        self.label.setStyleSheet("color: rgb(255, 255, 255);")
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setObjectName("label")
        self.verticalLayout_4.addWidget(self.label)
        self.combo_devices = QtWidgets.QComboBox(self.widget_2)
        self.combo_devices.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.combo_devices.setFont(font)
        self.combo_devices.setStyleSheet("")
        self.combo_devices.setCurrentText("")
        self.combo_devices.setObjectName("combo_devices")
        self.verticalLayout_4.addWidget(self.combo_devices)
        self.verticalLayout.addLayout(self.verticalLayout_4)
        self.widget_password = QtWidgets.QWidget(self.widget_2)
        self.widget_password.setObjectName("widget_password")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.widget_password)
        self.verticalLayout_2.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_2.setSpacing(0)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.verticalLayout_5 = QtWidgets.QVBoxLayout()
        self.verticalLayout_5.setSpacing(0)
        self.verticalLayout_5.setObjectName("verticalLayout_5")
        self.label_2 = QtWidgets.QLabel(self.widget_password)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_2.setFont(font)
        self.label_2.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_2.setAlignment(QtCore.Qt.AlignCenter)
        self.label_2.setObjectName("label_2")
        self.verticalLayout_5.addWidget(self.label_2)
        self.line_edit_password = QtWidgets.QLineEdit(self.widget_password)
        self.line_edit_password.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_password.setFont(font)
        self.line_edit_password.setStyleSheet("")
        self.line_edit_password.setText("")
        self.line_edit_password.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_password.setObjectName("line_edit_password")
        self.verticalLayout_5.addWidget(self.line_edit_password)
        self.verticalLayout_2.addLayout(self.verticalLayout_5)
        self.verticalLayout.addWidget(self.widget_password)
        self.widget_pkcs11 = QtWidgets.QWidget(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.widget_pkcs11.setFont(font)
        self.widget_pkcs11.setStyleSheet("color: rgb(255, 255, 255);")
        self.widget_pkcs11.setObjectName("widget_pkcs11")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.widget_pkcs11)
        self.verticalLayout_3.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_3.setSpacing(10)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.verticalLayout_7 = QtWidgets.QVBoxLayout()
        self.verticalLayout_7.setSpacing(0)
        self.verticalLayout_7.setObjectName("verticalLayout_7")
        self.label_3 = QtWidgets.QLabel(self.widget_pkcs11)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_3.setFont(font)
        self.label_3.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_3.setAlignment(QtCore.Qt.AlignCenter)
        self.label_3.setObjectName("label_3")
        self.verticalLayout_7.addWidget(self.label_3)
        self.line_edit_pkcs11_pin = QtWidgets.QLineEdit(self.widget_pkcs11)
        self.line_edit_pkcs11_pin.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_pkcs11_pin.setFont(font)
        self.line_edit_pkcs11_pin.setStyleSheet("color: rgb(0, 0, 0);")
        self.line_edit_pkcs11_pin.setText("")
        self.line_edit_pkcs11_pin.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_edit_pkcs11_pin.setObjectName("line_edit_pkcs11_pin")
        self.verticalLayout_7.addWidget(self.line_edit_pkcs11_pin)
        self.verticalLayout_3.addLayout(self.verticalLayout_7)
        self.verticalLayout_8 = QtWidgets.QVBoxLayout()
        self.verticalLayout_8.setSpacing(0)
        self.verticalLayout_8.setObjectName("verticalLayout_8")
        self.label_4 = QtWidgets.QLabel(self.widget_pkcs11)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_4.setFont(font)
        self.label_4.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_4.setAlignment(QtCore.Qt.AlignCenter)
        self.label_4.setObjectName("label_4")
        self.verticalLayout_8.addWidget(self.label_4)
        self.combo_pkcs11_token = QtWidgets.QComboBox(self.widget_pkcs11)
        self.combo_pkcs11_token.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.combo_pkcs11_token.setFont(font)
        self.combo_pkcs11_token.setStyleSheet("color: black;")
        self.combo_pkcs11_token.setObjectName("combo_pkcs11_token")
        self.verticalLayout_8.addWidget(self.combo_pkcs11_token)
        self.verticalLayout_3.addLayout(self.verticalLayout_8)
        self.verticalLayout_9 = QtWidgets.QVBoxLayout()
        self.verticalLayout_9.setSpacing(0)
        self.verticalLayout_9.setObjectName("verticalLayout_9")
        self.label_5 = QtWidgets.QLabel(self.widget_pkcs11)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_5.setFont(font)
        self.label_5.setStyleSheet("color: rgb(255, 255, 255);")
        self.label_5.setAlignment(QtCore.Qt.AlignCenter)
        self.label_5.setObjectName("label_5")
        self.verticalLayout_9.addWidget(self.label_5)
        self.combo_pkcs11_key = QtWidgets.QComboBox(self.widget_pkcs11)
        self.combo_pkcs11_key.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.combo_pkcs11_key.setFont(font)
        self.combo_pkcs11_key.setStyleSheet("color: black;")
        self.combo_pkcs11_key.setObjectName("combo_pkcs11_key")
        self.verticalLayout_9.addWidget(self.combo_pkcs11_key)
        self.verticalLayout_3.addLayout(self.verticalLayout_9)
        self.verticalLayout.addWidget(self.widget_pkcs11)
        self.check_box_use_pkcs11 = QtWidgets.QCheckBox(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.check_box_use_pkcs11.setFont(font)
        self.check_box_use_pkcs11.setStyleSheet("")
        self.check_box_use_pkcs11.setCheckable(True)
        self.check_box_use_pkcs11.setChecked(False)
        self.check_box_use_pkcs11.setObjectName("check_box_use_pkcs11")
        self.verticalLayout.addWidget(self.check_box_use_pkcs11)
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_3.setSpacing(0)
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_3.addItem(spacerItem)
        self.button_login = QtWidgets.QPushButton(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(13)
        self.button_login.setFont(font)
        self.button_login.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_login.setStyleSheet(
            "border: 1px solid rgb(255, 255, 255);\n" "color: rgb(255, 255, 255);\n" "padding: 7px;"
        )
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/login.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_login.setIcon(icon)
        self.button_login.setIconSize(QtCore.QSize(28, 28))
        self.button_login.setAutoDefault(True)
        self.button_login.setDefault(True)
        self.button_login.setFlat(True)
        self.button_login.setObjectName("button_login")
        self.horizontalLayout_3.addWidget(self.button_login)
        self.verticalLayout.addLayout(self.horizontalLayout_3)
        self.verticalLayout_6.addWidget(self.widget_2)
        self.horizontalLayout.addWidget(self.widget)

        self.retranslateUi(LoginLoginWidget)
        self.check_box_use_pkcs11.toggled["bool"].connect(self.widget_pkcs11.setVisible)
        self.check_box_use_pkcs11.toggled["bool"].connect(self.widget_password.setHidden)
        QtCore.QMetaObject.connectSlotsByName(LoginLoginWidget)

    def retranslateUi(self, LoginLoginWidget):
        _translate = QtCore.QCoreApplication.translate
        LoginLoginWidget.setWindowTitle(_translate("LoginLoginWidget", "Form"))
        self.label.setText(_translate("LoginLoginWidget", "Identifier"))
        self.label_2.setText(_translate("LoginLoginWidget", "Password"))
        self.label_3.setText(_translate("LoginLoginWidget", "PKCS #11 PIN"))
        self.label_4.setText(_translate("LoginLoginWidget", "PKCS #11 Token"))
        self.label_5.setText(_translate("LoginLoginWidget", "PKCS #11 Key"))
        self.check_box_use_pkcs11.setText(
            _translate("LoginLoginWidget", "Authenticate using PKCS #11")
        )
        self.button_login.setText(_translate("LoginLoginWidget", "Log In"))


from parsec.core.gui import resources_rc
