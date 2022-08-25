# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import base64
import json
import pytest
from unittest.mock import patch, Mock

import urllib

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

    # Webhook specific errors are tested in other test
    s4 = sequester_service_factory(
        authority=coolorg.sequester_authority,
        label="Sequester webhook service",
        service_type=SequesterServiceType.WEBHOOK,
        webhook_url="http://somewhere.post",
    )
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s4.backend_service
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
        b4 = f"<blob v{vlob_version} for s4>".encode()

        # 1) Try without sequester blob
        rep = await vlob_cmd(alice_ws, **cmd_kwargs, check_rep=False)
        assert rep == {
            "status": "sequester_inconsistency",
            "sequester_authority_certificate": coolorg.sequester_authority.certif,
            "sequester_services_certificates": [s1.certif, s2.certif, s4.certif],
        }

        # 2) Try with sequester blob missing for one service
        rep = await vlob_cmd(
            alice_ws, **cmd_kwargs, sequester_blob={s1.service_id: b1}, check_rep=False
        )
        assert rep == {
            "status": "sequester_inconsistency",
            "sequester_authority_certificate": coolorg.sequester_authority.certif,
            "sequester_services_certificates": [s1.certif, s2.certif, s4.certif],
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
            "sequester_services_certificates": [s1.certif, s2.certif, s4.certif],
        }

        # 4) Try with blob for a removed sequester service
        rep = await vlob_cmd(
            alice_ws,
            **cmd_kwargs,
            sequester_blob={
                s1.service_id: b1,
                s2.service_id: b2,
                s3.service_id: b3,
                s4.service_id: b4,
            },
            check_rep=False,
        )
        assert rep == {
            "status": "sequester_inconsistency",
            "sequester_authority_certificate": coolorg.sequester_authority.certif,
            "sequester_services_certificates": [s1.certif, s2.certif, s4.certif],
        }

        # 5) Finally the valid operation
        rep = await vlob_cmd(
            alice_ws,
            **cmd_kwargs,
            sequester_blob={s1.service_id: b1, s2.service_id: b2, s4.service_id: b4},
            check_rep=False,
        )
        assert rep == {"status": "ok"}

    # First test vlob create&update

    with patch("parsec.backend.http_utils.urllib.request") as mock:
        await _test(vlob_create, realm_id=realm, vlob_id=vlob_id, blob=blob)
        await _test(vlob_update, vlob_id=vlob_id, version=2, blob=blob)
        mock.Request.assert_called()

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


async def _register_service_and_create_vlob(
    coolorg, backend, alice_ws, realm, vlob_id, blob, sequester_blob, url
):
    # Register webhook service
    service = sequester_service_factory(
        "TestWebhookService",
        coolorg.sequester_authority,
        service_type=SequesterServiceType.WEBHOOK,
        webhook_url=url,
    )
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=service.backend_service
    )

    # Create one vlob
    rep = await vlob_create(
        alice_ws,
        realm_id=realm,
        vlob_id=vlob_id,
        blob=blob,
        sequester_blob={service.service_id: sequester_blob},
        check_rep=False,
    )

    assert rep == {"status": "ok"}

    return service


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_webhook_vlob_create_update(
    coolorg: OrganizationFullData, alice, alice_ws, realm, backend
):
    vlob_id = VlobID.from_hex("00000000000000000000000000000001")
    blob = b"<encrypted with workspace's key>"
    sequester_blob = b"<encrypted sequester blob>"

    url = "http://somewhere.post"

    with patch("parsec.backend.http_utils.urllib.request") as mock:
        # Helper
        def _assert_webhook_posted(expected_sequester_data):
            mock.Request.assert_called_once()
            # Extract args
            args, kwargs = mock.Request.call_args
            assert url in args
            assert kwargs["method"] == "POST"
            # Extract http data
            row_data = kwargs["data"]
            posted_data = urllib.parse.parse_qs(row_data.decode())
            assert coolorg.organization_id.str == posted_data["organization_id"][0]
            assert alice.device_id.str == posted_data["author"][0]
            assert str(vlob_id) == posted_data["vlob_id"][0]
            assert expected_sequester_data == base64.b64decode(posted_data["sequester_blob"][0])
            # Reset
            mock.reset_mock()

        # Register webhook service
        service = await _register_service_and_create_vlob(
            coolorg, backend, alice_ws, realm, vlob_id, blob, sequester_blob, url
        )

        # Vlob has been created, assert that data have been posted
        _assert_webhook_posted(sequester_blob)

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
        _assert_webhook_posted(sequester_blob)


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_webhook_errors(coolorg: OrganizationFullData, alice_ws, realm, backend):
    vlob_id = VlobID.from_hex("00000000000000000000000000000001")
    blob = b"<encrypted with workspace's key>"
    sequester_blob = b"<encrypted sequester blob>"

    url = "http://somewhere.post"

    with patch("parsec.backend.http_utils.urllib.request") as mock:
        service = await _register_service_and_create_vlob(
            coolorg, backend, alice_ws, realm, vlob_id, blob, sequester_blob, url
        )

        new_vlob_id = VlobID.from_hex("00000000000000000000000000000002")

        # Test htttURLErro
        def raise_urlerror(*args, **kwargs):
            raise urllib.error.URLError(reason="CONNECTION REFUSED")

        mock.build_opener.side_effect = raise_urlerror
        rep = await vlob_create(
            alice_ws,
            vlob_id=new_vlob_id,
            realm_id=realm,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert rep["status"] == "sequester_webhook_failed"

        rep = await vlob_update(
            alice_ws,
            vlob_id=vlob_id,
            version=2,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert rep["status"] == "sequester_webhook_failed"

        # Test httperror
        def raise_httperror(*args, **kwargs):
            raise urllib.error.HTTPError(url, 405, "METHOD NOT ALLOWED", None, None)

        mock.build_opener.side_effect = raise_httperror
        rep = await vlob_create(
            alice_ws,
            vlob_id=new_vlob_id,
            realm_id=realm,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert rep["status"] == "sequester_rejected"
        assert rep["service_label"] == service.backend_service.service_label
        assert rep["service_id"] == service.service_id
        assert rep["service_error"] == "405:METHOD NOT ALLOWED"

        rep = await vlob_update(
            alice_ws,
            vlob_id=vlob_id,
            version=2,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert rep["status"] == "sequester_rejected"
        assert rep["service_label"] == service.backend_service.service_label
        assert rep["service_id"] == service.service_id
        assert rep["service_error"] == "405:METHOD NOT ALLOWED"

        # Test error from service

        def raise_httperror_400(*args, **kwargs):
            fp = Mock()
            fp.read.return_value = json.dumps({"error": "some_error_from_service"})
            raise urllib.error.HTTPError(url, 400, "", None, fp)

        mock.build_opener.side_effect = raise_httperror_400
        rep = await vlob_create(
            alice_ws,
            vlob_id=new_vlob_id,
            realm_id=realm,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert rep["status"] == "sequester_rejected"
        assert rep["service_label"] == service.backend_service.service_label
        assert rep["service_id"] == service.service_id
        assert rep["service_error"] == "some_error_from_service"

        rep = await vlob_update(
            alice_ws,
            vlob_id=vlob_id,
            version=2,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert rep["status"] == "sequester_rejected"
        assert rep["service_label"] == service.backend_service.service_label
        assert rep["service_id"] == service.service_id
        assert rep["service_error"] == "some_error_from_service"


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_missing_webhook_url(coolorg: OrganizationFullData, alice_ws, realm, backend):
    vlob_id = VlobID.from_hex("00000000000000000000000000000001")
    blob = b"<encrypted with workspace's key>"
    sequester_blob = b"<encrypted sequester blob>"

    # Register service without url webhook
    broken_service = sequester_service_factory(
        "BrokenService",
        coolorg.sequester_authority,
        service_type=SequesterServiceType.WEBHOOK,
        webhook_url=None,
    )

    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=broken_service.backend_service
    )

    with patch("parsec.backend.http_utils.urllib.request"):
        rep = await vlob_create(
            alice_ws,
            realm_id=realm,
            vlob_id=vlob_id,
            blob=blob,
            check_rep=False,
            sequester_blob={
                broken_service.service_id: sequester_blob,
            },
        )
        assert rep["status"] == "sequester_webhook_failed"


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_sequester_dump_realm(
    coolorg: OrganizationFullData, alice_ws, bob_ws, realm, backend
):
    vlob_id = VlobID.from_hex("00000000000000000000000000000001")
    blob = b"<encrypted with workspace's key>"
    sequester_blob = b"<encrypted sequester blob>"

    # Create and update vlob without sequester
    await vlob_create(
        alice_ws, realm_id=realm, vlob_id=vlob_id, blob=blob, sequester_blob={}, check_rep=True
    )
    await vlob_update(
        alice_ws, version=2, vlob_id=vlob_id, sequester_blob={}, blob=blob, check_rep=True
    )

    # Create sequester service
    s1 = sequester_service_factory(
        authority=coolorg.sequester_authority, label="Sequester service 1"
    )

    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s1.backend_service
    )

    # Create updates
    await vlob_update(
        alice_ws,
        version=3,
        vlob_id=vlob_id,
        blob=blob,
        sequester_blob={s1.service_id: sequester_blob},
        check_rep=True,
    )
    await vlob_update(
        alice_ws,
        version=4,
        vlob_id=vlob_id,
        blob=blob,
        sequester_blob={s1.service_id: sequester_blob},
        check_rep=True,
    )
    # Dump realm
    dump = await backend.sequester.dump_realm(
        organization_id=coolorg.organization_id, service_id=s1.service_id, realm_id=realm
    )

    assert dump == [(vlob_id, 3, sequester_blob), (vlob_id, 4, sequester_blob)]

    # Create another vlob
    another_vlob_id = VlobID.from_hex("00000000000000000000000000000002")
    another_sequester_blob = b"<encrypted sequester blob 2>"

    await vlob_create(
        alice_ws,
        realm_id=realm,
        vlob_id=another_vlob_id,
        blob=blob,
        sequester_blob={s1.service_id: another_sequester_blob},
        check_rep=True,
    )
    await vlob_update(
        alice_ws,
        version=2,
        vlob_id=another_vlob_id,
        sequester_blob={s1.service_id: another_sequester_blob},
        blob=blob,
        check_rep=True,
    )

    dump = await backend.sequester.dump_realm(
        organization_id=coolorg.organization_id, service_id=s1.service_id, realm_id=realm
    )

    assert dump == [
        (vlob_id, 3, sequester_blob),
        (vlob_id, 4, sequester_blob),
        (another_vlob_id, 1, another_sequester_blob),
        (another_vlob_id, 2, another_sequester_blob),
    ]
