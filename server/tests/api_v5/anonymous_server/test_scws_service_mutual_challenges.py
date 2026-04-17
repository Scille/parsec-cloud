# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey, RSAPublicKey
from cryptography.hazmat.primitives.serialization import load_pem_private_key

from parsec._parsec import anonymous_server_cmds
from parsec.components.scws import _raw_rsa_sign
from parsec.config import ScwsConfig
from tests.common import AnonymousServerRpcClient, Backend, HttpCommonErrorsTester

IDOPTE_PRIVATE_KEY_PEM = b"""\
-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQDjtDYe9EH3iKyUWMiWzZ5uec5xnIGgyLkhtZmz0nL70Pk4lURz
gnaGXhQhZESLXFFlW1b1ZiFEkRGg+dQL6S8XGb3NcNDk1RcZdnKAYS+mUTLKMoRg
A2qHgYoMtlDNmWLi9LGhhrmBIShPmH2LK17Utvg/2B4M1ZRir/T6eiE+IQIDAQAB
AoGAUjjzxg+rdmfq8ZJxtErBZ+EiE9JQ2K9CcpKx0sYucRHPvIfh9NfqVrDIEhqz
yaLnXXMzHl/nqWu88uTjlf142+7xVf/gn34oJBPcSfmqkqzmlXyHkcH6dpfZnZ6U
pxO9mF35WJs2IojUsiS5Ax2YsARETs8PRhzd3lyEgqaneNUCQQD/gYJmlz0oVk6u
7siOkHLZVbwwyQ/1dF7ZwCP1iAd/iH8cxfwsuY0g50c55WEZpBBRLnNprrSKATDx
VlPwOq2LAkEA5CTwO/KA0fWBCQtBHDVhBCpwqaHNMrp4uXDdKkv8JxCjsaaUZO2v
UezGMX5uTO0KgeOZDeh1JPeJV4SZi31QgwJAB4s8XWkTvjAmLJYSR9tN+MrGPq6B
ER85ebZpmWNFzWcrUZ8q3eswlosdmEEEh+xHKw5zC+jDnduBJW8GFW62FwJBAN9i
s79e4XeztVbPqD8gnQ/hJlNSVIN5Rj/9HPEPWaQ+jKmj++UfPe0vs5g+vW2hJAqu
eey+UgWSQu5orZeBalUCQB0Yep1rrn4LPcPZjLEZpfsFCNl6xnf4A0KsE/3xZQoj
llEpHGpOfFAmqAXnbO5hZ/tJb17drCV0+DjQGa5cU/o=
-----END RSA PRIVATE KEY-----
"""

WEB_APP_PRIVATE_KEY_PEM = b"""\
-----BEGIN RSA PRIVATE KEY-----
MIICWwIBAAKBgQDNmVVDL+Yg8zx5rT1lWgHlUzVaN/h6r1dXVJEqTojrOEARfX+0
6bVsTeBnfipkqJZl4IFct9EgMXWaDsSAlVqzEjjwfD2awdHJgeAsr+Kde7cpo9UM
g2IoNl16fnChPMoSSlxnMxoArviRIKxl38n3SNFKMa6FfPOk0yHsUJUgEQIDAQAB
AoGAdqJbiIFDERBJfQxxuOHO5jy1NHHHd0Nl6oZpnTfj2ZaDoZQA9KtGfFAThKRQ
YfTFk9OP0ahfi2v+p/6NdIS56EEmTyuMowmhBpcrEFri+g3HvBGggecfPitWZlS0
iasrqIXEkky7BthTRvl1ncon8bO/dLz1sXW5zuapBXlvleECQQDxjpimjZb8ayev
Ek6qGG/LhALNKb5PC3P+YTYGji6afF2fC1y4/hLWarHP65Ct0H8kIJU859nGu16E
5eefSWkfAkEA2eRX0nduJaocN4ZiIBZl5elSwCm89Tzxj6J2wlS64aGR5Ang0kt0
Zz5SCsBZuAdA0vWlOIKNTnyyncqjygLgzwJAHJKJa+oDmgfywbqgo24QizoqOqpd
YGwyZDyLb2sSCCP9zvpBFYC4KbSlI7rxeh3XbCaOgI3MLL2tCHtJUoYUcQJAIJbA
c+Ac/1EkC0H0JyxybSKql8cmVd+ZmYwJCyO1F84cjejUUV+rt01g4+7E/HtJPMQ0
w/DyxYxtAqCuQqaPjQJALRmzX18wfy5+sjYIN+29jMEEuTS3M28Qwh3Lll0gWaYH
5bIYlQ5TASCx/7oU0rd+R2B1wc/lHGXHEo/t2txb6g==
-----END RSA PRIVATE KEY-----
"""


def _load_private_key(pem: bytes) -> RSAPrivateKey:
    key = load_pem_private_key(pem, password=None)
    assert isinstance(key, RSAPrivateKey)
    return key


def _setup_scws_config() -> tuple[ScwsConfig, RSAPrivateKey, RSAPublicKey]:
    idopte_private_key = _load_private_key(IDOPTE_PRIVATE_KEY_PEM)
    web_app_private_key = _load_private_key(WEB_APP_PRIVATE_KEY_PEM)

    scws_config = ScwsConfig(
        idopte_public_keys=[idopte_private_key.public_key()],
        web_application_private_key=web_app_private_key,
    )
    return scws_config, idopte_private_key, web_app_private_key.public_key()


