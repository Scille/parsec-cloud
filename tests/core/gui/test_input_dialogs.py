# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from PyQt5 import QtCore, QtWidgets

from parsec.core.gui import custom_dialogs


@pytest.mark.gui
def test_get_text_dialog_close(qtbot):
    w = custom_dialogs.TextInputWidget(message="Message")
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    qtbot.add_widget(d)
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
        message="Message", placeholder="Placeholder", default_text="Default", selection=(1, 3)
    )
    qtbot.wait(10)
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    w.dialog = d
    w.line_edit_text.setFocus()
    qtbot.add_widget(d)
    d.show()

    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    assert w.line_edit_text.placeholderText() == "Placeholder"
    assert w.line_edit_text.text() == "Default"
    assert w.line_edit_text.selectedText() == "efa"
    w.line_edit_text.setText("")
    qtbot.keyClicks(w.line_edit_text, "test")
    qtbot.mouseClick(w.button_ok, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Accepted
    qtbot.wait_until(lambda: w.text == "test")
    assert w.isVisible() is False


@pytest.mark.gui
def test_ask_question_no(qtbot):
    w = custom_dialogs.QuestionWidget(message="Message", button_texts=["YES", "NO"])
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    w.dialog = d
    qtbot.add_widget(d)

    d.show()
    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    button_no = w.layout_buttons.itemAt(1).widget()
    assert button_no.text() == "NO"
    qtbot.mouseClick(button_no, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Accepted
    assert w.status == "NO"
    assert w.isVisible() is False


@pytest.mark.gui
def test_ask_question_yes(qtbot):
    w = custom_dialogs.QuestionWidget(message="Message", button_texts=["YES", "NO"])
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    w.dialog = d
    qtbot.add_widget(d)

    d.show()
    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    button_yes = w.layout_buttons.itemAt(2).widget()
    assert button_yes.text() == "YES"
    qtbot.mouseClick(button_yes, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Accepted
    assert w.status == "YES"
    assert w.isVisible() is False


@pytest.mark.gui
def test_ask_question_close(qtbot):
    w = custom_dialogs.QuestionWidget(message="Message", button_texts=["YES", "NO"])
    d = custom_dialogs.GreyedDialog(w, title="Title", parent=None)
    w.dialog = d
    qtbot.add_widget(d)

    d.show()
    assert d.isVisible() is True
    assert w.isVisible() is True
    assert d.label_title.text() == "Title"
    assert w.label_message.text() == "Message"
    qtbot.mouseClick(d.button_close, QtCore.Qt.LeftButton)
    assert d.result() == QtWidgets.QDialog.Rejected
    assert w.status is None
    assert w.isVisible() is False
