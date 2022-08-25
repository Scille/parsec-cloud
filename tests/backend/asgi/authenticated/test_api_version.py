# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec.api.version import API_V2_VERSION, API_VERSION, ApiVersion

from tests.common.backend import AuthenticatedHttpApiClient


@pytest.mark.trio
async def test_api_version_missing_header(alice_http_client: AuthenticatedHttpApiClient):
    rep = await alice_http_client.send_dummy_request(
        extra_headers={"api_version": None}, check_rep=False
    )
    assert rep.status_code == 400  # bad request


@pytest.mark.trio
async def test_api_version_incompatible_version(alice_http_client: AuthenticatedHttpApiClient):
    rep = await alice_http_client.send_dummy_request(
        extra_headers={"api_version": ApiVersion(version=API_VERSION.version + 1, revision=0)},
        check_rep=False,
    )
    assert rep.status_code == 400


@pytest.mark.trio
async def test_api_version_different_but_compatible_version(
    alice_http_client: AuthenticatedHttpApiClient,
):
    rep = await alice_http_client.send_dummy_request(
        extra_headers={"api_version": API_V2_VERSION}, check_rep=False
    )
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == str(API_V2_VERSION)
    rep_data = alice_http_client.parse_response(await rep.get_data())
    assert rep_data == alice_http_client.expected_response()


@pytest.mark.trio
async def test_api_version_invalid_format(alice_http_client: AuthenticatedHttpApiClient):
    rep = await alice_http_client.send_dummy_request(
        extra_headers={"api_version": str(API_VERSION) + "-not-really-a-valid-api-version"},
        check_rep=False,
    )
    assert rep.status_code == 400  # bad request
