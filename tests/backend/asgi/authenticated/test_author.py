# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec.api.protocol.types import DeviceID

from tests.common.backend import AuthenticatedHttpApiClient


@pytest.mark.trio
async def test_author_missing_header(alice_http_client: AuthenticatedHttpApiClient):
    rep = await alice_http_client.send_dummy_request(
        extra_headers={"author": None}, check_rep=False
    )
    assert rep.status_code == 400  # bad request


@pytest.mark.trio
async def test_author_not_found(alice_http_client: AuthenticatedHttpApiClient):
    rep = await alice_http_client.send_dummy_request(
        extra_headers={"author": DeviceID("foo@bar")}, check_rep=False
    )
    assert rep.status_code == 400  # bad request
