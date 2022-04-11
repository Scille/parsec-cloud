# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.core.types import BackendAddr

SAAS_HOSTNAME = "saas.parsec.cloud"
SAAS_UPDATE_SUBSCRIPTION_URL = "https://parsec.cloud/comment-changer-dabonnement-parsec/"


def is_saas_addr(addr: BackendAddr) -> bool:
    return addr.hostname.lower() == SAAS_HOSTNAME
