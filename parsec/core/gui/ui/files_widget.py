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
        FilesWidget.resize(588, 373)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(FilesWidget.sizePolicy().hasHeightForWidth())
        FilesWidget.setSizePolicy(sizePolicy)
        self.verticalLayout = QtWidgets.QVBoxLayout(FilesWidget)
        self.verticalLayout.setObjectName("verticalLayout")
        self.widget_files = QtWidgets.QWidget(FilesWidget)
        self.widget_files.setObjectName("widget_files")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.widget_files)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.label_current_directory = QtWidgets.QLabel(self.widget_files)
        self.label_current_directory.setText("")
        self.label_current_directory.setObjectName("label_current_directory")
        self.verticalLayout_2.addWidget(self.label_current_directory)
        self.label_cd_elems = QtWidgets.QLabel(self.widget_files)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_cd_elems.sizePolicy().hasHeightForWidth())
        self.label_cd_elems.setSizePolicy(sizePolicy)
        self.label_cd_elems.setText("")
        self.label_cd_elems.setObjectName("label_cd_elems")
        self.verticalLayout_2.addWidget(self.label_cd_elems)
        self.edit_search = QtWidgets.QLineEdit(self.widget_files)
        self.edit_search.setObjectName("edit_search")
        self.verticalLayout_2.addWidget(self.edit_search)
        self.list_files = QtWidgets.QListWidget(self.widget_files)
        self.list_files.setObjectName("list_files")
        self.verticalLayout_2.addWidget(self.list_files)
        self.verticalLayout.addWidget(self.widget_files)
        self.widget_workspaces = QtWidgets.QWidget(FilesWidget)
        self.widget_workspaces.setObjectName("widget_workspaces")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.widget_workspaces)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.line_edit_new_workspace = QtWidgets.QLineEdit(self.widget_workspaces)
        self.line_edit_new_workspace.setObjectName("line_edit_new_workspace")
        self.horizontalLayout.addWidget(self.line_edit_new_workspace)
        self.button_add_workspace = QtWidgets.QPushButton(self.widget_workspaces)
        self.button_add_workspace.setObjectName("button_add_workspace")
        self.horizontalLayout.addWidget(self.button_add_workspace)
        self.verticalLayout_3.addLayout(self.horizontalLayout)
        self.layout_workspaces = QtWidgets.QGridLayout()
        self.layout_workspaces.setObjectName("layout_workspaces")
        self.verticalLayout_3.addLayout(self.layout_workspaces)
        self.verticalLayout.addWidget(self.widget_workspaces)
        spacerItem = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.verticalLayout.addItem(spacerItem)

        self.retranslateUi(FilesWidget)
        QtCore.QMetaObject.connectSlotsByName(FilesWidget)

    def retranslateUi(self, FilesWidget):
        _translate = QtCore.QCoreApplication.translate
        FilesWidget.setWindowTitle(_translate("FilesWidget", "Form"))
        self.edit_search.setPlaceholderText(_translate("FilesWidget", "Search files or folders"))
        self.line_edit_new_workspace.setPlaceholderText(_translate("FilesWidget", "New workspace name"))
        self.button_add_workspace.setText(_translate("FilesWidget", "Add a workspace"))

