# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations


from typing import Any, Dict
import trio
import urllib.request
import urllib.parse


async def http_request(
    url: str,
    url_params: Dict[str, Any] | None = None,
    data: bytes | None = None,
    headers: Dict[str, str] | None = None,
    method: str | None = None,
) -> bytes:
    """Raises: urllib.error.URLError or OSError"""

    if url_params:
        params = urllib.parse.urlencode(url_params)
        if params:
            url = f"{url}?{params}"

    def _target() -> bytes:
        request = urllib.request.Request(url, data=data, headers=headers or {}, method=method)
        with urllib.request.urlopen(request) as rep:
            return rep.read()

    return await trio.to_thread.run_sync(_target)
