# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/about_dialog.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_AboutDialog(object):
    def setupUi(self, AboutDialog):
        AboutDialog.setObjectName("AboutDialog")
        AboutDialog.resize(490, 330)
        icon = QtGui.QIcon()
        icon.addPixmap(QtGui.QPixmap(":/icons/parsec_icon"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        AboutDialog.setWindowIcon(icon)
        AboutDialog.setModal(True)
        self.verticalLayout = QtWidgets.QVBoxLayout(AboutDialog)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label = QtWidgets.QLabel(AboutDialog)
        self.label.setText("")
        self.label.setPixmap(QtGui.QPixmap(":/logos/images/logos/parsec.png"))
        self.label.setScaledContents(False)
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.label_3 = QtWidgets.QLabel(AboutDialog)
        self.label_3.setAlignment(QtCore.Qt.AlignCenter)
        self.label_3.setOpenExternalLinks(True)
        self.label_3.setObjectName("label_3")
        self.verticalLayout.addWidget(self.label_3)
        self.label_2 = QtWidgets.QLabel(AboutDialog)
        self.label_2.setText("")
        self.label_2.setPixmap(QtGui.QPixmap(":/logos/images/logos/scille.png"))
        self.label_2.setAlignment(QtCore.Qt.AlignCenter)
        self.label_2.setObjectName("label_2")
        self.verticalLayout.addWidget(self.label_2)
        self.label_4 = QtWidgets.QLabel(AboutDialog)
        self.label_4.setAlignment(QtCore.Qt.AlignCenter)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.buttonBox = QtWidgets.QDialogButtonBox(AboutDialog)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(AboutDialog)
        self.buttonBox.rejected.connect(AboutDialog.reject)
        QtCore.QMetaObject.connectSlotsByName(AboutDialog)

    def retranslateUi(self, AboutDialog):
        _translate = QtCore.QCoreApplication.translate
        AboutDialog.setWindowTitle(_translate("AboutDialog", "About Parsec"))
        self.label_3.setText(_translate("AboutDialog", "<html><head/><body><p><span style=\" font-size:12pt;\">Visit </span><a href=\"http://www.parsec.cloud\"><span style=\" font-size:12pt; text-decoration: underline; color:#3b4ba1;\">parsec.cloud</span></a><span style=\" font-size:12pt;\"> for more information.</span></p><p><span style=\" font-size:12pt;\">Want to help ? Find us on </span><a href=\"https://github.com/Scille/parsec-cloud\"><span style=\" font-size:12pt; text-decoration: underline; color:#3b4ba1;\">GitHub</span></a><span style=\" font-size:12pt;\">.</span></p></body></html>"))
        self.label_4.setText(_translate("AboutDialog", "<html><head/><body><p><span style=\" font-size:12pt;\">Parsec is created and maintained by </span><a href=\"http://scille.eu\"><span style=\" font-size:12pt; text-decoration: underline; color:#3b4ba1;\">Scille</span></a><span style=\" font-size:12pt;\">.</span></p></body></html>"))

from parsec.core.gui import resources_rc
