# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.core.gui.lang import translate as _


def size(bytes, system):
    """
    Format the specified number of bytes with the corresponding system.

    More specifically:
    - `0 <= bytes < 10`:          1 significant figures:     `X B`
    - `10 <= bytes < 100`:        2 significant figures:    `XY B`
    - `100 <= bytes < 1000`:      3 significant figures:   `XYZ B`
    - `1000 <= bytes < 1024`:     2 significant figures: `0.9X KB`
    - `1 <= kilobytes < 10`:      2 significant figures:  `X.Y KB`
    - `10 <= kilobytes < 100`:    3 significant figures: `XY.Z KB`
    - `100 <= kilobytes < 1000`:  3 significant figures:  `XYZ KB`
    - `1000 <= kilobytes < 1024`: 2 significant figures: `0.9X MB`
    - And so on for MB, GB and TB
    """
    # Bytes should be zero or greater
    assert bytes >= 0
    # Iterate over factors, expecting them to be in increasing order
    for factor, suffix in system:
        # Stop when the right factor is reached
        if bytes / factor < 999.5:
            break
    # Convert to the right unit
    amount = bytes / factor
    # Truncate to two decimals in order to avoid misleading rounding to 1.00
    amount = int(amount * 100) / 100
    # Factor is one, the amount is in integer
    if factor == 1:
        formatted_amount = f"{bytes:d}"
    # Amount is less than one, display either 0.97, 0.98 or 0.99
    elif amount < 1.0:
        formatted_amount = f"{amount:.2f}"
    # Amount is displayed with a one or two digits on the left side, add an extra significant digit
    elif amount < 99.95:
        formatted_amount = f"{amount:.1f}"
    # Amount is displayed with 3 digits on the left side, no need for an extra digit
    else:
        formatted_amount = f"{amount:.0f}"
    return f"{formatted_amount} {suffix}"


def get_filesize(bytesize):
    # Our system of unit in increasing order
    # We're using the 1K=1024 conversion in order to match the Windows file explorer.
    # TODO: adapt the conversion depending on the system:
    # - Windows: 1K=1024
    # - Linux: 1k=1000
    # - MacOS: 1k=1000
    SYSTEM = [
        (1024**0, _("TEXT_FILE_SIZE_B")),
        (1024**1, _("TEXT_FILE_SIZE_KB")),
        (1024**2, _("TEXT_FILE_SIZE_MB")),
        (1024**3, _("TEXT_FILE_SIZE_GB")),
        (1024**4, _("TEXT_FILE_SIZE_TB")),
    ]
    return size(bytesize, system=SYSTEM)
