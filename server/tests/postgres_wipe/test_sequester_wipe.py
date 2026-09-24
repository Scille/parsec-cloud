# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest

from parsec.backend import Backend
from parsec.components.postgresql.sequester import PGSequesterComponent
from tests.common.client import SequesteredOrgRpcClients
from tests.postgres_wipe.utils import (
    fetch_many_row_ids_from_references,
    fetch_org_id_from_org_name,
    gen_table_ids,
)


@pytest.mark.postgresql
async def test_sequester_cascade_wipe(sequestered_org: SequesteredOrgRpcClients, backend: Backend):
    sequester = backend.sequester
    assert isinstance(sequester, PGSequesterComponent)

    # 1. Acquire related info about the sequester
    async with sequester.pool.acquire() as conn:
        pg_org_id = await fetch_org_id_from_org_name(conn, sequestered_org.organization_id)
        sequester_svc_ids = await fetch_many_row_ids_from_references(
            conn, "sequester_service", "organization", (pg_org_id,)
        )
        assert sequester_svc_ids

        keys_bundle_ids = await fetch_many_row_ids_from_references(
            conn, "realm_sequester_keys_bundle_access", "sequester_service", sequester_svc_ids
        )
        assert keys_bundle_ids

    # 2. Wipe the sequester services
    async with sequester.pool.acquire() as conn:
        await conn.fetchmany(
            "DELETE FROM sequester_service WHERE _id = $1", ((id,) for id in sequester_svc_ids)
        )

    # 3. Verify that the wipe has cascaded
    async with sequester.pool.acquire() as conn:
        for table, id in itertools.chain(
            gen_table_ids("sequester_service", sequester_svc_ids),
            gen_table_ids("realm_sequester_keys_bundle_access", keys_bundle_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted sequester"
