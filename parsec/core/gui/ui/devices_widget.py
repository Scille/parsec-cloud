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
        self.verticalLayout = QtWidgets.QVBoxLayout(DevicesWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.button_add_device = QtWidgets.QPushButton(DevicesWidget)
        self.button_add_device.setObjectName("button_add_device")
        self.horizontalLayout.addWidget(self.button_add_device)
        spacerItem = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.horizontalLayout.addItem(spacerItem)
        self.verticalLayout.addLayout(self.horizontalLayout)
        spacerItem1 = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.verticalLayout.addItem(spacerItem1)

        self.retranslateUi(DevicesWidget)
        QtCore.QMetaObject.connectSlotsByName(DevicesWidget)

    def retranslateUi(self, DevicesWidget):
        _translate = QtCore.QCoreApplication.translate
        DevicesWidget.setWindowTitle(_translate("DevicesWidget", "Form"))
        self.button_add_device.setText(_translate("DevicesWidget", "Add a new device"))

