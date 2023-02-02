# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import UserProfile

class UsersPerProfileDetailItem:
    def __init__(self, profile: UserProfile, active: int, revoked: int) -> None: ...
    @property
    def profile(self) -> UserProfile: ...
    @property
    def active(self) -> int: ...
    @property
    def revoked(self) -> int: ...
