# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mount_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_MountWidget(object):
    def setupUi(self, MountWidget):
        MountWidget.setObjectName("MountWidget")
        MountWidget.resize(533, 407)
        MountWidget.setStyleSheet("background-color: rgb(255, 255, 255);\n" "")
        self.verticalLayout = QtWidgets.QVBoxLayout(MountWidget)
        self.verticalLayout.setContentsMargins(25, 30, 25, 25)
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setSpacing(15)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label = QtWidgets.QLabel(MountWidget)
        self.label.setMaximumSize(QtCore.QSize(40, 30))
        self.label.setText("")
        self.label.setPixmap(QtGui.QPixmap(":/icons/images/icons/hard-drive.png"))
        self.label.setScaledContents(True)
        self.label.setObjectName("label")
        self.horizontalLayout.addWidget(self.label)
        self.label_mountpoint = QtWidgets.QLabel(MountWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_mountpoint.sizePolicy().hasHeightForWidth())
        self.label_mountpoint.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setPointSize(14)
        font.setBold(True)
        font.setWeight(75)
        self.label_mountpoint.setFont(font)
        self.label_mountpoint.setStyleSheet("font-weight: bold;")
        self.label_mountpoint.setText("")
        self.label_mountpoint.setObjectName("label_mountpoint")
        self.horizontalLayout.addWidget(self.label_mountpoint)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.layout_content = QtWidgets.QVBoxLayout()
        self.layout_content.setSpacing(0)
        self.layout_content.setObjectName("layout_content")
        self.verticalLayout.addLayout(self.layout_content)

        self.retranslateUi(MountWidget)
        QtCore.QMetaObject.connectSlotsByName(MountWidget)

    def retranslateUi(self, MountWidget):
        _translate = QtCore.QCoreApplication.translate
        MountWidget.setWindowTitle(_translate("MountWidget", "Form"))


from parsec.core.gui import resources_rc
