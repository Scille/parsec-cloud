# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest


@pytest.mark.rust
@pytest.mark.parametrize("bytes_cls", [bytes, bytearray])
def test_hash_digest(bytes_cls):
    from parsec.crypto import _Py_HashDigest, _Rs_HashDigest, HashDigest

    assert HashDigest is _Rs_HashDigest

    rst = HashDigest.from_data(bytes_cls(b"abc"))
    py = _Py_HashDigest.from_data(bytes_cls(b"abc"))
    assert rst.digest == py.digest
    assert rst.hexdigest() == py.hexdigest()
    assert repr(rst) == repr(py)

    rst2 = HashDigest.from_data(bytes_cls(b"abc"))
    rst3 = HashDigest.from_data(bytes_cls(b"def"))
    assert rst == rst2
    assert not rst != rst2
    assert rst != rst3
    assert not rst == rst3
