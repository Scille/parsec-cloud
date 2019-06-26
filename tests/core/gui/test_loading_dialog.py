# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.core.gui.loading_dialog import LoadingDialog


@pytest.mark.gui
def test_loading_dialog(qtbot):

    w = LoadingDialog(total_size=10000, parent=None)

    qtbot.addWidget(w)
    assert w.progress_bar.text() == "0%"
    assert w.progress_bar.value() == 0

    w.set_current_file("Small_file.txt")
    assert w.label.text() == '"Small_file.txt"'

    w.set_current_file("A_very_long_file_name_to_check_if_it_is_shortened.txt")
    assert w.label.text() == '"A_very_long_file_name_to_c...ed.txt"'

    w.set_progress(2000)
    assert w.progress_bar.text() == "20%"
    w.set_progress(7000)
    assert w.progress_bar.text() == "70%"
    w.set_progress(11000)
    assert w.progress_bar.text() == "70%"
    w.set_progress(10000)
    assert w.progress_bar.text() == "100%"

    qtbot.mouseClick(w.button_cancel, QtCore.Qt.LeftButton)
