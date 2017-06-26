import pytest
from unittest.mock import patch

from parsec.server import BaseServer, BaseClientContext
from parsec.backend import InMemoryPubKeyService
from parsec.crypto import RSAPrivateKey, RSAPublicKey
from parsec.session import AuthSession
from parsec.exceptions import PubKeyError, PubKeyNotFound

from parsec.core import IdentityService, PrivkeyService


ALICE_PRIVATE_RSA = b"""
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
ALICE_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MFswDQYJKoZIhvcNAQEBBQADSgAwRwJBALEpXv8eI7a/pGsYr/QWJS/oH3x/1Swv
kP1k0wftRSFNX3ZoHWVRmiJQFiR6q/ahGQX8F2M5UL/KSDJ/luaarx0CAgEB
-----END PUBLIC KEY-----
"""


def privkey_svc(request, event_loop):
    return PrivkeyService()


@pytest.fixture
def privkey_svc(event_loop):
    service = PrivkeyService()
    server = BaseServer()
    server.register_service(IdentityService())
    server.register_service(service)
    event_loop.run_until_complete(server.bootstrap_services())
    yield service
    event_loop.run_until_complete(server.teardown_services())


@pytest.fixture
@pytest.mark.asyncio
async def alice(pubkey_svc):
    await pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)
    return {
        'id': 'alice',
        'private_key': RSAPrivateKey(ALICE_PRIVATE_RSA),
        'public_key': RSAPublicKey(ALICE_PUBLIC_RSA)
    }


class TestPrivKeyService:

    @pytest.mark.asyncio
    async def test_add_privkey(self, privkey_svc):
        await privkey_svc.add_privkey('alice', 'secret', ALICE_PRIVATE_RSA)

    @pytest.mark.asyncio
    async def privkey_load_identity(self, privkey_svc):
        pass
