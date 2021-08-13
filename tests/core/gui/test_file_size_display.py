# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.lang import switch_language


@pytest.mark.parametrize(
    "expected,bytesize",
    [
        ("0 B", 0),
        ("1 B", 1),
        ("25 B", 25),
        ("99 B", 99),
        ("1023 B", 1023),
        ("1.0 KB", 1024),
        ("1.0 KB", 1024 + 1),
        ("1.1 KB", 1024 + 100),
        ("1.9 KB", 1024 * 2 - 52),
        ("2.0 KB", 1024 * 2 - 51),
        ("2.0 KB", 1024 * 2),
        ("12.3 KB", 1024 * 12 + 300),
        ("123 KB", 1024 * 123 + 400),
        ("1023 KB", 1023 * 1024),
        ("2.5 MB", 1024 ** 2 * 2.5),
        ("1024 TB", 1024 ** 5),
        (f"{1024 ** 2} TB", 1024 ** 6 + 5 * 1024 ** 3),
    ],
)
def test_size_display(expected, bytesize, core_config):
    # Setup translation for correct display of units
    switch_language(core_config, "en")
    assert get_filesize(bytesize) == expected
