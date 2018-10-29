# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/folder_item_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_FolderItemWidget(object):
    def setupUi(self, FolderItemWidget):
        FolderItemWidget.setObjectName("FolderItemWidget")
        FolderItemWidget.resize(520, 56)
        FolderItemWidget.setMinimumSize(QtCore.QSize(0, 32))
        FolderItemWidget.setMaximumSize(QtCore.QSize(16777215, 16777215))
        FolderItemWidget.setStyleSheet("background-color: rgb(255, 255, 255);")
        self.horizontalLayout = QtWidgets.QHBoxLayout(FolderItemWidget)
        self.horizontalLayout.setContentsMargins(10, 6, 10, 6)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label_type = QtWidgets.QLabel(FolderItemWidget)
        self.label_type.setMinimumSize(QtCore.QSize(32, 32))
        self.label_type.setMaximumSize(QtCore.QSize(32, 32))
        self.label_type.setText("")
        self.label_type.setPixmap(QtGui.QPixmap(":/icons/images/icons/folder.png"))
        self.label_type.setScaledContents(True)
        self.label_type.setObjectName("label_type")
        self.horizontalLayout.addWidget(self.label_type)
        self.label_name = QtWidgets.QLabel(FolderItemWidget)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_name.setFont(font)
        self.label_name.setText("")
        self.label_name.setObjectName("label_name")
        self.horizontalLayout.addWidget(self.label_name)
        self.button_delete = QtWidgets.QPushButton(FolderItemWidget)
        self.button_delete.setMinimumSize(QtCore.QSize(32, 32))
        self.button_delete.setMaximumSize(QtCore.QSize(32, 32))
        self.button_delete.setText("")
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/garbage.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_delete.setIcon(icon)
        self.button_delete.setIconSize(QtCore.QSize(32, 32))
        self.button_delete.setFlat(True)
        self.button_delete.setObjectName("button_delete")
        self.horizontalLayout.addWidget(self.button_delete)

        self.retranslateUi(FolderItemWidget)
        QtCore.QMetaObject.connectSlotsByName(FolderItemWidget)

    def retranslateUi(self, FolderItemWidget):
        _translate = QtCore.QCoreApplication.translate
        FolderItemWidget.setWindowTitle(_translate("FolderItemWidget", "Form"))


from parsec.core.gui import resources_rc