async def test_anonymous_server_scws_service_mutual_challenges_ok(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    backend.config.scws_config, idopte_private_key, _ = _setup_scws_config()

    challenge_payload = b"some-challenge-data"
    challenge_signature = _raw_rsa_sign(idopte_private_key, challenge_payload)
    web_app_challenge_payload = b"web-app-challenge"

    rep = await anonymous_server.scws_service_mutual_challenges(
        scws_service_challenge_payload=challenge_payload,
        scws_service_challenge_signature=challenge_signature,
        scws_service_challenge_key_id=0,
        web_application_challenge_payload=web_app_challenge_payload,
    )

    assert isinstance(rep, anonymous_server_cmds.latest.scws_service_mutual_challenges.RepOk)
    # Verify the server signed the web application challenge with its private key
    from parsec.components.scws import _raw_rsa_verify

    web_app_public_key = backend.config.scws_config.web_application_private_key.public_key()
    assert _raw_rsa_verify(
        web_app_public_key,
        web_app_challenge_payload,
        rep.web_application_challenge_signature,
    )


async def test_anonymous_server_scws_service_mutual_challenges_invalid_scws_service_challenge_signature(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    backend.config.scws_config, _, _ = _setup_scws_config()

    rep = await anonymous_server.scws_service_mutual_challenges(
        scws_service_challenge_payload=b"some-challenge-data",
        scws_service_challenge_signature=b"bad-signature",
        scws_service_challenge_key_id=0,
        web_application_challenge_payload=b"web-app-challenge",
    )

    assert isinstance(
        rep,
        anonymous_server_cmds.latest.scws_service_mutual_challenges.RepInvalidScwsServiceChallengeSignature,
    )


async def test_anonymous_server_scws_service_mutual_challenges_not_available(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    assert backend.config.scws_config is None

    rep = await anonymous_server.scws_service_mutual_challenges(
        scws_service_challenge_payload=b"some-challenge-data",
        scws_service_challenge_signature=b"signature",
        scws_service_challenge_key_id=0,
        web_application_challenge_payload=b"web-app-challenge",
    )

    assert isinstance(
        rep,
        anonymous_server_cmds.latest.scws_service_mutual_challenges.RepNotAvailable,
    )


async def test_anonymous_server_scws_service_mutual_challenges_unknown_scws_service_challenge_key_id(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    backend.config.scws_config, _, _ = _setup_scws_config()

    # Key ID out of range (only key 0 exists)
    rep = await anonymous_server.scws_service_mutual_challenges(
        scws_service_challenge_payload=b"some-challenge-data",
        scws_service_challenge_signature=b"signature",
        scws_service_challenge_key_id=999,
        web_application_challenge_payload=b"web-app-challenge",
    )

    assert isinstance(
        rep,
        anonymous_server_cmds.latest.scws_service_mutual_challenges.RepUnknownScwsServiceChallengeKeyId,
    )


async def test_anonymous_server_scws_service_mutual_challenges_unknown_scws_service_challenge_key_id_revoked(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    # Key at index 0 is revoked (None)
    scws_config = ScwsConfig(
        idopte_public_keys=[None],
        web_application_private_key=_load_private_key(WEB_APP_PRIVATE_KEY_PEM),
    )
    backend.config.scws_config = scws_config

    challenge_payload = b"some-challenge-data"
    challenge_signature = _raw_rsa_sign(
        _load_private_key(IDOPTE_PRIVATE_KEY_PEM), challenge_payload
    )

    rep = await anonymous_server.scws_service_mutual_challenges(
        scws_service_challenge_payload=challenge_payload,
        scws_service_challenge_signature=challenge_signature,
        scws_service_challenge_key_id=0,
        web_application_challenge_payload=b"web-app-challenge",
    )

    assert isinstance(
        rep,
        anonymous_server_cmds.latest.scws_service_mutual_challenges.RepUnknownScwsServiceChallengeKeyId,
    )


async def test_anonymous_server_scws_service_mutual_challenges_invalid_web_application_challenge_payload(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    backend.config.scws_config, idopte_private_key, _ = _setup_scws_config()

    # Payload too long for the 1024bits RSA key to sign (key_size_bytes - 11 is the max)
    too_long_payload = b"x" * 128

    challenge_payload = b"some-challenge-data"
    challenge_signature = _raw_rsa_sign(idopte_private_key, challenge_payload)

    rep = await anonymous_server.scws_service_mutual_challenges(
        scws_service_challenge_payload=challenge_payload,
        scws_service_challenge_signature=challenge_signature,
        scws_service_challenge_key_id=0,
        web_application_challenge_payload=too_long_payload,
    )

    assert isinstance(
        rep,
        anonymous_server_cmds.latest.scws_service_mutual_challenges.RepInvalidWebApplicationChallengePayload,
    )


async def test_anonymous_server_scws_service_mutual_challenges_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
):
    async def do():
        await anonymous_server.scws_service_mutual_challenges(
            scws_service_challenge_payload=b"",
            scws_service_challenge_signature=b"",
            scws_service_challenge_key_id=42,
            web_application_challenge_payload=b"",
        )

    await anonymous_server_http_common_errors_tester(do)
