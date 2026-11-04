# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest

from parsec.backend import Backend
from parsec.components.postgresql.user import PGUserComponent
from tests.common.client import CoolorgRpcClients
from tests.postgres_wipe.utils import (
    fetch_many_row_ids_from_references,
    fetch_org_id_from_org_name,
    fetch_references_across_tables,
    gen_table_ids,
)


@pytest.mark.postgresql
async def test_user_cascade_wipe(coolorg: CoolorgRpcClients, backend: Backend):
    user = backend.user
    assert isinstance(user, PGUserComponent)

    # 1. Acquire related info about users
    async with user.pool.acquire() as conn:
        pg_org_id = await fetch_org_id_from_org_name(conn, coolorg.organization_id)
        human_ids = await fetch_many_row_ids_from_references(
            conn, "human", "organization", (pg_org_id,)
        )
        assert human_ids
        user_ids = await fetch_many_row_ids_from_references(conn, "user_", "human", human_ids)
        assert user_ids
        (
            profile_ids,
            device_ids,
            invitation_ids,
            greeting_session_ids,
            realm_user_role_ids,
            realm_keys_bundle_access_ids,
        ) = await fetch_references_across_tables(
            conn,
            (
                ("profile", "user_"),
                ("device", "user_"),
                ("invitation", "device_invitation_claimer"),
                ("greeting_session", "greeter"),
                ("realm_user_role", "user_"),
                ("realm_keys_bundle_access", "user_"),
            ),
            human_ids,
        )
        # FIXME: No profile assigned, meaning every users are owner
        assert not profile_ids
        assert device_ids
        assert invitation_ids
        assert not greeting_session_ids, "No invitation in progress"
        assert realm_user_role_ids
        assert realm_keys_bundle_access_ids

    # 2. Wipe human 🤖
    async with user.pool.acquire() as conn:
        await conn.fetchmany("DELETE FROM human WHERE _id = $1", ((id,) for id in human_ids))

    # 3. Verify that the wipe has cascaded
    async with user.pool.acquire() as conn:
        for table, id in itertools.chain(
            gen_table_ids("human", human_ids),
            gen_table_ids("user_", user_ids),
            gen_table_ids("profile", profile_ids),
            gen_table_ids("device", device_ids),
            gen_table_ids("invitation", invitation_ids),
            gen_table_ids("greeting_session", greeting_session_ids),
            gen_table_ids("realm_user_role", realm_user_role_ids),
            gen_table_ids("realm_keys_bundle_access", realm_keys_bundle_access_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted realm"
