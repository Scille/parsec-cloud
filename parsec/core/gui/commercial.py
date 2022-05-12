# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.core.types import BackendAddr

SAAS_HOSTNAMES = ["saas.parsec.cloud"]


def is_saas_addr(addr: BackendAddr) -> bool:
    return addr.hostname.lower() in SAAS_HOSTNAMES
