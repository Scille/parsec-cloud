# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx


async def test_main_page(client: httpx.AsyncClient) -> None:
    response = await client.get("http://parsec.invalid/")
    assert response.status_code == 200
    assert "Parsec server is running!" in response.text


async def test_404(client: httpx.AsyncClient) -> None:
    response = await client.get("http://parsec.invalid/unknown")
    assert response.status_code == 404
    assert "The page you requested does not exist." in response.text


async def test_static(client: httpx.AsyncClient) -> None:
    response = await client.get("http://parsec.invalid/static/favicon.ico")
    assert response.status_code == 200
    assert response.content.startswith(b"\x89PNG")


async def test_static_404(client: httpx.AsyncClient) -> None:
    response = await client.get("http://parsec.invalid/static/unknown")
    assert response.status_code == 404
    assert response.json() == {"detail": "Not Found"}
