# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec.backend.cli.utils import _split_with_escaping


@pytest.mark.parametrize(
    "txt,expected_parts",
    [
        ("foo", ["foo"]),
        ("foo:bar", ["foo", "bar"]),
        ("foo\\:bar", ["foo:bar"]),
        ("::", ["", "", ""]),
        ("\\:\\:", ["::"]),
        (":foo:", ["", "foo", ""]),
        ("\\::\\:", [":", ":"]),
        ("\\\\:\\\\", ["\\", "\\"]),
        ("\\n\\", ["\\n\\"]),
    ],
)
def test_split_with_escaping(txt, expected_parts):
    parts = _split_with_escaping(txt)
    assert parts == expected_parts
