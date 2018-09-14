# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/parent_folder_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_ParentFolderWidget(object):
    def setupUi(self, ParentFolderWidget):
        ParentFolderWidget.setObjectName("ParentFolderWidget")
        ParentFolderWidget.resize(385, 57)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(ParentFolderWidget.sizePolicy().hasHeightForWidth())
        ParentFolderWidget.setSizePolicy(sizePolicy)
        ParentFolderWidget.setMinimumSize(QtCore.QSize(0, 32))
        ParentFolderWidget.setMaximumSize(QtCore.QSize(16777215, 57))
        self.horizontalLayout = QtWidgets.QHBoxLayout(ParentFolderWidget)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label_icon = QtWidgets.QLabel(ParentFolderWidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Minimum)
        sizePolicy.setHorizontalStretch(48)
        sizePolicy.setVerticalStretch(48)
        sizePolicy.setHeightForWidth(self.label_icon.sizePolicy().hasHeightForWidth())
        self.label_icon.setSizePolicy(sizePolicy)
        self.label_icon.setMinimumSize(QtCore.QSize(48, 48))
        self.label_icon.setMaximumSize(QtCore.QSize(48, 48))
        self.label_icon.setText("")
        self.label_icon.setScaledContents(True)
        self.label_icon.setObjectName("label_icon")
        self.horizontalLayout.addWidget(self.label_icon)
        self.label = QtWidgets.QLabel(ParentFolderWidget)
        self.label.setObjectName("label")
        self.horizontalLayout.addWidget(self.label)

        self.retranslateUi(ParentFolderWidget)
        QtCore.QMetaObject.connectSlotsByName(ParentFolderWidget)

    def retranslateUi(self, ParentFolderWidget):
        _translate = QtCore.QCoreApplication.translate
        ParentFolderWidget.setWindowTitle(_translate("ParentFolderWidget", "Form"))
        self.label.setText(_translate("ParentFolderWidget", "<html><head/><body><p><span style=\" font-size:14pt;\">Parent folder</span></p></body></html>"))

