# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/workspaces_widget.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_WorkspacesWidget(object):
    def setupUi(self, WorkspacesWidget):
        WorkspacesWidget.setObjectName("WorkspacesWidget")
        WorkspacesWidget.resize(679, 256)
        WorkspacesWidget.setStyleSheet("background-color: rgb(255, 255, 255);\n" "")
        self.verticalLayout = QtWidgets.QVBoxLayout(WorkspacesWidget)
        self.verticalLayout.setContentsMargins(0, 15, 0, 15)
        self.verticalLayout.setSpacing(10)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setSizeConstraint(QtWidgets.QLayout.SetMinimumSize)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.button_add_workspace = QtWidgets.QPushButton(WorkspacesWidget)
        self.button_add_workspace.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setFamily("DejaVu Sans")
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_add_workspace.setFont(font)
        self.button_add_workspace.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_add_workspace.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;\n"
            "font-weight: bold;"
        )
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/new_workspace.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_add_workspace.setIcon(icon)
        self.button_add_workspace.setIconSize(QtCore.QSize(24, 24))
        self.button_add_workspace.setObjectName("button_add_workspace")
        self.horizontalLayout.addWidget(self.button_add_workspace)
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout.addItem(spacerItem)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.layout_workspaces = QtWidgets.QGridLayout()
        self.layout_workspaces.setSizeConstraint(QtWidgets.QLayout.SetMaximumSize)
        self.layout_workspaces.setContentsMargins(20, 20, 20, 20)
        self.layout_workspaces.setSpacing(40)
        self.layout_workspaces.setObjectName("layout_workspaces")
        self.verticalLayout.addLayout(self.layout_workspaces)
        spacerItem1 = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem1)

        self.retranslateUi(WorkspacesWidget)
        QtCore.QMetaObject.connectSlotsByName(WorkspacesWidget)

    def retranslateUi(self, WorkspacesWidget):
        _translate = QtCore.QCoreApplication.translate
        WorkspacesWidget.setWindowTitle(_translate("WorkspacesWidget", "Form"))
        self.button_add_workspace.setText(_translate("WorkspacesWidget", "New workspace  "))


from parsec.core.gui import resources_rc
