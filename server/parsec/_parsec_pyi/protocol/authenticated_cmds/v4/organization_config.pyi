# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import ActiveUsersLimit

class Req:
    def __init__(
        self,
    ) -> None: ...
    def dump(self) -> bytes: ...

class Rep:
    @staticmethod
    def load(raw: bytes) -> Rep: ...
    def dump(self) -> bytes: ...

class RepUnknownStatus(Rep):
    def __init__(self, status: str, reason: str | None) -> None: ...
    @property
    def status(self) -> str: ...
    @property
    def reason(self) -> str | None: ...

class RepOk(Rep):
    def __init__(
        self,
        user_profile_outsider_allowed: bool,
        active_users_limit: ActiveUsersLimit,
        sequester_authority_certificate: bytes | None,
        sequester_services_certificates: list[bytes] | None,
    ) -> None: ...
    @property
    def user_profile_outsider_allowed(self) -> bool: ...
    @property
    def active_users_limit(self) -> ActiveUsersLimit: ...
    @property
    def sequester_authority_certificate(self) -> bytes | None: ...
    @property
    def sequester_services_certificates(self) -> list[bytes] | None: ...

class RepNotFound(Rep):
    def __init__(
        self,
    ) -> None: ...
