# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore, QtWidgets

from parsec.core.gui import custom_dialogs


@pytest.mark.gui
def test_get_text_dialog_close(qtbot):
    w = custom_dialogs.TextInputWidget(message="Message")
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    qtbot.addWidget(d)
    d.show()

    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    assert w.line_edit_text.placeholderText() == ""
    assert w.line_edit_text.text() == ""
    qtbot.mouseClick(d.button_close, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Rejected
    assert w.text == ""
    assert w.isVisible() is False


@pytest.mark.gui
def test_get_text_dialog_accept(qtbot):

    w = custom_dialogs.TextInputWidget(
        message="Message", placeholder="Placeholder", default_text="Default"
    )
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    w.dialog = d
    qtbot.addWidget(d)
    d.show()

    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    assert w.line_edit_text.placeholderText() == "Placeholder"
    assert w.line_edit_text.text() == "Default"
    w.line_edit_text.setText("")
    qtbot.keyClicks(w.line_edit_text, "test")
    qtbot.mouseClick(w.button_ok, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Accepted
    assert w.text == "test"
    assert w.isVisible() is False


@pytest.mark.gui
def test_ask_question_no(qtbot):
    w = custom_dialogs.QuestionWidget(message="Message", button_texts=["OK", "CANCEL"])
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    w.dialog = d
    qtbot.addWidget(d)

    d.show()
    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    button_no = w.layout_buttons.itemAt(0).widget()
    qtbot.mouseClick(button_no, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Accepted
    assert w.status == "CANCEL"
    assert w.isVisible() is False


@pytest.mark.gui
def test_ask_question_yes(qtbot):
    w = custom_dialogs.QuestionWidget(message="Message", button_texts=["OK", "CANCEL"])
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    w.dialog = d
    qtbot.addWidget(d)

    d.show()
    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    button_yes = w.layout_buttons.itemAt(1).widget()
    qtbot.mouseClick(button_yes, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Accepted
    assert w.status == "OK"
    assert w.isVisible() is False


@pytest.mark.gui
def test_ask_question_close(qtbot):
    w = custom_dialogs.QuestionWidget(message="Message", button_texts=["OK", "CANCEL"])
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    w.dialog = d
    qtbot.addWidget(d)

    d.show()
    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    qtbot.mouseClick(d.button_close, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Rejected
    assert w.status is None
    assert w.isVisible() is False
