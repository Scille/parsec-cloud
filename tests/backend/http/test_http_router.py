# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from parsec.backend.http.router import (
    _http_404,
    _redirect_to_parsec,
    is_route,
    get_method,
    get_method_and_execute,
    get_404_method,
)


@pytest.mark.trio
async def test_is_route():
    # test is_method with existent route
    assert is_route(b"/api/redirect?organization_id=thisistheorganizationid123456789") is not None
    # test is_method with non existent route
    assert is_route(b"fakeroute") is None


@pytest.mark.trio
async def test_get_method():
    # test get_method with existent route
    method = get_method(b"/api/redirect")
    assert method == _redirect_to_parsec
    # test get_method with non existent route
    method = get_method(b"fakeroute")
    assert method == _http_404


@pytest.mark.trio
async def test_get_404_method():
    # test get_404_method with existent route
    method = get_404_method()
    assert method == _http_404


@pytest.mark.trio
async def test_get_method_and_execute():
    # test get_404_method with existent route
    url = b"/404"
    method = get_404_method()
    assert method() == get_method_and_execute(url, None)
