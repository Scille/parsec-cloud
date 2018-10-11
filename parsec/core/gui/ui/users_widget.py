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
        UsersWidget.resize(602, 224)
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
        self.widget_info = QtWidgets.QWidget(self.groupBox)
        self.widget_info.setObjectName("widget_info")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.widget_info)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.label_3 = QtWidgets.QLabel(self.widget_info)
        self.label_3.setObjectName("label_3")
        self.verticalLayout_3.addWidget(self.label_3)
        self.formLayout_2 = QtWidgets.QFormLayout()
        self.formLayout_2.setObjectName("formLayout_2")
        self.label = QtWidgets.QLabel(self.widget_info)
        self.label.setObjectName("label")
        self.formLayout_2.setWidget(0, QtWidgets.QFormLayout.LabelRole, self.label)
        self.label_2 = QtWidgets.QLabel(self.widget_info)
        self.label_2.setObjectName("label_2")
        self.formLayout_2.setWidget(1, QtWidgets.QFormLayout.LabelRole, self.label_2)
        self.line_edit_user_id = QtWidgets.QLineEdit(self.widget_info)
        self.line_edit_user_id.setReadOnly(True)
        self.line_edit_user_id.setObjectName("line_edit_user_id")
        self.formLayout_2.setWidget(0, QtWidgets.QFormLayout.FieldRole, self.line_edit_user_id)
        self.line_edit_token = QtWidgets.QLineEdit(self.widget_info)
        self.line_edit_token.setReadOnly(True)
        self.line_edit_token.setObjectName("line_edit_token")
        self.formLayout_2.setWidget(1, QtWidgets.QFormLayout.FieldRole, self.line_edit_token)
        self.verticalLayout_3.addLayout(self.formLayout_2)
        self.verticalLayout_2.addWidget(self.widget_info)
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
        self.label_3.setText(
            _translate(
                "UsersWidget",
                "Transmit the following information to the new user so they can set up their account.",
            )
        )
        self.label.setText(_translate("UsersWidget", "User ID"))
        self.label_2.setText(_translate("UsersWidget", "Token"))
