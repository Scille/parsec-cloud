# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/workspace_button.ui'
#
# Created by: PyQt5 UI code generator 5.11.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_WorkspaceButton(object):
    def setupUi(self, WorkspaceButton):
        WorkspaceButton.setObjectName("WorkspaceButton")
        WorkspaceButton.resize(210, 230)
        WorkspaceButton.setMinimumSize(QtCore.QSize(210, 210))
        WorkspaceButton.setMaximumSize(QtCore.QSize(210, 500))
        font = QtGui.QFont()
        font.setPointSize(12)
        WorkspaceButton.setFont(font)
        WorkspaceButton.setStyleSheet(
            "QMenu\n"
            "{\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "}\n"
            "\n"
            "QMenu::item\n"
            "{\n"
            "background-color: rgb(255, 255, 255);\n"
            "color: rgb(0, 0, 0);\n"
            "}\n"
            "\n"
            "QMenu::item:selected\n"
            "{\n"
            "background-color: rgb(45, 144, 209);\n"
            "color: rgb(255, 255, 255);\n"
            "}"
        )
        self.verticalLayout = QtWidgets.QVBoxLayout(WorkspaceButton)
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout.setSpacing(6)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setSpacing(0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label = QtWidgets.QLabel(WorkspaceButton)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label.sizePolicy().hasHeightForWidth())
        self.label.setSizePolicy(sizePolicy)
        self.label.setMinimumSize(QtCore.QSize(200, 178))
        self.label.setMaximumSize(QtCore.QSize(200, 178))
        self.label.setStyleSheet("border: 2px solid rgb(45, 145, 207);\n" "padding: 20px;")
        self.label.setText("")
        self.label.setPixmap(QtGui.QPixmap(":/icons/images/icons/desk.png"))
        self.label.setScaledContents(True)
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setTextInteractionFlags(QtCore.Qt.NoTextInteraction)
        self.label.setObjectName("label")
        self.horizontalLayout.addWidget(self.label)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.label_workspace = QtWidgets.QLabel(WorkspaceButton)
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.label_workspace.setFont(font)
        self.label_workspace.setLayoutDirection(QtCore.Qt.LeftToRight)
        self.label_workspace.setStyleSheet("color: rgb(12, 65, 157);")
        self.label_workspace.setText("")
        self.label_workspace.setAlignment(QtCore.Qt.AlignCenter)
        self.label_workspace.setWordWrap(True)
        self.label_workspace.setObjectName("label_workspace")
        self.verticalLayout.addWidget(self.label_workspace)
        spacerItem = QtWidgets.QSpacerItem(
            20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding
        )
        self.verticalLayout.addItem(spacerItem)

        self.retranslateUi(WorkspaceButton)
        QtCore.QMetaObject.connectSlotsByName(WorkspaceButton)

    def retranslateUi(self, WorkspaceButton):
        _translate = QtCore.QCoreApplication.translate
        WorkspaceButton.setWindowTitle(_translate("WorkspaceButton", "Form"))


from parsec.core.gui import resources_rc
