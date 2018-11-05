# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/user_button.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_UserButton(object):
    def setupUi(self, UserButton):
        UserButton.setObjectName("UserButton")
        UserButton.resize(210, 210)
        UserButton.setMinimumSize(QtCore.QSize(210, 210))
        UserButton.setMaximumSize(QtCore.QSize(210, 210))
        font = QtGui.QFont()
        font.setPointSize(12)
        UserButton.setFont(font)
        UserButton.setStyleSheet(
            "QMenu::item\n"
            "{\n"
            "background-color: rgb(255, 255, 255);\n"
            "color: rgb(0, 0, 0);\n"
            "}\n"
            "\n"
            "QMenu::item:selected\n"
            "{\n"
            "background-color: rgb(45, 144, 209);\n"
            "color: rgb(255, 255, 255);\n"
            "}"
        )
        self.verticalLayout = QtWidgets.QVBoxLayout(UserButton)
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout.setSpacing(6)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setSpacing(0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label = QtWidgets.QLabel(UserButton)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label.sizePolicy().hasHeightForWidth())
        self.label.setSizePolicy(sizePolicy)
        self.label.setMinimumSize(QtCore.QSize(178, 178))
        self.label.setMaximumSize(QtCore.QSize(178, 178))
        self.label.setStyleSheet("border: 2px solid rgb(45, 145, 207);\n" "padding: 20px;")
        self.label.setText("")
        self.label.setPixmap(QtGui.QPixmap(":/icons/images/icons/user.png"))
        self.label.setScaledContents(True)
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setTextInteractionFlags(QtCore.Qt.NoTextInteraction)
        self.label.setObjectName("label")
        self.horizontalLayout.addWidget(self.label)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.label_user = QtWidgets.QLabel(UserButton)
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.label_user.setFont(font)
        self.label_user.setLayoutDirection(QtCore.Qt.LeftToRight)
        self.label_user.setStyleSheet("color: rgb(12, 65, 157);")
        self.label_user.setText("")
        self.label_user.setAlignment(QtCore.Qt.AlignCenter)
        self.label_user.setWordWrap(True)
        self.label_user.setObjectName("label_user")
        self.verticalLayout.addWidget(self.label_user)

        self.retranslateUi(UserButton)
        QtCore.QMetaObject.connectSlotsByName(UserButton)

    def retranslateUi(self, UserButton):
        _translate = QtCore.QCoreApplication.translate
        UserButton.setWindowTitle(_translate("UserButton", "Form"))


from parsec.core.gui import resources_rc
