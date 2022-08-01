# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.core.types import BackendAddr

SAAS_HOSTNAMES = ["saas.parsec.cloud"]


def is_saas_addr(addr: BackendAddr) -> bool:
    return addr.hostname.lower() in SAAS_HOSTNAMES
