# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest


@pytest.mark.trio
async def test_unauthorized_client(backend_asgi_app):
    client = backend_asgi_app.test_client()  # This client has no token
    rep = await client.get("/administration/stats")
    assert rep.status == "403 FORBIDDEN"


@pytest.mark.trio
async def test_bad_requests(backend_asgi_app):
    client = backend_asgi_app.test_client()  # This client has no token
    HEADERS = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    # No arguments in query string
    rep = await client.get("/administration/stats", headers=HEADERS)
    assert rep.status == "400 BAD REQUEST"

    # Missing format
    rep = await client.get("/administration/stats?from=2020-01-01&to=2021-01-01", headers=HEADERS)
    assert rep.status == "400 BAD REQUEST"

    # Missing from
    rep = await client.get("/administration/stats?format=csv&to=2021-01-01", headers=HEADERS)

    assert rep.status == "400 BAD REQUEST"


@pytest.mark.trio
async def test_bad_time_range(backend_asgi_app):
    client = backend_asgi_app.test_client()  # This client has no token
    HEADERS = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    rep = await client.get("/administration/stats?from=dummy", headers=HEADERS)
    assert rep.status == "400 BAD REQUEST"

    # Invalid time range from > to
    rep = await client.get(
        "/administration/stats?format=csv&from=2022-01-01&to=2000-01-01", headers=HEADERS
    )
    assert rep.status == "400 BAD REQUEST"
