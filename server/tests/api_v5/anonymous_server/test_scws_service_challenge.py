# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from tests.common import AnonymousServerRpcClient, Backend, HttpCommonErrorsTester


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_challenge_ok(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    raise NotImplementedError()


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_challenge_invalid_service_signature():
    raise NotImplementedError()


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_challenge_not_available():
    raise NotImplementedError()


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_challenge_unknown_service_public_key():
    raise NotImplementedError()


async def test_anonymous_server_scws_service_challenge_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
):
    async def do():
        await anonymous_server.scws_service_challenge(
            middleware_challenge=b"", middleware_signature=b"", server_challenge=b"", pubkey_id=42
        )

    await anonymous_server_http_common_errors_tester(do)
