# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from collections import namedtuple


class ApiVersion(namedtuple("ApiVersion", "version revision")):
    def __str__(self) -> str:
        return f"{self.version}.{self.revision}"


# API major versions:
# v1: Original API
# v2 (Parsec 1.14+): Incompatible hanshake with system with SAS-based authentication
# - v2.7 (Parsec +2.9): Add `organization_bootstrap` to anonymous commands
# - v2.8 (Parsec 2.11+): Sequester API
# v3 (Parsec 2.9+): Incompatible handshake challenge answer format
# - v3.1 (Parsec 2.10+): Add `user_revoked` return status to `realm_update_role` command
# - v3.2 (Parsec 2.11+): Sequester API
API_V1_VERSION = ApiVersion(version=1, revision=3)
API_V2_VERSION = ApiVersion(version=2, revision=8)
API_V3_VERSION = ApiVersion(version=3, revision=2)
API_VERSION = API_V3_VERSION
