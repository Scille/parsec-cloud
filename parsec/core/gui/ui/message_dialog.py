# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/message_dialog.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_MessageDialog(object):
    def setupUi(self, MessageDialog):
        MessageDialog.setObjectName("MessageDialog")
        MessageDialog.resize(300, 140)
        MessageDialog.setMinimumSize(QtCore.QSize(300, 140))
        MessageDialog.setStyleSheet(
            "QDialog#MessageDialog\n" "{\n" "border: 2px solid rgb(12, 65, 157);\n" "}"
        )
        MessageDialog.setModal(True)
        self.verticalLayout = QtWidgets.QVBoxLayout(MessageDialog)
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setSizeConstraint(QtWidgets.QLayout.SetMinimumSize)
        self.horizontalLayout.setContentsMargins(-1, 6, 6, 6)
        self.horizontalLayout.setObjectName("horizontalLayout")
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout.addItem(spacerItem)
        self.button_close = QtWidgets.QPushButton(MessageDialog)
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
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.label_icon = QtWidgets.QLabel(MessageDialog)
        self.label_icon.setMinimumSize(QtCore.QSize(48, 0))
        self.label_icon.setMaximumSize(QtCore.QSize(48, 16777215))
        self.label_icon.setText("")
        self.label_icon.setObjectName("label_icon")
        self.horizontalLayout_2.addWidget(self.label_icon)
        self.verticalLayout_2 = QtWidgets.QVBoxLayout()
        self.verticalLayout_2.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.label_title = QtWidgets.QLabel(MessageDialog)
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
        self.label_message = QtWidgets.QLabel(MessageDialog)
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
        self.horizontalLayout_2.addLayout(self.verticalLayout_2)
        self.verticalLayout.addLayout(self.horizontalLayout_2)

        self.retranslateUi(MessageDialog)
        QtCore.QMetaObject.connectSlotsByName(MessageDialog)

    def retranslateUi(self, MessageDialog):
        _translate = QtCore.QCoreApplication.translate
        MessageDialog.setWindowTitle(_translate("MessageDialog", "Dialog"))


from parsec.core.gui import resources_rc
