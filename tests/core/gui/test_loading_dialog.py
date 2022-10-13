# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.core.gui.loading_widget import LoadingWidget


@pytest.mark.gui
def test_loading_dialog(qtbot):

    w = LoadingWidget(total_size=10000)

    qtbot.add_widget(w)
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


@pytest.mark.gui
def test_loading_dialog_overflow(qtbot):

    # Test for a file length > to 2Go (overflow of signed int32)
    w = LoadingWidget(total_size=3000000000)

    qtbot.add_widget(w)
    assert w.progress_bar.text() == "0%"
    assert w.progress_bar.value() == 0

    w.set_current_file("Small_file.txt")
    assert w.label.text() == '"Small_file.txt"'

    w.set_current_file("A_very_long_file_name_to_check_if_it_is_shortened.txt")
    assert w.label.text() == '"A_very_long_file_name_to_c...ed.txt"'

    w.set_progress(500000000)
    assert w.progress_bar.text() == "16%"
    w.set_progress(1500000000)
    assert w.progress_bar.text() == "50%"
    w.set_progress(3000000000)
    assert w.progress_bar.text() == "100%"
