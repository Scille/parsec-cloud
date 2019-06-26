# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.gui.password_validation import get_password_strength, get_password_strength_text


@pytest.mark.gui
def test_password_validation():
    assert get_password_strength("passwor") == 0
    assert get_password_strength("password") == 1
    assert get_password_strength("password-1") == 2
    assert get_password_strength("password-123") == 3
    assert get_password_strength("password-123_test") == 4
    assert get_password_strength("password-123_test-abc") == 5


@pytest.mark.gui
def test_password_text():
    assert get_password_strength_text(0) == "Too short"
    assert get_password_strength_text(1) == "Very weak"
    assert get_password_strength_text(2) == "Weak"
    assert get_password_strength_text(3) == "Average"
    assert get_password_strength_text(4) == "Good"
    assert get_password_strength_text(5) == "Strong"
