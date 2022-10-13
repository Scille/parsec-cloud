# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations


from typing import Optional, Dict
import trio
import urllib.request


async def http_request(
    url: str,
    data: Optional[bytes] = None,
    headers: Dict[str, str] = {},
    method: Optional[str] = None,
) -> bytes:
    """Raises: urllib.error.URLError or OSError"""

    def _target() -> bytes:
        request = urllib.request.Request(url, data=data, headers=headers, method=method)

        with urllib.request.urlopen(request) as rep:
            return rep.read()

    return await trio.to_thread.run_sync(_target)
