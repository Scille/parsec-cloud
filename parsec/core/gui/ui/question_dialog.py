# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/question_dialog.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_QuestionDialog(object):
    def setupUi(self, QuestionDialog):
        QuestionDialog.setObjectName("QuestionDialog")
        QuestionDialog.resize(437, 139)
        self.verticalLayout = QtWidgets.QVBoxLayout(QuestionDialog)
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName("verticalLayout")
        self.widget = QtWidgets.QWidget(QuestionDialog)
        self.widget.setStyleSheet(
            "QWidget#widget\n"
            "{\n"
            "border: 2px solid rgb(12, 65, 157);\n"
            "background-color: rgb(255, 255, 255);\n"
            "}"
        )
        self.widget.setObjectName("widget")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.widget)
        self.verticalLayout_3.setContentsMargins(10, 0, 0, 10)
        self.verticalLayout_3.setSpacing(0)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setSizeConstraint(QtWidgets.QLayout.SetMinimumSize)
        self.horizontalLayout.setContentsMargins(-1, 5, 5, 0)
        self.horizontalLayout.setSpacing(0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout.addItem(spacerItem)
        self.button_close = QtWidgets.QPushButton(self.widget)
        self.button_close.setMinimumSize(QtCore.QSize(20, 20))
        self.button_close.setMaximumSize(QtCore.QSize(20, 20))
        self.button_close.setText("")
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/menu_cancel.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_close.setIcon(icon)
        self.button_close.setIconSize(QtCore.QSize(20, 20))
        self.button_close.setFlat(True)
        self.button_close.setObjectName("button_close")
        self.horizontalLayout.addWidget(self.button_close)
        self.verticalLayout_3.addLayout(self.horizontalLayout)
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.label_icon = QtWidgets.QLabel(self.widget)
        self.label_icon.setMinimumSize(QtCore.QSize(64, 64))
        self.label_icon.setMaximumSize(QtCore.QSize(64, 64))
        self.label_icon.setText("")
        self.label_icon.setPixmap(QtGui.QPixmap(":/icons/images/icons/question.png"))
        self.label_icon.setScaledContents(True)
        self.label_icon.setObjectName("label_icon")
        self.horizontalLayout_2.addWidget(self.label_icon)
        self.verticalLayout_2 = QtWidgets.QVBoxLayout()
        self.verticalLayout_2.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.verticalLayout_2.setContentsMargins(10, -1, 10, -1)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.label_title = QtWidgets.QLabel(self.widget)
        font = QtGui.QFont()
        font.setPointSize(14)
        font.setBold(True)
        font.setWeight(75)
        self.label_title.setFont(font)
        self.label_title.setStyleSheet("color: rgb(12, 65, 157);")
        self.label_title.setText("")
        self.label_title.setWordWrap(True)
        self.label_title.setObjectName("label_title")
        self.verticalLayout_2.addWidget(self.label_title)
        self.label_message = QtWidgets.QLabel(self.widget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_message.sizePolicy().hasHeightForWidth())
        self.label_message.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_message.setFont(font)
        self.label_message.setStyleSheet("color: rgb(12, 65, 157);")
        self.label_message.setText("")
        self.label_message.setWordWrap(True)
        self.label_message.setObjectName("label_message")
        self.verticalLayout_2.addWidget(self.label_message)
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_3.setSpacing(8)
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        spacerItem1 = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_3.addItem(spacerItem1)
        self.button_no = QtWidgets.QPushButton(self.widget)
        self.button_no.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_no.setFont(font)
        self.button_no.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_no.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;"
        )
        icon1 = QtGui.QIcon()
        icon1.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/cancel_white.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_no.setIcon(icon1)
        self.button_no.setIconSize(QtCore.QSize(24, 24))
        self.button_no.setObjectName("button_no")
        self.horizontalLayout_3.addWidget(self.button_no)
        self.button_yes = QtWidgets.QPushButton(self.widget)
        self.button_yes.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_yes.setFont(font)
        self.button_yes.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_yes.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;"
        )
        icon2 = QtGui.QIcon()
        icon2.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/checked_white.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_yes.setIcon(icon2)
        self.button_yes.setIconSize(QtCore.QSize(24, 24))
        self.button_yes.setObjectName("button_yes")
        self.horizontalLayout_3.addWidget(self.button_yes)
        self.verticalLayout_2.addLayout(self.horizontalLayout_3)
        self.horizontalLayout_2.addLayout(self.verticalLayout_2)
        self.verticalLayout_3.addLayout(self.horizontalLayout_2)
        self.verticalLayout.addWidget(self.widget)

        self.retranslateUi(QuestionDialog)
        self.button_yes.clicked.connect(QuestionDialog.accept)
        self.button_no.clicked.connect(QuestionDialog.reject)
        self.button_close.clicked.connect(QuestionDialog.close)
        QtCore.QMetaObject.connectSlotsByName(QuestionDialog)

    def retranslateUi(self, QuestionDialog):
        _translate = QtCore.QCoreApplication.translate
        QuestionDialog.setWindowTitle(_translate("QuestionDialog", "Dialog"))
        self.button_no.setText(_translate("QuestionDialog", "No"))
        self.button_yes.setText(_translate("QuestionDialog", "Yes"))


from parsec.core.gui import resources_rc
