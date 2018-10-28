# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/parent_item_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_ParentItemWidget(object):
    def setupUi(self, ParentItemWidget):
        ParentItemWidget.setObjectName("ParentItemWidget")
        ParentItemWidget.resize(376, 40)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Fixed
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(ParentItemWidget.sizePolicy().hasHeightForWidth())
        ParentItemWidget.setSizePolicy(sizePolicy)
        ParentItemWidget.setMinimumSize(QtCore.QSize(0, 40))
        ParentItemWidget.setMaximumSize(QtCore.QSize(16777215, 40))
        self.horizontalLayout = QtWidgets.QHBoxLayout(ParentItemWidget)
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label_icon = QtWidgets.QLabel(ParentItemWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Minimum
        )
        sizePolicy.setHorizontalStretch(48)
        sizePolicy.setVerticalStretch(48)
        sizePolicy.setHeightForWidth(self.label_icon.sizePolicy().hasHeightForWidth())
        self.label_icon.setSizePolicy(sizePolicy)
        self.label_icon.setMinimumSize(QtCore.QSize(32, 32))
        self.label_icon.setMaximumSize(QtCore.QSize(32, 32))
        self.label_icon.setText("")
        self.label_icon.setScaledContents(True)
        self.label_icon.setObjectName("label_icon")
        self.horizontalLayout.addWidget(self.label_icon)
        self.label = QtWidgets.QLabel(ParentItemWidget)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label.setFont(font)
        self.label.setObjectName("label")
        self.horizontalLayout.addWidget(self.label)

        self.retranslateUi(ParentItemWidget)
        QtCore.QMetaObject.connectSlotsByName(ParentItemWidget)

    def retranslateUi(self, ParentItemWidget):
        _translate = QtCore.QCoreApplication.translate
        ParentItemWidget.setWindowTitle(_translate("ParentItemWidget", "Form"))
        self.label.setText(_translate("ParentItemWidget", "Parent folder"))
