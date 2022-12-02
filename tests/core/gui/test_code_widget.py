# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from PyQt5 import QtCore

from parsec.api.data import SASCode
from parsec.core.gui.custom_widgets import CodeInputWidget


@pytest.mark.gui
def test_code_input_right_choice(qtbot):
    w = CodeInputWidget()
    qtbot.add_widget(w)
    # O and I are excluded from the SASCode alphabet
    w.set_choices(
        choices=[SASCode("ABCD"), SASCode("EFGH"), SASCode("JKLM"), SASCode("NPQR")],
        right_choice=SASCode("JKLM"),
    )
    right_btn = None
    for i in range(w.code_layout.count()):
        item = w.code_layout.itemAt(i)
        assert item
        b = item.widget()
        assert b
        assert b.text() in ["ABCD", "EFGH", "JKLM", "NPQR"]
        if b.text() == "JKLM":
            right_btn = b
    assert right_btn
    with qtbot.wait_signal(w.good_code_clicked):
        qtbot.mouseClick(right_btn, QtCore.Qt.LeftButton)


@pytest.mark.gui
def test_code_input_wrong_choice(qtbot):
    w = CodeInputWidget()
    qtbot.add_widget(w)
    w.set_choices(
        choices=[SASCode("ABCD"), SASCode("EFGH"), SASCode("JKLM"), SASCode("NPQR")],
        right_choice=SASCode("JKLM"),
    )
    wrong_btn = None
    for i in range(w.code_layout.count()):
        item = w.code_layout.itemAt(i)
        assert item
        b = item.widget()
        assert b
        assert b.text() in ["ABCD", "EFGH", "JKLM", "NPQR"]
        if not wrong_btn and b.text() != "JKLM":
            wrong_btn = b
    assert wrong_btn
    with qtbot.wait_signal(w.wrong_code_clicked):
        qtbot.mouseClick(wrong_btn, QtCore.Qt.LeftButton)


@pytest.mark.gui
def test_code_input_none(qtbot):
    w = CodeInputWidget()
    qtbot.add_widget(w)
    w.set_choices(
        choices=[SASCode("ABCD"), SASCode("EFGH"), SASCode("JKLM"), SASCode("NPQR")],
        right_choice=SASCode("JKLM"),
    )
    with qtbot.wait_signal(w.none_clicked):
        qtbot.mouseClick(w.button_none, QtCore.Qt.LeftButton)
