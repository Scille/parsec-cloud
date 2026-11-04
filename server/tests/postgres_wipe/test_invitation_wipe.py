# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest

from parsec.backend import Backend
from parsec.components.postgresql.vlob import PGVlobComponent
from tests.common.client import CoolorgRpcClients
from tests.postgres_wipe.utils import (
    fetch_many_row_ids_from_references,
    fetch_org_id_from_org_name,
    gen_table_ids,
)


@pytest.mark.postgresql
async def test_invitation_cascade_wipe(coolorg: CoolorgRpcClients, backend: Backend):
    vlob = backend.vlob
    assert isinstance(vlob, PGVlobComponent)

    # 1. Acquire related info about the invitation
    async with vlob.pool.acquire() as conn:
        pg_org_id = await fetch_org_id_from_org_name(conn, coolorg.organization_id)
        invitation_ids = await fetch_many_row_ids_from_references(
            conn, "invitation", "organization", (pg_org_id,)
        )
        assert invitation_ids

        greeting_session_ids = await fetch_many_row_ids_from_references(
            conn, "greeting_session", "invitation", invitation_ids
        )
        assert not greeting_session_ids, "coolorg does not have greeting session"

        greeting_attempt_ids = await fetch_many_row_ids_from_references(
            conn, "greeting_attempt", "greeting_session", greeting_session_ids
        )
        assert not greeting_attempt_ids, "coolorg does not have greeting session"

        greeting_step_ids = await fetch_many_row_ids_from_references(
            conn, "greeting_step", "greeting_attempt", greeting_attempt_ids
        )
        assert not greeting_step_ids, "coolorg does not have greeting session"

    # 2. Wipe the invitation
    async with vlob.pool.acquire() as conn:
        await conn.fetchmany(
            "DELETE FROM invitation WHERE _id = $1", ((id,) for id in invitation_ids)
        )

    # 3. Verify that the wipe has cascaded
    async with vlob.pool.acquire() as conn:
        for table, id in itertools.chain(
            gen_table_ids("invitation", invitation_ids),
            gen_table_ids("greeting_session", greeting_session_ids),
            gen_table_ids("greeting_attempt", greeting_attempt_ids),
            gen_table_ids("greeting_step", greeting_step_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted vlob"
