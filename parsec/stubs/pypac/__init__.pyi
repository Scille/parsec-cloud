# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Optional, Any

# import requests

def get_pac(
    url: Optional[str] = None,
    js: Optional[str] = None,
    from_dns: bool = True,
    timeout: int = 2,
    allowed_content_types: Optional[set[str]] = None,
    # sessions: Optional[requests.Session] = None,
) -> Optional[PACFile]: ...

class PACFile:
    def find_proxy_for_url(self, url: str, hostname: str) -> Any: ...
