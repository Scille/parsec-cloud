# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/users_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_UsersWidget(object):
    def setupUi(self, UsersWidget):
        UsersWidget.setObjectName("UsersWidget")
        UsersWidget.resize(400, 300)
        self.verticalLayout = QtWidgets.QVBoxLayout(UsersWidget)
        self.verticalLayout.setObjectName("verticalLayout")

        self.retranslateUi(UsersWidget)
        QtCore.QMetaObject.connectSlotsByName(UsersWidget)

    def retranslateUi(self, UsersWidget):
        _translate = QtCore.QCoreApplication.translate
        UsersWidget.setWindowTitle(_translate("UsersWidget", "Form"))

