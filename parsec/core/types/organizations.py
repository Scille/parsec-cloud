# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import attr
from typing import Tuple

from parsec._parsec import UsersPerProfileDetailItem


@attr.s(frozen=True, slots=True, auto_attribs=True)
class OrganizationStats:
    users: int
    active_users: int
    realms: int
    data_size: int
    metadata_size: int
    users_per_profile_detail: Tuple[UsersPerProfileDetailItem]


@attr.s(frozen=True, slots=True, auto_attribs=True)
class OrganizationConfig:
    user_profile_outsider_allowed: bool
    active_users_limit: int | None
    # TODO: Should this be `SequesterAuthorityCertificate` instead of bytes?
    sequester_authority: bytes | None
    # TODO: Should this be `SequesterServiceCertificate` instead of bytes?
    sequester_services: Tuple[bytes, ...] | None

    @property
    def is_sequestered_organization(self) -> bool:
        return self.sequester_authority is not None
