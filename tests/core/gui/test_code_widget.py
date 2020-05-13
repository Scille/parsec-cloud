# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.gui.custom_widgets import CodeInputWidget

@pytest.mark.gui
def test_code_input_right_choice(qtbot):
    w = CodeInputWidget()
    qtbot.addWidget(w)
    w.set_choices(choices=["A", "B", "C", "D"], right_choice="C")
    right_btn = None
    for i in range(w.code_layout.count()):
        item = w.code_layout.itemAt(i)
        assert item
        b = item.widget()
        assert b
        assert b.text() in ["A", "B", "C", "D"]
        if b.text() == "C":
            right_btn = b
    assert right_btn
    with qtbot.wait_signal(w.good_code_clicked):
        qtbot.mouseClick(right_btn, QtCore.Qt.LeftButton)

@pytest.mark.gui
def test_code_input_wrong_choice(qtbot):
    w = CodeInputWidget()
    qtbot.addWidget(w)
    w.set_choices(choices=["A", "B", "C", "D"], right_choice="C")
    wrong_btn = None
    for i in range(w.code_layout.count()):
        item = w.code_layout.itemAt(i)
        assert item
        b = item.widget()
        assert b
        assert b.text() in ["A", "B", "C", "D"]
        if b.text() != "C":
            wrong_btn = b
    assert wrong_btn
    with qtbot.wait_signal(w.wrong_code_clicked):
        qtbot.mouseClick(wrong_btn, QtCore.Qt.LeftButton)


@pytest.mark.gui
def test_code_input_none(qtbot):
    w = CodeInputWidget()
    qtbot.addWidget(w)
    w.set_choices(choices=["A", "B", "C", "D"], right_choice="C")
    with qtbot.wait_signal(w.none_clicked):
        qtbot.mouseClick(w.button_none, QtCore.Qt.LeftButton)
