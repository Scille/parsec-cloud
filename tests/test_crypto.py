import pytest
from collections import namedtuple

from parsec.crypto import LocalCryptoClient, CryptoError
from parsec.crypto import CryptoEngineService
from parsec.crypto.crypto_pb2 import Request, Response


class TestBaseCryptoClient:

    def setup_method(self):
        self.service = CryptoEngineService()
        self.client = LocalCryptoClient(service=self.service)
        self.test_key = b"""-----BEGIN PRIVATE KEY-----
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
        self.default_passphrase = b'thisisaT3st!'
        self.test_key_encrypted = b"""-----BEGIN ENCRYPTED PRIVATE KEY-----
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
-----END ENCRYPTED PRIVATE KEY-----
"""

    def test_gen_key(self):
        ret = self.client.genkey(passphrase=self.default_passphrase, key_size=2048)
        assert ret.key
        assert ret.status_code == Response.OK

    def test_load_key(self):
        ret = self.client.load_key(self.test_key)
        assert ret.status_code == Response.OK

    def test_encrypt(self):
        ret = self.client.load_key(self.test_key)
        assert ret.status_code == Response.OK

        ret = self.client.encrypt(content=b'EncryptMePlz')
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.content
        assert ret.key_signature
        assert ret.signature

    def test_decrypt(self):

        ret = self.client.load_key(self.test_key)
        assert ret.status_code == Response.OK

        ret = self.client.encrypt(content=b'EncryptMePlz')
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.content
        assert ret.key_signature
        assert ret.signature
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
        msg = Request(type=Request.GEN_KEY, passphrase=b'', key_size=256)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR
        assert ret.error_msg == "Incorrect RSA key size"

    # def test_gen_key_good_key_no_passphrase(self):
    #     # Smallest key available
    #     msg = Request(type=Request.GEN_KEY, passphrase='', key_size=1024)
    #     ret = self.service.dispatch_msg(msg)
    #     assert ret.status_code == Response.OK
    #     assert ret.key

    #     # Try to import it back in RSA module
    #     key = RSA.importKey(ret.key)
    #     assert key

    # def test_gen_key_good_key_passphrase(self):
    #     # Generate key with passphrase
    #     msg = Request(type=Request.GEN_KEY, passphrase='thisisaT3st!', key_size=1024)
    #     ret = self.service.dispatch_msg(msg)
    #     assert ret.status_code == Response.OK
    #     assert ret.key
    #     key = None
    #     with pytest.raises(ValueError):
    #         key = RSA.importKey(ret.key)
    #     assert not key
    #     key = RSA.importKey(ret.key, 'thisisaT3st!')
    #     assert key

    def test_load_key_bad_format(self):
        test_key = b'this is bullshit'
        msg = Request(type=Request.LOAD_KEY, key=test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR

    def test_load_key_no_passphrase(self):
        msg = Request(type=Request.LOAD_KEY, key=self.test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

    def test_load_key_with_passphrase(self):
        msg = Request(type=Request.LOAD_KEY, key=self.test_key_encrypted)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR
        msg = Request(type=Request.LOAD_KEY,
                      key=self.test_key_encrypted,
                      passphrase=self.default_passphrase)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

    def test_encrypt_all_good(self):
        msg = Request(type=Request.LOAD_KEY, key=self.test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.key_signature
        assert ret.signature
        assert data != ret.content

    def test_encrypt_no_key(self):
        msg = Request(type=Request.ENCRYPT, content=b'Not Working')
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_ERROR
        assert not ret.key
        assert not ret.key_signature
        assert not ret.content

    def test_decrypt_all_good(self):
        msg = Request(type=Request.LOAD_KEY, key=self.test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature
        assert data != ret.content

        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=ret.signature, key=ret.key, key_signature=ret.key_signature)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert data == ret.content

    def test_decrypt_bad_sig(self):
        msg = Request(type=Request.LOAD_KEY, key=self.test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        bad_sig = b'oops_this_does_not_look_like_a_signature_isnt_it'

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature != bad_sig
        assert data != ret.content

        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=bad_sig, key=ret.key, key_signature=ret.key_signature)
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
        msg = Request(type=Request.LOAD_KEY, key=self.test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        bad_key = ret.key
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert data != ret.content
        assert bad_key != ret.key

        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=ret.key, key=bad_key, key_signature=ret.key_signature)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_SIGN_ERROR

    def test_decrypt_no_aes_key(self):
        msg = Request(type=Request.LOAD_KEY, key=self.test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'
        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature
        assert data != ret.content

        msg = Request(type=Request.DECRYPT, content=ret.content,
                      signature=ret.signature, key=None)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.RSA_KEY_SIGN_ERROR

    def test_decrypt_bad_content(self):
        msg = Request(type=Request.LOAD_KEY, key=self.test_key)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK

        data = b'hellooooooooow'

        msg = Request(type=Request.ENCRYPT, content=data)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.OK
        assert ret.key
        assert ret.signature
        assert data != ret.content

        msg = Request(type=Request.DECRYPT, content=data,
                      signature=ret.signature, key=ret.key, key_signature=ret.key_signature)
        ret = self.service.dispatch_msg(msg)
        assert ret.status_code == Response.VERIFY_FAILED
        assert ret.error_msg == 'Invalid signature, content may be tampered'

    def test_service_bad_raw_msg(self):
        rep_buff = self.service.dispatch_raw_msg(b'dummy stuff')
        response = Response()
        response.ParseFromString(rep_buff)
        assert response.status_code == Response.BAD_REQUEST
        assert response.error_msg == 'Invalid request format'

    def test_service_good_raw_msg(self):
        # Smallest key available
        msg = Request(type=Request.GEN_KEY, passphrase=b'', key_size=2048)
        msg = msg.SerializeToString()
        rep_buff = self.service.dispatch_raw_msg(msg)
        response = Response()
        response.ParseFromString(rep_buff)
        assert response.status_code == Response.OK


class TestCryptoService(BaseTestCryptoEngineService):

    def setup_method(self):
        self.service = CryptoEngineService()
        self.test_key = b"""-----BEGIN PRIVATE KEY-----
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
        self.default_passphrase = b'thisisaT3st!'
        self.test_key_encrypted = b"""-----BEGIN ENCRYPTED PRIVATE KEY-----
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

# TODO Test on a mock version ? Do we need a mock version of crypto ?
# Maybe with a know and fixed RSA/AES key ? We'll need to mock AES and RSA classes
#
