# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any

import requests

def get_pac(
    url: str | None = None,
    js: str | None = None,
    from_dns: bool = True,
    timeout: int = 2,
    allowed_content_types: set[str] | None = None,
    sessions: requests.Session | None = None,
) -> PACFile | None: ...

class PACFile:
    def find_proxy_for_url(self, url: str, hostname: str) -> Any: ...
