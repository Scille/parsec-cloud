# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/file_item_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_FileItemWidget(object):
    def setupUi(self, FileItemWidget):
        FileItemWidget.setObjectName("FileItemWidget")
        FileItemWidget.resize(626, 46)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Fixed
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(FileItemWidget.sizePolicy().hasHeightForWidth())
        FileItemWidget.setSizePolicy(sizePolicy)
        FileItemWidget.setMinimumSize(QtCore.QSize(0, 40))
        FileItemWidget.setMaximumSize(QtCore.QSize(16777215, 16777215))
        FileItemWidget.setBaseSize(QtCore.QSize(0, 40))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        FileItemWidget.setFont(font)
        FileItemWidget.setStyleSheet("background-color: rgb(255, 255, 255);")
        self.horizontalLayout = QtWidgets.QHBoxLayout(FileItemWidget)
        self.horizontalLayout.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.horizontalLayout.setContentsMargins(10, 6, 10, 6)
        self.horizontalLayout.setSpacing(6)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label_type = QtWidgets.QLabel(FileItemWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Minimum
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_type.sizePolicy().hasHeightForWidth())
        self.label_type.setSizePolicy(sizePolicy)
        self.label_type.setMinimumSize(QtCore.QSize(32, 32))
        self.label_type.setMaximumSize(QtCore.QSize(32, 32))
        self.label_type.setText("")
        self.label_type.setPixmap(QtGui.QPixmap(":/icons/images/icons/file.png"))
        self.label_type.setScaledContents(True)
        self.label_type.setObjectName("label_type")
        self.horizontalLayout.addWidget(self.label_type)
        self.label_name = QtWidgets.QLabel(FileItemWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_name.sizePolicy().hasHeightForWidth())
        self.label_name.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_name.setFont(font)
        self.label_name.setText("")
        self.label_name.setObjectName("label_name")
        self.horizontalLayout.addWidget(self.label_name)
        self.label_created = QtWidgets.QLabel(FileItemWidget)
        self.label_created.setMinimumSize(QtCore.QSize(180, 0))
        self.label_created.setMaximumSize(QtCore.QSize(180, 16777215))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_created.setFont(font)
        self.label_created.setStyleSheet("")
        self.label_created.setText("")
        self.label_created.setAlignment(QtCore.Qt.AlignCenter)
        self.label_created.setObjectName("label_created")
        self.horizontalLayout.addWidget(self.label_created)
        self.label_updated = QtWidgets.QLabel(FileItemWidget)
        self.label_updated.setMinimumSize(QtCore.QSize(180, 0))
        self.label_updated.setMaximumSize(QtCore.QSize(180, 16777215))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_updated.setFont(font)
        self.label_updated.setStyleSheet("")
        self.label_updated.setText("")
        self.label_updated.setAlignment(QtCore.Qt.AlignCenter)
        self.label_updated.setObjectName("label_updated")
        self.horizontalLayout.addWidget(self.label_updated)
        self.label_size = QtWidgets.QLabel(FileItemWidget)
        self.label_size.setMinimumSize(QtCore.QSize(80, 0))
        self.label_size.setMaximumSize(QtCore.QSize(80, 16777215))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.label_size.setFont(font)
        self.label_size.setStyleSheet("")
        self.label_size.setText("")
        self.label_size.setAlignment(QtCore.Qt.AlignCenter)
        self.label_size.setObjectName("label_size")
        self.horizontalLayout.addWidget(self.label_size)
        self.button_delete = QtWidgets.QPushButton(FileItemWidget)
        self.button_delete.setMinimumSize(QtCore.QSize(32, 32))
        self.button_delete.setMaximumSize(QtCore.QSize(32, 32))
        self.button_delete.setText("")
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/garbage.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_delete.setIcon(icon)
        self.button_delete.setIconSize(QtCore.QSize(32, 32))
        self.button_delete.setFlat(True)
        self.button_delete.setObjectName("button_delete")
        self.horizontalLayout.addWidget(self.button_delete)

        self.retranslateUi(FileItemWidget)
        QtCore.QMetaObject.connectSlotsByName(FileItemWidget)

    def retranslateUi(self, FileItemWidget):
        _translate = QtCore.QCoreApplication.translate
        FileItemWidget.setWindowTitle(_translate("FileItemWidget", "Form"))


from parsec.core.gui import resources_rc
