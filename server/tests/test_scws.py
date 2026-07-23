# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from binascii import unhexlify

from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.padding import PKCS1v15
from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey
from cryptography.hazmat.primitives.serialization import load_pem_private_key

from parsec._parsec import RsaPrivateKey
from parsec.components.scws import _raw_rsa_sign


def convert_rsa_pkey_from_cryptography_to_rust(key: RSAPrivateKey) -> RsaPrivateKey:
    pkcs8_pem = key.private_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PrivateFormat.PKCS8,
        encryption_algorithm=serialization.NoEncryption(),
    )
    return RsaPrivateKey.load_pkcs8_pem(pkcs8_pem.decode())


def test_scws_sign():
    blob = unhexlify("fb6fdfe5fa570e544ee91335613c30a7eb54a24e970d7c7108bdee392b4abe5f")
    # Pkcs1 pem RSA 512 private key
    pkey_pem = (
        b"-----BEGIN RSA PRIVATE KEY-----"
        b"MIIBOgIBAAJBAMPMNNpbZZddeT/GTjU0PWuuN9VEGpxXJTAkmZY02o8238fQ2ynt"
        b"N40FVl08YksWBO/74XEjU30mAjuaz/FB2kkCAwEAAQJBALoMlsROSLCWD5q8EqCX"
        b"rS1e9IrgFfEtFZczkAWc33lo3FnFeFTXSMVCloNCBWU35od4zTOhdRPAWpQ1Mzxi"
        b"aCkCIQD9qjKjNvbDXjUcCNqdiJxPDlPGpa78yzyCCUA/+TNwVwIhAMWZoqZO3eWq"
        b"SCBTLelVQsg6CwJh9W7vlezvWxUni+ZfAiAopBAg3jmC66EOsMx12OFSOTVq6jiy"
        b"/8zd+KV2mnKHWQIgVpZiLZo1piQeAvwwDCUuZGr61Ap08C3QdsjUEssHhOUCIBee"
        b"72JZuJeABcv7lHhAWzsiCddVAkdnZKUo6ubaxw3u"
        b"-----END RSA PRIVATE KEY-----"
    )
    crypto_pkey = load_pem_private_key(
        pkey_pem,
        password=None,
    )
    assert isinstance(crypto_pkey, RSAPrivateKey)

    rust_pkey = convert_rsa_pkey_from_cryptography_to_rust(crypto_pkey)
    signature = _raw_rsa_sign(rust_pkey, blob)
    expected_signature = unhexlify(
        "222193e6b850b46b18abffb745cb856e284e15bbe9e28487f2b6f041274ccf847e18b4c749795cb0b71cb23f4972984e2a43f5bd8d6bef58db683eb406854d8b"
    )
    assert signature == expected_signature

    pubkey = crypto_pkey.public_key()
    recovered_data = pubkey.recover_data_from_signature(signature, PKCS1v15(), algorithm=None)
    assert recovered_data == blob
