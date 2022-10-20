# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Optional
from pypac.api import PACFile

import requests

def get_pac(
    url: Optional[str] = None,
    js: Optional[str] = None,
    from_dns: bool = True,
    timeout: int = 2,
    allowed_content_types: Optional[str] = None,
    sessions: Optional[requests.Session] = None,
) -> Optional[PACFile]: ...

class PACFile:
    def find_proxy_for_url(self, url: str, hostname: str) -> str: ...
