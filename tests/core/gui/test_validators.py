# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from PyQt5 import QtGui

from parsec.core.gui import validators
from parsec.core.gui.input_widgets import ValidatedLineEdit
from parsec.core.gui.lang import switch_language


@pytest.mark.gui
def test_device_label_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.DeviceLabelValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, "abcd")
    qtbot.wait_until(lambda: le.text() == "abcd")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable


@pytest.mark.gui
def test_email_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.EmailValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, "maurice")
    qtbot.wait_until(lambda: le.text() == "maurice")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    qtbot.keyClicks(le, ".moss@reynholm.com")
    qtbot.wait_until(lambda: le.text() == "maurice.moss@reynholm.com")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable

    qtbot.keyClicks(le, "#")
    qtbot.wait_until(lambda: le.text() == "maurice.moss@reynholm.com#")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid


@pytest.mark.gui
def test_organization_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.OrganizationIDValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, "Reynholm")
    qtbot.wait_until(lambda: le.text() == "Reynholm")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable

    qtbot.keyClicks(le, " Industries")
    qtbot.wait_until(lambda: le.text() == "Reynholm Industries")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid
