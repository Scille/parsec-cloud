# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from typing import Optional

from parsec._parsec import (
    DateTime,
    DeviceID,
    RealmRole,
    RealmRoleCertificate,
    UserID,
    VlobID,
)


def patch_realm_role_certificate(
    certif: RealmRoleCertificate,
    author: Optional[DeviceID] = None,
    timestamp: Optional[DateTime] = None,
    realm_id: Optional[VlobID] = None,
    user_id: Optional[UserID] = None,
    role: Optional[RealmRole] = None,
    force_no_role: bool = False,
) -> RealmRoleCertificate:
    """Utility function to patch one or more RealmRoleCertificate fields"""
    return RealmRoleCertificate(
        author=author or certif.author,
        timestamp=timestamp or certif.timestamp,
        realm_id=realm_id or certif.realm_id,
        user_id=user_id or certif.user_id,
        role=None if force_no_role else (role or certif.role),
    )
