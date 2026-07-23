# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey, RSAPublicKey
from cryptography.hazmat.primitives.serialization import load_pem_private_key

from parsec._parsec import RsaPrivateKey, anonymous_server_cmds
from parsec.components.scws import _raw_rsa_sign
from parsec.config import ScwsConfig
from tests.common import AnonymousServerRpcClient, Backend, HttpCommonErrorsTester
from tests.test_scws import convert_rsa_pkey_from_cryptography_to_rust

# PKCS8 RSA PRIVATE KEY
IDOPTE_PRIVATE_KEY_PEM = b"""\
-----BEGIN PRIVATE KEY-----
MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBAOO0Nh70QfeIrJRY
yJbNnm55znGcgaDIuSG1mbPScvvQ+TiVRHOCdoZeFCFkRItcUWVbVvVmIUSREaD5
1AvpLxcZvc1w0OTVFxl2coBhL6ZRMsoyhGADaoeBigy2UM2ZYuL0saGGuYEhKE+Y
fYsrXtS2+D/YHgzVlGKv9Pp6IT4hAgMBAAECgYBSOPPGD6t2Z+rxknG0SsFn4SIT
0lDYr0JykrHSxi5xEc+8h+H01+pWsMgSGrPJouddczMeX+epa7zy5OOV/Xjb7vFV
/+CffigkE9xJ+aqSrOaVfIeRwfp2l9mdnpSnE72YXflYmzYiiNSyJLkDHZiwBERO
zw9GHN3eXISCpqd41QJBAP+BgmaXPShWTq7uyI6QctlVvDDJD/V0XtnAI/WIB3+I
fxzF/Cy5jSDnRznlYRmkEFEuc2mutIoBMPFWU/A6rYsCQQDkJPA78oDR9YEJC0Ec
NWEEKnCpoc0yuni5cN0qS/wnEKOxppRk7a9R7MYxfm5M7QqB45kN6HUk94lXhJmL
fVCDAkAHizxdaRO+MCYslhJH2034ysY+roERHzl5tmmZY0XNZytRnyrd6zCWix2Y
QQSH7EcrDnML6MOd24ElbwYVbrYXAkEA32Kzv17hd7O1Vs+oPyCdD+EmU1JUg3lG
P/0c8Q9ZpD6MqaP75R897S+zmD69baEkCq557L5SBZJC7mitl4FqVQJAHRh6nWuu
fgs9w9mMsRml+wUI2XrGd/gDQqwT/fFlCiOWUSkcak58UCaoBeds7mFn+0lvXt2s
JXT4ONAZrlxT+g==
-----END PRIVATE KEY-----
"""

# PKCS8 RSA PRIVATE KEY
WEB_APP_PRIVATE_KEY_PEM = b"""\
-----BEGIN PRIVATE KEY-----
MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBAOO0Nh70QfeIrJRY
yJbNnm55znGcgaDIuSG1mbPScvvQ+TiVRHOCdoZeFCFkRItcUWVbVvVmIUSREaD5
1AvpLxcZvc1w0OTVFxl2coBhL6ZRMsoyhGADaoeBigy2UM2ZYuL0saGGuYEhKE+Y
fYsrXtS2+D/YHgzVlGKv9Pp6IT4hAgMBAAECgYBSOPPGD6t2Z+rxknG0SsFn4SIT
0lDYr0JykrHSxi5xEc+8h+H01+pWsMgSGrPJouddczMeX+epa7zy5OOV/Xjb7vFV
/+CffigkE9xJ+aqSrOaVfIeRwfp2l9mdnpSnE72YXflYmzYiiNSyJLkDHZiwBERO
zw9GHN3eXISCpqd41QJBAP+BgmaXPShWTq7uyI6QctlVvDDJD/V0XtnAI/WIB3+I
fxzF/Cy5jSDnRznlYRmkEFEuc2mutIoBMPFWU/A6rYsCQQDkJPA78oDR9YEJC0Ec
NWEEKnCpoc0yuni5cN0qS/wnEKOxppRk7a9R7MYxfm5M7QqB45kN6HUk94lXhJmL
fVCDAkAHizxdaRO+MCYslhJH2034ysY+roERHzl5tmmZY0XNZytRnyrd6zCWix2Y
QQSH7EcrDnML6MOd24ElbwYVbrYXAkEA32Kzv17hd7O1Vs+oPyCdD+EmU1JUg3lG
P/0c8Q9ZpD6MqaP75R897S+zmD69baEkCq557L5SBZJC7mitl4FqVQJAHRh6nWuu
fgs9w9mMsRml+wUI2XrGd/gDQqwT/fFlCiOWUSkcak58UCaoBeds7mFn+0lvXt2s
JXT4ONAZrlxT+g==
-----END PRIVATE KEY-----
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
        web_application_private_key=convert_rsa_pkey_from_cryptography_to_rust(web_app_private_key),
    )
    return scws_config, idopte_private_key, web_app_private_key.public_key()


async def test_anonymous_server_scws_service_mutual_challenges_ok(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    backend.config.scws_config, idopte_private_key, web_app_public_key = _setup_scws_config()

    challenge_payload = b"some-challenge-data"
    challenge_signature = _raw_rsa_sign(
        convert_rsa_pkey_from_cryptography_to_rust(idopte_private_key), challenge_payload
    )
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
        web_application_private_key=RsaPrivateKey.load_pkcs8_pem(WEB_APP_PRIVATE_KEY_PEM.decode()),
    )
    backend.config.scws_config = scws_config

    challenge_payload = b"some-challenge-data"
    challenge_signature = _raw_rsa_sign(
        RsaPrivateKey.load_pkcs8_pem(IDOPTE_PRIVATE_KEY_PEM.decode()), challenge_payload
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
    challenge_signature = _raw_rsa_sign(
        convert_rsa_pkey_from_cryptography_to_rust(idopte_private_key), challenge_payload
    )

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
