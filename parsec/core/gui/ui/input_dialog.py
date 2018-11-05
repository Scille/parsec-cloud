# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/input_dialog.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_InputDialog(object):
    def setupUi(self, InputDialog):
        InputDialog.setObjectName("InputDialog")
        InputDialog.resize(362, 185)
        InputDialog.setMinimumSize(QtCore.QSize(300, 140))
        InputDialog.setStyleSheet("")
        InputDialog.setModal(True)
        self.verticalLayout = QtWidgets.QVBoxLayout(InputDialog)
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName("verticalLayout")
        self.widget = QtWidgets.QWidget(InputDialog)
        self.widget.setStyleSheet(
            "QWidget#widget\n"
            "{\n"
            "border: 2px solid rgb(12, 65, 157);\n"
            "background-color: rgb(255, 255, 255);\n"
            "}"
        )
        self.widget.setObjectName("widget")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.widget)
        self.verticalLayout_2.setContentsMargins(10, 0, 0, 10)
        self.verticalLayout_2.setSpacing(0)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setSizeConstraint(QtWidgets.QLayout.SetMinimumSize)
        self.horizontalLayout.setContentsMargins(-1, 6, 6, 6)
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
        self.verticalLayout_2.addLayout(self.horizontalLayout)
        self.verticalLayout_3 = QtWidgets.QVBoxLayout()
        self.verticalLayout_3.setContentsMargins(-1, -1, 10, -1)
        self.verticalLayout_3.setSpacing(5)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.label_title = QtWidgets.QLabel(self.widget)
        font = QtGui.QFont()
        font.setPointSize(14)
        font.setBold(True)
        font.setWeight(75)
        self.label_title.setFont(font)
        self.label_title.setStyleSheet("color: rgb(12, 65, 157);")
        self.label_title.setText("")
        self.label_title.setObjectName("label_title")
        self.verticalLayout_3.addWidget(self.label_title)
        self.label_message = QtWidgets.QLabel(self.widget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_message.sizePolicy().hasHeightForWidth())
        self.label_message.setSizePolicy(sizePolicy)
        self.label_message.setStyleSheet("color: rgb(12, 65, 157);")
        self.label_message.setText("")
        self.label_message.setObjectName("label_message")
        self.verticalLayout_3.addWidget(self.label_message)
        self.verticalLayout_2.addLayout(self.verticalLayout_3)
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_3.setContentsMargins(-1, 4, 10, 4)
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        self.line_edit_text = QtWidgets.QLineEdit(self.widget)
        self.line_edit_text.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_text.setFont(font)
        self.line_edit_text.setStyleSheet(
            "border: 1px solid rgb(30, 78, 162);\n" "padding-left: 10px;"
        )
        self.line_edit_text.setObjectName("line_edit_text")
        self.horizontalLayout_3.addWidget(self.line_edit_text)
        self.verticalLayout_2.addLayout(self.horizontalLayout_3)
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setContentsMargins(-1, -1, 10, -1)
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        spacerItem1 = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_2.addItem(spacerItem1)
        self.button_ok = QtWidgets.QPushButton(self.widget)
        self.button_ok.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_ok.setFont(font)
        self.button_ok.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_ok.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;"
        )
        icon1 = QtGui.QIcon()
        icon1.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/checked_white.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_ok.setIcon(icon1)
        self.button_ok.setIconSize(QtCore.QSize(24, 24))
        self.button_ok.setObjectName("button_ok")
        self.horizontalLayout_2.addWidget(self.button_ok)
        self.verticalLayout_2.addLayout(self.horizontalLayout_2)
        self.verticalLayout.addWidget(self.widget)

        self.retranslateUi(InputDialog)
        self.button_close.clicked.connect(InputDialog.reject)
        self.button_ok.clicked.connect(InputDialog.accept)
        QtCore.QMetaObject.connectSlotsByName(InputDialog)

    def retranslateUi(self, InputDialog):
        _translate = QtCore.QCoreApplication.translate
        InputDialog.setWindowTitle(_translate("InputDialog", "Dialog"))
        self.button_ok.setText(_translate("InputDialog", "Ok"))


from parsec.core.gui import resources_rc
