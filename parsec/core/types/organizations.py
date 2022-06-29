# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr
from typing import Optional, List, Tuple

from parsec.api.protocol import UserProfile
from parsec.api.data import SequesterAuthorityCertificate, SequesterServiceCertificate


@attr.s(frozen=True, slots=True, auto_attribs=True)
class UsersPerProfileDetailItem:
    profile: UserProfile
    active: int
    revoked: int


@attr.s(frozen=True, slots=True, auto_attribs=True)
class OrganizationStats:
    users: int
    active_users: int
    realms: int
    data_size: int
    metadata_size: int
    users_per_profile_detail: List[UsersPerProfileDetailItem]


@attr.s(frozen=True, slots=True, auto_attribs=True)
class OrganizationConfig:
    user_profile_outsider_allowed: bool
    active_users_limit: Optional[int]
    sequester_authority: Optional[SequesterAuthorityCertificate]
    sequester_services: Optional[Tuple[SequesterServiceCertificate, ...]]

    @property
    def is_sequestered_organization(self):
        return self.sequester_authority is not None
