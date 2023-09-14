# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    ArchivingConfigRepOk,
    OrganizationConfigRepOk,
    RealmArchivingConfiguration,
    RealmArchivingStatus,
    RealmID,
)
from tests.backend.common import archiving_config, organization_config
from tests.common import OrganizationFullData, customize_fixtures, sequester_service_factory


@pytest.mark.trio
async def test_organization_config_ok(coolorg: OrganizationFullData, alice_ws, backend):
    rep = await organization_config(alice_ws)
    assert rep == OrganizationConfigRepOk(
        user_profile_outsider_allowed=True,
        active_users_limit=backend.config.organization_initial_active_users_limit,
        sequester_authority_certificate=None,
        sequester_services_certificates=None,
        minimum_archiving_period=2592000,
    )

    await backend.organization.update(
        id=coolorg.organization_id,
        user_profile_outsider_allowed=False,
        active_users_limit=ActiveUsersLimit.LimitedTo(42),
    )
    rep = await organization_config(alice_ws)
    assert rep == OrganizationConfigRepOk(
        user_profile_outsider_allowed=False,
        active_users_limit=ActiveUsersLimit.LimitedTo(42),
        sequester_authority_certificate=None,
        sequester_services_certificates=None,
        minimum_archiving_period=2592000,
    )

    # Test attribute access
    assert rep.user_profile_outsider_allowed is False
    assert rep.active_users_limit == ActiveUsersLimit.LimitedTo(42)
    assert rep.sequester_authority_certificate is None
    assert rep.sequester_services_certificates is None
    assert rep.minimum_archiving_period == 2592000


@pytest.mark.trio
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_organization_config_ok_sequestered_organization(
    coolorg: OrganizationFullData, alice_ws, backend
):
    rep = await organization_config(alice_ws)
    assert rep == OrganizationConfigRepOk(
        user_profile_outsider_allowed=True,
        active_users_limit=backend.config.organization_initial_active_users_limit,
        sequester_authority_certificate=coolorg.sequester_authority.certif,
        sequester_services_certificates=[],
        minimum_archiving_period=2592000,
    )

    # Add new services
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

    rep = await organization_config(alice_ws)
    assert rep == OrganizationConfigRepOk(
        user_profile_outsider_allowed=True,
        active_users_limit=backend.config.organization_initial_active_users_limit,
        sequester_authority_certificate=coolorg.sequester_authority.certif,
        sequester_services_certificates=[s1.certif, s2.certif],
        minimum_archiving_period=2592000,
    )

    # Delete a service, should no longer appear in config
    await backend.sequester.disable_service(
        organization_id=coolorg.organization_id, service_id=s1.service_id
    )

    rep = await organization_config(alice_ws)
    assert rep == OrganizationConfigRepOk(
        user_profile_outsider_allowed=True,
        active_users_limit=backend.config.organization_initial_active_users_limit,
        sequester_authority_certificate=coolorg.sequester_authority.certif,
        sequester_services_certificates=[s2.certif],
        minimum_archiving_period=2592000,
    )

    # Test attribute access
    assert rep.user_profile_outsider_allowed is True
    assert rep.active_users_limit == backend.config.organization_initial_active_users_limit
    assert rep.sequester_authority_certificate == coolorg.sequester_authority.certif
    assert rep.sequester_services_certificates == (s2.certif,)
    assert rep.minimum_archiving_period == 2592000


@pytest.mark.trio
async def test_archiving_config_ok(
    coolorg: OrganizationFullData,
    alice_ws,
    backend,
    realm,
    alice,
    archived_realm_factory,
    deleted_realm_factory,
    plan_realm_deletion,
):
    user_realm_id = RealmID.from_bytes(alice.user_manifest_id.bytes)
    expected_config = [
        RealmArchivingStatus(
            configuration=RealmArchivingConfiguration.available(),
            configured_on=None,
            configured_by=None,
            realm_id=user_realm_id,
        ),
        RealmArchivingStatus(
            configuration=RealmArchivingConfiguration.available(),
            configured_on=None,
            configured_by=None,
            realm_id=realm,
        ),
    ]
    rep = await archiving_config(alice_ws)
    assert rep == ArchivingConfigRepOk(
        archiving_config=expected_config,
    )

    archived_realm, timestamp = await archived_realm_factory(backend, alice)
    expected_config.append(
        RealmArchivingStatus(
            configuration=RealmArchivingConfiguration.archived(),
            configured_on=timestamp,
            configured_by=alice.device_id,
            realm_id=archived_realm,
        )
    )
    rep = await archiving_config(alice_ws)
    assert rep == ArchivingConfigRepOk(
        archiving_config=expected_config,
    )

    deleted_realm, timestamp = await deleted_realm_factory(backend, alice)
    expected_config.append(
        RealmArchivingStatus(
            configuration=RealmArchivingConfiguration.deletion_planned(timestamp),
            configured_on=timestamp,
            configured_by=alice.device_id,
            realm_id=deleted_realm,
        )
    )
    rep = await archiving_config(alice_ws)
    assert rep == ArchivingConfigRepOk(
        archiving_config=expected_config,
    )

    timestamp = await plan_realm_deletion(backend, alice, realm)
    expected_config[1] = RealmArchivingStatus(
        configuration=RealmArchivingConfiguration.deletion_planned(timestamp.add(days=31)),
        configured_on=timestamp,
        configured_by=alice.device_id,
        realm_id=realm,
    )

    rep = await archiving_config(alice_ws)
    assert rep == ArchivingConfigRepOk(
        archiving_config=expected_config,
    )
