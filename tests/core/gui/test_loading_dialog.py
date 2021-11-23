# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from parsec.core.gui.loading_widget import LoadingWidget


@pytest.mark.gui
def test_loading_dialog(qtbot):

    w = LoadingWidget(total_size=10000, message="Doesnt matter")

    qtbot.add_widget(w)
    assert w.progress_bar.text() == " 0%"
    assert w.progress_bar.value() == 0

    w.set_current_file("Small_file.txt")
    assert w.label.text() == "Doesnt matter"

    w.set_current_file("A_very_long_file_name_to_check_if_it_is_shortened.txt")
    w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt %0"

    w.set_progress(2000)
    assert w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt 20% (1.9 KB / 9.8 KB)"
    w.set_progress(7000)
    assert w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt 70% (6.8 KB / 9.8 KB)"
    w.set_progress(11000)
    assert w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt 70% (10.7 KB / 9.8 KB)"
    w.set_progress(10000)
    assert w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt 100% (9.8 KB / 9.8 KB)"


@pytest.mark.gui
def test_loading_dialog_overflow(qtbot):

    # Test for a file length > to 2Go (overflow of signed int32)
    w = LoadingWidget(total_size=3000000000, message="Doesnt matter")

    qtbot.add_widget(w)
    assert w.progress_bar.text() == " 0%"
    assert w.progress_bar.value() == 0

    w.set_current_file("Small_file.txt")
    assert w.label.text() == "Doesnt matter"

    w.set_current_file("A_very_long_file_name_to_check_if_it_is_shortened.txt")
    w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt %0"

    w.set_progress(500000000)
    assert w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt 16% (477 MB / 2.8 GB)"
    w.set_progress(1500000000)
    assert w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt 50% (1.4 GB / 2.8 GB)"
    w.set_progress(3000000000)
    assert w.progress_bar.text() == "A_very_long_file_name_to_c...ed.txt 100% (2.8 GB / 2.8 GB)"
