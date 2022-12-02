# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

def use_openssl(
    libcrypto_path: str,
    libssl_path: str,
    trust_list_path: str | None = None,
) -> None: ...
