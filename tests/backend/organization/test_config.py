# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import OrganizationConfigRepOk
from tests.common import customize_fixtures, OrganizationFullData, sequester_service_factory
from tests.backend.common import organization_config


@pytest.mark.trio
async def test_organization_config_ok(coolorg: OrganizationFullData, alice_ws, backend):
    rep = await organization_config(alice_ws)
    assert rep == OrganizationConfigRepOk(
        user_profile_outsider_allowed=True,
        active_users_limit=backend.config.organization_initial_active_users_limit,
        sequester_authority_certificate=None,
        sequester_services_certificates=None,
    )

    await backend.organization.update(
        id=coolorg.organization_id, user_profile_outsider_allowed=False, active_users_limit=42
    )
    rep = await organization_config(alice_ws)
    assert rep == OrganizationConfigRepOk(
        user_profile_outsider_allowed=False,
        active_users_limit=42,
        sequester_authority_certificate=None,
        sequester_services_certificates=None,
    )


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
    )
