# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from unittest.mock import ANY

import pytest

from parsec.api.protocol import organization_status_serializer
from tests.backend.test_organization import organization_create


async def organization_status(sock, organization_id):
    raw_rep = await sock.send(
        organization_status_serializer.req_dumps(
            {"cmd": "organization_status", "organization_id": organization_id}
        )
    )
    raw_rep = await sock.recv()
    return organization_status_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_organization_status_bootstrapped(coolorg, administration_backend_sock):
    rep = await organization_status(administration_backend_sock, coolorg.organization_id)
    assert rep == {"status": "ok", "is_bootstrapped": True}


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
    assert rep == {"status": "ok", "is_bootstrapped": False}


@pytest.mark.trio
async def test_status_unknown_organization(administration_backend_sock):
    rep = await organization_status(administration_backend_sock, organization_id="dummy")
    assert rep == {"status": "not_found"}
