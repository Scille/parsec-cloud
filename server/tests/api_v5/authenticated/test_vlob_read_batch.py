# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, DeviceID, OrganizationID, RealmRole, VlobID, authenticated_cmds
from parsec.components.vlob import VLOB_READ_REQUEST_ITEMS_LIMIT
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    get_last_realm_certificate_timestamp,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)


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
            )
        else:
            outcome = await backend.vlob.update(
                now=dt,
                organization_id=organization_id,
                author=author,
                realm_id=realm_id,
                vlob_id=vlob_id,
                key_index=1,
                version=version,
                timestamp=dt,
                blob=v1_blob,
            )
        assert outcome is None, outcome
        dt = dt.add(days=1)
    return vlob_id


@pytest.mark.parametrize(
    "kind",
    (
        "with_use_at_param",
        "as_reader",
        "as_contributor",
        "as_manager",
        "as_owner",
    ),
)
async def test_authenticated_vlob_read_batch_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    use_at = False
    last_realm_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=coolorg.wksp1_id,
    )
    match kind:
        case "with_use_at_param":
            use_at = True
            author = coolorg.bob

        case "as_reader":
            author = coolorg.bob

        case "as_contributor":
            last_realm_certificate_timestamp = DateTime(2019, 1, 1)
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.CONTRIBUTOR,
                now=last_realm_certificate_timestamp,
            )
            author = coolorg.bob

        case "as_manager":
            last_realm_certificate_timestamp = DateTime(2019, 1, 1)
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.MANAGER,
                now=last_realm_certificate_timestamp,
            )
            author = coolorg.bob

        case "as_owner":
            author = coolorg.alice

        case unknown:
            assert False, unknown

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
        rep = await author.vlob_read_batch(
            realm_id=coolorg.wksp1_id, vlobs=[vlob1_id, vlob2_id], at=dt1
        )
        assert rep == authenticated_cmds.latest.vlob_read_batch.RepOk(
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
            needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
        )

    else:
        rep = await author.vlob_read_batch(
            realm_id=coolorg.wksp1_id, vlobs=[vlob1_id, vlob2_id], at=None
        )
        assert rep == authenticated_cmds.latest.vlob_read_batch.RepOk(
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
            needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
        )


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_vlob_read_batch_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "never_allowed":
            author = coolorg.mallory

        case "no_longer_allowed":
            await wksp1_bob_becomes_owner_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_role=None
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    rep = await author.vlob_read_batch(realm_id=coolorg.wksp1_id, vlobs=[VlobID.new()], at=None)
    assert rep == authenticated_cmds.latest.vlob_read_batch.RepAuthorNotAllowed()


async def test_authenticated_vlob_read_batch_realm_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    bad_realm_id = VlobID.new()
    rep = await coolorg.alice.vlob_read_batch(realm_id=bad_realm_id, vlobs=[VlobID.new()], at=None)
    assert rep == authenticated_cmds.latest.vlob_read_batch.RepRealmNotFound()


async def test_authenticated_vlob_read_batch_too_many_elements(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    too_many_vlobs = [VlobID.new()] * (VLOB_READ_REQUEST_ITEMS_LIMIT + 1)
    rep = await coolorg.alice.vlob_read_batch(
        realm_id=coolorg.wksp1_id, vlobs=too_many_vlobs, at=None
    )
    assert rep == authenticated_cmds.latest.vlob_read_batch.RepTooManyElements()


async def test_authenticated_vlob_read_batch_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    vlob_id = VlobID.new()
    v1_blob = b"<block content 1>"
    v1_timestamp = DateTime.now()
    outcome = await backend.vlob.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        blob=v1_blob,
        timestamp=v1_timestamp,
    )
    assert outcome is None, outcome

    async def do():
        await coolorg.alice.vlob_read_batch(
            realm_id=coolorg.wksp1_id, vlobs=[vlob_id], at=v1_timestamp
        )

    await authenticated_http_common_errors_tester(do)
