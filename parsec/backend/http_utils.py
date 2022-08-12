# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
from typing import Optional, Dict
import trio
import ssl
import certifi
import urllib.request

from parsec.core.backend_connection.proxy import blocking_io_get_proxy


# TODO : MOVE IT
async def http_request(
    url: str,
    data: Optional[bytes] = None,
    headers: Dict[str, str] = {},
    method: Optional[str] = None,
) -> bytes:
    """Raises: urllib.error.URLError or OSError"""

    def _target():
        request = urllib.request.Request(url, data=data, headers=headers, method=method)
        handlers = []
        proxy_url = blocking_io_get_proxy(target_url=url, hostname=request.host)
        if proxy_url:
            handlers.append(urllib.request.ProxyHandler(proxies={request.type: proxy_url}))
        else:
            # Prevent urllib from trying to handle proxy configuration by itself.
            # We use `blocking_io_get_proxy` to give us the proxy to use
            # (or, in our case, the fact we should not use proxy).
            # However this information may come from a weird place (e.g. PAC config)
            # that urllib is not aware of, so urllib may use an invalid configuration
            # (e.g. using HTTP_PROXY env var) if we let it handle proxy.
            handlers.append(urllib.request.ProxyHandler(proxies={}))

        if request.type == "https":
            context = ssl.create_default_context(cafile=certifi.where())
            # Also provide custom certificates if any
            cafile = os.environ.get("SSL_CAFILE")
            if cafile:
                context.load_verify_locations(cafile)
            handlers.append(urllib.request.HTTPSHandler(context=context))

        with urllib.request.build_opener(*handlers).open(request) as rep:
            return rep.read()

    return await trio.to_thread.run_sync(_target)
