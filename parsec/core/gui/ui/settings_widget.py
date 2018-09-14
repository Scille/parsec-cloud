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
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(SettingsWidget.sizePolicy().hasHeightForWidth())
        SettingsWidget.setSizePolicy(sizePolicy)
        self.verticalLayout = QtWidgets.QVBoxLayout(SettingsWidget)
        self.verticalLayout.setContentsMargins(5, 5, 5, 5)
        self.verticalLayout.setSpacing(5)
        self.verticalLayout.setObjectName("verticalLayout")
        self.tab_settings = QtWidgets.QTabWidget(SettingsWidget)
        self.tab_settings.setObjectName("tab_settings")
        self.tab_app = QtWidgets.QWidget()
        self.tab_app.setObjectName("tab_app")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.tab_app)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.groupBox = QtWidgets.QGroupBox(self.tab_app)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Minimum)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.groupBox.sizePolicy().hasHeightForWidth())
        self.groupBox.setSizePolicy(sizePolicy)
        self.groupBox.setObjectName("groupBox")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.groupBox)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.line_edit_mountpoint = QtWidgets.QLineEdit(self.groupBox)
        self.line_edit_mountpoint.setReadOnly(True)
        self.line_edit_mountpoint.setObjectName("line_edit_mountpoint")
        self.horizontalLayout.addWidget(self.line_edit_mountpoint)
        self.button_choose_mountpoint = QtWidgets.QPushButton(self.groupBox)
        self.button_choose_mountpoint.setObjectName("button_choose_mountpoint")
        self.horizontalLayout.addWidget(self.button_choose_mountpoint)
        self.verticalLayout_3.addLayout(self.horizontalLayout)
        self.verticalLayout_2.addWidget(self.groupBox)
        spacerItem = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.verticalLayout_2.addItem(spacerItem)
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
        self.groupBox.setTitle(_translate("SettingsWidget", "Mount options"))
        self.button_choose_mountpoint.setText(_translate("SettingsWidget", "Choose mount folder"))
        self.tab_settings.setTabText(self.tab_settings.indexOf(self.tab_app), _translate("SettingsWidget", "Global"))
        self.tab_settings.setTabText(self.tab_settings.indexOf(self.tab_network), _translate("SettingsWidget", "Network"))

from parsec.core.gui import resources_rc
