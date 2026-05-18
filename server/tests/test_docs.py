# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest
from httpx import AsyncClient


@pytest.mark.parametrize("which_docs", ("docs", "redoc", "openapi.json"))
async def test_get_docs(which_docs: str, client: AsyncClient):
    rep = await client.get(f"http://parsec.invalid/{which_docs}")
    assert rep.status_code == 200
