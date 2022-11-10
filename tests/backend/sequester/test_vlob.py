# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import json
import pytest
from unittest.mock import patch, Mock

import urllib.error
from urllib.parse import urlsplit, parse_qs
from parsec._parsec import (
    VlobCreateRepOk,
    VlobCreateRepSequesterInconsistency,
    VlobCreateRepRejectedBySequesterService,
    VlobCreateRepTimeout,
    VlobUpdateRepOk,
    VlobUpdateRepSequesterInconsistency,
    VlobUpdateRepRejectedBySequesterService,
    VlobUpdateRepTimeout,
)
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

    vlob_id = VlobID.from_str("00000000000000000000000000000001")
    dummy_service_id = SequesterServiceID.from_str("0000000000000000000000000000000A")
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

        # vlob_cmd can be create or update so we test field
        assert isinstance(
            rep, (VlobCreateRepSequesterInconsistency, VlobUpdateRepSequesterInconsistency)
        )
        assert rep.sequester_authority_certificate == coolorg.sequester_authority.certif
        assert rep.sequester_services_certificates == (s1.certif, s2.certif, s4.certif)

        # 2) Try with sequester blob missing for one service
        rep = await vlob_cmd(
            alice_ws, **cmd_kwargs, sequester_blob={s1.service_id: b1}, check_rep=False
        )

        assert isinstance(
            rep, (VlobCreateRepSequesterInconsistency, VlobUpdateRepSequesterInconsistency)
        )
        assert rep.sequester_authority_certificate == coolorg.sequester_authority.certif
        assert rep.sequester_services_certificates == (s1.certif, s2.certif, s4.certif)

        # 3) Try with unknown additional sequester blob
        rep = await vlob_cmd(
            alice_ws,
            **cmd_kwargs,
            sequester_blob={s1.service_id: b1, s2.service_id: b2, dummy_service_id: b"<whatever>"},
            check_rep=False,
        )

        assert isinstance(
            rep, (VlobCreateRepSequesterInconsistency, VlobUpdateRepSequesterInconsistency)
        )
        assert rep.sequester_authority_certificate == coolorg.sequester_authority.certif
        assert rep.sequester_services_certificates == (s1.certif, s2.certif, s4.certif)

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

        assert isinstance(
            rep, (VlobCreateRepSequesterInconsistency, VlobUpdateRepSequesterInconsistency)
        )
        assert rep.sequester_authority_certificate == coolorg.sequester_authority.certif
        assert rep.sequester_services_certificates == (s1.certif, s2.certif, s4.certif)

        # 5) Finally the valid operation
        rep = await vlob_cmd(
            alice_ws,
            **cmd_kwargs,
            sequester_blob={s1.service_id: b1, s2.service_id: b2, s4.service_id: b4},
            check_rep=False,
        )
        assert isinstance(rep, (VlobCreateRepOk, VlobUpdateRepOk))

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

    assert isinstance(rep, VlobCreateRepOk)

    return service


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_webhook_vlob_create_update(
    coolorg: OrganizationFullData, alice, alice_ws, realm, backend
):
    vlob_id = VlobID.from_str("00000000000000000000000000000001")
    blob = b"<encrypted with workspace's key>"
    sequester_blob = b"<encrypted sequester blob>"

    url = "http://somewhere.post"

    with patch("parsec.backend.http_utils.urllib.request") as mock:
        # Register webhook service
        service = await _register_service_and_create_vlob(
            coolorg, backend, alice_ws, realm, vlob_id, blob, sequester_blob, url
        )

        # Helper
        def _assert_webhook_posted(expected_sequester_data):
            mock.Request.assert_called_once()
            # Extract args
            args, kwargs = mock.Request.call_args
            assert args[0].startswith(url)
            assert kwargs["method"] == "POST"
            # Extract url params
            params = parse_qs(urlsplit(args[0]).query)
            assert coolorg.organization_id.str == params["organization_id"][0]
            assert service.service_id.str == params["service_id"][0]
            # Extract http data
            assert expected_sequester_data == kwargs["data"]
            # Reset
            mock.reset_mock()

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

        assert isinstance(rep, VlobUpdateRepOk)
        _assert_webhook_posted(sequester_blob)


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_webhook_errors(caplog, coolorg: OrganizationFullData, alice_ws, realm, backend):
    vlob_id = VlobID.from_str("00000000000000000000000000000001")
    blob = b"<encrypted with workspace's key>"
    sequester_blob = b"<encrypted sequester blob>"

    url = "http://somewhere.post"

    with patch("parsec.backend.http_utils.urllib.request") as mock:
        service = await _register_service_and_create_vlob(
            coolorg, backend, alice_ws, realm, vlob_id, blob, sequester_blob, url
        )

        new_vlob_id = VlobID.from_str("00000000000000000000000000000002")

        # Test htttURLError
        def raise_urlerror(*args, **kwargs):
            raise urllib.error.URLError(reason="CONNECTION REFUSED")

        mock.urlopen.side_effect = raise_urlerror
        rep = await vlob_create(
            alice_ws,
            vlob_id=new_vlob_id,
            realm_id=realm,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert isinstance(rep, VlobCreateRepTimeout)
        caplog.assert_occured_once(
            f"[warning  ] Cannot reach webhook server    [parsec.backend.vlob] service_id={service.service_id.str} service_label=TestWebhookService"
        )
        caplog.clear()

        rep = await vlob_update(
            alice_ws,
            vlob_id=vlob_id,
            version=2,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert isinstance(rep, VlobUpdateRepTimeout)
        caplog.assert_occured_once(
            f"[warning  ] Cannot reach webhook server    [parsec.backend.vlob] service_id={service.service_id.str} service_label=TestWebhookService"
        )
        caplog.clear()

        # Test httperror
        def raise_httperror(*args, **kwargs):
            fp = Mock()
            raise urllib.error.HTTPError(url, 405, "METHOD NOT ALLOWED", None, fp)

        mock.urlopen.side_effect = raise_httperror
        rep = await vlob_create(
            alice_ws,
            vlob_id=new_vlob_id,
            realm_id=realm,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert isinstance(rep, VlobCreateRepTimeout)
        caplog.assert_occured_once(
            f"[warning  ] Invalid HTTP status returned by webhook [parsec.backend.vlob] service_id={service.service_id.str} service_label=TestWebhookService status=405"
        )
        caplog.clear()

        rep = await vlob_update(
            alice_ws,
            vlob_id=vlob_id,
            version=2,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        assert isinstance(rep, VlobUpdateRepTimeout)
        caplog.assert_occured_once(
            f"[warning  ] Invalid HTTP status returned by webhook [parsec.backend.vlob] service_id={service.service_id.str} service_label=TestWebhookService status=405"
        )
        caplog.clear()

        # Test error from service

        def raise_httperror_400(*args, **kwargs):
            fp = Mock()
            fp.read.return_value = json.dumps({"reason": "some_error_from_service"})
            raise urllib.error.HTTPError(url, 400, "", None, fp)

        mock.urlopen.side_effect = raise_httperror_400
        rep = await vlob_create(
            alice_ws,
            vlob_id=new_vlob_id,
            realm_id=realm,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        caplog.assert_not_occured("warning")
        assert isinstance(rep, VlobCreateRepRejectedBySequesterService)
        assert rep.service_label == service.backend_service.service_label
        assert rep.service_id == service.service_id
        assert rep.reason == "some_error_from_service"

        rep = await vlob_update(
            alice_ws,
            vlob_id=vlob_id,
            version=2,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        caplog.assert_not_occured("warning")
        assert isinstance(rep, VlobUpdateRepRejectedBySequesterService)
        assert rep.service_label == service.backend_service.service_label
        assert rep.service_id == service.service_id
        assert rep.reason == "some_error_from_service"

        # Test json error

        def raise_jsonerror_400(*args, **kwargs):
            fp = Mock()
            fp.read.return_value = b"not a json"
            raise urllib.error.HTTPError(url, 400, "", None, fp)

        mock.urlopen.side_effect = raise_jsonerror_400
        rep = await vlob_create(
            alice_ws,
            vlob_id=new_vlob_id,
            realm_id=realm,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        caplog.assert_occured_once(
            f"[warning  ] Invalid rejection reason body returned by webhook [parsec.backend.vlob] body=b'not a json' service_id={service.service_id.str} service_label=TestWebhookService"
        )
        caplog.clear()
        assert isinstance(rep, VlobCreateRepRejectedBySequesterService)
        assert rep.service_label == service.backend_service.service_label
        assert rep.service_id == service.service_id
        assert rep.reason == "File rejected (no reason)"

        rep = await vlob_update(
            alice_ws,
            vlob_id=vlob_id,
            version=2,
            blob=blob,
            sequester_blob={service.service_id: sequester_blob},
            check_rep=False,
        )
        caplog.assert_occured_once(
            f"[warning  ] Invalid rejection reason body returned by webhook [parsec.backend.vlob] body=b'not a json' service_id={service.service_id.str} service_label=TestWebhookService"
        )
        caplog.clear()
        assert isinstance(rep, VlobUpdateRepRejectedBySequesterService)
        assert rep.service_label == service.backend_service.service_label
        assert rep.service_id == service.service_id
        assert rep.reason == "File rejected (no reason)"


@customize_fixtures(coolorg_is_sequestered_organization=True)
@pytest.mark.trio
async def test_sequester_dump_realm(
    coolorg: OrganizationFullData, alice_ws, bob_ws, realm, backend
):
    vlob_id = VlobID.from_str("00000000000000000000000000000001")
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
    another_vlob_id = VlobID.from_str("00000000000000000000000000000002")
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
