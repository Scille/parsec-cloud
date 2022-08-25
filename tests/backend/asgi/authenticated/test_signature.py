# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime

from tests.common.backend import AuthenticatedHttpApiClient


@pytest.mark.trio
async def test_signature_missing_header(alice_http_client: AuthenticatedHttpApiClient):

    rep = await alice_http_client.send_dummy_request(
        extra_headers={
            "signature": None,
        },
        check_rep=False,
    )
    assert rep.status_code == 400  # bad request


@pytest.mark.trio
async def test_signature_invalid(alice_http_client: AuthenticatedHttpApiClient):
    timestamp = DateTime.now()

    request = alice_http_client.get_request()

    signature = alice_http_client.sign_request(timestamp.add(minutes=1), request)

    rep = await alice_http_client.send_dummy_request(
        extra_headers={
            "signature": signature,
        },
        check_rep=False,
    )
    assert rep.status_code == 401  # Unauthorized
