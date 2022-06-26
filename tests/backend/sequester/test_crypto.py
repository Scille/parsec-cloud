# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import oscrypto.asymmetric

from parsec.sequester_crypto import SequesterVerifyKeyDer, SequesterEncryptionKeyDer


def test_only_rsa_is_supported():
    # Unsupported format for service encryption key (only RSA is currently supported)
    unsupported_public_key, _ = oscrypto.asymmetric.generate_pair("dsa", bit_size=1024)
    with pytest.raises(ValueError):
        SequesterEncryptionKeyDer(unsupported_public_key),
    with pytest.raises(ValueError):
        SequesterVerifyKeyDer(unsupported_public_key),
