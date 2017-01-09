import pytest
from collections import namedtuple

from Crypto.PublicKey import RSA

from parsec.crypto import LocalCryptoClient, CryptoError
from parsec.crypto import CryptoEngineService
from parsec.crypto.crypto_pb2 import Request, Response


class TestBaseCryptoClient:

    def setup_method(self):
        self.service = CryptoEngineService()
        self.client = LocalCryptoClient(service=self.service)

    def test_gen_key(self):
        ret = self.client.genkey(passphrase='thisisaT3st!', key_size=1024)
        assert ret.key
        assert ret.status_code == Response.OK

    def test_load_key(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        ret = self.client.load_key(test_key)
        assert ret.status_code == Response.OK

    def test_encrypt(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        ret = self.client.load_key(test_key)
        assert ret.status_code == Response.OK

        ret = self.client.encrypt(content=b'EncryptMePlz')
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.content

    def test_decrypt(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        ret = self.client.load_key(test_key)
        assert ret.status_code == Response.OK

        ret = self.client.encrypt(content=b'EncryptMePlz')
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.content
        ret = self.client.decrypt(content=ret.content, key=ret.key,
                                  signature=ret.signature, key_signature=ret.key_signature)
        assert ret.status_code == Response.OK
        assert ret.content == b'EncryptMePlz'

    def test_error(self):
        with pytest.raises(CryptoError):
            self.client.encrypt(content=b'EncryptMePlz')


class BaseTestCryptoEngineService:

    def test_unknown_cmd(self):
        # Key too short
        msg = namedtuple('Request', ['type', 'argument'], verbose=True)
        msg.type = 'DUNNO'
        msg.argument = 'noideawhatimdoinghere'
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.BAD_REQUEST
        assert ret.error_msg == "Unknown msg `DUNNO`"

    def test_gen_key_too_short(self):
        # Key too short
        msg = Request(type=Request.GEN_KEY, passphrase='', key_size=256)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR
        assert ret.error_msg == "Incorrect RSA key size"

    def test_gen_key_not_256(self):
        # Key not multiple of 256
        msg = Request(type=Request.GEN_KEY, passphrase='', key_size=1025)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR
        assert ret.error_msg == "Incorrect RSA key size"

    def test_gen_key_good_key_no_passphrase(self):
        # Smallest key available
        msg = Request(type=Request.GEN_KEY, passphrase='', key_size=1024)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key

        # Try to import it back in RSA module
        key = RSA.importKey(ret.key)
        assert key

    def test_gen_key_good_key_passphrase(self):
        # Generate key with passphrase
        msg = Request(type=Request.GEN_KEY, passphrase='thisisaT3st!', key_size=1024)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        key = None
        with pytest.raises(ValueError):
            key = RSA.importKey(ret.key)
        assert not key
        key = RSA.importKey(ret.key, 'thisisaT3st!')
        assert key

    def test_load_key_bad_format(self):
        test_key = b'this is bullshit'
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR

    def test_load_key_no_passphrase(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

    def test_load_key_with_passphrase(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: DES-EDE3-CBC,911A9A7629A15B2F

8maFu4GAIx1an1ji8hSgxrc8wJpDqtxNYl6teLn6uw1Twco3EvYW+bwLIGQY1Ix2
UucUma450B4aJWAMAY0jo0PgWGK1phDggb9/dnM+2ZEJIQh7z+iDF+cdaN0du2ec
vpZjffWu6gfWCD5zoiwOhu6F4LWubkNziel/memoglSD44F0XfM/TwjxIRhOuvhx
wbGqQOtvc0t40XiCJWKnWxmn0m/rwVwJyMW948PDjazf+n3u5TBYmH1eua8J487v
otkHMi5JO1WmMyURSYWtRba7IDypd73CiMbkFdxdAvSyYUJ0xnXTVXl5C726QYr1
GTWgEDvKpT21e3y8K0q9YoVmfdsUuQ3CoUaB3fTbd45qXqZL80PzynyI9CMtgZHN
G85otukSz8Tu5Fhf81NKAKwVd91k5OfJFLjeIJeVrc2F21cM1LOKnwNmj8syoNvY
In9U9rp67qqFYb4KZEcfIjV1NFlQoe/wTwrokJPtO2LFkrzt8dZTIsQbwo8vMezC
PACUBY3Z+kDXWMzJJLkKwuOJ72n/zyb61veXrPLb0lNYX4AhJn3zUK+V3qS/DhJB
isLhPAfEx+EzNTnUBlO+EkWhSo2yarEayubKQ0OMOtXjbDxFDcr8ZvTdQHplAvFq
xa+P455NYGAkrIFjuFLUEyfKe0T7gZSXV0g9bVD28pxjShZhEKLYmxkIbjTRcCUF
QnRUjG73xK0otOM+Cml4jAkEF1+riOecz8aTz/+5mqI0jGzUsgsppDY0EQasNRIi
5DHh5SsVTXgmLe/Fp3VS4xs+uoAL4GsPVuF3KN4dWMDaB+fCcjX3ag==
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR
        msg = Request(type=Request.LOAD_KEY, key=test_key, passphrase='thisisaT3st!')
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

    def test_encrypt_all_good(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        expected_sig = (b'k\x9c\xcd\xb3my\xbaR\xc67\xdd\xe0\xc5\x9d\x1a\xf2d\x14\xaa\xc3"'
                        b'\xd7\xb08\xc9\xc2\x15t]\xc1\x06b\n\x18\xd8\xae\x0b\x82\x95y'
                        b'\xbc\xd9\xaa\x08\xee\xc0\x8d\'\x9b*F\x87\xdamL\xfb1W"\xca9p'
                        b'\x93U0Y\xd7y\x1d\x15\xeeO\x8bs\x12iO\x83\xb7z^6w\xb8i\x12'
                        b'\xbc\xcd\x881_7\x17\x96\x17\xd4\xac!+\x83L\xd2\xbfY'
                        b'U\x9dte\x9b\xbav\x89\xcf\xeb\x02\x0b v\xcc\xa9\x95U\x18|\x1aJ\x9dQ')

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.key_signature
        assert ret.signature == expected_sig
        assert data != ret.content

    def test_encrypt_no_key(self):
        msg = Request(type=Request.ENCRYPT, content=b'Not Working')
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR
        assert not ret.key
        assert not ret.key_signature
        assert not ret.content

    def test_decrypt_all_good(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        expected_sig = (b'k\x9c\xcd\xb3my\xbaR\xc67\xdd\xe0\xc5\x9d\x1a\xf2d\x14\xaa\xc3"'
                        b'\xd7\xb08\xc9\xc2\x15t]\xc1\x06b\n\x18\xd8\xae\x0b\x82\x95y'
                        b'\xbc\xd9\xaa\x08\xee\xc0\x8d\'\x9b*F\x87\xdamL\xfb1W"\xca9p'
                        b'\x93U0Y\xd7y\x1d\x15\xeeO\x8bs\x12iO\x83\xb7z^6w\xb8i\x12'
                        b'\xbc\xcd\x881_7\x17\x96\x17\xd4\xac!+\x83L\xd2\xbfY'
                        b'U\x9dte\x9b\xbav\x89\xcf\xeb\x02\x0b v\xcc\xa9\x95U\x18|\x1aJ\x9dQ')

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature == expected_sig
        assert data != ret.content

        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=expected_sig, key=ret.key, key_signature=ret.key_signature)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert data == ret.content

    def test_decrypt_bad_sig(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        expected_sig = b'oops_this_does_not_look_like_a_signature_isnt_it'

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature != expected_sig
        assert data != ret.content

        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=expected_sig, key=ret.key, key_signature=ret.key_signature)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.VERIFY_FAILED
        assert not ret.content

    def test_decrypt_no_key(self):
        data = b'hellooooooooow'
        msg = Request(type=Request.DECRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR
        assert not ret.key
        assert not ret.signature
        assert not ret.content

    def test_decrypt_bad_aes_key(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        expected_sig = (b'k\x9c\xcd\xb3my\xbaR\xc67\xdd\xe0\xc5\x9d\x1a\xf2d\x14\xaa\xc3"'
                        b'\xd7\xb08\xc9\xc2\x15t]\xc1\x06b\n\x18\xd8\xae\x0b\x82\x95y'
                        b'\xbc\xd9\xaa\x08\xee\xc0\x8d\'\x9b*F\x87\xdamL\xfb1W"\xca9p'
                        b'\x93U0Y\xd7y\x1d\x15\xeeO\x8bs\x12iO\x83\xb7z^6w\xb8i\x12'
                        b'\xbc\xcd\x881_7\x17\x96\x17\xd4\xac!+\x83L\xd2\xbfY'
                        b'U\x9dte\x9b\xbav\x89\xcf\xeb\x02\x0b v\xcc\xa9\x95U\x18|\x1aJ\x9dQ')

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        bad_key = ret.key
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature == expected_sig
        assert data != ret.content
        assert bad_key != ret.key

        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=expected_sig, key=bad_key, key_signature=ret.key_signature)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_DECRYPT_FAILED

    def test_decrypt_no_aes_key(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        expected_sig = (b'k\x9c\xcd\xb3my\xbaR\xc67\xdd\xe0\xc5\x9d\x1a\xf2d\x14\xaa\xc3"'
                        b'\xd7\xb08\xc9\xc2\x15t]\xc1\x06b\n\x18\xd8\xae\x0b\x82\x95y'
                        b'\xbc\xd9\xaa\x08\xee\xc0\x8d\'\x9b*F\x87\xdamL\xfb1W"\xca9p'
                        b'\x93U0Y\xd7y\x1d\x15\xeeO\x8bs\x12iO\x83\xb7z^6w\xb8i\x12'
                        b'\xbc\xcd\x881_7\x17\x96\x17\xd4\xac!+\x83L\xd2\xbfY'
                        b'U\x9dte\x9b\xbav\x89\xcf\xeb\x02\x0b v\xcc\xa9\x95U\x18|\x1aJ\x9dQ')

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature == expected_sig
        assert data != ret.content

        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=expected_sig, key=None)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_DECRYPT_FAILED

    def test_decrypt_invalid_aes_key(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert data != ret.content

        # alter key
        ret.key = b"This Will Probably not work..."
        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=ret.signature, key=ret.key, key_signature=b'Nope')
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_DECRYPT_FAILED

    def test_decrypt_bad_content(self):
        test_key = b"""-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDRHfGy9r/LaOBHuaH+CRv4JMhfJyWkyLwA8HB9WrGAa3B2q4oO
cBXSzC2KZ3lwJLqhaEcMPvalCxwSAh8YseQIkD73RwSLfbDJWcCaS0CbIsMimMO3
44vpMRUsnltcu+WWLdiMw6oTG9rYHkg/1V6WTgXmilI+bFYmSFoqdGrqGQIDAQAB
AoGAJaRjPpjGG4JsZNzYeRcAruFIJECyuP/dP7oINbhenUQ5wVLNjh3E/+X7CJ/p
rzMdWTKhH2YyFbFzQxaYrGRRLJng6axLbA+CjHEsqMkyKNaC0Z6RVt57/b7uB0t2
PgA0CqChkeQ8DELhJbnU65qPkSb+7FjEhJUYsO5F60hYxEUCQQDVdk2Zh0CwjNuo
IgvG5ANjY6Isb9uXSncii7qRkwPZMFLktv+dbNxb7KLyTBNLdHSy/1FBE5j8mHVJ
JWvEHTpnAkEA+sn5fMUXWeBKDo9jkzS6dCUld9+D2cn1PNQtStWNf9yWcuZtlFky
WJsahi4CqwwkXvqeaDFFa6I30oy/8QjnfwJAOey4cgj5zO7sTFuwxm/pW3cV8ukH
ta5HVeCE6Cv0x2MNm3LtOlLoGSnFrepm8frQECKocfhXc3QLn6W/8J/d0QJBAJvA
4J+q0EvTTmsohpEgCESl5VVDjeGu2g4DQHXfl1e3qgCGN7wQgYIiIiD/ZkzQ563N
PKA9KX4la0HqhDKwcwUCQHecDjFt4dvvQgt0TzHNZI9eE/I4xFB12MJ9KxN1+AvF
lCoZCXHy1VegtTRKsUuu/trbmz15FW75c/T1ceK7c6o=
-----END RSA PRIVATE KEY-----"""
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        expected_sig = (b'k\x9c\xcd\xb3my\xbaR\xc67\xdd\xe0\xc5\x9d\x1a\xf2d\x14\xaa\xc3"'
                        b'\xd7\xb08\xc9\xc2\x15t]\xc1\x06b\n\x18\xd8\xae\x0b\x82\x95y'
                        b'\xbc\xd9\xaa\x08\xee\xc0\x8d\'\x9b*F\x87\xdamL\xfb1W"\xca9p'
                        b'\x93U0Y\xd7y\x1d\x15\xeeO\x8bs\x12iO\x83\xb7z^6w\xb8i\x12'
                        b'\xbc\xcd\x881_7\x17\x96\x17\xd4\xac!+\x83L\xd2\xbfY'
                        b'U\x9dte\x9b\xbav\x89\xcf\xeb\x02\x0b v\xcc\xa9\x95U\x18|\x1aJ\x9dQ')

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature == expected_sig
        assert data != ret.content

        msg = Request(type=Request.DECRYPT, content=data,
                      signature=expected_sig, key=ret.key, key_signature=ret.key_signature)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.AES_DECRYPT_FAILED
        assert ret.error_msg == 'Cannot Decrypt file content'

    def test_service_bad_raw_msg(self):
        rep_buff = self.service.dispatch_raw_msg(b'dummy stuff')
        response = Response()
        response.ParseFromString(rep_buff)
        assert response.status_code == Response.BAD_REQUEST
        assert response.error_msg == 'Invalid request format'

    def test_service_good_raw_msg(self):
        # Smallest key available
        msg = Request(type=Request.GEN_KEY, passphrase='', key_size=1024)
        msg = msg.SerializeToString()
        rep_buff = self.service.dispatch_raw_msg(msg)
        response = Response()
        response.ParseFromString(rep_buff)
        assert response.status_code == Response.OK


class TestCryptoService(BaseTestCryptoEngineService):

    def setup_method(self):
        self.service = CryptoEngineService()

# TODO Test on a mock version ? Do we need a mock version of crypto ?
# Maybe with a know and fixed RSA/AES key ? We'll need to mock AES and RSA classes
