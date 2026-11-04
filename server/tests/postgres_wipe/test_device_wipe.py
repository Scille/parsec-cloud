# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest

from parsec.backend import Backend
from parsec.components.postgresql.organization import PGOrganizationComponent
from tests.common.client import CoolorgRpcClients
from tests.postgres_wipe.utils import (
    fetch_many_row_ids_from_references,
    fetch_org_id_from_org_name,
    fetch_references_across_tables,
    gen_table_ids,
)


@pytest.mark.postgresql
async def test_device_cascade_wipe(coolorg: CoolorgRpcClients, backend: Backend):
    orgs = backend.organization
    assert isinstance(orgs, PGOrganizationComponent)

    coolorg.alice.device_id

    # 1. Acquire related info about the device
    async with orgs.pool.acquire() as conn:
        pg_org_id = await fetch_org_id_from_org_name(conn, coolorg.organization_id)
        dev_ids = await fetch_many_row_ids_from_references(
            conn, "device", "organization", (pg_org_id,)
        )
        assert len(dev_ids) > 1
        print(dev_ids)

        (
            user_ids,
            revoked_user_ids,
            profile_ids,
            block_ids,
            vlob_atom_ids,
            realm_name_ids,
            cryptpad_session_ids,
        ) = await fetch_references_across_tables(
            conn,
            (
                ("user_", "user_certifier"),
                ("user_", "revoked_user_certifier"),
                ("profile", "certified_by"),
                ("block", "author"),
                ("vlob_atom", "author"),
                ("realm_name", "certified_by"),
                ("cryptpad_session", "author"),
            ),
            dev_ids,
        )
        # FIXME: All users on coolorg are the first user: there can only be one
        assert not user_ids
        assert not revoked_user_ids, "no user are revoked"
        # FIXME: No profile assigned, meaning every users are owner
        assert not profile_ids
        assert not block_ids, "no data added"
        assert vlob_atom_ids
        assert realm_name_ids
        assert not cryptpad_session_ids, "no cryptpad session currently active"

    # 2. Wipe the device
    async with orgs.pool.acquire() as conn:
        await conn.fetchmany("DELETE FROM device WHERE _id = $1", ((id,) for id in dev_ids))

    # 3. Verify that the wipe has cascaded
    async with orgs.pool.acquire() as conn:
        for table, id in itertools.chain(
            gen_table_ids("device", dev_ids),
            gen_table_ids("user", user_ids),
            gen_table_ids("revoked_user", revoked_user_ids),
            gen_table_ids("profile", profile_ids),
            gen_table_ids("block", block_ids),
            gen_table_ids("vlob_atom", vlob_atom_ids),
            gen_table_ids("realm_name", realm_name_ids),
            gen_table_ids("cryptpad_session", cryptpad_session_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted vlob"
