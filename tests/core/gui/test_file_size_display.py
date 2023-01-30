# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

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
        ("0.99 KB", 1023),
        ("1.00 KB", 1024),
        ("1.00 KB", 1024 + 1),
        ("1.09 KB", 1024 + 100),
        ("1.94 KB", 1024 * 2 - 52),
        ("1.95 KB", 1024 * 2 - 51),
        ("2.00 KB", 1024 * 2),
        ("12.3 KB", 1024 * 12 + 300),
        ("123 KB", 1024 * 123 + 400),
        ("0.99 MB", 1023 * 1024),
        ("2.50 MB", 1024**2 * 2.5),
        ("1024 TB", 1024**5),
        (f"{1024 ** 2} TB", 1024**6 + 5 * 1024**3),
        # Test transitions systematically
        ("0 B", 0),
        ("1 B", 1),
        ("9 B", 9),
        ("10 B", 10),
        ("99 B", 99),
        ("100 B", 100),
        ("999 B", 999),
        ("0.97 KB", 1000),
        ("0.97 KB", 1003),
        ("0.98 KB", 1004),
        ("0.98 KB", 1013),
        ("0.99 KB", 1014),
        ("0.99 KB", 1023),
        ("1.00 KB", 1024),
        ("9.99 KB", 10239),
        ("10.0 KB", 10240),
        ("99.9 KB", 102348),
        ("100 KB", 102349),
        ("999 KB", 1023487),
        ("0.97 MB", 1023488),
        ("0.97 MB", 1027604),
        ("0.98 MB", 1027605),
        ("0.98 MB", 1038090),
        ("0.99 MB", 1038091),
        ("0.99 MB", 1048575),
        ("1.00 MB", 1048576),
        ("9.99 MB", 10485759),
        ("10.0 MB", 10485760),
        ("99.9 MB", 104805171),
        ("100 MB", 104805172),
        ("999 MB", 1048051711),
        ("0.97 GB", 1048051712),
        ("0.97 GB", 1052266987),
        ("0.98 GB", 1052266988),
        ("0.98 GB", 1063004405),
        ("0.99 GB", 1063004406),
        ("0.99 GB", 1073741823),
        ("1.00 GB", 1073741824),
        ("9.99 GB", 10737418239),
        ("10.0 GB", 10737418240),
    ],
)
def test_size_display(expected, bytesize, core_config):
    # Setup translation for correct display of units
    switch_language(core_config, "en")
    assert get_filesize(bytesize) == expected
