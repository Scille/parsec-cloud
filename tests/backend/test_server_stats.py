# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from typing import Optional

from parsec._parsec import VlobID, DateTime, BlockID, RealmID, RealmRole
from parsec.api.protocol.types import UserProfile
from parsec.backend.app import BackendApp
from parsec.backend.realm import RealmGrantedRole

from tests.common import customize_fixtures


async def server_stats(
    client,
    headers,
    at: Optional[str] = None,
    format: str = "json",
):
    query_string = {"format": format}
    if at:
        query_string["at"] = at
    rep = await client.get(
        f"/administration/stats",
        headers=headers,
        query_string=query_string,
    )
    assert rep.status_code == 200

    return await rep.get_data(as_text=True) if format == "csv" else await rep.get_json()


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_unauthorized_client(backend_asgi_app):
    client = backend_asgi_app.test_client()  # This client has no token
    rep = await client.get("/administration/stats")
    assert rep.status == "403 FORBIDDEN"


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_bad_requests(backend_asgi_app):
    client = backend_asgi_app.test_client()  # This client has no token
    headers = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    async def _do_req(query_string: dict) -> dict:
        rep = await client.get("/administration/stats", headers=headers, query_string=query_string)
        assert rep.status == "400 BAD REQUEST"
        return await rep.get_json()

    # No arguments in query string
    assert await _do_req({}) == {
        "error": "bad_data",
        "reason": "Missing/invalid mandatory query argument `format` (expected `csv` or `json`)",
    }

    # Missing `format` argument
    assert await _do_req({"to": "2021-01-01T01:01:01Z"}) == {
        "error": "bad_data",
        "reason": "Missing/invalid mandatory query argument `format` (expected `csv` or `json`)",
    }

    # Bad `format` argument
    assert await _do_req({"format": "mp3"}) == {
        "error": "bad_data",
        "reason": "Missing/invalid mandatory query argument `format` (expected `csv` or `json`)",
    }

    # Bad `at` argument
    assert await _do_req({"format": "json", "at": "dummy"}) == {
        "error": "bad_data",
        "reason": "Invalid `at` query argument (expected RFC3339 datetime)",
    }


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_stats(
    backend_asgi_app,
    backend: BackendApp,
    backend_data_binder,
    organization_factory,
    local_device_factory,
):
    client = backend_asgi_app.test_client()
    headers = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    async def _get_stats(at=""):
        rep = await client.get(
            "/administration/stats",
            headers=headers,
            query_string={"format": "json", "at": at},
        )
        assert rep.status_code == 200
        return await rep.get_json()

    # Stat on empty server
    stats = await _get_stats()
    assert stats == {"stats": []}

    orgs = [organization_factory(), organization_factory()]

    # Non-bootstrapped organization are present but empty
    dt1 = DateTime(2000, 1, 1)
    for org in orgs:
        await backend.organization.create(
            org.organization_id, bootstrap_token=org.bootstrap_token, created_on=dt1
        )
    assert await _get_stats() == {
        "stats": [
            {
                "organization_id": org,
                "metadata_size": 0,
                "data_size": 0,
                "realms": 0,
                "active_users": 0,
                "users_per_profile_detail": [
                    {"active": 0, "revoked": 0, "profile": "ADMIN"},
                    {"active": 0, "revoked": 0, "profile": "STANDARD"},
                    {"active": 0, "revoked": 0, "profile": "OUTSIDER"},
                ],
            }
            for org in ["Org1", "Org2"]
        ]
    }

    # Bootstrap & populate two organizations on February and March
    for i in range(2, 4):

        dtx1 = DateTime(2000, i, 1)
        org = orgs[i - 2]
        org_d1 = local_device_factory(org=org, profile=UserProfile.ADMIN)
        await backend_data_binder.bind_organization(
            org=org,
            first_device=org_d1,
            initial_user_manifest="not_synced",
            timestamp=dtx1,
            create_needed=False,
        )

        dtx2 = DateTime(2000, i, 2)
        org_d2 = local_device_factory(org=org, profile=UserProfile.STANDARD)
        org_d3 = local_device_factory(org=org, profile=UserProfile.OUTSIDER)
        await backend_data_binder.bind_device(
            device=org_d2, initial_user_manifest="not_synced", timestamp=dtx2
        )
        await backend_data_binder.bind_device(
            device=org_d3, initial_user_manifest="not_synced", timestamp=dtx2
        )

        dtx3 = DateTime(2000, i, 3)
        dtx4 = DateTime(2000, i, 4)
        for _ in range(2):
            realm_id = RealmID.new()
            await backend.realm.create(
                organization_id=org.organization_id,
                self_granted_role=RealmGrantedRole(
                    realm_id=realm_id,
                    user_id=org_d1.user_id,
                    certificate=b"<whatever>",
                    role=RealmRole.OWNER,
                    granted_by=org_d1.device_id,
                    granted_on=dtx3,
                ),
            )
            vlob_id = VlobID.new()
            await backend.vlob.create(
                organization_id=org.organization_id,
                author=org_d1.device_id,
                encryption_revision=1,
                vlob_id=vlob_id,
                realm_id=realm_id,
                timestamp=dtx4,
                blob=b"\x00" * 10,
            )
            await backend.block.create(
                organization_id=org.organization_id,
                author=org_d1.device_id,
                block_id=BlockID.new(),
                realm_id=realm_id,
                created_on=dtx4,
                block=b"\x00" * 100,
            )

    # Now check the stats
    dty0_stats = await _get_stats()
    assert dty0_stats == {
        "stats": [
            {
                "organization_id": org,
                "metadata_size": 20,
                "data_size": 200,
                "realms": 2,
                "active_users": 3,
                "users_per_profile_detail": [
                    {"active": 1, "revoked": 0, "profile": "ADMIN"},
                    {"active": 1, "revoked": 0, "profile": "STANDARD"},
                    {"active": 1, "revoked": 0, "profile": "OUTSIDER"},
                ],
            }
            for org in ["Org1", "Org2"]
        ]
    }

    # Update the 2nd organization
    dty1 = DateTime(2001, 1, 1)
    await backend.user.revoke_user(
        organization_id=org.organization_id,
        user_id=org_d2.user_id,
        revoked_user_certificate=b"<dummy>",
        revoked_user_certifier=org_d1.device_id,
        revoked_on=dty1,
    )
    await backend.user.revoke_user(
        organization_id=org.organization_id,
        user_id=org_d3.user_id,
        revoked_user_certificate=b"<dummy>",
        revoked_user_certifier=org_d1.device_id,
        revoked_on=dty1,
    )
    await backend.realm.create(
        organization_id=org.organization_id,
        self_granted_role=RealmGrantedRole(
            realm_id=RealmID.new(),
            user_id=org_d1.user_id,
            certificate=b"<whatever>",
            role=RealmRole.OWNER,
            granted_by=org_d1.device_id,
            granted_on=dty1,
        ),
    )
    await backend.vlob.update(
        organization_id=org.organization_id,
        author=org_d1.device_id,
        encryption_revision=1,
        vlob_id=vlob_id,
        version=2,
        timestamp=dty1,
        blob=b"\x00" * 10,
    )
    await backend.block.create(
        organization_id=org.organization_id,
        author=org_d1.device_id,
        block_id=BlockID.new(),
        realm_id=realm_id,
        created_on=dty1,
        block=b"\x00" * 100,
    )

    # Stats should have changed
    dty1_stats = await _get_stats()
    assert dty1_stats == {
        "stats": [
            {
                "organization_id": "Org1",
                "metadata_size": 20,
                "data_size": 200,
                "realms": 2,
                "active_users": 3,
                "users_per_profile_detail": [
                    {"active": 1, "revoked": 0, "profile": "ADMIN"},
                    {"active": 1, "revoked": 0, "profile": "STANDARD"},
                    {"active": 1, "revoked": 0, "profile": "OUTSIDER"},
                ],
            },
            {
                "organization_id": "Org2",
                "metadata_size": 30,
                "data_size": 300,
                "realms": 3,
                "active_users": 1,
                "users_per_profile_detail": [
                    {"active": 1, "revoked": 0, "profile": "ADMIN"},
                    {"active": 0, "revoked": 1, "profile": "STANDARD"},
                    {"active": 0, "revoked": 1, "profile": "OUTSIDER"},
                ],
            },
        ]
    }

    # Check `at` option, note the bound is included
    # assert await _get_stats(at=dty1.to_rfc3339()) == dty1_stats
    # assert await _get_stats(at=dty1.add(microseconds=-1).to_rfc3339()) == dty0_stats

    # `at` filter don't excludes organization that are not bootstrapped at that time
    assert await _get_stats(at="2000-02-01T00:00:00Z") == {
        "stats": [
            # At that time, Org1 was already bootstrapped
            {
                "organization_id": "Org1",
                "metadata_size": 0,
                "data_size": 0,
                "realms": 0,
                "active_users": 1,
                "users_per_profile_detail": [
                    {"active": 1, "revoked": 0, "profile": "ADMIN"},
                    {"active": 0, "revoked": 0, "profile": "STANDARD"},
                    {"active": 0, "revoked": 0, "profile": "OUTSIDER"},
                ],
            },
            # At that time, Org2 was created but not yet bootstrapped
            {
                "organization_id": "Org2",
                "metadata_size": 0,
                "data_size": 0,
                "realms": 0,
                "active_users": 0,
                "users_per_profile_detail": [
                    {"active": 0, "revoked": 0, "profile": "ADMIN"},
                    {"active": 0, "revoked": 0, "profile": "STANDARD"},
                    {"active": 0, "revoked": 0, "profile": "OUTSIDER"},
                ],
            },
        ]
    }
    # At that time, the server was empty
    assert await _get_stats(at="1999-01-01T00:00:00Z") == {"stats": []}

    # Finally test the csv format
    rep = await client.get(
        "/administration/stats",
        headers=headers,
        query_string={"format": "csv"},
    )
    assert rep.status_code == 200
    assert rep.headers["content-type"] == "text/csv"
    csv_data = await rep.get_data()
    # We use Excel like CSV that use carriage return and line feed ('\r\n') as line separator
    assert (
        csv_data
        == b"""organization_id,data_size,metadata_size,realms,active_users,admin_users_active,admin_users_revoked,standard_users_active,standard_users_revoked,outsider_users_active,outsider_users_revoked\r
Org1,200,20,2,3,1,0,1,0,1,0\r
Org2,300,30,3,1,1,0,0,1,0,1\r
"""
    )
