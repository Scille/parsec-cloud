# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from PyQt5 import QtGui

from parsec.core.gui import validators
from parsec.core.gui.input_widgets import ValidatedLineEdit
from parsec.core.gui.lang import switch_language


@pytest.mark.gui
def test_device_name_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.DeviceNameValidator())
    qtbot.addWidget(le)
    le.show()

    qtbot.keyClicks(le, "abcd")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable

    qtbot.keyClicks(le, "~")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid


@pytest.mark.gui
def test_email_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.EmailValidator())
    qtbot.addWidget(le)
    le.show()

    qtbot.keyClicks(le, "maurice")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    qtbot.keyClicks(le, ".moss@reynholm.com")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable

    qtbot.keyClicks(le, "#")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid


@pytest.mark.gui
def test_organization_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.OrganizationIDValidator())
    qtbot.addWidget(le)
    le.show()

    qtbot.keyClicks(le, "Reynholm")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable

    qtbot.keyClicks(le, " Industries")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid
