# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from unittest.mock import patch

from parsec.api.protocol import OrganizationID, VlobID, SequesterServiceID
from parsec.backend.sequester import (
    SequesterOrganizationNotFoundError,
    SequesterServiceNotFoundError,
    SequesterServiceType,
)

from tests.common import OrganizationFullData, customize_fixtures, sequester_service_factory
from tests.backend.common import vlob_create, vlob_update


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_vlob_create_update_and_sequester_access(
    coolorg: OrganizationFullData, alice_ws, realm, backend
):
    # s1&s2 are valid sequester services, s3 is a disabled sequester service
    s1 = sequester_service_factory(
        authority=coolorg.sequester_authority, label="Sequester service 1"
    )
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s1.backend_service
    )
    s2 = sequester_service_factory(
        authority=coolorg.sequester_authority, label="Sequester service 2"
    )
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s2.backend_service
    )
    s3 = sequester_service_factory(
        authority=coolorg.sequester_authority, label="Sequester service 3"
    )
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s3.backend_service
    )
    await backend.sequester.disable_service(
        organization_id=coolorg.organization_id, service_id=s3.service_id
    )

    vlob_id = VlobID.from_hex("00000000000000000000000000000001")
    dummy_service_id = SequesterServiceID.from_hex("0000000000000000000000000000000A")
    blob = b"<encrypted with workspace's key>"
    vlob_version = 0

    async def _test(vlob_cmd, **cmd_kwargs):
        nonlocal vlob_version
        vlob_version += 1
        b1 = f"<blob v{vlob_version} for s1>".encode()
        b2 = f"<blob v{vlob_version} for s2>".encode()
        b3 = f"<blob v{vlob_version} for s3>".encode()

        # 1) Try without sequester blob
        rep = await vlob_cmd(alice_ws, **cmd_kwargs, check_rep=False)
        assert rep == {
            "status": "sequester_inconsistency",
            "sequester_authority_certificate": coolorg.sequester_authority.certif,
            "sequester_services_certificates": [s1.certif, s2.certif],
        }

        # 2) Try with sequester blob missing for one service
        rep = await vlob_cmd(
            alice_ws, **cmd_kwargs, sequester_blob={s1.service_id: b1}, check_rep=False
        )
        assert rep == {
            "status": "sequester_inconsistency",
            "sequester_authority_certificate": coolorg.sequester_authority.certif,
            "sequester_services_certificates": [s1.certif, s2.certif],
        }

        # 3) Try with unknown additional sequester blob
        rep = await vlob_cmd(
            alice_ws,
            **cmd_kwargs,
            sequester_blob={s1.service_id: b1, s2.service_id: b2, dummy_service_id: b"<whatever>"},
            check_rep=False,
        )
        assert rep == {
            "status": "sequester_inconsistency",
            "sequester_authority_certificate": coolorg.sequester_authority.certif,
            "sequester_services_certificates": [s1.certif, s2.certif],
        }

        # 4) Try with blob for a removed sequester service
        rep = await vlob_cmd(
            alice_ws,
            **cmd_kwargs,
            sequester_blob={s1.service_id: b1, s2.service_id: b2, s3.service_id: b3},
            check_rep=False,
        )
        assert rep == {
            "status": "sequester_inconsistency",
            "sequester_authority_certificate": coolorg.sequester_authority.certif,
            "sequester_services_certificates": [s1.certif, s2.certif],
        }

        # 5) Finally the valid operation
        rep = await vlob_cmd(
            alice_ws,
            **cmd_kwargs,
            sequester_blob={s1.service_id: b1, s2.service_id: b2},
            check_rep=False,
        )
        assert rep == {"status": "ok"}

    # First test vlob create&update
    await _test(vlob_create, realm_id=realm, vlob_id=vlob_id, blob=blob)
    await _test(vlob_update, vlob_id=vlob_id, version=2, blob=blob)

    # Then test vlob access from sequester services

    # 1) Tests service 1 & 2

    realm_s1_dump = await backend.sequester.dump_realm(
        organization_id=coolorg.organization_id, service_id=s1.service_id, realm_id=realm
    )
    assert realm_s1_dump == [(vlob_id, 1, b"<blob v1 for s1>"), (vlob_id, 2, b"<blob v2 for s1>")]
    realm_s2_dump = await backend.sequester.dump_realm(
        organization_id=coolorg.organization_id, service_id=s2.service_id, realm_id=realm
    )
    assert realm_s2_dump == [(vlob_id, 1, b"<blob v1 for s2>"), (vlob_id, 2, b"<blob v2 for s2>")]

    # 2) Ensure service 3 is empty

    realm_s3_dump = await backend.sequester.dump_realm(
        organization_id=coolorg.organization_id, service_id=s3.service_id, realm_id=realm
    )
    assert realm_s3_dump == []

    # 3) Test various errors in sequester vlob acces

    # Unknown organization
    with pytest.raises(SequesterOrganizationNotFoundError):
        await backend.sequester.dump_realm(
            organization_id=OrganizationID("DummyOrg"), service_id=s1.service_id, realm_id=realm
        )

    # Unknown sequester service
    with pytest.raises(SequesterServiceNotFoundError):
        await backend.sequester.dump_realm(
            organization_id=coolorg.organization_id, service_id=dummy_service_id, realm_id=realm
        )


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_webhook_vlob_create_update(coolorg: OrganizationFullData, alice_ws, realm, backend):
    vlob_id = VlobID.from_hex("00000000000000000000000000000001")
    blob = b"<encrypted with workspace's key>"
    sequester_blob = b"<encrypted sequester blob>"

    with patch("parsec.backend.http_utils.urllib.request") as mock:

        # Register webhook service
        url = "http://somewhere.post"
        service = sequester_service_factory(
            "TestWebhookService",
            coolorg.sequester_authority,
            service_type=SequesterServiceType.WEBHOOK,
            webhook_url=url,
        )

        await backend.sequester.create_service(
            organization_id=coolorg.organization_id, service=service.backend_service
        )

        rep = await vlob_create(
            alice_ws,
            realm_id=realm,
            vlob_id=vlob_id,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert rep == {"status": "ok"}

        mock.Request.assert_called_once_with(url, data=sequester_blob, headers={}, method="POST")
        mock.reset_mock()

        sequester_blob = b"<another encrypted sequester blob>"
        rep = await vlob_update(
            alice_ws,
            vlob_id=vlob_id,
            version=2,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )

        assert rep == {"status": "ok"}
        mock.Request.assert_called_once_with(url, data=sequester_blob, headers={}, method="POST")
