# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/files_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_FilesWidget(object):
    def setupUi(self, FilesWidget):
        FilesWidget.setObjectName("FilesWidget")
        FilesWidget.resize(612, 637)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(FilesWidget.sizePolicy().hasHeightForWidth())
        FilesWidget.setSizePolicy(sizePolicy)
        self.verticalLayout = QtWidgets.QVBoxLayout(FilesWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_current_directory = QtWidgets.QLabel(FilesWidget)
        self.label_current_directory.setText("")
        self.label_current_directory.setObjectName("label_current_directory")
        self.verticalLayout.addWidget(self.label_current_directory)
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.label_cd_elems = QtWidgets.QLabel(FilesWidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_cd_elems.sizePolicy().hasHeightForWidth())
        self.label_cd_elems.setSizePolicy(sizePolicy)
        self.label_cd_elems.setText("")
        self.label_cd_elems.setObjectName("label_cd_elems")
        self.horizontalLayout_2.addWidget(self.label_cd_elems)
        self.label_cd_size = QtWidgets.QLabel(FilesWidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_cd_size.sizePolicy().hasHeightForWidth())
        self.label_cd_size.setSizePolicy(sizePolicy)
        self.label_cd_size.setText("")
        self.label_cd_size.setObjectName("label_cd_size")
        self.horizontalLayout_2.addWidget(self.label_cd_size)
        self.verticalLayout.addLayout(self.horizontalLayout_2)
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.edit_search = QtWidgets.QLineEdit(FilesWidget)
        self.edit_search.setObjectName("edit_search")
        self.horizontalLayout.addWidget(self.edit_search)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.list_files = QtWidgets.QListWidget(FilesWidget)
        self.list_files.setObjectName("list_files")
        self.verticalLayout.addWidget(self.list_files)

        self.retranslateUi(FilesWidget)
        QtCore.QMetaObject.connectSlotsByName(FilesWidget)

    def retranslateUi(self, FilesWidget):
        _translate = QtCore.QCoreApplication.translate
        FilesWidget.setWindowTitle(_translate("FilesWidget", "Form"))
        self.edit_search.setPlaceholderText(_translate("FilesWidget", "Search files or folders"))

