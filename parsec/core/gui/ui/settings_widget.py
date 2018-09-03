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
        self.groupBox = QtWidgets.QGroupBox(self.tab_app)
        self.groupBox.setObjectName("groupBox")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.groupBox)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.label_2 = QtWidgets.QLabel(self.groupBox)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Minimum)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_2.sizePolicy().hasHeightForWidth())
        self.label_2.setSizePolicy(sizePolicy)
        self.label_2.setObjectName("label_2")
        self.horizontalLayout_2.addWidget(self.label_2)
        self.label_current_language = QtWidgets.QLabel(self.groupBox)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Minimum)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_current_language.sizePolicy().hasHeightForWidth())
        self.label_current_language.setSizePolicy(sizePolicy)
        self.label_current_language.setText("")
        self.label_current_language.setObjectName("label_current_language")
        self.horizontalLayout_2.addWidget(self.label_current_language)
        self.verticalLayout_3.addLayout(self.horizontalLayout_2)
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.combo_languages = QtWidgets.QComboBox(self.groupBox)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.combo_languages.sizePolicy().hasHeightForWidth())
        self.combo_languages.setSizePolicy(sizePolicy)
        self.combo_languages.setCurrentText("")
        self.combo_languages.setObjectName("combo_languages")
        self.horizontalLayout.addWidget(self.combo_languages)
        self.button_switch_language = QtWidgets.QPushButton(self.groupBox)
        self.button_switch_language.setObjectName("button_switch_language")
        self.horizontalLayout.addWidget(self.button_switch_language)
        self.verticalLayout_3.addLayout(self.horizontalLayout)
        self.verticalLayout_2.addWidget(self.groupBox)
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
        self.groupBox.setTitle(_translate("SettingsWidget", "Languages"))
        self.label_2.setText(_translate("SettingsWidget", "Current language :"))
        self.button_switch_language.setText(_translate("SettingsWidget", "Switch to language"))
        self.tab_settings.setTabText(self.tab_settings.indexOf(self.tab_app), _translate("SettingsWidget", "Global"))
        self.tab_settings.setTabText(self.tab_settings.indexOf(self.tab_network), _translate("SettingsWidget", "Network"))

from parsec.core.gui import resources_rc
