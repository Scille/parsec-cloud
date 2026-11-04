# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest

from parsec.backend import Backend
from parsec.components.postgresql.vlob import PGVlobComponent
from tests.common.client import CoolorgRpcClients
from tests.postgres_wipe.utils import (
    fetch_many_row_ids_from_references,
    fetch_org_id_from_org_name,
    fetch_references_across_tables,
    gen_table_ids,
)


@pytest.mark.postgresql
async def test_vlob_cascade_wipe(coolorg: CoolorgRpcClients, backend: Backend):
    vlob = backend.vlob
    assert isinstance(vlob, PGVlobComponent)

    # 1. Acquire related info about the realm vlobs
    async with vlob.pool.acquire() as conn:
        pg_org_id = await fetch_org_id_from_org_name(conn, coolorg.organization_id)
        realm_ids = await fetch_many_row_ids_from_references(
            conn, "realm", "organization", (pg_org_id,)
        )
        assert realm_ids

        vlob_atom_ids, realm_vlob_update_ids = await fetch_references_across_tables(
            conn, (("vlob_atom", "realm"), ("realm_vlob_update", "realm")), realm_ids
        )
        assert vlob_atom_ids

        assert realm_vlob_update_ids

    # 2. Wipe the realm vlobs
    async with vlob.pool.acquire() as conn:
        await conn.fetchmany(
            "DELETE FROM vlob_atom WHERE _id = $1", ((id,) for id in vlob_atom_ids)
        )

    # 3. Verify that the wipe has cascaded
    async with vlob.pool.acquire() as conn:
        for table, id in itertools.chain(
            gen_table_ids("vlob_atom", vlob_atom_ids),
            gen_table_ids("realm_vlob_update", realm_vlob_update_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted vlob"
