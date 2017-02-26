import pytest
from collections import namedtuple

from .common import b64
from parsec.crypto import CryptoService, MockSymCipher, MockAsymCipher, AESCipher, RSACipher


class BaseTestCryptoEngineService:

    # Helpers

    async def load_key(self):
        return await self.service.dispatch_msg({'cmd': 'load_key', 'passphrase': '', 'key': self.test_key})

    async def encrypt(self, content=b'foo'):
        return await self.service.dispatch_msg({'cmd': 'encrypt', 'content': b64(content)})

    # Tests

    @pytest.mark.asyncio
    async def test_gen_key_too_short(self):
        ret = await self.service.dispatch_msg({'cmd': 'gen_key', 'passphrase': '', 'key_size': self.key_size_small})
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid key size.'}

    @pytest.mark.asyncio
    async def test_gen_key_good_key_no_passphrase(self):
        # Smallest key available
        ret = await self.service.dispatch_msg({'cmd': 'gen_key', 'passphrase': '', 'key_size': self.key_size_request})
        assert ret['status'] == 'ok'
        assert ret['key']
        # Try to load it back
        ret = await self.service.dispatch_msg({'cmd': 'load_key', 'key': ret['key']})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    async def test_gen_key_good_key_passphrase(self):
        # Generate key with passphrase
        pw = 'thisisaT3st'
        ret = await self.service.dispatch_msg({'cmd': 'gen_key', 'passphrase': pw, 'key_size': self.key_size_request})
        assert ret['status'] == 'ok'
        assert ret['key']
        key = ret['key']
        # Try to load it back
        ret = await self.service.dispatch_msg({'cmd': 'load_key', 'key': key})
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid key or bad passphrase.'}
        ret = await self.service.dispatch_msg({'cmd': 'load_key', 'key': key, 'passphrase': 'dummy'})
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid key or bad passphrase.'}
        ret = await self.service.dispatch_msg({'cmd': 'load_key', 'key': key, 'passphrase': pw})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    async def test_load_key_too_small(self):
        ret = await self.service.dispatch_msg({'cmd': 'load_key', 'key': self.small_test_key})
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid key or bad passphrase.'}

    @pytest.mark.asyncio
    async def test_load_key_bad_format(self):
        ret = await self.service.dispatch_msg({'cmd': 'load_key', 'key': 'not a key'})
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid key or bad passphrase.'}

    @pytest.mark.asyncio
    async def test_load_key_no_passphrase(self):
        ret = await self.service.dispatch_msg({'cmd': 'load_key', 'key': self.test_key})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    async def test_load_key_with_passphrase(self):
        ret = await self.service.dispatch_msg({
            'cmd': 'load_key',
            'passphrase': 'Not working.',
            'key': self.test_key_encrypted
        })
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid key or bad passphrase.'}
        ret = await self.service.dispatch_msg({
            'cmd': 'load_key',
            'passphrase': self.default_passphrase,
            'key': self.test_key_encrypted
        })
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    async def test_encrypt_all_good(self):
        await self.load_key()
        data = b'hellooooooooow'
        ret = await self.service.dispatch_msg({'cmd': 'encrypt', 'content': b64(data)})
        assert ret['status'] == 'ok'
        assert ret['key']
        assert ret['key_signature']
        assert ret['signature']
        assert ret['content']

    @pytest.mark.xfail(reason='State machine not implemented')
    @pytest.mark.asyncio
    async def test_encrypt_no_key(self):
        ret = await self.service.dispatch_msg({'cmd': 'encrypt', 'content': b64(b'Not Working')})
        assert ret == {'status': 'asymetric_key_error'}

    @pytest.mark.asyncio
    async def test_decrypt_all_good(self):
        await self.load_key()
        data = b'hellooooooooow'
        encrypted = await self.encrypt(data)
        encrypted.pop('status')
        ret = await self.service.dispatch_msg({'cmd': 'decrypt', **encrypted})
        assert ret == {'status': 'ok', 'content': b64(data)}

    @pytest.mark.asyncio
    async def test_decrypt_bad_sig(self):
        await self.load_key()
        encrypted = await self.encrypt()
        encrypted.pop('status')
        encrypted['signature'] += '42'  # Tempering the signature...
        ret = await self.service.dispatch_msg({'cmd': 'decrypt', **encrypted})
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid signature, content may be tampered.'}

    @pytest.mark.xfail(reason='State machine not implemented')
    @pytest.mark.asyncio
    async def test_decrypt_no_key(self):
        data = b'hellooooooooow'
        msg = Request(type=Request.DECRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.ASYMETRIC_KEY_ERROR
        assert not ret.key
        assert not ret.signature
        assert not ret.content

    @pytest.mark.asyncio
    async def test_decrypt_bad_sym_key(self):
        await self.load_key()
        data = b'hellooooooooow'
        # Encrypt two time the same data
        encrypted_first = await self.encrypt(data)
        encrypted_second = await self.encrypt(data)
        assert encrypted_first['key'] != encrypted_second['key']
        # Make sure we cannot mix the keys
        ret = await self.service.dispatch_msg({
            'cmd': 'decrypt',
            'content': encrypted_first['content'],
            'signature': encrypted_first['signature'],
            'key': encrypted_second['key'],
            'key_signature': encrypted_first['key_signature']
        })
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid signature, content may be tampered.'}

    @pytest.mark.asyncio
    async def test_decrypt_no_sym_key(self):
        await self.load_key()
        encrypted = await self.encrypt()
        ret = await self.service.dispatch_msg({
            'cmd': 'decrypt',
            'content': encrypted['content'],
            'signature': encrypted['signature'],
            'key': '',
            'key_signature': encrypted['key_signature']
        })
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid signature, content may be tampered.'}

    @pytest.mark.asyncio
    async def test_decrypt_bad_content(self):
        await self.load_key()
        data = b'hellooooooooow'
        encrypted = await self.encrypt(data)
        ret = await self.service.dispatch_msg({
            'cmd': 'decrypt',
            'content': data,
            'signature': encrypted['signature'],
            'key': encrypted['key'],
            'key_signature': encrypted['key_signature']
        })
        assert ret == {'status': 'bad_params', 'label': 'Param `content` is not valid base64 data.'}
        ret = await self.service.dispatch_msg({
            'cmd': 'decrypt',
            'content': b64(data),
            'signature': encrypted['signature'],
            'key': encrypted['key'],
            'key_signature': encrypted['key_signature']
        })
        assert ret == {'status': 'asym_crypto_error', 'label': 'Invalid signature, content may be tampered.'}


class TestCryptoService(BaseTestCryptoEngineService):

    def setup_method(self):
        self.service = CryptoService(symetric=AESCipher(), asymetric=RSACipher())
        self.key_size_small = 256
        self.key_size_request = 2048
        self.test_key = """-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCUKiMBsx4lMVrx
0tRU6q/P5s0n3Rf9y0ogtrkCC2NwUrwSrx/xvfUyRXZnWxhtEqACGzwBDvExWY2Y
MykWRjRvZRpKg5fOos4gc0woIhxJzIyOvWvjxE21bfpH6QZqJuCf9ty7ughwNNb6
FctEKKMeDh2K0gqoQL+KxFHpKzv3b8SRiDOIjxZInFk335qJOVuRauoYeelzDz2s
Chxusekv1/OTAsLAZrvb0V7ufSsg9/ycn6bGx0dlRVKJvmlNrPLT9WcMQ5SNBfT/
fkxnJJ7ASRQKxZTZkwafFSvPaClHGGAc2f9l1pfcaZdQwOmMtXzJLa8AY1QtaF56
jpFND5XFAgMBAAECggEAeNj9gI9mERP2h7NceH6LM9mej9snjFvZdGFU+TPswVra
B6tLNNOpQH2jm52TiLNeSxmHkZ1sYMIYWYGxC3froMgn74rxsRrdYV5pSXq49ACg
zHP3oeklMMwpDaolD0PyhsbFN2D/LPYMOiK4jjlPAl6k/etfweg90qNZ5ALdgG0t
fj1E94QhA6GrCjurjI3pILfgWDxFKe2+A6ld/q5PPSRmgTQr8gn3KQ5SwlJmO8Zj
84F/isbBOidBJvfqVjDSULpwHbgaPU+b+uDqoF6+oAYCrvQXHOQF0e+NOi4ciofh
MqPBJXYNKkMD9Atg0LsIJiiSw2Hb5u8kTAGiVRgwqQKBgQDFS3clrGaesc6/hT8D
Jfn5HsGEg6P4EGTy1oyqNlpzWikVxt286ocwur77/KL6vXXtm+mj4YgeGv3hBgcB
43aAuRlza8ENQXFHCQRFlElag5k3mtLJ49wHA4Fs0Y2J0uK9sOjYUBopBXBHy65X
KfYHTiXlc9/t3m1Pzr3xgL9VPwKBgQDAQEcUaGYSifIcGPzNj8ejR9UKR7ye5Vj7
brU7ZLxZZjZTJyx/eq2p0qWdqpfzxEF5Jnsg2Ho2MBR9crNAE4EMhPH1aGYjdo85
KY242ZLtHlSHQKqDsT5WO/0I+dWSVpAEUbsrlLXSJ+th+OCz8psEy6Dr1HZ0AuUg
ag9exuO/+wKBgGIuqf5/ixoSVlcNEkyYy4tj+N3fPOwoDHSkvJ/AKMca6TNDIfnv
pJNle8Ge+eRaAKPcYSsDA2AoAovHGhmgfsqUUswTpaDZHmxBWnTd1JtMviTj0V5T
HJ4I6pGivxMFdXz82wM66ancYQH5pKsP4LXF+Cn1vkx70l5S/kd+0Li1AoGAeHjP
Ee7J59whp5HQ+U+cHqmoyqRhgoDd3dFmKC3cCXmPmVP3Antxz/V8auy4A717+dsv
VUnSa5p9fI8f3ItcVugIZ2xgdOCap4tuj+NnusdC2O6g651qHsfArJtCRk2QOeSt
kYXC2krBqcc3qAvjMIIZ+S5OfCxEQKe1sgKYPXkCgYB7w71LVJ1oN8JZpwD0N3b2
5+58S878/iwpm4xDmTqbHi1zVPlIr2OiBkosuzy6oNc3TNcqTvUv7Tf+Bm4IHAJ2
mkEuiZnkwiChKkR2RG8uwOveP+ne1uCC1uWfG5CGf+fZOlYm3tFcFlbLWpiDyeJS
hKP2d9ktDCa61hAOpAKsUQ==
-----END PRIVATE KEY-----"""
        self.default_passphrase = 'thisisaT3st!'
        self.test_key_encrypted = """-----BEGIN ENCRYPTED PRIVATE KEY-----
MIIFHzBJBgkqhkiG9w0BBQ0wPDAbBgkqhkiG9w0BBQwwDgQI71k6tEU7R38CAggA
MB0GCWCGSAFlAwQBKgQQYPeuPReKhe07np/Biqu6CgSCBNAkcfXLC+CwaSzqgIEp
Fx49j8LRe5BlxZ3OrwPax7I5vI59dWdqUHSsj0AihY/N9VCOpr4kBj88ApOfRkVK
gD1+3an6zz9KaSGen6NONvo29oPBZWn7uNjdbZqFmHQEi4Tv0SGDaNjDakNR+P7o
Yp/Khwqe5U7l5UTra8ZkRjLi96b+OER5Rtz4wD6MkM596sWKI2WHoBxMZg8oIOrp
I1TfsxKrI5IFN7+z0FulTZlH81e2eiiMHIT5LlE4QWvIHzhSUdU5MMQCYKA0jKoE
udw2Bo3M/5TU00hBM4/XtjbzncznonV1dVvxrNONvakzLj+Z2dAy9tf1soB5OSEd
MFz56HIiR+aHUDJNRQSoyPgBwZQeaZFyrSskVVgacxAiej0MWY3xW0fnwPTvgu0u
C/aua6lIjBYw4pGqCoYurUnE0rr2CI4LxF9aodiktL4NvRSKS4nbA7geaucv4T//
4s0PN+7t705rT+XmLX//SavpqwcQ8tSeXvr8SFXsluJVOjtcUL9w5tmTNxoKUBc4
t59/GEOs0GEbrBp+5v6nep9Z340Mbk9OOOTm+x/MwSiTsx0rRN2hW4IxxotQYMvp
z9Yds8pbwo9Hr1t75NE9UlaLmiNOFLhSOWV3CKzkGhxAw8Q120b7QYT4Mb8p+Pqs
aBhHW15jLuci52AyFZ/Gl8XiXjI/54w+do55a9SPSgDNeHpKosGbCPjJMKmZBVS4
PyjsGbgVBTKsBoAYw9Tda9AnYJQ2jdOSgcATKm7RU8Ra3X/aIQ0nkjmNtJWqkp0m
4odihUmpsXO5RD+LJw6aF2Mg80ZOllPG95a+EtutAMaPLx2YcAq8uLiAhosC9xO+
AYLGeI5LiVk7XeMxw65de6Pd/EGdAbbToZZJhmlSK28csDdbVDPq3mxdFQI9V9rD
sMdPUQ8KCGwnOeIzwgLTF9B4j03WMV0x+gHW2WpDnSB1OLTcb3WDiT/SwYmZBmse
DUIN7zBwANfW72EHOeq2OfKqh7Cri/NMpQS2mxjsBKWvXb4cAjD7szg9Q0Ag9WOd
82GkJg1poVuAxhEvb9RRTqUPZoYxlaUiadLl27e/GwhvR9mkxBgQJxJmCIg/bSLf
e1SUCtni+A69aC59iSvw+IZ1BxIyQc3Vk8iTnRZYiyXdRztR1pRAI95RRpE27pP/
twk4CH1Em72VsdrKjVAeOdxZf5gwzgOu8NcDxvRHUaZzV62yUhN4cXNU2o/JqyU1
FbJOkdLH5qzPPFt+1v9mlwzdm6K7DE19HrADCe0HDYDc9H9aMG/MSyb9Iy+pcx9z
JH++2i+5zWwf1CYTlXquwj8pNgourOIpGYjLKTpPPIp6xOPmXXEm3kCiGmKNBWWG
avoaJFZRq8YHcDkgDeklhXx/aPd9SzVJ2EHyhFcPBCn15sRn35qNVfcCramusbX3
fNFOAuee0CdTpEI/JaDb1QaijMINb0LD2bxYGD+PslEleb1ulw+JaRQ9yZAVtBjH
xGpG5euZ/dgFVK3yKIkm6XTFy62NkuneIQC4FMkb8j2HkkVsSmMZvxf5JyDVAOf7
SEqP+SV9/Vj8xW+lC5gnlbV88qsLiCO4HEfUCqlUkpFquL8Z0HTWNTN7fTUNyxpZ
l8CTZbRr7g4ooUKxa0p+RWIrNw==
-----END ENCRYPTED PRIVATE KEY-----"""
        self.small_test_key = """-----BEGIN RSA PRIVATE KEY-----
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


class TestMockCryptoService(BaseTestCryptoEngineService):

    def setup_method(self):
        override = 'I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE'
        self.service = CryptoService(symetric=MockSymCipher(override),
                                     asymetric=MockAsymCipher(override))
        self.test_key = 'AA123456FF'
        self.default_passphrase = 'secretpass'
        self.test_key_encrypted = 'AA123456FFsecretpass'
        self.small_test_key = 'A'
        self.key_size_request = 6
        self.key_size_small = 1
