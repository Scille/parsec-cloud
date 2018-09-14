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
        FilesWidget.resize(588, 493)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(FilesWidget.sizePolicy().hasHeightForWidth())
        FilesWidget.setSizePolicy(sizePolicy)
        self.verticalLayout = QtWidgets.QVBoxLayout(FilesWidget)
        self.verticalLayout.setContentsMargins(5, 5, 5, 5)
        self.verticalLayout.setSpacing(5)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout_4 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_4.setObjectName("horizontalLayout_4")
        self.label = QtWidgets.QLabel(FilesWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label.sizePolicy().hasHeightForWidth())
        self.label.setSizePolicy(sizePolicy)
        self.label.setObjectName("label")
        self.horizontalLayout_4.addWidget(self.label)
        self.label_mountpoint = QtWidgets.QLabel(FilesWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_mountpoint.sizePolicy().hasHeightForWidth())
        self.label_mountpoint.setSizePolicy(sizePolicy)
        self.label_mountpoint.setText("")
        self.label_mountpoint.setObjectName("label_mountpoint")
        self.horizontalLayout_4.addWidget(self.label_mountpoint)
        self.verticalLayout.addLayout(self.horizontalLayout_4)
        self.widget_files = QtWidgets.QWidget(FilesWidget)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.widget_files.sizePolicy().hasHeightForWidth())
        self.widget_files.setSizePolicy(sizePolicy)
        self.widget_files.setObjectName("widget_files")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.widget_files)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.button_to_workspaces = QtWidgets.QCommandLinkButton(self.widget_files)
        self.button_to_workspaces.setObjectName("button_to_workspaces")
        self.horizontalLayout_2.addWidget(self.button_to_workspaces)
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_2.addItem(spacerItem)
        self.verticalLayout_2.addLayout(self.horizontalLayout_2)
        self.label_current_directory = QtWidgets.QLabel(self.widget_files)
        self.label_current_directory.setText("")
        self.label_current_directory.setObjectName("label_current_directory")
        self.verticalLayout_2.addWidget(self.label_current_directory)
        self.label_cd_elems = QtWidgets.QLabel(self.widget_files)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_cd_elems.sizePolicy().hasHeightForWidth())
        self.label_cd_elems.setSizePolicy(sizePolicy)
        self.label_cd_elems.setText("")
        self.label_cd_elems.setObjectName("label_cd_elems")
        self.verticalLayout_2.addWidget(self.label_cd_elems)
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        self.button_create_folder = QtWidgets.QPushButton(self.widget_files)
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/folder_add.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_create_folder.setIcon(icon)
        self.button_create_folder.setObjectName("button_create_folder")
        self.horizontalLayout_3.addWidget(self.button_create_folder)
        spacerItem1 = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_3.addItem(spacerItem1)
        self.verticalLayout_2.addLayout(self.horizontalLayout_3)
        self.line_edit_search = QtWidgets.QLineEdit(self.widget_files)
        self.line_edit_search.setObjectName("line_edit_search")
        self.verticalLayout_2.addWidget(self.line_edit_search)
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
        self.button_add_workspace = QtWidgets.QPushButton(self.widget_workspaces)
        icon1 = QtGui.QIcon()
        icon1.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/add.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_add_workspace.setIcon(icon1)
        self.button_add_workspace.setObjectName("button_add_workspace")
        self.horizontalLayout.addWidget(self.button_add_workspace)
        spacerItem2 = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout.addItem(spacerItem2)
        self.verticalLayout_3.addLayout(self.horizontalLayout)
        self.layout_workspaces = QtWidgets.QGridLayout()
        self.layout_workspaces.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.layout_workspaces.setObjectName("layout_workspaces")
        self.verticalLayout_3.addLayout(self.layout_workspaces)
        spacerItem3 = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout_3.addItem(spacerItem3)
        self.verticalLayout.addWidget(self.widget_workspaces)

        self.retranslateUi(FilesWidget)
        self.button_to_workspaces.clicked.connect(self.widget_files.hide)
        self.button_to_workspaces.clicked.connect(self.widget_workspaces.show)
        QtCore.QMetaObject.connectSlotsByName(FilesWidget)

    def retranslateUi(self, FilesWidget):
        _translate = QtCore.QCoreApplication.translate
        FilesWidget.setWindowTitle(_translate("FilesWidget", "Form"))
        self.label.setText(_translate("FilesWidget", "Mounted on"))
        self.button_to_workspaces.setText(_translate("FilesWidget", "Return to workspaces list"))
        self.button_create_folder.setText(_translate("FilesWidget", "Create new folder"))
        self.line_edit_search.setPlaceholderText(
            _translate("FilesWidget", "Search files or folders")
        )
        self.button_add_workspace.setText(_translate("FilesWidget", "Add a workspace"))


from parsec.core.gui import resources_rc
