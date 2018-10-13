# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/new_login_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_LoginWidget(object):
    def setupUi(self, LoginWidget):
        LoginWidget.setObjectName("LoginWidget")
        LoginWidget.resize(673, 300)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(LoginWidget.sizePolicy().hasHeightForWidth())
        LoginWidget.setSizePolicy(sizePolicy)
        LoginWidget.setMinimumSize(QtCore.QSize(650, 0))
        LoginWidget.setAutoFillBackground(False)
        LoginWidget.setStyleSheet("background-color: rgb(255, 255, 255);")
        self.horizontalLayout = QtWidgets.QHBoxLayout(LoginWidget)
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout.setSpacing(0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        spacerItem = QtWidgets.QSpacerItem(
            0, 0, QtWidgets.QSizePolicy.MinimumExpanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout.addItem(spacerItem)
        self.verticalLayout = QtWidgets.QVBoxLayout()
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName("verticalLayout")
        spacerItem1 = QtWidgets.QSpacerItem(
            0, 0, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem1)
        self.widget = QtWidgets.QWidget(LoginWidget)
        self.widget.setStyleSheet(
            "background-color: rgb(12, 65, 156);\n"
            "\n"
            "QLineEdit\n"
            "{\n"
            "border: 2px solid rgb(46, 145, 208);\n"
            "background-color: white;\n"
            "color: black;\n"
            "}\n"
            "\n"
            "QComboBox\n"
            "{\n"
            "border: 2px solid rgb(46, 145, 208);\n"
            "background-color: white;\n"
            "color: black;\n"
            "}\n"
            "\n"
            "QComboBox::down-arrow\n"
            "{\n"
            "image: url(:/icons/images/icons/down-arrow.png);\n"
            "}\n"
            "\n"
            "QCheckBox\n"
            "{\n"
            "color: rgb(255, 255, 255);\n"
            "}\n"
            "\n"
            "QCheckBox::indicator\n"
            "{\n"
            "width: 15px;\n"
            "height: 15px;\n"
            "background-color: rgb(255, 255, 255);\n"
            "border: 2px solid rgb(46, 145, 208);\n"
            "color: black;\n"
            "}\n"
            "\n"
            "QCheckBox::indicator:checked\n"
            "{\n"
            "image: url(:/icons/images/icons/checked.png)\n"
            "}"
        )
        self.widget.setObjectName("widget")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.widget)
        self.verticalLayout_2.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_2.setSpacing(0)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.widget_3 = QtWidgets.QWidget(self.widget)
        self.widget_3.setMaximumSize(QtCore.QSize(16777215, 16777215))
        self.widget_3.setStyleSheet("background-color: rgb(21, 84, 169);")
        self.widget_3.setObjectName("widget_3")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.widget_3)
        self.verticalLayout_3.setContentsMargins(10, 10, 10, 10)
        self.verticalLayout_3.setSpacing(0)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.label = QtWidgets.QLabel(self.widget_3)
        self.label.setMinimumSize(QtCore.QSize(0, 50))
        self.label.setMaximumSize(QtCore.QSize(16777215, 16777215))
        self.label.setText("")
        self.label.setPixmap(QtGui.QPixmap(":/logos/images/logos/parsec_white_medium.png"))
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setObjectName("label")
        self.verticalLayout_3.addWidget(self.label)
        self.verticalLayout_2.addWidget(self.widget_3)
        self.frame = QtWidgets.QFrame(self.widget)
        self.frame.setStyleSheet("background-color: rgb(46, 145, 208);")
        self.frame.setFrameShape(QtWidgets.QFrame.HLine)
        self.frame.setFrameShadow(QtWidgets.QFrame.Sunken)
        self.frame.setLineWidth(2)
        self.frame.setObjectName("frame")
        self.verticalLayout_2.addWidget(self.frame)
        self.widget_2 = QtWidgets.QWidget(self.widget)
        self.widget_2.setStyleSheet("")
        self.widget_2.setObjectName("widget_2")
        self.verticalLayout_4 = QtWidgets.QVBoxLayout(self.widget_2)
        self.verticalLayout_4.setContentsMargins(20, 0, 20, 20)
        self.verticalLayout_4.setSpacing(10)
        self.verticalLayout_4.setObjectName("verticalLayout_4")
        self.layout = QtWidgets.QVBoxLayout()
        self.layout.setContentsMargins(10, 20, 10, 10)
        self.layout.setSpacing(0)
        self.layout.setObjectName("layout")
        self.verticalLayout_4.addLayout(self.layout)
        self.button_register_user_instead = QtWidgets.QPushButton(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.button_register_user_instead.setFont(font)
        self.button_register_user_instead.setStyleSheet(
            "border: 0;\n" "color: rgb(255, 255, 255);\n" "text-align: left;\n" ""
        )
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/right-chevron.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_register_user_instead.setIcon(icon)
        self.button_register_user_instead.setFlat(True)
        self.button_register_user_instead.setObjectName("button_register_user_instead")
        self.verticalLayout_4.addWidget(self.button_register_user_instead)
        self.button_register_device_instead = QtWidgets.QPushButton(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.button_register_device_instead.setFont(font)
        self.button_register_device_instead.setStyleSheet(
            "border: 0;\n" "color: rgb(255, 255, 255);\n" "text-align: left;\n" ""
        )
        self.button_register_device_instead.setIcon(icon)
        self.button_register_device_instead.setFlat(True)
        self.button_register_device_instead.setObjectName("button_register_device_instead")
        self.verticalLayout_4.addWidget(self.button_register_device_instead)
        self.button_login_instead = QtWidgets.QPushButton(self.widget_2)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.button_login_instead.setFont(font)
        self.button_login_instead.setStyleSheet(
            "border: 0;\n" "color: rgb(255, 255, 255);\n" "text-align: left;\n" ""
        )
        self.button_login_instead.setIcon(icon)
        self.button_login_instead.setFlat(True)
        self.button_login_instead.setObjectName("button_login_instead")
        self.verticalLayout_4.addWidget(self.button_login_instead)
        self.verticalLayout_2.addWidget(self.widget_2)
        self.verticalLayout.addWidget(self.widget)
        spacerItem2 = QtWidgets.QSpacerItem(
            0, 0, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem2)
        self.horizontalLayout.addLayout(self.verticalLayout)
        spacerItem3 = QtWidgets.QSpacerItem(
            0, 0, QtWidgets.QSizePolicy.MinimumExpanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout.addItem(spacerItem3)

        self.retranslateUi(LoginWidget)
        self.button_login_instead.clicked.connect(self.button_register_device_instead.show)
        self.button_login_instead.clicked.connect(self.button_register_user_instead.show)
        self.button_login_instead.clicked.connect(self.button_login_instead.hide)
        self.button_register_device_instead.clicked.connect(self.button_register_user_instead.show)
        self.button_register_device_instead.clicked.connect(
            self.button_register_device_instead.hide
        )
        self.button_register_device_instead.clicked.connect(self.button_login_instead.show)
        self.button_register_user_instead.clicked.connect(self.button_login_instead.show)
        self.button_register_user_instead.clicked.connect(self.button_register_device_instead.show)
        self.button_register_user_instead.clicked.connect(self.button_register_user_instead.hide)
        QtCore.QMetaObject.connectSlotsByName(LoginWidget)

    def retranslateUi(self, LoginWidget):
        _translate = QtCore.QCoreApplication.translate
        LoginWidget.setWindowTitle(_translate("LoginWidget", "Form"))
        self.button_register_user_instead.setText(
            _translate("LoginWidget", "Register a new account")
        )
        self.button_register_device_instead.setText(
            _translate("LoginWidget", "Register a new device")
        )
        self.button_login_instead.setText(_translate("LoginWidget", "Log In"))


from parsec.core.gui import resources_rc
