# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/login_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_LoginWidget(object):
    def setupUi(self, LoginWidget):
        LoginWidget.setObjectName("LoginWidget")
        LoginWidget.resize(613, 191)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(LoginWidget.sizePolicy().hasHeightForWidth())
        LoginWidget.setSizePolicy(sizePolicy)
        self.verticalLayout = QtWidgets.QVBoxLayout(LoginWidget)
        self.verticalLayout.setContentsMargins(5, 5, 5, 5)
        self.verticalLayout.setSpacing(5)
        self.verticalLayout.setObjectName("verticalLayout")
        self.button_login_instead = QtWidgets.QCommandLinkButton(LoginWidget)
        self.button_login_instead.setObjectName("button_login_instead")
        self.verticalLayout.addWidget(self.button_login_instead)
        self.button_register_user_instead = QtWidgets.QCommandLinkButton(LoginWidget)
        self.button_register_user_instead.setObjectName("button_register_user_instead")
        self.verticalLayout.addWidget(self.button_register_user_instead)
        self.button_register_device_instead = QtWidgets.QCommandLinkButton(LoginWidget)
        self.button_register_device_instead.setObjectName("button_register_device_instead")
        self.verticalLayout.addWidget(self.button_register_device_instead)
        spacerItem = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.verticalLayout.addItem(spacerItem)

        self.retranslateUi(LoginWidget)
        self.button_login_instead.clicked.connect(self.button_register_user_instead.show)
        self.button_register_user_instead.clicked.connect(self.button_login_instead.show)
        self.button_register_user_instead.clicked.connect(self.button_register_user_instead.hide)
        self.button_login_instead.clicked.connect(self.button_login_instead.hide)
        self.button_login_instead.clicked.connect(self.button_register_device_instead.show)
        self.button_register_user_instead.clicked.connect(self.button_register_device_instead.show)
        self.button_register_device_instead.clicked.connect(self.button_register_user_instead.show)
        self.button_register_device_instead.clicked.connect(self.button_login_instead.show)
        self.button_register_device_instead.clicked.connect(self.button_register_device_instead.hide)
        QtCore.QMetaObject.connectSlotsByName(LoginWidget)

    def retranslateUi(self, LoginWidget):
        _translate = QtCore.QCoreApplication.translate
        LoginWidget.setWindowTitle(_translate("LoginWidget", "Form"))
        self.button_login_instead.setText(_translate("LoginWidget", "Log In instead"))
        self.button_register_user_instead.setText(_translate("LoginWidget", "Register a new account instead"))
        self.button_register_device_instead.setText(_translate("LoginWidget", "Register a new device instead"))

