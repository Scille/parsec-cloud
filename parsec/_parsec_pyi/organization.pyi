from __future__ import annotations

from typing import Iterable

from parsec._parsec import UsersPerProfileDetailItem

class OrganizationStats:
    def __init__(
        self,
        users: int,
        active_users: int,
        realms: int,
        data_size: int,
        metadata_size: int,
        users_per_profile_detail: Iterable[UsersPerProfileDetailItem],
    ) -> None: ...
    @property
    def users(self) -> int: ...
    @property
    def active_users(self) -> int: ...
    @property
    def realms(self) -> int: ...
    @property
    def data_size(self) -> int: ...
    @property
    def metadata_size(self) -> int: ...
    @property
    def users_per_profile_detail(self) -> tuple[UsersPerProfileDetailItem, ...]: ...

class OrganizationConfig:
    def __init__(
        self,
        user_profile_outsider_allowed: bool,
        active_users_limit: int | None,
        sequester_authority: bytes | None,
        sequester_services: tuple[bytes, ...] | None,
    ) -> None: ...
    @property
    def user_profile_outsider_allowed(self) -> bool: ...
    @property
    def active_users_limit(self) -> int | None: ...
    @property
    def sequester_authority(self) -> bytes | None: ...
    @property
    def sequester_services(self) -> tuple[bytes, ...] | None: ...
    @property
    def is_sequestered_organization(self) -> bool: ...
