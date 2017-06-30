import pytest
from unittest.mock import patch

from parsec.crypto import (
    load_private_key, load_public_key, load_sym_key, generate_sym_key,
    BasePrivateAsymKey, BasePublicAsymKey, InvalidSignature,
    RSAPublicKey, RSAPrivateKey, AESKey, InvalidTag
)


ALICE_PRIVATE_RSA = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICWgIBAAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO04KJSq1cH87KtmkqI3qewvXtW
qFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/mG/U9gURDi4aTTXT02RbHESBp
yMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49SLhqGNmNAnH2E3lxAgMBAAEC
gYBY2S0QFJG8AwCdfKKUK+t2q+UO6wscwdtqSk/grBg8MWXTb+8XjteRLy3gD9Eu
E1IpwPStjj7KYEnp2blAvOKY0E537d2a4NLrDbSi84q8kXqvv0UeGMC0ZB2r4C89
/6BTZii4mjIlg3iPtkbRdLfexjqmtELliPkHKJIIMH3fYQJBAKd/k1hhnoxEx4sq
GRKueAX7orR9iZHraoR9nlV69/0B23Na0Q9/mP2bLphhDS4bOyR8EXF3y6CjSVO4
LBDPOmUCQQCV5hr3RxGPuYi2n2VplocLK/6UuXWdrz+7GIqZdQhvhvYSKbqZ5tvK
Ue8TYK3Dn4K/B+a7wGTSx3soSY3RBqwdAkAv94jqtooBAXFjmRq1DuGwVO+zYIAV
GaXXa2H8eMqr2exOjKNyHMhjWB1v5dswaPv25tDX/caCqkBFiWiVJ8NBAkBnEnqo
Xe3tbh5btO7+08q4G+BKU9xUORURiaaELr1GMv8xLhBpkxy+2egS4v+Y7C3zPXOi
1oB9jz1YTnt9p6DhAkBy0qgscOzo4hAn062MAYWA6hZOTkvzRbRpnyTRctKwZPSC
+tnlGk8FAkuOm/oKabDOY1WZMkj5zWAXrW4oR3Q2
-----END RSA PRIVATE KEY-----
"""
ALICE_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO0
4KJSq1cH87KtmkqI3qewvXtWqFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/m
G/U9gURDi4aTTXT02RbHESBpyMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49
SLhqGNmNAnH2E3lxAgMBAAE=
-----END PUBLIC KEY-----
"""
BOB_PRIVATE_RSA = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICXQIBAAKBgQDCqVQVdVhJqW9rrbObvDZ4ET6FoIyVn6ldWhOJaycMeFYBN3t+
cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S3rAV6JhDKDfDFrkzUVZmcc/g
HhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOKG889JN85nABKR9WkdwIDAQAB
AoGBAJFU3Dr9FgJA5rfMwpiV51CzByu61trqjgbtNkLVZhzwRr23z5Jxmd+yLHik
J6ia6sYvdUuHFLKQegGt/2xOjXn8UBpa725gLojHn2umtJDL7amTlBwiJfNXuZrF
BSKK9+xZnNDWMq1IuCqPeintbve+MNSc62JYuGGtXSz9L5f5AkEA/xBkUksBfEUl
65oEPgxvMKHNjLq48otRmCaG+i3MuQqTYQ+c8Z/l26yQL4OV2b36a8/tTaLhwhAZ
Ibtv05NKfQJBAMNgMbOsUWpY8A1Cec79Oj6RVe79E5ciZ4mW3lx5tjJRyNxwlQag
u+T6SwBIa6xMfLBQeizzxqXqxAyW/riQ6QMCQQCadUu7Re6tWZaAGTGufYsr8R/v
s/dh8ZpEwDgG8otCFzRul6zb6Y+huttJ2q55QIGQnka/N/7srSD6+23Zux1lAkBx
P30PzL6UimD7DqFUnev5AH1zPjbwz/x8AHt71wEJQebQAGIhqWHAZGS9ET14bg2I
ld172QI4glCJi6yyhyzJAkBzfmHZEE8FyLCz4z6b+Z2ghMds2Xz7RwgVqCIXt9Ku
P7Bq0eXXgyaBo+jpr3h4K7QnPh+PaHSlGqSfczZ6GIpx
-----END RSA PRIVATE KEY-----
"""
BOB_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCqVQVdVhJqW9rrbObvDZ4ET6F
oIyVn6ldWhOJaycMeFYBN3t+cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S
3rAV6JhDKDfDFrkzUVZmcc/gHhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOK
G889JN85nABKR9WkdwIDAQAB
-----END PUBLIC KEY-----
"""
TOO_SMALL_PRIVATE_RSA = b"""
-----BEGIN PRIVATE KEY-----
MIIBVQIBADANBgkqhkiG9w0BAQEFAASCAT8wggE7AgEAAkEAsSle/x4jtr+kaxiv
9BYlL+gffH/VLC+Q/WTTB+1FIU1fdmgdZVGaIlAWJHqr9qEZBfwXYzlQv8pIMn+W
5pqvHQICAQECQDv5FjKAvWW1a3Twc1ia67eQUmDugu8VFTDsV2BRUS0jlxJ0yCL+
TEBpOwH95TFgvfRBYee97APHjhvLLlzmEyECIQDZdjMg/j9N7sw602DER0ciERPI
Ps9rU8RqRXaWPYtWOQIhANCO1h/z7iFjlpENbKDOCinfsXd9ulVsoNYWhKm58gAF
AiEAzMT3XdKFUlljq+/hl/Nt0GPA8lkHDGjG5ZAaAAYnj/ECIQCB125lkuHy61LH
4INhH6azeFaUGnn7aHwJxE6beL6BgQIhALbajJWsBf5LmeO190adM2jAVN94YqVD
aOrHGFFqrjJ3
-----END PRIVATE KEY-----
"""
TOO_SMALL_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MFswDQYJKoZIhvcNAQEBBQADSgAwRwJBALEpXv8eI7a/pGsYr/QWJS/oH3x/1Swv
kP1k0wftRSFNX3ZoHWVRmiJQFiR6q/ahGQX8F2M5UL/KSDJ/luaarx0CAgEB
-----END PUBLIC KEY-----
"""


@pytest.fixture
def mock_crypto_passthrough():
    count = 0

    def mocked_urandom(size):
        nonlocal count
        count += 1
        if size > 16:
            template = '<dummy-key-{:0>%s}>' % (size - 12)
        else:
            template = '{:0>%s}' % size
        return template.format(count).encode()

    with patch.object(RSAPublicKey, 'encrypt', new=lambda _, txt: txt), \
            patch.object(RSAPublicKey, 'verify', new=lambda _, sign, txt: None), \
            patch.object(RSAPrivateKey, 'decrypt', new=lambda _, txt: txt), \
            patch.object(RSAPrivateKey, 'sign', new=lambda _, txt: '<mock-signature>'), \
            patch.object(AESKey, 'encrypt', new=lambda _, txt: txt), \
            patch.object(AESKey, 'decrypt', new=lambda _, txt: txt), \
            patch('parsec.crypto.urandom', new=mocked_urandom):
        yield


def test_mocked_crypto(mock_crypto_passthrough, alice):
    assert alice.pub_key.encrypt(b'foo') == b'foo'
    assert alice.decrypt(b'foo') == b'foo'
    assert alice.sign(b'foo') == '<mock-signature>'
    alice.pub_key.verify(b'dummy-signature', b'foo')  # Should not raise error
    symkey = generate_sym_key()
    assert symkey.encrypt(b'foo') == b'foo'
    assert symkey.decrypt(b'foo') == b'foo'
    symkey2 = generate_sym_key()
    symkey3 = generate_sym_key()
    assert symkey.key == b'<dummy-key-00000000000000000001>'
    assert symkey2.key == b'<dummy-key-00000000000000000002>'
    assert symkey3.key == b'<dummy-key-00000000000000000003>'


@pytest.fixture
def alice():
    return load_private_key(ALICE_PRIVATE_RSA)


@pytest.fixture
def bob():
    return load_private_key(BOB_PRIVATE_RSA)


@pytest.fixture
def symkey():
    raw_key = b'0' * 32
    return load_sym_key(raw_key)


def _shrink_ids(x):
    return x if len(x) < 20 else x[:10] + b'...' + x[-10:]


class TestRSAAsymCrypto:
    def test_load_private_key(self):
        key = load_private_key(ALICE_PRIVATE_RSA)
        assert isinstance(key, BasePrivateAsymKey)

    def test_load_too_small_key(self):
        with pytest.raises(RuntimeError):
            key = load_public_key(TOO_SMALL_PUBLIC_RSA)
        with pytest.raises(RuntimeError):
            key = load_private_key(TOO_SMALL_PRIVATE_RSA)

    @pytest.mark.parametrize('badkey', [
        ALICE_PUBLIC_RSA,
        b'fooo'
    ], ids=_shrink_ids)
    def test_bad_load_private_key(self, badkey):
        with pytest.raises(ValueError):
            load_private_key(badkey)

    def test_load_public_key(self):
        key = load_public_key(ALICE_PUBLIC_RSA)
        assert isinstance(key, BasePublicAsymKey)

    def test_pub_key_in_private_key(self, alice):
        assert isinstance(alice.pub_key, BasePublicAsymKey)

    @pytest.mark.parametrize('badkey', [
        ALICE_PRIVATE_RSA,
        b'fooo'
    ], ids=_shrink_ids)
    def test_bad_load_public_key(self, badkey):
        with pytest.raises(ValueError):
            load_public_key(badkey)

    @pytest.mark.parametrize('msg', [
        b'fooo',
        b'l' + b'o' * 1000000 + b'ng'
    ], ids=_shrink_ids)
    def test_sign_and_verify(self, msg, alice, bob):
        signature = alice.sign(msg)
        assert isinstance(signature, bytes)
        alice.pub_key.verify(signature, msg)
        with pytest.raises(InvalidSignature):
            bad_signature = signature + b'.'
            alice.pub_key.verify(bad_signature, msg)
        with pytest.raises(InvalidSignature):
            bad_msg = msg + b'.'
            alice.pub_key.verify(signature, bad_msg)
        with pytest.raises(InvalidSignature):
            bob.pub_key.verify(signature, msg)

    @pytest.mark.parametrize('msg', [
        b'fooo',
        b'l' + b'o' * 1000000 + b'ng'
    ], ids=_shrink_ids)
    def test_crypt_and_decrypt(self, msg, alice, bob):
        crypted = alice.pub_key.encrypt(msg)
        assert isinstance(crypted, bytes)
        decrypted = alice.decrypt(crypted)
        assert isinstance(decrypted, bytes)
        assert decrypted == msg
        with pytest.raises(ValueError):
            bob.decrypt(crypted)


class TestAESSymCrypto:

    def test_export_key(self, symkey):
        assert isinstance(symkey.key, bytes)

    @pytest.mark.parametrize('msg', [
        b'fooo',
        b'l' + b'o' * 1000000 + b'ng'
    ], ids=_shrink_ids)
    def test_aes_encrypt_good(self, msg, symkey):
        crypted = symkey.encrypt(msg)
        assert isinstance(crypted, bytes)
        msg2 = symkey.decrypt(crypted)
        assert msg == msg2

    def test_aes_decrypt_bad_iv(self, symkey):
        crypted = symkey.encrypt(b'foo')
        bad_crypted = crypted[:15] + b'x' + crypted[16:]
        with pytest.raises(InvalidTag):
            symkey.decrypt(bad_crypted)

    def test_aes_decrypt_bad_tag(self, symkey):
        crypted = symkey.encrypt(b'foo')
        bad_crypted = crypted[:-16] + b'x' * 16
        with pytest.raises(InvalidTag):
            symkey.decrypt(bad_crypted)

    def test_aes_decrypt_bad_key(self, symkey):
        crypted = symkey.encrypt(b'foo')
        badsymkey = load_sym_key(symkey.key[:31] + b'x')
        with pytest.raises(InvalidTag):
            badsymkey.decrypt(crypted)
