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
        FilesWidget.resize(590, 493)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(FilesWidget.sizePolicy().hasHeightForWidth())
        FilesWidget.setSizePolicy(sizePolicy)
        FilesWidget.setStyleSheet(
            "QWidget#FilesWidget\n"
            "{\n"
            "background-color: rgb(255, 255, 255);\n"
            "}\n"
            "\n"
            "QListWidget\n"
            "{\n"
            "border: 1px solid rgb(28, 78, 163);\n"
            "}\n"
            "\n"
            "QListView::item\n"
            "{\n"
            "background-color: rgb(255, 255, 255);\n"
            "color: rgb(0, 0, 0);\n"
            "}\n"
            "\n"
            "QListView::item:selected\n"
            "{\n"
            "background-color: rgb(45, 144, 209);\n"
            "color: rgb(255, 255, 255);\n"
            "}"
        )
        self.verticalLayout = QtWidgets.QVBoxLayout(FilesWidget)
        self.verticalLayout.setContentsMargins(0, 15, 0, 15)
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName("verticalLayout")
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
        self.verticalLayout_2.setContentsMargins(0, 0, 0, 0)
        self.verticalLayout_2.setSpacing(10)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setSpacing(24)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label = QtWidgets.QLabel(self.widget_files)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label.sizePolicy().hasHeightForWidth())
        self.label.setSizePolicy(sizePolicy)
        self.label.setMinimumSize(QtCore.QSize(32, 32))
        self.label.setMaximumSize(QtCore.QSize(32, 32))
        self.label.setText("")
        self.label.setPixmap(QtGui.QPixmap(":/icons/images/icons/workspace.png"))
        self.label.setScaledContents(True)
        self.label.setAlignment(QtCore.Qt.AlignCenter)
        self.label.setObjectName("label")
        self.horizontalLayout.addWidget(self.label)
        self.label_current_directory = QtWidgets.QLabel(self.widget_files)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_current_directory.sizePolicy().hasHeightForWidth())
        self.label_current_directory.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setPointSize(14)
        font.setBold(True)
        font.setWeight(75)
        self.label_current_directory.setFont(font)
        self.label_current_directory.setStyleSheet("font-weight: bold;")
        self.label_current_directory.setText("")
        self.label_current_directory.setObjectName("label_current_directory")
        self.horizontalLayout.addWidget(self.label_current_directory)
        self.verticalLayout_2.addLayout(self.horizontalLayout)
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.button_back = QtWidgets.QPushButton(self.widget_files)
        self.button_back.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_back.setFont(font)
        self.button_back.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_back.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;\n"
            "font-weight: bold;"
        )
        icon = QtGui.QIcon()
        icon.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/go-back.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_back.setIcon(icon)
        self.button_back.setIconSize(QtCore.QSize(24, 24))
        self.button_back.setAutoDefault(False)
        self.button_back.setDefault(False)
        self.button_back.setFlat(True)
        self.button_back.setObjectName("button_back")
        self.horizontalLayout_2.addWidget(self.button_back)
        self.button_create_folder = QtWidgets.QPushButton(self.widget_files)
        self.button_create_folder.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_create_folder.setFont(font)
        self.button_create_folder.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_create_folder.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;\n"
            "font-weight: bold;"
        )
        icon1 = QtGui.QIcon()
        icon1.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/new_document.png"),
            QtGui.QIcon.Normal,
            QtGui.QIcon.Off,
        )
        self.button_create_folder.setIcon(icon1)
        self.button_create_folder.setIconSize(QtCore.QSize(24, 24))
        self.button_create_folder.setAutoDefault(False)
        self.button_create_folder.setDefault(False)
        self.button_create_folder.setFlat(True)
        self.button_create_folder.setObjectName("button_create_folder")
        self.horizontalLayout_2.addWidget(self.button_create_folder)
        self.button_import = QtWidgets.QPushButton(self.widget_files)
        self.button_import.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.button_import.setFont(font)
        self.button_import.setLayoutDirection(QtCore.Qt.RightToLeft)
        self.button_import.setStyleSheet(
            "background-color: rgb(45, 144, 209);\n"
            "border: 1px solid rgb(11, 56, 166);\n"
            "color: rgb(255, 255, 255);\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;\n"
            "font-weight: bold;"
        )
        icon2 = QtGui.QIcon()
        icon2.addPixmap(
            QtGui.QPixmap(":/icons/images/icons/import.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off
        )
        self.button_import.setIcon(icon2)
        self.button_import.setIconSize(QtCore.QSize(24, 24))
        self.button_import.setAutoDefault(False)
        self.button_import.setDefault(False)
        self.button_import.setFlat(True)
        self.button_import.setObjectName("button_import")
        self.horizontalLayout_2.addWidget(self.button_import)
        spacerItem = QtWidgets.QSpacerItem(
            40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum
        )
        self.horizontalLayout_2.addItem(spacerItem)
        self.verticalLayout_2.addLayout(self.horizontalLayout_2)
        self.line_edit_search = QtWidgets.QLineEdit(self.widget_files)
        self.line_edit_search.setMinimumSize(QtCore.QSize(0, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        self.line_edit_search.setFont(font)
        self.line_edit_search.setStyleSheet(
            "background-image: url(:/icons/images/icons/search.png);\n"
            "background-repeat: no-repeat;\n"
            "background-position: right;\n"
            "border: 1px solid rgb(28, 78, 163);\n"
            "padding-left: 10px;\n"
            "color: rgb(28, 78, 163);"
        )
        self.line_edit_search.setObjectName("line_edit_search")
        self.verticalLayout_2.addWidget(self.line_edit_search)
        self.verticalLayout_3 = QtWidgets.QVBoxLayout()
        self.verticalLayout_3.setSpacing(0)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.horizontalLayout_4 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_4.setSpacing(0)
        self.horizontalLayout_4.setObjectName("horizontalLayout_4")
        self.label_2 = QtWidgets.QLabel(self.widget_files)
        self.label_2.setMinimumSize(QtCore.QSize(38, 32))
        self.label_2.setMaximumSize(QtCore.QSize(38, 32))
        self.label_2.setStyleSheet("background-color: rgb(12, 65, 157);")
        self.label_2.setText("")
        self.label_2.setObjectName("label_2")
        self.horizontalLayout_4.addWidget(self.label_2)
        self.pushButton = QtWidgets.QPushButton(self.widget_files)
        sizePolicy = QtWidgets.QSizePolicy(
            QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Fixed
        )
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.pushButton.sizePolicy().hasHeightForWidth())
        self.pushButton.setSizePolicy(sizePolicy)
        self.pushButton.setMinimumSize(QtCore.QSize(0, 32))
        self.pushButton.setMaximumSize(QtCore.QSize(16777215, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.pushButton.setFont(font)
        self.pushButton.setStyleSheet(
            "background-color: rgb(12, 65, 157);\n" "color: rgb(255, 255, 255);\n" "border: 0;"
        )
        self.pushButton.setDefault(False)
        self.pushButton.setFlat(False)
        self.pushButton.setObjectName("pushButton")
        self.horizontalLayout_4.addWidget(self.pushButton)
        self.pushButton_2 = QtWidgets.QPushButton(self.widget_files)
        self.pushButton_2.setMinimumSize(QtCore.QSize(186, 32))
        self.pushButton_2.setMaximumSize(QtCore.QSize(176, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.pushButton_2.setFont(font)
        self.pushButton_2.setStyleSheet(
            "background-color: rgb(12, 65, 157);\n"
            "color: rgb(255, 255, 255);\n"
            "border: 0;\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;"
        )
        self.pushButton_2.setObjectName("pushButton_2")
        self.horizontalLayout_4.addWidget(self.pushButton_2)
        self.pushButton_3 = QtWidgets.QPushButton(self.widget_files)
        self.pushButton_3.setMinimumSize(QtCore.QSize(186, 32))
        self.pushButton_3.setMaximumSize(QtCore.QSize(176, 32))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.pushButton_3.setFont(font)
        self.pushButton_3.setStyleSheet(
            "background-color: rgb(12, 65, 157);\n"
            "color: rgb(255, 255, 255);\n"
            "border: 0;\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;"
        )
        self.pushButton_3.setObjectName("pushButton_3")
        self.horizontalLayout_4.addWidget(self.pushButton_3)
        self.pushButton_4 = QtWidgets.QPushButton(self.widget_files)
        self.pushButton_4.setMinimumSize(QtCore.QSize(86, 32))
        self.pushButton_4.setMaximumSize(QtCore.QSize(86, 16777215))
        font = QtGui.QFont()
        font.setPointSize(12)
        font.setBold(True)
        font.setWeight(75)
        self.pushButton_4.setFont(font)
        self.pushButton_4.setStyleSheet(
            "background-color: rgb(12, 65, 157);\n"
            "color: rgb(255, 255, 255);\n"
            "border: 0;\n"
            "padding-left: 10px;\n"
            "padding-right: 10px;"
        )
        self.pushButton_4.setObjectName("pushButton_4")
        self.horizontalLayout_4.addWidget(self.pushButton_4)
        self.label_3 = QtWidgets.QLabel(self.widget_files)
        self.label_3.setMinimumSize(QtCore.QSize(38, 32))
        self.label_3.setMaximumSize(QtCore.QSize(38, 32))
        self.label_3.setStyleSheet("background-color: rgb(12, 65, 157);")
        self.label_3.setText("")
        self.label_3.setObjectName("label_3")
        self.horizontalLayout_4.addWidget(self.label_3)
        self.verticalLayout_3.addLayout(self.horizontalLayout_4)
        self.list_files = QtWidgets.QListWidget(self.widget_files)
        self.list_files.setContextMenuPolicy(QtCore.Qt.CustomContextMenu)
        self.list_files.setAutoFillBackground(True)
        self.list_files.setStyleSheet(
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
        self.list_files.setFrameShadow(QtWidgets.QFrame.Plain)
        self.list_files.setLineWidth(0)
        self.list_files.setHorizontalScrollBarPolicy(QtCore.Qt.ScrollBarAlwaysOff)
        self.list_files.setEditTriggers(QtWidgets.QAbstractItemView.NoEditTriggers)
        self.list_files.setSelectionRectVisible(False)
        self.list_files.setObjectName("list_files")
        self.verticalLayout_3.addWidget(self.list_files)
        self.verticalLayout_2.addLayout(self.verticalLayout_3)
        self.verticalLayout.addWidget(self.widget_files)

        self.retranslateUi(FilesWidget)
        QtCore.QMetaObject.connectSlotsByName(FilesWidget)

    def retranslateUi(self, FilesWidget):
        _translate = QtCore.QCoreApplication.translate
        FilesWidget.setWindowTitle(_translate("FilesWidget", "Form"))
        self.button_back.setText(_translate("FilesWidget", "Back"))
        self.button_create_folder.setText(_translate("FilesWidget", "New folder"))
        self.button_import.setText(_translate("FilesWidget", "Import"))
        self.line_edit_search.setPlaceholderText(
            _translate("FilesWidget", "Search files or folders")
        )
        self.pushButton.setText(_translate("FilesWidget", "Name"))
        self.pushButton_2.setText(_translate("FilesWidget", "Created on"))
        self.pushButton_3.setText(_translate("FilesWidget", "Updated on"))
        self.pushButton_4.setText(_translate("FilesWidget", "Size"))


from parsec.core.gui import resources_rc
