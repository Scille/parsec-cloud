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
        MainWindow.resize(800, 600)
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/parsec.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        MainWindow.setWindowIcon(icon)
        self.centralwidget = QtWidgets.QWidget(MainWindow)
        self.centralwidget.setObjectName("centralwidget")
        self.gridLayout = QtWidgets.QGridLayout(self.centralwidget)
        self.gridLayout.setObjectName("gridLayout")
        self.verticalLayout = QtWidgets.QVBoxLayout()
        self.verticalLayout.setSizeConstraint(QtWidgets.QLayout.SetMinimumSize)
        self.verticalLayout.setObjectName("verticalLayout")
        self.button_files = QtWidgets.QToolButton(self.centralwidget)
        self.button_files.setEnabled(False)
        self.button_files.setAccessibleName("")
        self.button_files.setAccessibleDescription("")
        icon1 = QtGui.QIcon()
        icon1.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/files.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_files.setIcon(icon1)
        self.button_files.setIconSize(QtCore.QSize(64, 64))
        self.button_files.setCheckable(True)
        self.button_files.setAutoRaise(True)
        self.button_files.setObjectName("button_files")
        self.verticalLayout.addWidget(self.button_files)
        self.button_users = QtWidgets.QToolButton(self.centralwidget)
        self.button_users.setEnabled(False)
        icon2 = QtGui.QIcon()
        icon2.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/users_manage.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_users.setIcon(icon2)
        self.button_users.setIconSize(QtCore.QSize(64, 64))
        self.button_users.setCheckable(True)
        self.button_users.setAutoRaise(True)
        self.button_users.setObjectName("button_users")
        self.verticalLayout.addWidget(self.button_users)
        self.button_settings = QtWidgets.QToolButton(self.centralwidget)
        self.button_settings.setEnabled(True)
        icon3 = QtGui.QIcon()
        icon3.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/settings.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_settings.setIcon(icon3)
        self.button_settings.setIconSize(QtCore.QSize(64, 64))
        self.button_settings.setCheckable(True)
        self.button_settings.setAutoRaise(True)
        self.button_settings.setObjectName("button_settings")
        self.verticalLayout.addWidget(self.button_settings)
        spacerItem = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem)
        self.gridLayout.addLayout(self.verticalLayout, 0, 0, 1, 1)
        self.main_widget_layout = QtWidgets.QVBoxLayout()
        self.main_widget_layout.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.main_widget_layout.setObjectName("main_widget_layout")
        self.gridLayout.addLayout(self.main_widget_layout, 0, 1, 1, 1)
        MainWindow.setCentralWidget(self.centralwidget)
        self.menubar = QtWidgets.QMenuBar(MainWindow)
        self.menubar.setGeometry(QtCore.QRect(0, 0, 800, 22))
        self.menubar.setObjectName("menubar")
        self.menu_file = QtWidgets.QMenu(self.menubar)
        self.menu_file.setObjectName("menu_file")
        self.menu_help = QtWidgets.QMenu(self.menubar)
        self.menu_help.setObjectName("menu_help")
        MainWindow.setMenuBar(self.menubar)
        self.action_quit = QtWidgets.QAction(MainWindow)
        icon4 = QtGui.QIcon()
        icon4.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/exit.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.action_quit.setIcon(icon4)
        self.action_quit.setShortcutContext(QtCore.Qt.ApplicationShortcut)
        self.action_quit.setObjectName("action_quit")
        self.action_about_parsec = QtWidgets.QAction(MainWindow)
        self.action_about_parsec.setIcon(icon)
        self.action_about_parsec.setObjectName("action_about_parsec")
        self.action_disconnect = QtWidgets.QAction(MainWindow)
        self.action_disconnect.setEnabled(False)
        self.action_disconnect.setObjectName("action_disconnect")
        self.action_remount = QtWidgets.QAction(MainWindow)
        self.action_remount.setEnabled(False)
        self.action_remount.setObjectName("action_remount")
        self.action_login = QtWidgets.QAction(MainWindow)
        self.action_login.setObjectName("action_login")
        self.action_in_tray = QtWidgets.QAction(MainWindow)
        self.action_in_tray.setObjectName("action_in_tray")
        self.menu_file.addAction(self.action_login)
        self.menu_file.addAction(self.action_remount)
        self.menu_file.addAction(self.action_disconnect)
        self.menu_file.addAction(self.action_quit)
        self.menu_help.addAction(self.action_about_parsec)
        self.menubar.addAction(self.menu_file.menuAction())
        self.menubar.addAction(self.menu_help.menuAction())

        self.retranslateUi(MainWindow)
        QtCore.QMetaObject.connectSlotsByName(MainWindow)

    def retranslateUi(self, MainWindow):
        _translate = QtCore.QCoreApplication.translate
        MainWindow.setWindowTitle(_translate("MainWindow", "Parsec"))
        self.button_files.setToolTip(
            _translate("MainWindow", "Display and manage your files and folders.")
        )
        self.button_files.setText(_translate("MainWindow", "Files"))
        self.button_users.setToolTip(_translate("MainWindow", "Display and manage your users."))
        self.button_users.setText(_translate("MainWindow", "Users"))
        self.button_settings.setToolTip(_translate("MainWindow", "Configure the application."))
        self.button_settings.setText(_translate("MainWindow", "Settings"))
        self.menu_file.setTitle(_translate("MainWindow", "File"))
        self.menu_help.setTitle(_translate("MainWindow", "Help"))
        self.action_quit.setText(_translate("MainWindow", "Quit"))
        self.action_quit.setShortcut(_translate("MainWindow", "Ctrl+Q"))
        self.action_about_parsec.setText(_translate("MainWindow", "About Parsec"))
        self.action_disconnect.setText(_translate("MainWindow", "Disconnect"))
        self.action_remount.setText(_translate("MainWindow", "Remount"))
        self.action_login.setText(_translate("MainWindow", "Log In"))
        self.action_in_tray.setText(_translate("MainWindow", "Reduce in tray"))


from parsec.core.gui import resources_rc
