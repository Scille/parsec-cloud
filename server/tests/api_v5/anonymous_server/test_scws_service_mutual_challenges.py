# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from tests.common import AnonymousServerRpcClient, Backend, HttpCommonErrorsTester


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_mutual_challenges_ok(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    raise NotImplementedError()


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_mutual_challenges_invalid_scws_service_challenge_signature():
    raise NotImplementedError()


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_mutual_challenges_not_available():
    raise NotImplementedError()


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_mutual_challenges_unknown_scws_service_challenge_key_id():
    raise NotImplementedError()


@pytest.mark.xfail()
async def test_anonymous_server_scws_service_mutual_challenges_invalid_web_application_challenge_payload():
    raise NotImplementedError()


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
