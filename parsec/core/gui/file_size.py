# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.core.gui.lang import translate as _


def size(bytes, system):
    """
    Format the specified number of bytes with the corresponding system.

    More specifically:
    - `0 <= bytes < 10`:         1 significant figures:    `X B`
    - `10 <= bytes < 100`:       2 significant figures:   `XY B`
    - `100 <= bytes < 100`:      3 significant figures:  `XYZ B`
    - `1 <= kilobytes < 10`:     2 significant figures: `X.Y KB`
    - `10 <= kilobytes < 100`:   3 significant figures: XY.Z KB`
    - `100 <= kilobytes < 1000`: 3 significant figures: `XYZ KB`
    - And so on for MB, GB and TB
    """
    # Iterate over factors, expecting them to be in decreasing order
    for factor, suffix in system:
        # Stop when the right factor is reached
        if bytes >= factor:
            break
    # Convert to the right unit
    amount = bytes / factor
    # Pick the right formatter
    formatter = ".1f" if amount < 99.95 and factor > 1 else ".0f"
    # Format the amount and add the suffix
    formatted_amount = format(bytes / factor, formatter)
    return f"{formatted_amount} {suffix}"


def get_filesize(bytesize):
    # Our system of unit in decreasing order
    # We're using the 1K=1024 conversion in order to match the Windows file explorer.
    # TODO: adapt the conversion depending on the system:
    # - Windows: 1K=1024
    # - Linux: 1k=1000
    # - MacOS: 1k=1000
    SYSTEM = [
        (1024 ** 4, _("TEXT_FILE_SIZE_TB")),
        (1024 ** 3, _("TEXT_FILE_SIZE_GB")),
        (1024 ** 2, _("TEXT_FILE_SIZE_MB")),
        (1024 ** 1, _("TEXT_FILE_SIZE_KB")),
        (1024 ** 0, _("TEXT_FILE_SIZE_B")),
    ]
    return size(bytesize, system=SYSTEM)
