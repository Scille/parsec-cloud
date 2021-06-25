# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS
import attr
from pendulum import DateTime
from typing import Dict, Counter


@attr.s(frozen=True, slots=True, auto_attribs=True)
class OrganizationStats:
    users: int
    active_users: int
    data_size: int
    metadata_size: int
    users_per_profile_detail: Dict[str, Counter]


@attr.s(frozen=True, slots=True, auto_attribs=True)
class OrganizationConfig:
    expiration_date: DateTime
    user_profile_outsider_allowed: bool
