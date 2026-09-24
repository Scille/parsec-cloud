# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest
from asyncpg import Record
from asyncpg.pool import PoolConnectionProxy

from parsec._parsec import BlockID, DateTime
from parsec._parsec import testbed as tb
from parsec.backend import Backend
from parsec.components.postgresql.block import PGBlockComponent
from tests.common.client import CoolorgRpcClients
from tests.postgres_wipe.test_account_wipe import fetch_many_row_ids_from_references
from tests.postgres_wipe.test_sequester_wipe import fetch_org_id_from_org_name

type Conn = PoolConnectionProxy[Record]


@pytest.mark.postgresql
async def test_block_cascade_wipe(coolorg: CoolorgRpcClients, backend: Backend):
    block = backend.block
    assert isinstance(block, PGBlockComponent)

    new_wksp = next(
        ev for ev in coolorg.testbed_template.events if isinstance(ev, tb.TestbedEventNewRealm)
    )

    key_bundle = next(
        ev
        for ev in coolorg.testbed_template.events
        if isinstance(ev, tb.TestbedEventRotateKeyRealm) and ev.realm == new_wksp.realm_id
    )
    res = await block.create(
        DateTime.now(),
        coolorg.organization_id,
        coolorg.alice.device_id,
        new_wksp.realm_id,
        BlockID.new(),
        key_bundle.key_index,
        b"Lorem ipsum dolor sit amet quis.",
    )
    assert res is None

    # 1. Acquire related info about block
    async with block.pool.acquire() as conn:
        pg_org_id = await fetch_org_id_from_org_name(conn, coolorg.organization_id)
        realm_ids = await fetch_many_row_ids_from_references(
            conn, "realm", "organization", (pg_org_id,)
        )
        assert realm_ids

        block_ids = await fetch_many_row_ids_from_references(conn, "block", "realm", realm_ids)
        assert block_ids

    # 2. Wipe the block
    async with block.pool.acquire() as conn:
        await conn.fetchmany("DELETE FROM block WHERE _id = $1", ((id,) for id in block_ids))

    # 3. Verify that the wipe has cascaded
    async with block.pool.acquire() as conn:
        for table, id in itertools.chain(
            (("block", id) for id in block_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted block"
