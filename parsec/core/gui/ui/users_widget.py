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
        UsersWidget.resize(557, 300)
        self.verticalLayout = QtWidgets.QVBoxLayout(UsersWidget)
        self.verticalLayout.setContentsMargins(5, 5, 5, 5)
        self.verticalLayout.setSpacing(5)
        self.verticalLayout.setObjectName("verticalLayout")
        self.groupBox = QtWidgets.QGroupBox(UsersWidget)
        self.groupBox.setObjectName("groupBox")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.groupBox)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.button_register_device = QtWidgets.QPushButton(self.groupBox)
        self.button_register_device.setObjectName("button_register_device")
        self.horizontalLayout_2.addWidget(self.button_register_device)
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_2.addItem(spacerItem)
        self.verticalLayout_2.addLayout(self.horizontalLayout_2)
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        self.button_register_user = QtWidgets.QPushButton(self.groupBox)
        self.button_register_user.setObjectName("button_register_user")
        self.horizontalLayout_3.addWidget(self.button_register_user)
        spacerItem1 = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_3.addItem(spacerItem1)
        self.verticalLayout_2.addLayout(self.horizontalLayout_3)
        self.label_help = QtWidgets.QLabel(self.groupBox)
        self.label_help.setObjectName("label_help")
        self.verticalLayout_2.addWidget(self.label_help)
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label_new_user_login = QtWidgets.QLabel(self.groupBox)
        self.label_new_user_login.setText("")
        self.label_new_user_login.setTextInteractionFlags(
            QtCore.Qt.LinksAccessibleByMouse
            | QtCore.Qt.TextSelectableByKeyboard
            | QtCore.Qt.TextSelectableByMouse
        )
        self.label_new_user_login.setObjectName("label_new_user_login")
        self.horizontalLayout.addWidget(self.label_new_user_login)
        self.label_new_user_token = QtWidgets.QLabel(self.groupBox)
        self.label_new_user_token.setText("")
        self.label_new_user_token.setTextInteractionFlags(
            QtCore.Qt.LinksAccessibleByMouse
            | QtCore.Qt.TextSelectableByKeyboard
            | QtCore.Qt.TextSelectableByMouse
        )
        self.label_new_user_token.setObjectName("label_new_user_token")
        self.horizontalLayout.addWidget(self.label_new_user_token)
        self.verticalLayout_2.addLayout(self.horizontalLayout)
        self.label_register_error = QtWidgets.QLabel(self.groupBox)
        self.label_register_error.setText("")
        self.label_register_error.setObjectName("label_register_error")
        self.verticalLayout_2.addWidget(self.label_register_error)
        self.verticalLayout.addWidget(self.groupBox)
        spacerItem2 = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem2)

        self.retranslateUi(UsersWidget)
        QtCore.QMetaObject.connectSlotsByName(UsersWidget)

    def retranslateUi(self, UsersWidget):
        _translate = QtCore.QCoreApplication.translate
        UsersWidget.setWindowTitle(_translate("UsersWidget", "Form"))
        self.groupBox.setTitle(_translate("UsersWidget", "Add a user"))
        self.button_register_device.setText(_translate("UsersWidget", "Register a new device"))
        self.button_register_user.setText(_translate("UsersWidget", "Register a new user"))
        self.label_help.setText(
            _translate(
                "UsersWidget",
                "Transmit these information to the new user so they can set up their account. ",
            )
        )
