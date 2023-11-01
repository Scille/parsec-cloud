# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx
import pytest

from parsec._parsec import (
    BlockID,
    DateTime,
    OrganizationID,
    RevokedUserCertificate,
    UserProfile,
    UserUpdateCertificate,
    VlobID,
)
from tests.common import Backend, CoolorgRpcClients, MinimalorgRpcClients


def _strip_template_orgs(stats: dict) -> dict:
    stats["stats"] = [s for s in stats["stats"] if not s["organization_id"].endswith("Template")]
    return stats


@pytest.mark.parametrize(
    "route", ("/administration/stats", "/administration/organizations/{organization_id}/stats")
)
async def test_organization_stats_auth(
    route: str, client: httpx.AsyncClient, coolorg: CoolorgRpcClients
) -> None:
    url = "http://parsec.invalid" + route.format(organization_id=coolorg.organization_id.str)
    # No Authorization header
    response = await client.get(url)
    assert response.status_code == 403
    # Invilad Authorization header
    response = await client.get(
        url,
        headers={
            "Authorization": "DUMMY",
        },
    )
    assert response.status_code == 403
    # Bad bearer token
    response = await client.get(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
    )
    assert response.status_code == 403


async def test_ok(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
    minimalorg: MinimalorgRpcClients,
) -> None:
    not_bootstrapped_org_id = OrganizationID("NotBootstrappedOrg")
    await backend.organization.create(
        now=DateTime.now(),
        id=not_bootstrapped_org_id,
    )

    async def server_stats():
        r = await client.get(
            "http://parsec.invalid/administration/stats",
            headers={
                "Authorization": f"Bearer {backend.config.administration_token}",
            },
        )
        assert r.status_code == 200
        return _strip_template_orgs(r.json())

    async def org_stats(organization_id: OrganizationID):
        r = await client.get(
            f"http://parsec.invalid/administration/organizations/{organization_id.str}/stats",
            headers={
                "Authorization": f"Bearer {backend.config.administration_token}",
            },
        )
        assert r.status_code == 200
        return r.json()

    expected_coolorg_stats = {
        "active_users": 3,
        "data_size": 0,
        "metadata_size": 671,
        "realms": 3,
        "users": 3,
        "users_per_profile_detail": {
            "ADMIN": {"active": 1, "revoked": 0},
            "OUTSIDER": {"active": 1, "revoked": 0},
            "STANDARD": {"active": 1, "revoked": 0},
        },
    }
    stats = await org_stats(coolorg.organization_id)
    assert stats == expected_coolorg_stats

    expected_minimal_stats = {
        "active_users": 1,
        "data_size": 0,
        "metadata_size": 0,
        "realms": 0,
        "users": 1,
        "users_per_profile_detail": {
            "ADMIN": {"active": 1, "revoked": 0},
            "OUTSIDER": {"active": 0, "revoked": 0},
            "STANDARD": {"active": 0, "revoked": 0},
        },
    }
    stats = await org_stats(minimalorg.organization_id)
    assert stats == expected_minimal_stats

    expected_server_stats = {
        "stats": [
            {
                **expected_coolorg_stats,
                "organization_id": coolorg.organization_id.str,
            },
            {
                **expected_minimal_stats,
                "organization_id": minimalorg.organization_id.str,
            },
            {
                "organization_id": "NotBootstrappedOrg",
                "users": 0,
                "active_users": 0,
                "data_size": 0,
                "metadata_size": 0,
                "realms": 0,
                "users_per_profile_detail": {
                    "ADMIN": {"active": 0, "revoked": 0},
                    "OUTSIDER": {"active": 0, "revoked": 0},
                    "STANDARD": {"active": 0, "revoked": 0},
                },
            },
        ]
    }
    stats = await server_stats()
    assert stats == expected_server_stats

    # Now update Coolorg...

    await backend.vlob.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        blob=b"0" * 10,
    )
    await backend.vlob.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        blob=b"0" * 20,
    )
    expected_coolorg_stats["metadata_size"] += 30
    expected_server_stats["stats"][0]["metadata_size"] += 30

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=BlockID.new(),
        realm_id=coolorg.wksp1_id,
        block=b"0" * 10,
    )
    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=BlockID.new(),
        realm_id=coolorg.wksp1_id,
        block=b"0" * 20,
    )
    expected_coolorg_stats["data_size"] += 30
    expected_server_stats["stats"][0]["data_size"] += 30

    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        new_profile=UserProfile.ADMIN,
        user_id=coolorg.bob.device_id.user_id,
        timestamp=DateTime.now(),
    )
    await backend.user.update_user(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    expected_coolorg_stats["users_per_profile_detail"]["STANDARD"]["active"] -= 1
    expected_coolorg_stats["users_per_profile_detail"]["ADMIN"]["active"] += 1

    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.mallory.device_id.user_id,
        timestamp=DateTime.now(),
    )
    await backend.user.revoke_user(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    expected_coolorg_stats["active_users"] -= 1
    expected_coolorg_stats["users_per_profile_detail"]["OUTSIDER"]["active"] -= 1
    expected_coolorg_stats["users_per_profile_detail"]["OUTSIDER"]["revoked"] += 1
    expected_server_stats["stats"][0]["active_users"] -= 1

    # ...and ensure stats reflect that
    stats = await org_stats(coolorg.organization_id)
    assert stats == expected_coolorg_stats

    stats = await org_stats(minimalorg.organization_id)
    assert stats == expected_minimal_stats

    stats = await server_stats()
    assert stats == expected_server_stats


async def test_server_stats_format(
    client: httpx.AsyncClient, backend: Backend, minimalorg: MinimalorgRpcClients
) -> None:
    def _strip_template_orgs(stats: dict) -> dict:
        stats["stats"] = [
            s for s in stats["stats"] if not s["organization_id"].endswith("Template")
        ]
        return stats

    async def server_stats(format: str):
        r = await client.get(
            f"http://parsec.invalid/administration/stats?format={format}",
            headers={
                "Authorization": f"Bearer {backend.config.administration_token}",
            },
        )
        assert r.status_code == 200
        return r

    async def org_stats(format: str):
        r = await client.get(
            f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/stats?format={format}",
            headers={
                "Authorization": f"Bearer {backend.config.administration_token}",
            },
        )
        assert r.status_code == 200
        return r

    response = await server_stats("json")
    assert _strip_template_orgs(response.json()) == {
        "stats": [
            {
                "active_users": 1,
                "data_size": 0,
                "metadata_size": 0,
                "organization_id": "Org1",
                "realms": 0,
                "users": 1,
                "users_per_profile_detail": {
                    "ADMIN": {"active": 1, "revoked": 0},
                    "OUTSIDER": {"active": 0, "revoked": 0},
                    "STANDARD": {"active": 0, "revoked": 0},
                },
            }
        ]
    }

    response = await server_stats("csv")
    assert response.headers["Content-Type"] == "text/csv; charset=utf-8"
    assert response.content.decode("utf8") == (
        "organization_id,data_size,metadata_size,realms,active_users,admin_users_active,"
        "admin_users_revoked,standard_users_active,standard_users_revoked,"
        "outsider_users_active,outsider_users_revoked\r\n"
        "MinimalOrgTemplate,0,0,0,1,1,0,0,0,0,0\r\n"
        "Org1,0,0,0,1,1,0,0,0,0,0\r\n"
    )
