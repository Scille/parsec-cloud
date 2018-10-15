# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/users_widget.ui.autosave'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_UsersWidget(object):
    def setupUi(self, UsersWidget):
        UsersWidget.setObjectName("UsersWidget")
        UsersWidget.resize(602, 427)
        self.verticalLayout = QtWidgets.QVBoxLayout(UsersWidget)
        self.verticalLayout.setContentsMargins(5, 5, 5, 5)
        self.verticalLayout.setSpacing(5)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.button_add_user = QtWidgets.QPushButton(UsersWidget)
        self.button_add_user.setObjectName("button_add_user")
        self.horizontalLayout_2.addWidget(self.button_add_user)
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_2.addItem(spacerItem)
        self.verticalLayout.addLayout(self.horizontalLayout_2)
        self.label_register_error = QtWidgets.QLabel(UsersWidget)
        self.label_register_error.setText("")
        self.label_register_error.setObjectName("label_register_error")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.label_register_error)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.verticalLayout.addWidget(self.label_register_error)

        self.retranslateUi(UsersWidget)
        QtCore.QMetaObject.connectSlotsByName(UsersWidget)

    def retranslateUi(self, UsersWidget):
        _translate = QtCore.QCoreApplication.translate
        UsersWidget.setWindowTitle(_translate("UsersWidget", "Form"))
        self.button_add_user.setText(_translate("UsersWidget", "Add a new user"))
