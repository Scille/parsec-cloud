# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations


from typing import Optional, Dict, Any
import trio
import urllib.request
import urllib.parse


async def http_request(
    url: str,
    url_params: Optional[Dict[str, Any]] = None,
    data: Optional[bytes] = None,
    headers: Optional[Dict[str, str]] = None,
    method: Optional[str] = None,
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
