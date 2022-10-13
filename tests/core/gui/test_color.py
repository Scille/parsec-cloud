# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.core.gui.color import StringToColor


@pytest.mark.gui
def test_create():
    c = StringToColor(0.5, 0.5, 0.5)
    assert c.hue == 0.5
    assert c.lightness == 0.5
    assert c.saturation == 0.5
    assert c.rgb255() == (63, 191, 191)
    assert c.hex() == "#3fbfbf"


@pytest.mark.gui
def test_create_from_string():
    c = StringToColor.from_string("String")
    assert 0.0 <= c.hue <= 1.0
    assert 0.6 <= c.lightness <= 0.85
    assert 0.4 <= c.saturation <= 0.8
