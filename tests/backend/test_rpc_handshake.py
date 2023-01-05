# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import logging
from base64 import b64encode
from typing import Union
from unittest.mock import patch

import pytest

from parsec._parsec import DateTime, DeviceID, LocalDevice
from parsec.api.version import API_VERSION, ApiVersion
from parsec.serde import unpackb
from tests.common import AnonymousRpcApiClient, AuthenticatedRpcApiClient


async def _test_good_handshake(client: Union[AuthenticatedRpcApiClient, AnonymousRpcApiClient]):
    # Sanity check: make sure base query is valid
    rep = await client.send_ping(check_rep=False)
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == str(API_VERSION)


async def _test_handshake_bad_organization(
    client: Union[AuthenticatedRpcApiClient, AnonymousRpcApiClient]
):
    for badorg in [
        "dummy",  # Unknown organization
        "a" * 65,  # Invalid organization ID
    ]:

        def _before_send_hook(args):
            args["path"] = args["path"].replace("CoolOrg", badorg)

        rep = await client.send_ping(
            before_send_hook=_before_send_hook,
            check_rep=False,
        )
        assert rep.status_code == 404
        assert rep.headers["Api-Version"] == str(API_VERSION)


async def _test_handshake_api_version_header(
    client: Union[AuthenticatedRpcApiClient, AnonymousRpcApiClient]
):
    server_versions = [ApiVersion(2, 1), ApiVersion(3, 1), ApiVersion(4, 1)]

    with patch("parsec.backend.asgi.rpc.SUPPORTED_API_VERSIONS", server_versions):

        # Plain invalide header value
        rep = await client.send_ping(
            extra_headers={"Api-Version": "dummy"},
            check_rep=False,
        )
        assert rep.status_code == 422
        assert rep.headers["Supported-Api-Versions"] == "2.1;3.1;4.1"

        # Missing header, fallback to default value for backward compatibility
        rep = await client.send_ping(
            extra_headers={"Api-Version": None},
            check_rep=False,
        )
        # Special case for anynous
        assert rep.status_code == 200
        rep.headers["Api-Version"] == "3.0"

        # Client provides an incompatible api version
        unknown_major = ApiVersion(version=5, revision=0)
        too_old_major = ApiVersion(version=1, revision=0)
        for bad_version in (too_old_major, unknown_major):
            rep = await client.send_ping(
                extra_headers={"Api-Version": str(bad_version)},
                check_rep=False,
            )
            assert rep.status_code == 422
            assert rep.headers["Supported-Api-Versions"] == "2.1;3.1;4.1"

        # Client provides a compatible api version
        unknown_minor = ApiVersion(version=4, revision=2)
        rep = await client.send_ping(
            extra_headers={"Api-Version": str(unknown_minor)},
            check_rep=False,
        )
        assert rep.status_code == 200
        assert rep.headers["Api-Version"] == "4.1"


async def _test_handshake_content_type_header(
    client: Union[AuthenticatedRpcApiClient, AnonymousRpcApiClient]
):
    # Bad header value
    rep = await client.send_ping(
        extra_headers={"Content-Type": "application/json"},
        check_rep=False,
    )
    assert rep.status_code == 415
    assert rep.headers["Api-Version"] == str(API_VERSION)

    # Missing header
    rep = await client.send_ping(
        extra_headers={"Content-Type": None},
        check_rep=False,
    )
    assert rep.status_code == 415
    assert rep.headers["Api-Version"] == str(API_VERSION)

    # Incorrect header that was used in Parsec <= 2.11.1
    rep = await client.send_ping(
        extra_headers={"Content-Type": "application/x-www-form-urlencoded"},
        check_rep=False,
    )
    assert rep.status_code == 200
    rep.headers["Content-Type"] == "application/msgpack"


