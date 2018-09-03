# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/welcome_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_WelcomeWidget(object):
    def setupUi(self, WelcomeWidget):
        WelcomeWidget.setObjectName("WelcomeWidget")
        WelcomeWidget.resize(655, 674)
        self.verticalLayout = QtWidgets.QVBoxLayout(WelcomeWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_2 = QtWidgets.QLabel(WelcomeWidget)
        self.label_2.setText("")
        self.label_2.setPixmap(QtGui.QPixmap(":/logos/parsec"))
        self.label_2.setScaledContents(False)
        self.label_2.setAlignment(QtCore.Qt.AlignCenter)
        self.label_2.setObjectName("label_2")
        self.verticalLayout.addWidget(self.label_2)
        self.label = QtWidgets.QLabel(WelcomeWidget)
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)

        self.retranslateUi(WelcomeWidget)
        QtCore.QMetaObject.connectSlotsByName(WelcomeWidget)

    def retranslateUi(self, WelcomeWidget):
        _translate = QtCore.QCoreApplication.translate
        WelcomeWidget.setWindowTitle(_translate("WelcomeWidget", "Form"))
        self.label.setText(_translate("WelcomeWidget", "<html><head/><body><p><span style=\" font-size:16pt;\">Welcome to Parsec</span></p></body></html>"))

from parsec.core.gui import resources_rc
