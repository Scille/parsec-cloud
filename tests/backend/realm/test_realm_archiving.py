# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    DateTime,
    LocalDevice,
    RealmArchivingCertificate,
    RealmArchivingConfiguration,
    RealmID,
)


@pytest.mark.trio
async def test_create_realm_archiving_certificate(
    alice: LocalDevice,
    realm: RealmID,
):
    now = DateTime.now()

    available = RealmArchivingConfiguration.available()
    assert available.str == "AVAILABLE"

    archived = RealmArchivingConfiguration.archived()
    assert archived.str == "ARCHIVED"

    deletion_planned = RealmArchivingConfiguration.deletion_planned(now.add(days=30))
    assert deletion_planned.str == "DELETION_PLANNED"
    assert deletion_planned.deletion_date == now.add(days=30)

    for config in (available, archived, deletion_planned):
        certificate = RealmArchivingCertificate(
            author=alice.device_id,
            timestamp=now,
            realm_id=realm,
            configuration=config,
        )
        assert certificate.author == alice.device_id
        assert certificate.timestamp == now
        assert certificate.realm_id == realm
        assert certificate.configuration == config

        signed = certificate.dump_and_sign(alice.signing_key)
        unsecure_certificate = RealmArchivingCertificate.unsecure_load(signed)
        assert unsecure_certificate == certificate
        verified_certificate = RealmArchivingCertificate.verify_and_load(
            signed,
            alice.verify_key,
            expected_author=alice.device_id,
            expected_realm=realm,
        )
        breakpoint()
        assert verified_certificate == certificate
