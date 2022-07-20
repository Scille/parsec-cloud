# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from collections import namedtuple


class ApiVersion(namedtuple("ApiVersion", "version revision")):
    def __str__(self) -> str:
        return f"{self.version}.{self.revision}"


# API major versions:
# v1: Original API
# - v1.1
# v2 (Parsec 1.14+): Incompatible hanshake with system with SAS-based authentication
# - v2.8 (Parsec 2.11+): Sequester API
# v3 (Parsec 2.9+): Incompatible handshake challenge answer format
# - v3.1 (Parsec 2.11+): Sequester API
API_V1_VERSION = ApiVersion(version=1, revision=3)
API_V2_VERSION = ApiVersion(version=2, revision=7)
API_V3_VERSION = ApiVersion(version=3, revision=0)
API_VERSION = API_V3_VERSION
