# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import re

from cryptography.exceptions import InvalidSignature
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey, RSAPublicKey
from cryptography.hazmat.primitives.serialization import load_pem_public_key

from parsec._parsec import anonymous_server_cmds
from parsec.api import api
from parsec.client_context import AnonymousServerClientContext
from parsec.config import BackendConfig
from parsec.logging import get_logger

logger = get_logger()

# The code in this file is based on the Idopte code example from
# https://idopte.fr/scwsapi/javascript/2_API/envsetup.html#server-side

# Regex to extract PEM public key blocks from the `IdoptePublicKeys`` file
_PEM_PUBLIC_KEY_RE = rb"-----BEGIN PUBLIC KEY-----.*?-----END PUBLIC KEY-----"


def parse_idopte_public_keys(pem: bytes) -> list[RSAPublicKey | None]:
    """
    Parse the `IdoptePublicKeys` file and return a list of public keys.

    Revoked keys (whose PEM body contains `!REVOKED!`) are represented as `None`.
    """
    keys: list[RSAPublicKey | None] = []
    for match in re.finditer(_PEM_PUBLIC_KEY_RE, pem, re.DOTALL):
        block = match.group()
        if b"!REVOKED!" in block:
            keys.append(None)
        else:
            public_key = load_pem_public_key(block)
            assert isinstance(public_key, RSAPublicKey)
            keys.append(public_key)
    return keys


def _raw_rsa_sign(private_key: RSAPrivateKey, data: bytes) -> bytes:
    """
    Sign raw data using PKCS#1 v1.5 signature padding *without* DigestInfo.

    This is the equivalent of PHP's `openssl_private_encrypt` used in Idopte's example:

        $k = <<<EOT
            -----BEGIN RSA PRIVATE KEY-----
            MIIBOgIBAAJBAMPMNNpbZZddeT/GTjU0PWuuN9VEGpxXJTAkmZY02o8238fQ2ynt
            N40FVl08YksWBO/74XEjU30mAjuaz/FB2kkCAwEAAQJBALoMlsROSLCWD5q8EqCX
            rS1e9IrgFfEtFZczkAWc33lo3FnFeFTXSMVCloNCBWU35od4zTOhdRPAWpQ1Mzxi
            aCkCIQD9qjKjNvbDXjUcCNqdiJxPDlPGpa78yzyCCUA/+TNwVwIhAMWZoqZO3eWq
            SCBTLelVQsg6CwJh9W7vlezvWxUni+ZfAiAopBAg3jmC66EOsMx12OFSOTVq6jiy
            /8zd+KV2mnKHWQIgVpZiLZo1piQeAvwwDCUuZGr61Ap08C3QdsjUEssHhOUCIBee
            72JZuJeABcv7lHhAWzsiCddVAkdnZKUo6ubaxw3u
            -----END RSA PRIVATE KEY-----
        EOT;
        $output = "";
        openssl_private_encrypt(hex2bin($_GET["rnd"]), $output, openssl_pkey_get_private($k));
        echo bin2hex($output);

    see: https://idopte.fr/scwsapi/javascript/2_API/envsetup.html#server-side
    """
    key_size_bytes = (private_key.key_size + 7) // 8

    # PKCS#1 v1.5 signature type padding: `0x00 0x01 || PS || 0x00 || data`
    # where PS is a string of `0xFF` bytes making the total equal to the key
    # size in bytes (with at least 8 bytes of `0xFF`).
    ps_len = key_size_bytes - len(data) - 3
    if ps_len < 8:
        raise ValueError("Data too long for the RSA key size")
    padded = b"\x00\x01" + (b"\xff" * ps_len) + b"\x00" + data

    # Raw RSA: compute m^d mod n
    padded_int = int.from_bytes(padded, byteorder="big")
    private_numbers = private_key.private_numbers()
    signature_int = pow(padded_int, private_numbers.d, private_numbers.public_numbers.n)
    return signature_int.to_bytes(key_size_bytes, byteorder="big")


def _raw_rsa_verify(public_key: RSAPublicKey, data: bytes, signature: bytes) -> bool:
    """
    Verify a raw RSA PKCS#1 v1.5 signature without DigestInfo.

    This is the equivalent of PHP's `openssl_public_decrypt` used in Idopte's example:

        $challenge = $_SESSION['challenge'];
        $output = "";
        openssl_public_decrypt(hex2bin($_GET["cryptogram"]), $output, openssl_pkey_get_public($k));
        if ($output == $challenge) {
            ...

    see: https://idopte.fr/scwsapi/javascript/2_API/envsetup.html#server-side
    """
    try:
        recovered = public_key.recover_data_from_signature(
            signature,
            padding.PKCS1v15(),
            # TODO: use `hashes.NoDigestInfo` instead of `None` once we upgrade to cryptography >= 47.0.0
            None,
        )
        return recovered == data
    except InvalidSignature:
        return False


class ScwsComponent:
    def __init__(self, config: BackendConfig) -> None:
        self._config = config

    @api
    async def anonymous_server_scws_service_mutual_challenges(
        self,
        client_ctx: AnonymousServerClientContext,
        req: anonymous_server_cmds.latest.scws_service_mutual_challenges.Req,
    ) -> anonymous_server_cmds.latest.scws_service_mutual_challenges.Rep:
        Rep = anonymous_server_cmds.latest.scws_service_mutual_challenges

        scws_config = self._config.scws_config
        if scws_config is None:
            return Rep.RepNotAvailable()

        # Validate key ID
        key_id = req.scws_service_challenge_key_id
        if key_id < 0 or key_id >= len(scws_config.idopte_public_keys):
            return Rep.RepUnknownScwsServiceChallengeKeyId()

        public_key = scws_config.idopte_public_keys[key_id]
        if public_key is None:
            # Key has been revoked
            return Rep.RepUnknownScwsServiceChallengeKeyId()

        # Verify the SCWS service challenge signature
        if not _raw_rsa_verify(
            public_key,
            req.scws_service_challenge_payload,
            req.scws_service_challenge_signature,
        ):
            return Rep.RepInvalidScwsServiceChallengeSignature()

        # Sign the web application challenge payload
        try:
            web_application_challenge_signature = _raw_rsa_sign(
                scws_config.web_application_private_key,
                req.web_application_challenge_payload,
            )
        except ValueError:
            return Rep.RepInvalidWebApplicationChallengePayload()

        return Rep.RepOk(
            web_application_challenge_signature=web_application_challenge_signature,
        )
