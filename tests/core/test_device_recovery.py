# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.crypto import generate_recovery_passphrase, derivate_secret_key_from_recovery_passphrase

import pytest


def test_recovery_passphrase():
    passphrase, key = generate_recovery_passphrase()

    key2 = derivate_secret_key_from_recovery_passphrase(passphrase)
    assert key2 == key

    # Add dummy stuff to the passphrase should not cause issues
    altered_passphrase = passphrase.lower().replace("-", "@  白")
    key3 = derivate_secret_key_from_recovery_passphrase(altered_passphrase)
    assert key3 == key


@pytest.mark.parametrize(
    "bad_passphrase",
    [
        # Empty
        "",
        # Only invalid characters (so endup empty)
        "-@//白",
        # Too short
        "D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO",
        # Too long
        "D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO-NU4Q-NU4Q",
    ],
)
def test_invalid_passphrase(bad_passphrase):
    with pytest.raises(ValueError):
        derivate_secret_key_from_recovery_passphrase(bad_passphrase)
