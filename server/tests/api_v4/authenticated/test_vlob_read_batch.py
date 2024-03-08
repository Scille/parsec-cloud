# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, DeviceID, OrganizationID, VlobID, authenticated_cmds
from parsec.components.vlob import VLOB_READ_REQUEST_ITEMS_LIMIT
from tests.common import Backend, CoolorgRpcClients


async def create_vlob(
    backend: Backend,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    dt: DateTime,
    versions: int = 1,
) -> VlobID:
    vlob_id = VlobID.new()
    for version in range(1, versions + 1):
        v1_blob = f"<block content v{version}>".encode()
        if version == 1:
            outcome = await backend.vlob.create(
                now=dt,
                organization_id=organization_id,
                author=author,
                realm_id=realm_id,
                vlob_id=vlob_id,
                key_index=1,
                timestamp=dt,
                blob=v1_blob,
                sequester_blob=None,
            )
        else:
            outcome = await backend.vlob.update(
                now=dt,
                organization_id=organization_id,
                author=author,
                vlob_id=vlob_id,
                key_index=1,
                version=version,
                timestamp=dt,
                blob=v1_blob,
                sequester_blob=None,
            )
        assert outcome is None, outcome
        dt = dt.add(days=1)
    return vlob_id


@pytest.mark.parametrize("use_at", (False, True))
async def test_authenticated_vlob_read_batch_ok(
    use_at: bool, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    dt1 = DateTime(2020, 1, 1)
    dt2 = DateTime(2020, 1, 2)
    vlob1_id = await create_vlob(
        backend=backend,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        dt=dt1,
        versions=3,
    )
    vlob2_id = await create_vlob(
        backend=backend,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        dt=dt2,
    )
    await create_vlob(
        backend=backend,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        dt=dt1,
    )

    if use_at:
        rep = await coolorg.alice.vlob_read_batch(
            realm_id=coolorg.wksp1_id, vlobs=[vlob1_id, vlob2_id], at=dt1
        )
        assert rep == authenticated_cmds.v4.vlob_read_batch.RepOk(
            items=[
                (
                    vlob1_id,
                    1,
                    coolorg.alice.device_id,
                    1,
                    DateTime(2020, 1, 1),
                    b"<block content v1>",
                ),
            ],
            needed_common_certificate_timestamp=DateTime(2000, 1, 6),
            needed_realm_certificate_timestamp=DateTime(2000, 1, 12),
        )

    else:
        rep = await coolorg.alice.vlob_read_batch(
            realm_id=coolorg.wksp1_id, vlobs=[vlob1_id, vlob2_id], at=None
        )
        assert rep == authenticated_cmds.v4.vlob_read_batch.RepOk(
            items=[
                (
                    vlob1_id,
                    1,
                    coolorg.alice.device_id,
                    3,
                    DateTime(2020, 1, 3),
                    b"<block content v3>",
                ),
                (
                    vlob2_id,
                    1,
                    coolorg.alice.device_id,
                    1,
                    DateTime(2020, 1, 2),
                    b"<block content v1>",
                ),
            ],
            needed_common_certificate_timestamp=DateTime(2000, 1, 6),
            needed_realm_certificate_timestamp=DateTime(2000, 1, 12),
        )


async def test_authenticated_vlob_read_batch_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep = await coolorg.mallory.vlob_read_batch(
        realm_id=coolorg.wksp1_id, vlobs=[VlobID.new()], at=None
    )
    assert rep == authenticated_cmds.v4.vlob_read_batch.RepAuthorNotAllowed()


async def test_authenticated_vlob_read_batch_realm_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    bad_realm_id = VlobID.new()
    rep = await coolorg.alice.vlob_read_batch(realm_id=bad_realm_id, vlobs=[VlobID.new()], at=None)
    assert rep == authenticated_cmds.v4.vlob_read_batch.RepRealmNotFound()


async def test_authenticated_vlob_read_batch_too_many_elements(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    too_many_vlobs = [VlobID.new()] * (VLOB_READ_REQUEST_ITEMS_LIMIT + 1)
    rep = await coolorg.alice.vlob_read_batch(
        realm_id=coolorg.wksp1_id, vlobs=too_many_vlobs, at=None
    )
    assert rep == authenticated_cmds.v4.vlob_read_batch.RepTooManyElements()
