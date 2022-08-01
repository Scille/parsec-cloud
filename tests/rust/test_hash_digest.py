# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest


@pytest.mark.rust
@pytest.mark.parametrize("bytes_cls", [bytes, bytearray])
def test_hash_digest(bytes_cls):
    from parsec.crypto import _PyHashDigest, _RsHashDigest, HashDigest

    assert HashDigest is _RsHashDigest

    rst = HashDigest.from_data(bytes_cls(b"abc"))
    py = _PyHashDigest.from_data(bytes_cls(b"abc"))
    assert rst.digest == py.digest
    assert rst.hexdigest() == py.hexdigest()
    assert repr(rst) == repr(py)

    rst2 = HashDigest.from_data(bytes_cls(b"abc"))
    rst3 = HashDigest.from_data(bytes_cls(b"def"))
    assert rst == rst2
    assert not rst != rst2
    assert rst != rst3
    assert not rst == rst3
