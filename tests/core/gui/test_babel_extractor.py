# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import io

from parsec.core.gui.babel_qt_extractor import extract_qt


def test_qt_babel_extractor():
    s = io.StringIO(
        """<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE TS><TS version="2.0">
<context>
    <name>ClassName</name>
    <message>
        <location filename="myclass.ui" line="14"/>
        <source>A string to translate.</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="myclass.ui" line="34"/>
        <source>Another string to translate.</source>
        <translation type="unfinished"></translation>
    </message>
</context>
</TS>
"""
    )
    results = [_ for _ in extract_qt(s, None, None, None)]
    assert results == [
        (14, None, "A string to translate.", []),
        (34, None, "Another string to translate.", []),
    ]
