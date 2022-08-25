# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec.api.version import API_VERSION
from parsec._parsec import DateTime

from tests.common.backend import AuthenticatedHeaders, AuthenticatedHttpApiClient, to_http_headers


@pytest.mark.trio
async def test_valid_request(alice_http_client: AuthenticatedHttpApiClient):
    rep = await alice_http_client.send_dummy_request(check_rep=False)
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == str(API_VERSION)
    rep_data = alice_http_client.parse_response(await rep.get_data())
    assert rep_data == alice_http_client.expected_response()


@pytest.mark.trio
async def test_body_not_msgpack(alice_http_client: AuthenticatedHttpApiClient):
    timestamp = DateTime.now()

    request = b"hello world ;)"
    signature = alice_http_client.sign_request(timestamp, request)

    headers = to_http_headers(
        AuthenticatedHeaders(
            **alice_http_client.base_headers,
            timestamp=timestamp,
            signature=signature,
        )
    )

    rep = await alice_http_client.send_authenticated_request(headers, request)
    assert rep.status_code == 200
    rep_data = alice_http_client.parse_response(await rep.get_data())
    assert rep_data == {"status": "invalid_msg_format"}
