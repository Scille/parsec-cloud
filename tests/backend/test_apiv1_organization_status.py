# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Dict
from unittest.mock import ANY
from pendulum import datetime
import pytest

from parsec.api.protocol import (
    apiv1_organization_status_serializer,
    apiv1_organization_update_serializer,
)
from tests.backend.test_apiv1_organization import organization_create
from parsec.backend.backend_events import BackendEvent


async def organization_status(sock, organization_id):
    raw_rep = await sock.send(
        apiv1_organization_status_serializer.req_dumps(
            {"cmd": "organization_status", "organization_id": organization_id}
        )
    )
    raw_rep = await sock.recv()
    return apiv1_organization_status_serializer.rep_loads(raw_rep)


async def organization_update(sock, organization_id, **fields: Dict):
    raw_rep = await sock.send(
        apiv1_organization_update_serializer.req_dumps(
            {"cmd": "organization_update", "organization_id": organization_id, **fields}
        )
    )
    raw_rep = await sock.recv()
    return apiv1_organization_update_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_organization_status_bootstrapped(coolorg, administration_backend_sock):
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": False,
    }


@pytest.mark.trio
async def test_organization_status_not_bootstrapped(
    organization_factory, administration_backend_sock
):
    # 1) Create organization
    neworg = organization_factory("NewOrg")
    rep = await organization_create(administration_backend_sock, neworg.organization_id)
    assert rep == {"status": "ok", "bootstrap_token": ANY}

    # 2) Check its status
    rep = await organization_status(administration_backend_sock, neworg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": False,
        "expiration_date": None,
        "allow_outsider_profile": False,
    }


@pytest.mark.trio
async def test_organization_update_only_expiration_date(
    coolorg, organization_factory, administration_backend_sock, backend
):
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": False,
    }

    rep = await organization_update(
        administration_backend_sock, coolorg.organization_id, expiration_date=datetime(2077, 1, 1)
    )
    assert rep == {"status": "ok"}
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": datetime(2077, 1, 1),
        "allow_outsider_profile": False,
    }

    rep = await organization_update(
        administration_backend_sock, coolorg.organization_id, expiration_date=None
    )
    assert rep == {"status": "ok"}
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": False,
    }


@pytest.mark.trio
async def test_organization_update_expiration_date_with_expired_event(
    coolorg, organization_factory, administration_backend_sock, backend
):
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": False,
    }

    # New expired organization
    with backend.event_bus.listen() as spy:
        rep = await organization_update(
            administration_backend_sock,
            coolorg.organization_id,
            expiration_date=datetime(1999, 12, 31),
        )
        assert rep == {"status": "ok"}
        await spy.wait_with_timeout(BackendEvent.ORGANIZATION_EXPIRED)

        rep = await organization_status(administration_backend_sock, coolorg.organization_id)
        assert rep == {
            "status": "ok",
            "is_bootstrapped": True,
            "expiration_date": datetime(1999, 12, 31),
            "allow_outsider_profile": False,
        }

    # Already Expired organization
    with backend.event_bus.listen() as spy:
        rep = await organization_update(
            administration_backend_sock,
            coolorg.organization_id,
            expiration_date=datetime(2000, 1, 31),
        )
        assert rep == {"status": "ok"}
        await spy.wait_with_timeout(BackendEvent.ORGANIZATION_EXPIRED)
        rep = await organization_status(administration_backend_sock, coolorg.organization_id)
        assert rep == {
            "status": "ok",
            "is_bootstrapped": True,
            "expiration_date": datetime(2000, 1, 31),
            "allow_outsider_profile": False,
        }


@pytest.mark.trio
async def test_organization_update_only_allow_outsider_profile(
    coolorg, organization_factory, administration_backend_sock, backend
):
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": False,
    }

    rep = await organization_update(
        administration_backend_sock, coolorg.organization_id, allow_outsider_profile=True
    )
    assert rep == {"status": "ok"}
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": True,
    }

    rep = await organization_update(
        administration_backend_sock, coolorg.organization_id, allow_outsider_profile=False
    )
    assert rep == {"status": "ok"}
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": False,
    }


@pytest.mark.trio
async def test_organization_update_multiple_fields(
    coolorg, organization_factory, administration_backend_sock, backend
):
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": False,
    }

    rep = await organization_update(
        administration_backend_sock,
        coolorg.organization_id,
        expiration_date=datetime(2077, 1, 1),
        allow_outsider_profile=True,
    )
    assert rep == {"status": "ok"}
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": datetime(2077, 1, 1),
        "allow_outsider_profile": True,
    }

    rep = await organization_update(
        administration_backend_sock,
        coolorg.organization_id,
        expiration_date=None,
        allow_outsider_profile=True,
    )
    assert rep == {"status": "ok"}
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": None,
        "allow_outsider_profile": True,
    }

    rep = await organization_update(
        administration_backend_sock,
        coolorg.organization_id,
        expiration_date=datetime(2077, 1, 1),
        allow_outsider_profile=False,
    )
    assert rep == {"status": "ok"}
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "is_bootstrapped": True,
        "expiration_date": datetime(2077, 1, 1),
        "allow_outsider_profile": False,
    }


@pytest.mark.trio
async def test_organization_update_without_fields(
    coolorg, organization_factory, administration_backend_sock
):
    rep = await organization_update(administration_backend_sock, "dummy")
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_organization_update_unknown_organization(
    coolorg, organization_factory, administration_backend_sock
):
    rep = await organization_update(
        administration_backend_sock, "dummy", expiration_date=datetime(2077, 1, 1)
    )
    assert rep == {"status": "not_found"}

    rep = await organization_update(
        administration_backend_sock, "dummy", allow_outsider_profile=False
    )
    assert rep == {"status": "not_found"}

    rep = await organization_update(
        administration_backend_sock,
        "dummy",
        allow_outsider_profile=False,
        expiration_date=datetime(2077, 1, 1),
    )
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_status_unknown_organization(administration_backend_sock):
    rep = await organization_status(administration_backend_sock, organization_id="dummy")
    assert rep == {"status": "not_found"}
