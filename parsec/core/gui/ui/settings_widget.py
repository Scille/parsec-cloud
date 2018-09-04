# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/settings_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_SettingsWidget(object):
    def setupUi(self, SettingsWidget):
        SettingsWidget.setObjectName("SettingsWidget")
        SettingsWidget.resize(508, 356)
        self.verticalLayout = QtWidgets.QVBoxLayout(SettingsWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.tab_settings = QtWidgets.QTabWidget(SettingsWidget)
        self.tab_settings.setObjectName("tab_settings")
        self.tab_app = QtWidgets.QWidget()
        self.tab_app.setObjectName("tab_app")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.tab_app)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        icon = QtGui.QIcon()
        icon.addPixmap(QtGui.QPixmap(":/icons/images/icons/settings.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.tab_settings.addTab(self.tab_app, icon, "")
        self.tab_network = QtWidgets.QWidget()
        self.tab_network.setObjectName("tab_network")
        icon1 = QtGui.QIcon()
        icon1.addPixmap(QtGui.QPixmap(":/icons/images/icons/cloud.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.tab_settings.addTab(self.tab_network, icon1, "")
        self.verticalLayout.addWidget(self.tab_settings)

        self.retranslateUi(SettingsWidget)
        self.tab_settings.setCurrentIndex(0)
        QtCore.QMetaObject.connectSlotsByName(SettingsWidget)

    def retranslateUi(self, SettingsWidget):
        _translate = QtCore.QCoreApplication.translate
        SettingsWidget.setWindowTitle(_translate("SettingsWidget", "Form"))
        self.tab_settings.setTabText(self.tab_settings.indexOf(self.tab_app), _translate("SettingsWidget", "Global"))
        self.tab_settings.setTabText(self.tab_settings.indexOf(self.tab_network), _translate("SettingsWidget", "Network"))

