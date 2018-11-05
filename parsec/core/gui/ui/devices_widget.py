# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/devices_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_DevicesWidget(object):
    def setupUi(self, DevicesWidget):
        DevicesWidget.setObjectName("DevicesWidget")
        DevicesWidget.resize(400, 300)
        DevicesWidget.setStyleSheet(
            "QWidget#DevicesWidget\n" "{\n" "background-color: rgb(255, 255, 255);\n" "}"
        )
        self.verticalLayout = QtWidgets.QVBoxLayout(DevicesWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.button_add_device = QtWidgets.QPushButton(DevicesWidget)
        self.button_add_device.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_add_device.setFont(font)
        self.button_add_device.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;"
        )
        self.button_add_device.setObjectName("button_add_device")
        self.horizontalLayout.addWidget(self.button_add_device)
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout.addItem(spacerItem)
        self.verticalLayout.addLayout(self.horizontalLayout)
        spacerItem1 = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem1)

        self.retranslateUi(DevicesWidget)
        QtCore.QMetaObject.connectSlotsByName(DevicesWidget)

    def retranslateUi(self, DevicesWidget):
        _translate = QtCore.QCoreApplication.translate
        DevicesWidget.setWindowTitle(_translate("DevicesWidget", "Form"))
        self.button_add_device.setText(_translate("DevicesWidget", "Add a new device"))