async def _test_authenticated_handshake_bad_signature_header(
    client: AuthenticatedRpcApiClient, alice: LocalDevice, bob: LocalDevice
):
    # First test missing & plain bad headers
    for expected_status_code, extra_headers in [
        (401, {"Signature": None}),  # Missing Signature header
        (401, {"Signature": "dummy"}),  # Bad Signature header
        (401, {"Signature": b64encode(b"dummy")}),  # Base64 as expected but still bad signature
        (401, {"Authorization": None}),  # Missing Authorization header
        (401, {"Authorization": "PARSEC-SIGN-RSA-4096"}),  # Bad Authorization header
        (401, {"Author": None}),  # Missing Author header
        (401, {"Author": "dummy"}),  # Bad Author header
    ]:
        rep = await client.send_ping(
            extra_headers=extra_headers,
            check_rep=False,
        )
        assert rep.status_code == expected_status_code, (rep, extra_headers)
        assert rep.headers["Api-Version"] == str(API_VERSION)

    # Valid signature format, but bad signing key
    def _before_send_hook(args):
        signature = bob.signing_key.sign_only_signature(args["data"])
        args["headers"]["signature"] = b64encode(signature).decode("ascii")

    rep = await client.send_ping(
        before_send_hook=_before_send_hook,
        check_rep=False,
    )
    assert rep.status_code == 401
    assert rep.headers["Api-Version"] == str(API_VERSION)

    # Valid signature, but bad body
    def _before_send_hook(args):
        signature = alice.signing_key.sign_only_signature(b"dummy")
        args["headers"]["signature"] = b64encode(signature).decode("ascii")

    rep = await client.send_ping(
        before_send_hook=_before_send_hook,
        check_rep=False,
    )
    assert rep.status_code == 401
    assert rep.headers["Api-Version"] == str(API_VERSION)


async def _test_handshake_body_not_msgpack(
    client: Union[AuthenticatedRpcApiClient, AnonymousRpcApiClient], alice: LocalDevice
):
    now = DateTime.now()

    def _before_send_hook(args):
        bad_body = b"dummy"
        args["data"] = bad_body
        if isinstance(client, AuthenticatedRpcApiClient):
            signature = alice.signing_key.sign_only_signature(bad_body)
            args["headers"]["signature"] = b64encode(signature).decode("ascii")

    rep = await client.send_ping(
        before_send_hook=_before_send_hook,
        check_rep=False,
        now=now,
    )
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == str(API_VERSION)  # This header must always be present !
    body = await rep.get_data()
    assert unpackb(body) == {"status": "invalid_msg_format"}


async def _test_authenticated_handshake_author_not_found(
    alice_http_client: AuthenticatedRpcApiClient,
):
    rep = await alice_http_client.send_ping(
        extra_headers={"Author": DeviceID("foo@bar")}, check_rep=False
    )
    assert rep.status_code == 401
    assert rep.headers["Api-Version"] == str(API_VERSION)  # This header must always be present !


@pytest.mark.trio
async def test_handshake(
    alice_rpc: AuthenticatedRpcApiClient,
    anonymous_rpc: AuthenticatedRpcApiClient,
    alice: LocalDevice,
    bob: LocalDevice,
):
    # Merging all those tests into a single one saves plenty of time given
    # we don't have to recreate the fixtures

    await _test_good_handshake(alice_rpc)
    await _test_good_handshake(anonymous_rpc)

    await _test_handshake_bad_organization(alice_rpc)
    await _test_handshake_bad_organization(anonymous_rpc)

    await _test_handshake_api_version_header(alice_rpc)
    await _test_handshake_api_version_header(anonymous_rpc)

    await _test_handshake_content_type_header(alice_rpc)
    await _test_handshake_content_type_header(anonymous_rpc)

    await _test_authenticated_handshake_bad_signature_header(alice_rpc, alice, bob)

    await _test_handshake_body_not_msgpack(alice_rpc, alice)
    await _test_handshake_body_not_msgpack(anonymous_rpc, alice)

    await _test_authenticated_handshake_author_not_found(alice_rpc)


@pytest.mark.trio
async def test_client_version_in_logs(
    alice_rpc: AuthenticatedRpcApiClient, anonymous_rpc: AnonymousRpcApiClient, caplog
):
    client_api_version = ApiVersion(3, 99)
    alice_rpc.API_VERSION = client_api_version
    anonymous_rpc.API_VERSION = client_api_version
    with caplog.at_level(logging.INFO):
        # Authenticated
        await _test_good_handshake(alice_rpc)
        assert (
            f"Authenticated client successfully connected (client/server API version: {client_api_version}/{API_VERSION})"
            in caplog.text
        )

        # Anonymous
        await _test_good_handshake(anonymous_rpc)
        assert (
            f"Anonymous client successfully connected (client/server API version: {client_api_version}/{API_VERSION})"
            in caplog.text
        )
