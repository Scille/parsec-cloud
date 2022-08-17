# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec.core.gui.password_validation import get_password_strength, get_password_strength_text
from parsec.core.gui.lang import switch_language
from parsec.core.gui.password_authentication_widget import PasswordAuthenticationWidget


@pytest.mark.gui
def test_password_validation():
    assert get_password_strength("passwor") == 0
    assert get_password_strength("password") == 1
    assert get_password_strength("password-1") == 2
    assert get_password_strength("password-123") == 3
    assert get_password_strength("password-123_test") == 4
    assert get_password_strength("password-123_test-abc") == 5


@pytest.mark.gui
def test_password_text(core_config):
    switch_language(core_config, "en")
    assert get_password_strength_text(0) == "TOO SHORT"
    assert get_password_strength_text(1) == "VERY WEAK"
    assert get_password_strength_text(2) == "WEAK"
    assert get_password_strength_text(3) == "AVERAGE"
    assert get_password_strength_text(4) == "GOOD"
    assert get_password_strength_text(5) == "EXCELLENT"


@pytest.mark.gui
def test_password_validation_with_user_inputs():
    assert get_password_strength("William J Blazkowicz") == 5
    assert (
        get_password_strength("William J Blazkowicz", excluded_strings=["william", "blazkowicz"])
        == 4
    )


@pytest.mark.gui
def test_password_choice_widget(qtbot, core_config):
    switch_language(core_config, "en")

    p = PasswordAuthenticationWidget(parent=None)
    qtbot.add_widget(p)

    p.line_edit_password.setText("William J Blazkowicz")
    p.line_edit_password_check.setText("William J Blazkowicz")

    assert p.pwd_str_widget.label.text() == "Password strength: EXCELLENT"
    assert p.password == "William J Blazkowicz"
    assert p.is_auth_valid()


@pytest.mark.gui
def test_password_choice_widget_mismatch(qtbot, core_config):
    switch_language(core_config, "en")

    p = PasswordAuthenticationWidget(parent=None)
    qtbot.add_widget(p)

    p.line_edit_password.setText("William J Blazkowicz")
    p.line_edit_password_check.setText("William J Blazkowiz")

    assert p.pwd_str_widget.label.text() == "Password strength: EXCELLENT"
    assert p.password == "William J Blazkowicz"
    assert not p.is_auth_valid()
    assert not p.label_mismatch.isHidden()
    assert p.label_mismatch.text() == "Does not match the password."


@pytest.mark.gui
def test_password_choice_widget_with_excluded_strings(qtbot, core_config):
    switch_language(core_config, "en")

    p = PasswordAuthenticationWidget(parent=None)
    p.set_excluded_strings(["william.j.blazkowicz@wolfenstein.de"])
    assert p.pwd_str_widget._excluded_strings == ["william", "blazkowicz", "wolfenstein"]
    qtbot.add_widget(p)

    p.line_edit_password.setText("William J Blazkowicz")

    # With user inputs, excellent becomes good
    assert p.pwd_str_widget.label.text() == "Password strength: GOOD"
