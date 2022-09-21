# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec._parsec import VlobID, DateTime, BlockID, RevokedUserCertificate
from tests.backend.common import user_revoke


async def server_stats(client, headers, from_date: str, to_date: str, format: str = "json"):
    rep = await client.get(
        f"/administration/stats?from={from_date}&to={to_date}&format={format}", headers=headers
    )
    assert rep.status_code == 200

    return await rep.get_data(as_text=True) if format == "csv" else await rep.get_json()


@pytest.mark.trio
async def test_unauthorized_client(backend_asgi_app):
    client = backend_asgi_app.test_client()  # This client has no token
    rep = await client.get("/administration/stats")
    assert rep.status == "403 FORBIDDEN"


@pytest.mark.trio
async def test_bad_requests(backend_asgi_app):
    client = backend_asgi_app.test_client()  # This client has no token
    HEADERS = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    # No arguments in query string
    rep = await client.get("/administration/stats", headers=HEADERS)
    assert rep.status == "400 BAD REQUEST"

    # Missing format
    rep = await client.get("/administration/stats?from=2020-01-01&to=2021-01-01", headers=HEADERS)
    assert rep.status == "400 BAD REQUEST"

    # Missing from
    rep = await client.get("/administration/stats?format=csv&to=2021-01-01", headers=HEADERS)

    assert rep.status == "400 BAD REQUEST"


@pytest.mark.trio
async def test_bad_time_range(backend_asgi_app):
    client = backend_asgi_app.test_client()  # This client has no token
    HEADERS = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    rep = await client.get("/administration/stats?from=dummy", headers=HEADERS)
    assert rep.status == "400 BAD REQUEST"

    # Invalid time range from > to
    rep = await client.get(
        "/administration/stats?format=csv&from=2022-01-01&to=2000-01-01", headers=HEADERS
    )
    assert rep.status == "400 BAD REQUEST"


@pytest.mark.trio
async def test_json_server_stats(backend_asgi_app, realm, alice):
    client = backend_asgi_app.test_client()
    HEADERS = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    rep = await server_stats(
        client, HEADERS, from_date="1900-01-01", to_date="3000-01-01", format="json"
    )
    first_size, second_size, third_size = (org["metadata_size"] for org in rep["stats"])
    expected = {
        "stats": [
            {
                "data_size": 0,
                "id": "CoolOrg",
                "metadata_size": first_size,
                "realms": 4,
                "users": 3,
                "user_per_profiles": {
                    "ADMIN": {"active": 2, "revoked": 0},
                    "OUTSIDER": {"active": 0, "revoked": 0},
                    "STANDARD": {"active": 1, "revoked": 0},
                },
            },
            {
                "data_size": 0,
                "id": "ExpiredOrg",
                "metadata_size": second_size,
                "realms": 1,
                "users": 1,
                "user_per_profiles": {
                    "ADMIN": {"active": 1, "revoked": 0},
                    "OUTSIDER": {"active": 0, "revoked": 0},
                    "STANDARD": {"active": 0, "revoked": 0},
                },
            },
            {
                "data_size": 0,
                "id": "OtherOrg",
                "metadata_size": third_size,
                "realms": 1,
                "users": 1,
                "user_per_profiles": {
                    "ADMIN": {"active": 1, "revoked": 0},
                    "OUTSIDER": {"active": 0, "revoked": 0},
                    "STANDARD": {"active": 0, "revoked": 0},
                },
            },
        ]
    }
    assert rep == expected

    # Create new metadata
    await backend_asgi_app.backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=VlobID.new(),
        timestamp=DateTime.now(),
        blob=b"aliice",
    )

    rep = await server_stats(
        client, HEADERS, from_date="1900-01-01", to_date="3000-01-01", format="json"
    )
    expected["stats"][0]["metadata_size"] += 6  # New size
    assert rep == expected

    # Create new data
    await backend_asgi_app.backend.block.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        block_id=BlockID.new(),
        realm_id=realm,
        block=b"ooof",
    )
    rep = await server_stats(
        client, HEADERS, from_date="1900-01-01", to_date="3000-01-01", format="json"
    )
    expected["stats"][0]["data_size"] += 4  # New data size
    assert rep == expected


@pytest.mark.trio
async def test_json_server_constrained_range(backend_asgi_app, realm, alice):
    client = backend_asgi_app.test_client()
    HEADERS = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    rep = await server_stats(client, HEADERS, from_date="2021-01-01", to_date="2021-12-31")
    expected = rep

    # Create new vlob but in the past
    await backend_asgi_app.backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=VlobID.new(),
        timestamp=DateTime(2020, 5, 5, 15, 43, 23),
        blob=b"aliice",
    )
    assert rep == expected  # Still the same vlob is too old

    # A vlob in 2021
    await backend_asgi_app.backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=VlobID.new(),
        timestamp=DateTime(2021, 5, 5, 15, 43, 23),
        blob=b"yay",
    )
    expected["stats"][0]["metadata_size"] += 3
    rep = await server_stats(client, HEADERS, from_date="2021-01-01", to_date="2021-12-31")
    assert rep == expected

    # A vlob today is not in the range
    await backend_asgi_app.backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=VlobID.new(),
        timestamp=DateTime.now(),
        blob=b"meow",
    )
    rep = await server_stats(client, HEADERS, from_date="2021-01-01", to_date="2021-12-31")
    assert rep == expected


@pytest.mark.trio
async def test_json_server_stats_revoked_user(backend_asgi_app, alice_ws, alice, bob):
    client = backend_asgi_app.test_client()
    HEADERS = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}
    rep = await server_stats(client, HEADERS, from_date="2021-01-01", to_date="2021-12-31")
    expected = rep

    alice_revocation = RevokedUserCertificate(
        author=alice.device_id, timestamp=DateTime.now(), user_id=bob.user_id
    ).dump_and_sign(alice.signing_key)

    assert await user_revoke(alice_ws, revoked_user_certificate=alice_revocation) == {
        "status": "ok"
    }  # Sanity check
    rep = await server_stats(client, HEADERS, from_date="2021-01-01", to_date="2021-12-31")

    # Bob has been revoked
    expected["stats"][0]["user_per_profiles"]["STANDARD"]["revoked"] += 1
    expected["stats"][0]["user_per_profiles"]["STANDARD"]["active"] -= 1
    assert rep == expected
