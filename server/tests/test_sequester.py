# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    DateTime,
    OrganizationID,
    SequesterPrivateKeyDer,
    SequesterRevokedServiceCertificate,
    SequesterServiceCertificate,
    SequesterServiceID,
    SequesterSigningKeyDer,
    testbed,
)
from parsec.ballpark import RequireGreaterTimestamp
from parsec.components.sequester import (
    SequesterCreateServiceStoreBadOutcome,
    SequesterCreateServiceValidateBadOutcome,
    SequesterGetOrganizationServicesBadOutcome,
    SequesterGetServiceBadOutcome,
    SequesterRevokeServiceStoreBadOutcome,
    SequesterRevokeServiceValidateBadOutcome,
    SequesterServiceType,
    SequesterUpdateConfigForServiceStoreBadOutcome,
    StorageSequesterService,
    WebhookSequesterService,
)
from parsec.events import EventSequesterCertificate
from tests.common import Backend, MinimalorgRpcClients, SequesteredOrgRpcClients


@pytest.mark.parametrize("kind", ("storage", "webhook"))
async def test_create_service_ok(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend, kind: str
):
    t0 = DateTime(2001, 1, 1)
    _, service_pub_key = SequesterPrivateKeyDer.generate_pair(1024)
    certif = SequesterServiceCertificate(
        timestamp=t0,
        service_id=SequesterServiceID.new(),
        service_label="My Sequester Service",
        encryption_key_der=service_pub_key,
    )
    raw_certif = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    expected_topics = await backend.organization.test_dump_topics(sequestered_org.organization_id)
    expected_topics.sequester = t0

    match kind:
        case "storage":
            config = SequesterServiceType.STORAGE
        case "webhook":
            config = (SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook")
        case unknown:
            assert False, unknown

    with backend.event_bus.spy() as spy:
        outcome = await backend.sequester.create_service(
            now=t0.add(seconds=1),
            organization_id=sequestered_org.organization_id,
            service_certificate=raw_certif,
            config=config,
        )
        assert isinstance(outcome, SequesterServiceCertificate)

        await spy.wait_event_occurred(
            EventSequesterCertificate(
                organization_id=sequestered_org.organization_id,
                timestamp=t0,
            )
        )

    topics = await backend.organization.test_dump_topics(sequestered_org.organization_id)
    assert topics == expected_topics


@pytest.mark.parametrize("kind", ("dummy_payload", "bad_signature"))
async def test_create_service_invalid_certificate(
    sequestered_org: SequesteredOrgRpcClients,
    backend: Backend,
    kind: str,
):
    t0 = DateTime(2001, 1, 1)
    now = t0.add(seconds=1)
    match kind:
        case "dummy_payload":
            raw_certif = b"<dummy>"
        case "bad_signature":
            _, service_pub_key = SequesterPrivateKeyDer.generate_pair(1024)
            certif = SequesterServiceCertificate(
                timestamp=t0,
                service_id=SequesterServiceID.new(),
                service_label="My Sequester Service",
                encryption_key_der=service_pub_key,
            )
            dummy_sign_key, _ = SequesterSigningKeyDer.generate_pair(1024)
            raw_certif = dummy_sign_key.sign(certif.dump())
        case unknown:
            assert False, unknown

    outcome = await backend.sequester.create_service(
        now=now,
        organization_id=sequestered_org.organization_id,
        service_certificate=raw_certif,
        config=SequesterServiceType.STORAGE,
    )
    assert outcome == SequesterCreateServiceValidateBadOutcome.INVALID_CERTIFICATE


async def test_create_service_require_greater_timestamp(
    sequestered_org: SequesteredOrgRpcClients,
    backend: Backend,
):
    for event in reversed(sequestered_org.testbed_template.events):
        if isinstance(
            event,
            testbed.TestbedEventNewSequesterService | testbed.TestbedEventRevokeSequesterService,
        ):
            last_sequester_topic_timestamp = event.timestamp
            break
    else:
        assert False

    _, service_pub_key = SequesterPrivateKeyDer.generate_pair(1024)
    certif = SequesterServiceCertificate(
        timestamp=last_sequester_topic_timestamp,
        service_id=SequesterServiceID.new(),
        service_label="My Sequester Service",
        encryption_key_der=service_pub_key,
    )
    raw_certif = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    outcome = await backend.sequester.create_service(
        now=last_sequester_topic_timestamp.add(seconds=3600),
        organization_id=sequestered_org.organization_id,
        service_certificate=raw_certif,
        config=SequesterServiceType.STORAGE,
    )
    assert outcome == RequireGreaterTimestamp(strictly_greater_than=last_sequester_topic_timestamp)


@pytest.mark.parametrize("kind", ("existing_and_valid", "existing_but_revoked"))
async def test_create_service_already_exists(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend, kind: str
):
    t0 = DateTime(2001, 1, 1)
    _, service_pub_key = SequesterPrivateKeyDer.generate_pair(1024)
    match kind:
        case "existing_but_revoked":
            service_id = sequestered_org.sequester_service_1_id
        case "existing_and_valid":
            service_id = sequestered_org.sequester_service_2_id
        case unknown:
            assert False, unknown

    certif = SequesterServiceCertificate(
        timestamp=t0,
        service_id=service_id,
        service_label="My Sequester Service",
        encryption_key_der=service_pub_key,
    )
    raw_certif = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    outcome = await backend.sequester.create_service(
        now=t0.add(seconds=1),
        organization_id=sequestered_org.organization_id,
        service_certificate=raw_certif,
        config=SequesterServiceType.STORAGE,
    )
    assert outcome == SequesterCreateServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_EXISTS


async def test_create_sequester_disabled(minimalorg: MinimalorgRpcClients, backend: Backend):
    t0 = DateTime(2001, 1, 1)
    _, service_pub_key = SequesterPrivateKeyDer.generate_pair(1024)

    certif = SequesterServiceCertificate(
        timestamp=t0,
        service_id=SequesterServiceID.new(),
        service_label="My Sequester Service",
        encryption_key_der=service_pub_key,
    )
    dummy_sign_key, _ = SequesterSigningKeyDer.generate_pair(1024)
    raw_certif = dummy_sign_key.sign(certif.dump())

    outcome = await backend.sequester.create_service(
        now=t0.add(seconds=1),
        organization_id=minimalorg.organization_id,
        service_certificate=raw_certif,
        config=SequesterServiceType.STORAGE,
    )
    assert outcome == SequesterCreateServiceStoreBadOutcome.SEQUESTER_DISABLED


async def test_create_organization_not_found(backend: Backend):
    t0 = DateTime(2001, 1, 1)
    _, service_pub_key = SequesterPrivateKeyDer.generate_pair(1024)

    certif = SequesterServiceCertificate(
        timestamp=t0,
        service_id=SequesterServiceID.new(),
        service_label="My Sequester Service",
        encryption_key_der=service_pub_key,
    )
    dummy_sign_key, _ = SequesterSigningKeyDer.generate_pair(1024)
    raw_certif = dummy_sign_key.sign(certif.dump())

    outcome = await backend.sequester.create_service(
        now=t0.add(seconds=1),
        organization_id=OrganizationID("DummyOrg"),
        service_certificate=raw_certif,
        config=SequesterServiceType.STORAGE,
    )
    assert outcome == SequesterCreateServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND


@pytest.mark.parametrize("kind", ("valid_service", "revoked_service"))
async def test_update_config_for_service_ok(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend, kind: str
):
    match kind:
        case "valid_service":
            service_id = sequestered_org.sequester_service_2_id
        case "revoked_service":
            service_id = sequestered_org.sequester_service_1_id
        case unknown:
            assert False, unknown

    outcome = await backend.sequester.update_config_for_service(
        organization_id=sequestered_org.organization_id,
        service_id=service_id,
        config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
    )
    assert outcome is None
    service = await backend.sequester.get_service(
        organization_id=sequestered_org.organization_id, service_id=service_id
    )
    assert isinstance(service, WebhookSequesterService)
    assert service.webhook_url == "https://parsec.invalid/webhook"


async def test_update_config_for_service_organization_not_found(backend: Backend):
    outcome = await backend.sequester.update_config_for_service(
        organization_id=OrganizationID("DummyOrg"),
        service_id=SequesterServiceID.new(),
        config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
    )
    assert outcome == SequesterUpdateConfigForServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND


async def test_update_config_for_service_sequester_service_not_found(
    sequestered_org: SequesteredOrgRpcClients,
    backend: Backend,
):
    organization_id = sequestered_org.organization_id
    service_id = SequesterServiceID.new()

    outcome = await backend.sequester.update_config_for_service(
        organization_id=organization_id,
        service_id=service_id,
        config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
    )
    assert outcome == SequesterUpdateConfigForServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND


async def test_update_config_for_service_sequester_service_disabled(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
):
    organization_id = minimalorg.organization_id
    service_id = SequesterServiceID.new()

    outcome = await backend.sequester.update_config_for_service(
        organization_id=organization_id,
        service_id=service_id,
        config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
    )
    assert outcome == SequesterUpdateConfigForServiceStoreBadOutcome.SEQUESTER_DISABLED


async def test_revoke_service_ok(sequestered_org: SequesteredOrgRpcClients, backend: Backend):
    t0 = DateTime(2001, 1, 1)
    certif = SequesterRevokedServiceCertificate(
        timestamp=t0,
        service_id=sequestered_org.sequester_service_2_id,
    )
    raw_certif = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    expected_topics = await backend.organization.test_dump_topics(sequestered_org.organization_id)
    expected_topics.sequester = t0

    with backend.event_bus.spy() as spy:
        outcome = await backend.sequester.revoke_service(
            now=t0.add(seconds=1),
            organization_id=sequestered_org.organization_id,
            revoked_service_certificate=raw_certif,
        )
        assert isinstance(outcome, SequesterRevokedServiceCertificate)

        await spy.wait_event_occurred(
            EventSequesterCertificate(
                organization_id=sequestered_org.organization_id,
                timestamp=t0,
            )
        )

    topics = await backend.organization.test_dump_topics(sequestered_org.organization_id)
    assert topics == expected_topics


@pytest.mark.parametrize("kind", ("dummy_payload", "bad_signature"))
async def test_revoke_service_invalid_certificate(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend, kind: str
):
    t0 = DateTime(2001, 1, 1)
    now = t0.add(seconds=1)
    match kind:
        case "dummy_payload":
            raw_certif = b"<dummy>"
        case "bad_signature":
            certif = SequesterRevokedServiceCertificate(
                timestamp=t0,
                service_id=SequesterServiceID.new(),
            )
            dummy_sign_key, _ = SequesterSigningKeyDer.generate_pair(1024)
            raw_certif = dummy_sign_key.sign(certif.dump())
        case unknown:
            assert False, unknown

    outcome = await backend.sequester.revoke_service(
        now=now,
        organization_id=sequestered_org.organization_id,
        revoked_service_certificate=raw_certif,
    )
    assert outcome == SequesterRevokeServiceValidateBadOutcome.INVALID_CERTIFICATE


async def test_revoke_service_require_greater_timestamp(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend
):
    for event in reversed(sequestered_org.testbed_template.events):
        if isinstance(
            event,
            testbed.TestbedEventNewSequesterService | testbed.TestbedEventRevokeSequesterService,
        ):
            last_sequester_topic_timestamp = event.timestamp
            break
    else:
        assert False

    certif = SequesterRevokedServiceCertificate(
        timestamp=last_sequester_topic_timestamp,
        service_id=sequestered_org.sequester_service_2_id,
    )
    raw_certif = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    outcome = await backend.sequester.revoke_service(
        now=last_sequester_topic_timestamp.add(seconds=3600),
        organization_id=sequestered_org.organization_id,
        revoked_service_certificate=raw_certif,
    )
    assert outcome == RequireGreaterTimestamp(strictly_greater_than=last_sequester_topic_timestamp)


async def test_revoke_service_organization_not_found(backend: Backend):
    _, service_pub_key = SequesterPrivateKeyDer.generate_pair(1024)
    t0 = DateTime(2001, 1, 1)
    certif = SequesterServiceCertificate(
        timestamp=t0,
        service_id=SequesterServiceID.new(),
        service_label="My Sequester Service",
        encryption_key_der=service_pub_key,
    )
    dummy_sign_key, _ = SequesterSigningKeyDer.generate_pair(1024)
    raw_certif = dummy_sign_key.sign(certif.dump())

    outcome = await backend.sequester.revoke_service(
        now=t0.add(seconds=1),
        organization_id=OrganizationID("DummyOrg"),
        revoked_service_certificate=raw_certif,
    )
    assert outcome == SequesterRevokeServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND


async def test_revoke_service_sequester_disabled(
    minimalorg: MinimalorgRpcClients, backend: Backend
):
    _, service_pub_key = SequesterPrivateKeyDer.generate_pair(1024)
    t0 = DateTime(2001, 1, 1)
    certif = SequesterServiceCertificate(
        timestamp=t0,
        service_id=SequesterServiceID.new(),
        service_label="My Sequester Service",
        encryption_key_der=service_pub_key,
    )
    dummy_sign_key, _ = SequesterSigningKeyDer.generate_pair(1024)
    raw_certif = dummy_sign_key.sign(certif.dump())

    outcome = await backend.sequester.revoke_service(
        now=t0.add(seconds=1),
        organization_id=minimalorg.organization_id,
        revoked_service_certificate=raw_certif,
    )
    assert outcome == SequesterRevokeServiceStoreBadOutcome.SEQUESTER_DISABLED


async def test_revoke_service_sequester_service_not_found(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend
):
    t0 = DateTime(2001, 1, 1)
    certif = SequesterRevokedServiceCertificate(
        timestamp=t0,
        service_id=SequesterServiceID.new(),
    )
    raw_certif = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    outcome = await backend.sequester.revoke_service(
        now=t0.add(seconds=1),
        organization_id=sequestered_org.organization_id,
        revoked_service_certificate=raw_certif,
    )
    assert outcome == SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND


async def test_revoke_service_sequester_service_already_revoked(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend
):
    t0 = DateTime(2001, 1, 1)
    certif = SequesterRevokedServiceCertificate(
        timestamp=t0,
        service_id=sequestered_org.sequester_service_1_id,
    )
    raw_certif = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    outcome = await backend.sequester.revoke_service(
        now=t0.add(seconds=1),
        organization_id=sequestered_org.organization_id,
        revoked_service_certificate=raw_certif,
    )
    assert outcome == SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_REVOKED


@pytest.mark.parametrize("kind", ("storage_service", "webhook_service"))
async def test_get_service_ok(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend, kind: str
):
    match kind:
        case "storage_service":
            service_id = sequestered_org.sequester_service_1_id
            expected_outcome = StorageSequesterService(
                service_id=sequestered_org.sequester_service_1_id,
                service_label="Sequester Service 1",
                service_certificate=ANY,
                created_on=DateTime(2000, 1, 11),
                revoked_on=DateTime(2000, 1, 16),
            )

        case "webhook_service":
            outcome = await backend.sequester.update_config_for_service(
                organization_id=sequestered_org.organization_id,
                service_id=sequestered_org.sequester_service_2_id,
                config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
            )
            assert outcome is None

            service_id = sequestered_org.sequester_service_2_id
            expected_outcome = WebhookSequesterService(
                service_id=sequestered_org.sequester_service_2_id,
                service_label="Sequester Service 2",
                service_certificate=ANY,
                created_on=DateTime(2000, 1, 18),
                revoked_on=None,
                webhook_url="https://parsec.invalid/webhook",
            )

        case unknown:
            assert False, unknown

    outcome = await backend.sequester.get_service(
        organization_id=sequestered_org.organization_id,
        service_id=service_id,
    )
    assert outcome == expected_outcome


async def test_get_service_organization_not_found(backend: Backend):
    outcome = await backend.sequester.get_service(
        organization_id=OrganizationID("DummyOrg"),
        service_id=SequesterServiceID.new(),
    )
    assert outcome == SequesterGetServiceBadOutcome.ORGANIZATION_NOT_FOUND


async def test_get_service_sequester_disabled(minimalorg: MinimalorgRpcClients, backend: Backend):
    outcome = await backend.sequester.get_service(
        organization_id=minimalorg.organization_id,
        service_id=SequesterServiceID.new(),
    )
    assert outcome == SequesterGetServiceBadOutcome.SEQUESTER_DISABLED


async def test_get_service_sequester_service_not_found(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend
):
    outcome = await backend.sequester.get_service(
        organization_id=sequestered_org.organization_id,
        service_id=SequesterServiceID.new(),
    )
    assert outcome == SequesterGetServiceBadOutcome.SEQUESTER_SERVICE_NOT_FOUND


async def test_get_organization_services_ok(
    sequestered_org: SequesteredOrgRpcClients, backend: Backend
):
    outcome = await backend.sequester.update_config_for_service(
        organization_id=sequestered_org.organization_id,
        service_id=sequestered_org.sequester_service_2_id,
        config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
    )
    assert outcome is None

    expected_outcome = [
        StorageSequesterService(
            service_id=sequestered_org.sequester_service_1_id,
            service_label="Sequester Service 1",
            service_certificate=ANY,
            created_on=DateTime(2000, 1, 11),
            revoked_on=DateTime(2000, 1, 16),
        ),
        WebhookSequesterService(
            service_id=sequestered_org.sequester_service_2_id,
            service_label="Sequester Service 2",
            service_certificate=ANY,
            created_on=DateTime(2000, 1, 18),
            revoked_on=None,
            webhook_url="https://parsec.invalid/webhook",
        ),
    ]

    outcome = await backend.sequester.get_organization_services(
        organization_id=sequestered_org.organization_id,
    )
    assert outcome == expected_outcome


async def test_get_organization_services_organization_not_found(backend: Backend):
    outcome = await backend.sequester.get_organization_services(
        organization_id=OrganizationID("DummyOrg"),
    )
    assert outcome == SequesterGetOrganizationServicesBadOutcome.ORGANIZATION_NOT_FOUND


async def test_get_organization_services_sequester_disabled(
    minimalorg: MinimalorgRpcClients, backend: Backend
):
    outcome = await backend.sequester.get_organization_services(
        organization_id=minimalorg.organization_id,
    )
    assert outcome == SequesterGetOrganizationServicesBadOutcome.SEQUESTER_DISABLED
