# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.core.types import BackendAddrType

SAAS_HOSTNAMES = ["saas.parsec.cloud"]


def is_saas_addr(addr: BackendAddrType) -> bool:
    return addr.hostname.lower() in SAAS_HOSTNAMES
