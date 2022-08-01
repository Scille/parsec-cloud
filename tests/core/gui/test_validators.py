# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from PyQt5 import QtGui
from parsec import UNSTABLE_OXIDATION

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


@pytest.mark.gui
def test_backend_addr_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.BackendAddrValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, "http://host:1337")
    qtbot.wait_until(lambda: le.text() == "http://host:1337")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid

    le.setText("")
    qtbot.wait_until(lambda: le.text() == "")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    qtbot.keyClicks(le, "parsec://host:1337")
    qtbot.wait_until(lambda: le.text() == "parsec://host:1337")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable

    qtbot.keyClicks(le, "/org")
    qtbot.wait_until(lambda: le.text() == "parsec://host:1337/org")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid


@pytest.mark.gui
def test_backend_organization_addr_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.BackendOrganizationAddrValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, "http://host:1337")
    qtbot.wait_until(lambda: le.text() == "http://host:1337")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    le.setText("")
    qtbot.wait_until(lambda: le.text() == "")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    qtbot.keyClicks(
        le, "parsec://host:1337/org?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss"
    )
    qtbot.wait_until(
        lambda: le.text()
        == "parsec://host:1337/org?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss"
    )
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable


@pytest.mark.gui
def test_backend_organization_bootstrap_addr_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.BackendOrganizationBootstrapAddrValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, "http://host:1337")
    qtbot.wait_until(lambda: le.text() == "http://host:1337")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    le.setText("")
    qtbot.wait_until(lambda: le.text() == "")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    qtbot.keyClicks(le, "parsec://host:1337/org?action=bootstrap_organization&token=1234ABCD")
    qtbot.wait_until(
        lambda: le.text() == "parsec://host:1337/org?action=bootstrap_organization&token=1234ABCD"
    )
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable


@pytest.mark.gui
@pytest.mark.parametrize(
    "action_addr",
    [
        "parsec://host:1337/org?action=file_link&workspace_id=80fd5ff399a3416282991bf3cc56d3f9&path=F5RGEsss",
        "parsec://host:1337/org?action=bootstrap_organization&token=1234ABC",
        "parsec://host:1337/org?action=claim_user&token=3a50b191122b480ebb113b10216ef343",
        "parsec://host:1337/org?action=pki_enrollment",
    ],
)
def test_backend_action_addr_validator(qtbot, core_config, action_addr):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.BackendActionAddrValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, "http://host:1337")
    qtbot.wait_until(lambda: le.text() == "http://host:1337")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    le.setText("")
    qtbot.wait_until(lambda: le.text() == "")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Intermediate

    qtbot.keyClicks(le, action_addr)
    qtbot.wait_until(lambda: le.text() == action_addr)
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable


@pytest.mark.gui
def test_workspace_name_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.UserIDValidator())
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


@pytest.mark.gui
def test_user_id_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.UserIDValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, "Reynholm")
    qtbot.wait_until(lambda: le.text() == "Reynholm")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable

    qtbot.keyClicks(le, "a" * 255)
    qtbot.wait_until(lambda: le.text() == "Reynholm" + "a" * 255)
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid


@pytest.mark.gui
def test_not_empty_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.UserIDValidator())
    qtbot.add_widget(le)
    le.show()

    assert not le.is_input_valid()
    assert le.property("validity") is None

    qtbot.keyClicks(le, "Reynholm")
    qtbot.wait_until(lambda: le.text() == "Reynholm")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable


@pytest.mark.gui
def test_file_name_validator(qtbot, core_config):
    switch_language(core_config, "en")

    le = ValidatedLineEdit()
    le.set_validator(validators.FileNameValidator())
    qtbot.add_widget(le)
    le.show()

    # Only "." is forbidden
    qtbot.keyClicks(le, ".")
    qtbot.wait_until(lambda: le.text() == ".")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid

    # But a "." can be present in the string
    qtbot.keyClicks(le, "additional_text")
    qtbot.wait_until(lambda: le.text() == ".additional_text")
    assert le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Acceptable

    # "/" anywhere in the string is forbidden
    le.setText("")
    qtbot.keyClicks(le, "a/b")
    qtbot.wait_until(lambda: le.text() == "a/b")
    assert not le.is_input_valid()
    assert le.property("validity") == QtGui.QValidator.Invalid


@pytest.mark.gui
@pytest.mark.xfail(UNSTABLE_OXIDATION, reason="TODO: remove when #2667 is merged")
@pytest.mark.parametrize(
    "name_and_expected",
    [("    ", False), ("[Test]", False), ("Maurice Moss", True), ("    Maurice    Moss   ", True)],
)
def test_user_name_validator(qtbot, core_config, name_and_expected):
    switch_language(core_config, "en")
    user_name, expected = name_and_expected

    le = ValidatedLineEdit()
    le.set_validator(validators.UserNameValidator())
    qtbot.add_widget(le)
    le.show()

    qtbot.keyClicks(le, user_name)
    qtbot.wait_until(lambda: le.text() == user_name)
    if expected:
        assert le.is_input_valid()
        assert le.property("validity") == QtGui.QValidator.Acceptable
    else:
        assert not le.is_input_valid()
        assert (
            le.property("validity") == QtGui.QValidator.Invalid
            or le.property("validity") == QtGui.QValidator.Intermediate
        )

    if user_name.startswith(" "):
        assert le.clean_text() == validators.trim_string(user_name)
