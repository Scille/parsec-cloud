# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import oscrypto.asymmetric

from parsec.sequester_crypto import (
    SequesterVerifyKeyDer,
    SequesterEncryptionKeyDer,
    sequester_service_decrypt,
    sequester_authority_sign,
)


def test_only_rsa_is_supported():
    # Unsupported format for service encryption key (only RSA is currently supported)
    unsupported_public_key, _ = oscrypto.asymmetric.generate_pair("dsa", bit_size=1024)
    with pytest.raises(ValueError):
        SequesterEncryptionKeyDer(unsupported_public_key),
    with pytest.raises(ValueError):
        SequesterVerifyKeyDer(unsupported_public_key),


def test_encryption_output_has_key_size(monkeypatch):
    key_bit_size = 1024
    key_byte_size = key_bit_size // 8
    patched_called = 0

    def patched_rsa_oaep_encrypt(key, data):
        nonlocal patched_called
        patched_called += 1
        assert key.byte_size == key_byte_size
        assert len(data) == 32  # Data here is a Salsa20 key
        # Output is 64bytes so much smaller than the 128bytes key
        return b"\x01" * 32 + data

    def patched_rsa_oaep_decrypt(key, encrypted):
        nonlocal patched_called
        patched_called += 1
        assert key.byte_size == key_byte_size
        assert len(encrypted) == key_byte_size
        assert encrypted[:64] == b"\x00" * 64  # Null bytes added to have the correct size
        assert encrypted[64:96] == b"\x01" * 32
        return encrypted[96:]

    monkeypatch.setattr("oscrypto.asymmetric.rsa_oaep_encrypt", patched_rsa_oaep_encrypt)
    monkeypatch.setattr("oscrypto.asymmetric.rsa_oaep_decrypt", patched_rsa_oaep_decrypt)

    public_key, private_key = oscrypto.asymmetric.generate_pair("rsa", bit_size=key_bit_size)
    encryption_key = SequesterEncryptionKeyDer(public_key)

    encrypted = encryption_key.encrypt(b"foo")
    decrypted = sequester_service_decrypt(private_key, encrypted)
    assert decrypted == b"foo"

    assert patched_called == 2  # Make sure the mock has been used


def test_signature_output_has_key_size(monkeypatch):
    key_bit_size = 1024
    key_byte_size = key_bit_size // 8
    patched_called = 0

    def patched_rsa_pss_sign(key, data, hash_algorithm):
        nonlocal patched_called
        patched_called += 1
        assert hash_algorithm == "sha256"
        assert key.byte_size == key_byte_size
        # Output is 64bytes so much smaller than the 128bytes key
        return b"\x01" * 64

    def patched_rsa_pss_verify(key, signed, data, hash_algorithm):
        nonlocal patched_called
        patched_called += 1
        assert hash_algorithm == "sha256"
        assert key.byte_size == key_byte_size
        assert len(signed) == key_byte_size
        assert signed[:64] == b"\x00" * 64  # Null bytes added to have the correct size
        assert signed[64:] == b"\x01" * 64

    monkeypatch.setattr("oscrypto.asymmetric.rsa_pss_sign", patched_rsa_pss_sign)
    monkeypatch.setattr("oscrypto.asymmetric.rsa_pss_verify", patched_rsa_pss_verify)

    public_key, private_key = oscrypto.asymmetric.generate_pair("rsa", bit_size=key_bit_size)
    verify_key = SequesterVerifyKeyDer(public_key)

    signed = sequester_authority_sign(private_key, b"foo")
    verified = verify_key.verify(signed)
    assert verified == b"foo"

    assert patched_called == 2  # Make sure the mock has been used
