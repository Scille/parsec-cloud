# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.backend.cli.run import _split_with_escaping


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
