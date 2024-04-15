# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncpg

from parsec._parsec import OrganizationID, RealmRoleCertificate, RealmRole
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_organization_internal_id,
    q_user_internal_id,
    query,
)
from parsec.components.realm import CertificateBasedActionIdempotentOutcome


_q_insert_realm_encryption_revision = Q(
    """
INSERT INTO vlob_encryption_revision (
    realm,
    encryption_revision
)
SELECT
    $_id,
    1
"""
)
