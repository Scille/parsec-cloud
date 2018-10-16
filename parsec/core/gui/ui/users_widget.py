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
        UsersWidget.resize(501, 376)
        self.verticalLayout = QtWidgets.QVBoxLayout(UsersWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.button_add_user = QtWidgets.QPushButton(UsersWidget)
        self.button_add_user.setObjectName("button_add_user")
        self.horizontalLayout.addWidget(self.button_add_user)
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout.addItem(spacerItem)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.widget_info = QtWidgets.QWidget(UsersWidget)
        self.widget_info.setObjectName("widget_info")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.widget_info)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.label_3 = QtWidgets.QLabel(self.widget_info)
        self.label_3.setWordWrap(True)
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
        self.verticalLayout.addWidget(self.widget_info)
        spacerItem1 = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem1)

        self.retranslateUi(UsersWidget)
        QtCore.QMetaObject.connectSlotsByName(UsersWidget)

    def retranslateUi(self, UsersWidget):
        _translate = QtCore.QCoreApplication.translate
        UsersWidget.setWindowTitle(_translate("UsersWidget", "Form"))
        self.button_add_user.setText(_translate("UsersWidget", "Add a new user"))
        self.label_3.setText(
            _translate(
                "UsersWidget",
                "Transmit the following information to the new user so they can set up their account.",
            )
        )
        self.label.setText(_translate("UsersWidget", "User ID"))
        self.label_2.setText(_translate("UsersWidget", "Token"))
