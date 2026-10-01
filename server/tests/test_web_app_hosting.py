# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from collections.abc import AsyncIterator
from contextlib import asynccontextmanager
from functools import partial
from pathlib import Path
from unittest.mock import ANY

import pytest
from httpx import ASGITransport, AsyncClient, Response

from parsec._parsec import ParsecAddr
from parsec.asgi import app_factory
from parsec.backend import Backend
from tests.common.backend import SERVER_DOMAIN

MAIN_SERVER_DOMAIN = SERVER_DOMAIN
EDITICS_SERVER_DOMAIN = "editics." + SERVER_DOMAIN

MAIN_INDEX_HTML = "<html><body>Hello, I'm the main web app !</body></html>"
EDITICS_OFFLINE_HTML = "<html><body>Hello, I'm the editics offline page !</body></html>"
ASSETS_STYLE_CSS = "body { color: red }"

STATIC_BASE_CSS = "static/base-jFjh9D00.css"
HTML_CONTENT_TYPE = "text/html; charset=utf-8"


@pytest.fixture
def web_app_dir(tmp_path: Path) -> Path:
    # We don't use the real web app content here (it would require building
    # the Parsec GUI, which is not possible in CI where the client and server
    # tests run isolated from each other)
    (tmp_path / "index.html").write_text(MAIN_INDEX_HTML)
    (tmp_path / "assets").mkdir()
    (tmp_path / "assets" / "style.css").write_text(ASSETS_STYLE_CSS)
    (tmp_path / "editics").mkdir()
    (tmp_path / "editics" / "offline-abc123456.html").write_text(EDITICS_OFFLINE_HTML)
    return tmp_path


@asynccontextmanager
async def web_app_client_factory(backend: Backend, web_app_dir: Path) -> AsyncIterator[AsyncClient]:
    app = app_factory(backend, with_client_web_app=web_app_dir)
    async with AsyncClient(transport=ASGITransport(app)) as client:
        yield client


async def assert_route(
    client: AsyncClient,
    domain: str,
    route: str,
    expected: tuple[int, str, str] | int,
    expected_cached: bool = False,
) -> Response:
    rep = await client.get(f"http://{domain}{route}")
    match expected:
        case int() as expected_status_code:
            assert rep.status_code == expected_status_code
        case (expected_status_code, expected_content_type, expected_content):
            assert rep.status_code == expected_status_code
            assert rep.headers["content-type"] == expected_content_type
            assert rep.text == expected_content
    if expected_cached:
        assert rep.headers["Cache-Control"] == "max-age=31536000, public, immutable"
    else:
        assert "Cache-Control" not in rep.headers
    return rep


@pytest.mark.parametrize("editics_server_addr_set", (True, False))
@pytest.mark.asyncio
async def test_web_app_served(backend: Backend, web_app_dir: Path, editics_server_addr_set: bool):
    if editics_server_addr_set:
        backend.config.editics_server_addr = ParsecAddr(
            hostname=EDITICS_SERVER_DOMAIN, port=None, use_ssl=True
        )

    async with web_app_client_factory(backend, web_app_dir) as client:
        main_assert_route = partial(assert_route, client, MAIN_SERVER_DOMAIN)

        async def editics_assert_route(*args, **kwargs):
            if not editics_server_addr_set:
                # If the server doesn't know the editics domain, then a request
                # coming from there is equivalent to any other domain name, including
                # the main server domain!
                return
            return await assert_route(client, EDITICS_SERVER_DOMAIN, *args, **kwargs)

        rep = await main_assert_route("/", 301)
        assert rep.headers["location"] == "/client/"
        # Editics domain is redirected to a route that leads to a 404, this is a small acceptable quirk
        rep = await editics_assert_route("/", 301)
        assert not rep or rep.headers["location"] == "/client/"

        await main_assert_route("/client/", (200, "text/html; charset=utf-8", MAIN_INDEX_HTML))
        await editics_assert_route("/client/", 403)

        await main_assert_route(
            "/client/assets/style.css",
            (200, "text/css; charset=utf-8", ASSETS_STYLE_CSS),
            expected_cached=True,
        )
        await editics_assert_route("/client/assets/style.css", 403)

        await main_assert_route("/client/editics/offline-abc123456.html", 403)
        await editics_assert_route(
            "/client/editics/offline-abc123456.html",
            (200, "text/html; charset=utf-8", EDITICS_OFFLINE_HTML),
            expected_cached=True,
        )

        await main_assert_route("/client/editics/", 403)
        # Editics directory is not a valid route anyway
        await editics_assert_route("/client/editics/", 404)


@pytest.mark.asyncio
async def test_web_app_not_served(client: AsyncClient):
    main_assert_route = partial(assert_route, client, MAIN_SERVER_DOMAIN)
    editics_assert_route = partial(assert_route, client, EDITICS_SERVER_DOMAIN)

    # Server landing page
    await main_assert_route("/", (200, "text/html; charset=utf-8", ANY))
    await editics_assert_route("/", 200)

    for bad_route in (
        "/client/",
        "/client/index.html",
        "/client/assets/style.css",
        "/client/editics/offline-abc123456.html",
    ):
        await main_assert_route(bad_route, 404)
        await editics_assert_route(bad_route, 404)
