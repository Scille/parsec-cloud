# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/home_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_HomeWidget(object):
    def setupUi(self, HomeWidget):
        HomeWidget.setObjectName("HomeWidget")
        HomeWidget.resize(655, 674)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(HomeWidget.sizePolicy().hasHeightForWidth())
        HomeWidget.setSizePolicy(sizePolicy)
        self.verticalLayout = QtWidgets.QVBoxLayout(HomeWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_2 = QtWidgets.QLabel(HomeWidget)
        self.label_2.setText("")
        self.label_2.setPixmap(QtGui.QPixmap(":/logos/parsec"))
        self.label_2.setScaledContents(False)
        self.label_2.setAlignment(QtCore.Qt.AlignCenter)
        self.label_2.setObjectName("label_2")
        self.verticalLayout.addWidget(self.label_2)
        self.label = QtWidgets.QLabel(HomeWidget)
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)

        self.retranslateUi(HomeWidget)
        QtCore.QMetaObject.connectSlotsByName(HomeWidget)

    def retranslateUi(self, HomeWidget):
        _translate = QtCore.QCoreApplication.translate
        HomeWidget.setWindowTitle(_translate("HomeWidget", "Form"))
        self.label.setText(_translate("HomeWidget", "<html><head/><body><p><span style=\" font-size:16pt;\">Welcome to Parsec</span></p></body></html>"))

from parsec.core.gui import resources_rc
