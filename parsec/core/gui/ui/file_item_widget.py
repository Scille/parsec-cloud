# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/file_item_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_FileItemWidget(object):
    def setupUi(self, FileItemWidget):
        FileItemWidget.setObjectName("FileItemWidget")
        FileItemWidget.resize(554, 86)
        self.horizontalLayout = QtWidgets.QHBoxLayout(FileItemWidget)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label_file_type = QtWidgets.QLabel(FileItemWidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Minimum)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_file_type.sizePolicy().hasHeightForWidth())
        self.label_file_type.setSizePolicy(sizePolicy)
        self.label_file_type.setMinimumSize(QtCore.QSize(64, 64))
        self.label_file_type.setMaximumSize(QtCore.QSize(64, 64))
        self.label_file_type.setText("")
        self.label_file_type.setPixmap(QtGui.QPixmap(":/icons/images/icons/file_ok.png"))
        self.label_file_type.setScaledContents(True)
        self.label_file_type.setObjectName("label_file_type")
        self.horizontalLayout.addWidget(self.label_file_type)
        self.verticalLayout = QtWidgets.QVBoxLayout()
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.label_file_name = QtWidgets.QLabel(FileItemWidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_file_name.sizePolicy().hasHeightForWidth())
        self.label_file_name.setSizePolicy(sizePolicy)
        self.label_file_name.setObjectName("label_file_name")
        self.horizontalLayout_2.addWidget(self.label_file_name)
        self.label_file_size = QtWidgets.QLabel(FileItemWidget)
        self.label_file_size.setObjectName("label_file_size")
        self.horizontalLayout_2.addWidget(self.label_file_size)
        self.verticalLayout.addLayout(self.horizontalLayout_2)
        self.label_created = QtWidgets.QLabel(FileItemWidget)
        self.label_created.setObjectName("label_created")
        self.verticalLayout.addWidget(self.label_created)
        self.label_modified = QtWidgets.QLabel(FileItemWidget)
        self.label_modified.setObjectName("label_modified")
        self.verticalLayout.addWidget(self.label_modified)
        self.horizontalLayout.addLayout(self.verticalLayout)
        self.button_delete = QtWidgets.QToolButton(FileItemWidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.button_delete.sizePolicy().hasHeightForWidth())
        self.button_delete.setSizePolicy(sizePolicy)
        self.button_delete.setMinimumSize(QtCore.QSize(48, 48))
        icon = QtGui.QIcon()
        icon.addPixmap(QtGui.QPixmap(":/icons/images/icons/error.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.button_delete.setIcon(icon)
        self.button_delete.setIconSize(QtCore.QSize(48, 48))
        self.button_delete.setToolButtonStyle(QtCore.Qt.ToolButtonIconOnly)
        self.button_delete.setAutoRaise(True)
        self.button_delete.setObjectName("button_delete")
        self.horizontalLayout.addWidget(self.button_delete)

        self.retranslateUi(FileItemWidget)
        QtCore.QMetaObject.connectSlotsByName(FileItemWidget)

    def retranslateUi(self, FileItemWidget):
        _translate = QtCore.QCoreApplication.translate
        FileItemWidget.setWindowTitle(_translate("FileItemWidget", "Form"))
        self.label_file_name.setText(_translate("FileItemWidget", "<html><head/><body><p><span style=\" font-size:14pt;\">File1.txt</span></p></body></html>"))
        self.label_file_size.setText(_translate("FileItemWidget", "<html><head/><body><p>42 Ko</p></body></html>"))
        self.label_created.setText(_translate("FileItemWidget", "<html><head/><body><p><span style=\" font-style:italic;\">Created on 01/01/2018 by Max</span></p></body></html>"))
        self.label_modified.setText(_translate("FileItemWidget", "<html><head/><body><p><span style=\" font-style:italic;\">Last modified on 02/01/2018 by Max</span></p></body></html>"))
        self.button_delete.setText(_translate("FileItemWidget", "..."))

from parsec.core.gui import resources_rc
