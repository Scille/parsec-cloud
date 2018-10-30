# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/main_window.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        MainWindow.setObjectName("MainWindow")
        MainWindow.resize(923, 607)
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/parsec.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        MainWindow.setWindowIcon(icon)
        MainWindow.setAutoFillBackground(True)
        MainWindow.setStyleSheet("")
        MainWindow.setDocumentMode(False)
        MainWindow.setTabShape(QtWidgets.QTabWidget.Rounded)
        MainWindow.setUnifiedTitleAndToolBarOnMac(True)
        self.centralwidget = QtWidgets.QWidget(MainWindow)
        self.centralwidget.setObjectName("centralwidget")
        self.horizontalLayout = QtWidgets.QHBoxLayout(self.centralwidget)
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout.setSpacing(0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.widget_2 = QtWidgets.QWidget(self.centralwidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.widget_2.sizePolicy().hasHeightForWidth())
        self.widget_2.setSizePolicy(sizePolicy)
        self.widget_2.setObjectName("widget_2")
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout(self.widget_2)
        self.horizontalLayout_2.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout_2.setSpacing(0)
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.widget_menu = QtWidgets.QWidget(self.widget_2)
        self.widget_menu.setMinimumSize(QtCore.QSize(200, 0))
        self.widget_menu.setAutoFillBackground(False)
        self.widget_menu.setStyleSheet("background-color: rgb(12, 65, 156);\n" "")
        self.widget_menu.setObjectName("widget_menu")
        self.verticalLayout = QtWidgets.QVBoxLayout(self.widget_menu)
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName("verticalLayout")
        self.widget_3 = QtWidgets.QWidget(self.widget_menu)
        self.widget_3.setStyleSheet("background-color: rgb(21, 84, 169);")
        self.widget_3.setObjectName("widget_3")
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout(self.widget_3)
        self.horizontalLayout_3.setContentsMargins(10, 10, 10, 10)
        self.horizontalLayout_3.setSpacing(6)
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        self.label = QtWidgets.QLabel(self.widget_3)
        self.label.setMaximumSize(QtCore.QSize(80000, 80000))
        self.label.setText("")
        self.label.setPixmap(QtGui.QPixmap(":/logos/images/logos/parsec_white_small.png"))
        self.label.setScaledContents(False)
        self.label.setObjectName("label")
        self.horizontalLayout_3.addWidget(self.label)
        self.verticalLayout.addWidget(self.widget_3)
        self.frame = QtWidgets.QFrame(self.widget_menu)
        self.frame.setStyleSheet("background-color: rgb(46, 145, 208);")
        self.frame.setFrameShape(QtWidgets.QFrame.HLine)
        self.frame.setFrameShadow(QtWidgets.QFrame.Sunken)
        self.frame.setLineWidth(2)
        self.frame.setObjectName("frame")
        self.verticalLayout.addWidget(self.frame)
        self.button_files = QtWidgets.QPushButton(self.widget_menu)
        self.button_files.setEnabled(True)
        self.button_files.setMinimumSize(QtCore.QSize(0, 64))
        self.button_files.setMaximumSize(QtCore.QSize(16777215, 64))
        self.button_files.setBaseSize(QtCore.QSize(64, 0))
        font = QtGui.QFont()
        font.setFamily("Proxima Nova")
        font.setPointSize(13)
        font.setBold(False)
        font.setItalic(False)
        font.setWeight(50)
        self.button_files.setFont(font)
        self.button_files.setLayoutDirection(QtCore.Qt.LeftToRight)
        self.button_files.setAutoFillBackground(False)
        self.button_files.setStyleSheet(
            "QPushButton\n"
            "{\n"
            "color: rgb(255, 255, 255);\n"
            "text-align: left;\n"
            "padding-left: 10px;\n"
            "}\n"
            "\n"
            "QPushButton:checked\n"
            "{\n"
            "background-color: rgb(46, 145, 208);\n"
            "border: 0;\n"
            "}"
        )
        icon1 = QtGui.QIcon()
        icon1.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/menu_folder.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_files.setIcon(icon1)
        self.button_files.setIconSize(QtCore.QSize(32, 32))
        self.button_files.setCheckable(True)
        self.button_files.setFlat(True)
        self.button_files.setObjectName("button_files")
        self.verticalLayout.addWidget(self.button_files)
        self.button_users = QtWidgets.QPushButton(self.widget_menu)
        self.button_users.setEnabled(True)
        self.button_users.setMinimumSize(QtCore.QSize(0, 64))
        self.button_users.setMaximumSize(QtCore.QSize(16777215, 64))
        self.button_users.setBaseSize(QtCore.QSize(0, 64))
        font = QtGui.QFont()
        font.setFamily("Proxima Nova")
        font.setPointSize(13)
        font.setBold(False)
        font.setItalic(False)
        font.setWeight(50)
        self.button_users.setFont(font)
        self.button_users.setStyleSheet(
            "QPushButton\n"
            "{\n"
            "color: rgb(255, 255, 255);\n"
            "text-align: left;\n"
            "padding-left: 10px;\n"
            "}\n"
            "\n"
            "QPushButton:checked\n"
            "{\n"
            "background-color: rgb(46, 145, 208);\n"
            "border: 0;\n"
            "}"
        )
        icon2 = QtGui.QIcon()
        icon2.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/menu_group.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_users.setIcon(icon2)
        self.button_users.setIconSize(QtCore.QSize(32, 32))
        self.button_users.setCheckable(True)
        self.button_users.setFlat(True)
        self.button_users.setObjectName("button_users")
        self.verticalLayout.addWidget(self.button_users)
        self.button_devices = QtWidgets.QPushButton(self.widget_menu)
        self.button_devices.setEnabled(True)
        self.button_devices.setMinimumSize(QtCore.QSize(0, 64))
        self.button_devices.setMaximumSize(QtCore.QSize(16777215, 64))
        self.button_devices.setBaseSize(QtCore.QSize(0, 64))
        font = QtGui.QFont()
        font.setFamily("Proxima Nova")
        font.setPointSize(13)
        font.setBold(False)
        font.setItalic(False)
        font.setWeight(50)
        self.button_devices.setFont(font)
        self.button_devices.setStyleSheet(
            "QPushButton\n"
            "{\n"
            "color: rgb(255, 255, 255);\n"
            "text-align: left;\n"
            "padding-left: 10px;\n"
            "}\n"
            "\n"
            "QPushButton:checked\n"
            "{\n"
            "background-color: rgb(46, 145, 208);\n"
            "border: 0;\n"
            "}"
        )
        icon3 = QtGui.QIcon()
        icon3.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/menu_devices.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_devices.setIcon(icon3)
        self.button_devices.setIconSize(QtCore.QSize(32, 32))
        self.button_devices.setCheckable(True)
        self.button_devices.setChecked(False)
        self.button_devices.setFlat(True)
        self.button_devices.setObjectName("button_devices")
        self.verticalLayout.addWidget(self.button_devices)
        self.button_settings = QtWidgets.QPushButton(self.widget_menu)
        self.button_settings.setEnabled(True)
        self.button_settings.setMinimumSize(QtCore.QSize(0, 64))
        self.button_settings.setMaximumSize(QtCore.QSize(16777215, 64))
        self.button_settings.setBaseSize(QtCore.QSize(0, 64))
        font = QtGui.QFont()
        font.setFamily("Proxima Nova")
        font.setPointSize(13)
        font.setBold(False)
        font.setItalic(False)
        font.setWeight(50)
        self.button_settings.setFont(font)
        self.button_settings.setStyleSheet(
            "QPushButton\n"
            "{\n"
            "color: rgb(255, 255, 255);\n"
            "text-align: left;\n"
            "padding-left: 10px;\n"
            "}\n"
            "\n"
            "QPushButton:checked\n"
            "{\n"
            "background-color: rgb(46, 145, 208);\n"
            "border: 0;\n"
            "}"
        )
        icon4 = QtGui.QIcon()
        icon4.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/menu_settings.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_settings.setIcon(icon4)
        self.button_settings.setIconSize(QtCore.QSize(32, 32))
        self.button_settings.setCheckable(True)
        self.button_settings.setFlat(True)
        self.button_settings.setObjectName("button_settings")
        self.verticalLayout.addWidget(self.button_settings)
        spacerItem = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem)
        self.button_user = QtWidgets.QPushButton(self.widget_menu)
        self.button_user.setMinimumSize(QtCore.QSize(0, 64))
        self.button_user.setMaximumSize(QtCore.QSize(16777215, 64))
        font = QtGui.QFont()
        font.setPointSize(13)
        self.button_user.setFont(font)
        self.button_user.setStyleSheet(
            "border-top: 2px solid rgb(46, 146, 208);\n"
            "border-bottom: 2px solid rgb(46, 146, 208);\n"
            "color: rgb(255, 255, 255);\n"
            "text-align: left;\n"
            "padding-left: 10px;\n"
            "background-color: rgb(22, 84, 169);\n"
            ""
        )
        self.button_user.setText("")
        self.button_user.setIconSize(QtCore.QSize(32, 32))
        self.button_user.setFlat(True)
        self.button_user.setObjectName("button_user")
        self.verticalLayout.addWidget(self.button_user)
        self.button_logout = QtWidgets.QPushButton(self.widget_menu)
        self.button_logout.setEnabled(True)
        self.button_logout.setMinimumSize(QtCore.QSize(0, 64))
        self.button_logout.setMaximumSize(QtCore.QSize(16777215, 64))
        self.button_logout.setBaseSize(QtCore.QSize(0, 64))
        font = QtGui.QFont()
        font.setFamily("Proxima Nova")
        font.setPointSize(13)
        font.setBold(False)
        font.setItalic(False)
        font.setWeight(50)
        self.button_logout.setFont(font)
        self.button_logout.setStyleSheet(
            "color: rgb(255, 255, 255);\n" "text-align: left;\n" "padding-left: 10px;\n" ""
        )
        icon5 = QtGui.QIcon()
        icon5.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/menu_logout.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_logout.setIcon(icon5)
        self.button_logout.setIconSize(QtCore.QSize(32, 32))
        self.button_logout.setCheckable(False)
        self.button_logout.setFlat(True)
        self.button_logout.setObjectName("button_logout")
        self.verticalLayout.addWidget(self.button_logout)
        self.horizontalLayout_2.addWidget(self.widget_menu)
        self.widget_main = QtWidgets.QWidget(self.widget_2)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.widget_main.sizePolicy().hasHeightForWidth())
        self.widget_main.setSizePolicy(sizePolicy)
        self.widget_main.setStyleSheet(
            "QWidget#widget_main\n" "{\n" "    background-color: rgb(255, 255, 255);\n" "}"
        )
        self.widget_main.setObjectName("widget_main")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.widget_main)
        self.verticalLayout_2.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_2.setSpacing(0)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.main_widget_layout = QtWidgets.QVBoxLayout()
        self.main_widget_layout.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.main_widget_layout.setContentsMargins(10, 10, 10, 10)
        self.main_widget_layout.setSpacing(0)
        self.main_widget_layout.setObjectName("main_widget_layout")
        self.verticalLayout_2.addLayout(self.main_widget_layout)
        self.horizontalLayout_2.addWidget(self.widget_main)
        self.horizontalLayout.addWidget(self.widget_2)
        MainWindow.setCentralWidget(self.centralwidget)

        self.retranslateUi(MainWindow)
        QtCore.QMetaObject.connectSlotsByName(MainWindow)

    def retranslateUi(self, MainWindow):
        _translate = QtCore.QCoreApplication.translate
        MainWindow.setWindowTitle(_translate("MainWindow", "Parsec"))
        self.button_files.setText(_translate("MainWindow", "   Documents"))
        self.button_users.setText(_translate("MainWindow", "   Users"))
        self.button_devices.setText(_translate("MainWindow", "   Devices"))
        self.button_settings.setText(_translate("MainWindow", "   Settings"))
        self.button_logout.setText(_translate("MainWindow", "   Log Out"))


from parsec.core.gui import resources_rc
