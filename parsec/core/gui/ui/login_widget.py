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
        LoginWidget.resize(416, 130)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(LoginWidget.sizePolicy().hasHeightForWidth())
        LoginWidget.setSizePolicy(sizePolicy)
        self.verticalLayout = QtWidgets.QVBoxLayout(LoginWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.line_input_login = QtWidgets.QLineEdit(LoginWidget)
        self.line_input_login.setObjectName("line_input_login")
        self.verticalLayout.addWidget(self.line_input_login)
        self.line_input_password = QtWidgets.QLineEdit(LoginWidget)
        self.line_input_password.setEchoMode(QtWidgets.QLineEdit.Password)
        self.line_input_password.setObjectName("line_input_password")
        self.verticalLayout.addWidget(self.line_input_password)
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        spacerItem = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.horizontalLayout.addItem(spacerItem)
        self.button_log_in = QtWidgets.QPushButton(LoginWidget)
        self.button_log_in.setEnabled(False)
        self.button_log_in.setObjectName("button_log_in")
        self.horizontalLayout.addWidget(self.button_log_in)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.label_error = QtWidgets.QLabel(LoginWidget)
        self.label_error.setText("")
        self.label_error.setAlignment(QtCore.Qt.AlignCenter)
        self.label_error.setObjectName("label_error")
        self.verticalLayout.addWidget(self.label_error)

        self.retranslateUi(LoginWidget)
        QtCore.QMetaObject.connectSlotsByName(LoginWidget)

    def retranslateUi(self, LoginWidget):
        _translate = QtCore.QCoreApplication.translate
        LoginWidget.setWindowTitle(_translate("LoginWidget", "Form"))
        self.line_input_login.setPlaceholderText(_translate("LoginWidget", "Login"))
        self.line_input_password.setPlaceholderText(_translate("LoginWidget", "Password"))
        self.button_log_in.setText(_translate("LoginWidget", "Log In"))

