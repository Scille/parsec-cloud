# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest

from parsec.backend import Backend
from parsec.components.postgresql.shamir import PGShamirComponent
from tests.common.client import ShamirOrgRpcClients
from tests.postgres_wipe.utils import (
    fetch_org_id_from_org_name,
    fetch_references_across_tables,
    gen_table_ids,
)


@pytest.mark.postgresql
async def test_shamir_cascade_wipe(shamirorg: ShamirOrgRpcClients, backend: Backend):
    shamir = backend.shamir
    assert isinstance(shamir, PGShamirComponent)

    # 1. Acquire related info about shared recoveries
    async with shamir.pool.acquire() as conn:
        pg_org_id = await fetch_org_id_from_org_name(conn, shamirorg.organization_id)
        (
            shamir_recovery_setup_ids,
            shamir_recovery_share_ids,
        ) = await fetch_references_across_tables(
            conn,
            (
                ("shamir_recovery_setup", "organization"),
                ("shamir_recovery_share", "organization"),
            ),
            (pg_org_id,),
        )
        assert shamir_recovery_setup_ids
        assert shamir_recovery_share_ids

    # 2. Wipe shared recoveries
    async with shamir.pool.acquire() as conn:
        await conn.fetchmany(
            "DELETE FROM shamir_recovery_setup WHERE _id = $1",
            ((id,) for id in shamir_recovery_setup_ids),
        )

    # 3. Verify that the wipe has cascaded
    async with shamir.pool.acquire() as conn:
        for table, id in itertools.chain(
            gen_table_ids("shamir_recovery_setup", shamir_recovery_setup_ids),
            gen_table_ids("shamir_recovery_share", shamir_recovery_share_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted shamir"
