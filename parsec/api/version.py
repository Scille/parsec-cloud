# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import NamedTuple


class ApiVersion(NamedTuple):
    version: int
    revision: int

    def __str__(self) -> str:
        return f"{self.version}.{self.revision}"

    @classmethod
    def from_str(cls, value: str) -> "ApiVersion":
        """
        Raises: ValueError
        """
        raw_version, raw_revision = value.split(".")
        version, revision = int(raw_version), int(raw_revision)
        return cls(version, revision)


# API major versions:
# v1: Original API
# v2 (Parsec 1.14+): Incompatible handshake with system with SAS-based authentication
# - v2.7 (Parsec +2.9): Add `organization_bootstrap` to anonymous commands
# - v2.8 (Parsec 2.11+): Sequester API
# - v2.9 (Parsec 2.13+): Add `sequester_initial_services_certificate` field to `organization_bootstrap` command
# v3 (Parsec 2.9+): Incompatible handshake challenge answer format
# - v3.1 (Parsec 2.10+): Add `user_revoked` return status to `realm_update_role` command
# - v3.2 (Parsec 2.11+): Sequester API
# - v3.3 (Parsec 2.13+): Add `sequester_initial_services_certificate` field to `organization_bootstrap` command
API_V1_VERSION = ApiVersion(version=1, revision=3)
API_V2_VERSION = ApiVersion(version=2, revision=9)
API_V3_VERSION = ApiVersion(version=3, revision=3)
API_VERSION = API_V3_VERSION
