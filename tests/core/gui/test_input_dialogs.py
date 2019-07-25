# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore, QtWidgets

from parsec.core.gui import custom_dialogs
from parsec.core.gui.replace_dialog import ReplaceDialog


@pytest.mark.gui
def test_get_text_dialog_close(qtbot):

    w = custom_dialogs.TextInputDialog(title="Title", message="Message", parent=None)
    qtbot.addWidget(w)
    w.show()

    assert w.isVisible() is True
    assert w.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    assert w.line_edit_text.placeholderText() == ""
    assert w.line_edit_text.text() == ""
    qtbot.mouseClick(w.button_close, QtCore.Qt.LeftButton)
    assert w.result() == QtWidgets.QDialog.Rejected
    assert w.text == ""
    assert w.isVisible() is False


@pytest.mark.gui
def test_get_text_dialog_accept(qtbot):

    w = custom_dialogs.TextInputDialog(
        title="Title",
        message="Message",
        placeholder="Placeholder",
        default_text="Default",
        parent=None,
    )
    qtbot.addWidget(w)
    w.show()

    assert w.isVisible() is True
    assert w.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    assert w.line_edit_text.placeholderText() == "Placeholder"
    assert w.line_edit_text.text() == "Default"
    w.line_edit_text.setText("")
    qtbot.keyClicks(w.line_edit_text, "test")
    qtbot.mouseClick(w.button_ok, QtCore.Qt.LeftButton)
    assert w.result() == QtWidgets.QDialog.Accepted
    assert w.text == "test"
    assert w.isVisible() is False


@pytest.mark.gui
def test_ask_question_no(qtbot):
    w = custom_dialogs.QuestionDialog(title="Title", message="Message", parent=None)
    qtbot.addWidget(w)

    w.show()
    assert w.isVisible() is True
    assert w.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    qtbot.mouseClick(w.button_no, QtCore.Qt.LeftButton)
    assert w.result() == QtWidgets.QDialog.Rejected
    assert w.isVisible() is False


@pytest.mark.gui
def test_ask_question_yes(qtbot):
    w = custom_dialogs.QuestionDialog(title="Title", message="Message", parent=None)
    qtbot.addWidget(w)

    w.show()

    assert w.isVisible() is True
    assert w.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    qtbot.mouseClick(w.button_yes, QtCore.Qt.LeftButton)
    assert w.result() == QtWidgets.QDialog.Accepted
    assert w.isVisible() is False


@pytest.mark.gui
def test_replace_dialog_skip(qtbot):

    w = ReplaceDialog("file.txt", parent=None)
    qtbot.addWidget(w)

    w.show()

    assert w.isVisible() is True
    assert w.check_box_all.isChecked() is False
    qtbot.mouseClick(w.button_skip, QtCore.Qt.LeftButton)
    assert w.skip is True
    assert w.replace is False
    assert w.all_files is False
    assert w.isVisible() is False


@pytest.mark.gui
def test_replace_dialog_replace_all(qtbot):

    w = ReplaceDialog("file.txt", parent=None)
    qtbot.addWidget(w)

    w.show()

    assert w.isVisible() is True
    assert w.check_box_all.isChecked() is False
    qtbot.mouseClick(w.check_box_all, QtCore.Qt.LeftButton)
    qtbot.mouseClick(w.button_replace, QtCore.Qt.LeftButton)
    assert w.skip is False
    assert w.replace is True
    assert w.all_files is True
    assert w.isVisible() is False
