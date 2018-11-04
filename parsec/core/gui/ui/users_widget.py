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
        UsersWidget.setStyleSheet(
            "QWidget#UsersWidget\n"
            "{\n"
            "background-color: rgb(255, 255, 255);\n"
            "}\n"
            "\n"
            "QWidget#widget_users\n"
            "{\n"
            "border: 1px solid rgb(28, 78, 163);\n"
            "}"
        )
        self.verticalLayout = QtWidgets.QVBoxLayout(UsersWidget)
        self.verticalLayout.setContentsMargins(10, 10, 0, 0)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.button_add_user = QtWidgets.QPushButton(UsersWidget)
        self.button_add_user.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_add_user.setFont(font)
        self.button_add_user.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_add_user.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;\n"
            "font-weight: bold;"
        )
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/add_user.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_add_user.setIcon(icon)
        self.button_add_user.setIconSize(QtCore.QSize(24, 24))
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
        font = QtGui.QFont()
        font.setBold(True)
        font.setWeight(75)
        self.label_3.setFont(font)
        self.label_3.setStyleSheet("color: rgb(28, 78, 163);")
        self.label_3.setWordWrap(True)
        self.label_3.setObjectName("label_3")
        self.verticalLayout_3.addWidget(self.label_3)
        self.formLayout_2 = QtWidgets.QFormLayout()
        self.formLayout_2.setObjectName("formLayout_2")
        self.label = QtWidgets.QLabel(self.widget_info)
        font = QtGui.QFont()
        font.setBold(True)
        font.setWeight(75)
        self.label.setFont(font)
        self.label.setStyleSheet("color: rgb(28, 78, 163);")
        self.label.setObjectName("label")
        self.formLayout_2.setWidget(0, QtWidgets.QFormLayout.LabelRole, self.label)
        self.label_2 = QtWidgets.QLabel(self.widget_info)
        font = QtGui.QFont()
        font.setBold(True)
        font.setWeight(75)
        self.label_2.setFont(font)
        self.label_2.setStyleSheet("color: rgb(28, 78, 163);")
        self.label_2.setObjectName("label_2")
        self.formLayout_2.setWidget(1, QtWidgets.QFormLayout.LabelRole, self.label_2)
        self.line_edit_user_id = QtWidgets.QLineEdit(self.widget_info)
        self.line_edit_user_id.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_user_id.setFont(font)
        self.line_edit_user_id.setStyleSheet(
            "border: 1px solid rgb(28, 78, 163);\n"
            "padding-left: 10px;\n"
            "color: rgb(28, 78, 163);"
        )
        self.line_edit_user_id.setReadOnly(True)
        self.line_edit_user_id.setObjectName("line_edit_user_id")
        self.formLayout_2.setWidget(0, QtWidgets.QFormLayout.FieldRole, self.line_edit_user_id)
        self.line_edit_token = QtWidgets.QLineEdit(self.widget_info)
        self.line_edit_token.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_token.setFont(font)
        self.line_edit_token.setStyleSheet(
            "border: 1px solid rgb(28, 78, 163);\n"
            "padding-left: 10px;\n"
            "color: rgb(28, 78, 163);"
        )
        self.line_edit_token.setReadOnly(True)
        self.line_edit_token.setObjectName("line_edit_token")
        self.formLayout_2.setWidget(1, QtWidgets.QFormLayout.FieldRole, self.line_edit_token)
        self.verticalLayout_3.addLayout(self.formLayout_2)
        self.verticalLayout.addWidget(self.widget_info)
        self.line_edit_search = QtWidgets.QLineEdit(UsersWidget)
        self.line_edit_search.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_search.setFont(font)
        self.line_edit_search.setStyleSheet(
            "background-image: url(:/icons/images/icons/search.png);\n"
            "background-repeat: no-repeat;\n"
            "background-position: right;\n"
            "border: 1px solid rgb(28, 78, 163);\n"
            "padding-left: 10px;\n"
            "color: rgb(28, 78, 163);"
        )
        self.line_edit_search.setText("")
        self.line_edit_search.setObjectName("line_edit_search")
        self.verticalLayout.addWidget(self.line_edit_search)
        self.verticalLayout_2 = QtWidgets.QVBoxLayout()
        self.verticalLayout_2.setSpacing(0)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.pushButton_4 = QtWidgets.QPushButton(UsersWidget)
        self.pushButton_4.setMinimumSize(QtCore.QSize(0, 32))
        self.pushButton_4.setStyleSheet("background-color: rgb(12, 65, 157);\n" "border: 0;")
        self.pushButton_4.setText("")
        self.pushButton_4.setFlat(True)
        self.pushButton_4.setObjectName("pushButton_4")
        self.verticalLayout_2.addWidget(self.pushButton_4)
        self.widget_users = QtWidgets.QWidget(UsersWidget)
        self.widget_users.setMinimumSize(QtCore.QSize(0, 100))
        self.widget_users.setObjectName("widget_users")
        self.verticalLayout_4 = QtWidgets.QVBoxLayout(self.widget_users)
        self.verticalLayout_4.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_4.setSpacing(0)
        self.verticalLayout_4.setObjectName("verticalLayout_4")
        self.layout_users = QtWidgets.QGridLayout()
        self.layout_users.setObjectName("layout_users")
        self.verticalLayout_4.addLayout(self.layout_users)
        self.verticalLayout_2.addWidget(self.widget_users)
        self.verticalLayout.addLayout(self.verticalLayout_2)
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
        self.line_edit_search.setPlaceholderText(_translate("UsersWidget", "Search users"))


from parsec.core.gui import resources_rc
