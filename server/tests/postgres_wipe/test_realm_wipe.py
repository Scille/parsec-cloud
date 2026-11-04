# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest

from parsec.backend import Backend
from parsec.components.postgresql.realm import PGRealmComponent
from tests.common.client import SequesteredOrgRpcClients
from tests.postgres_wipe.utils import (
    fetch_many_row_ids_from_references,
    fetch_org_id_from_org_name,
    fetch_references_across_tables,
    gen_table_ids,
)


@pytest.mark.postgresql
async def test_realm_cascade_wipe(sequestered_org: SequesteredOrgRpcClients, backend: Backend):
    realm = backend.realm
    assert isinstance(realm, PGRealmComponent)

    # 1. Acquire related info about realms
    async with realm.pool.acquire() as conn:
        pg_org_id = await fetch_org_id_from_org_name(conn, sequestered_org.organization_id)
        realm_ids = await fetch_many_row_ids_from_references(
            conn, "realm", "organization", (pg_org_id,)
        )
        assert realm_ids
        (
            realm_archiving_ids,
            realm_keys_bundle_access_ids,
            realm_keys_bundle_ids,
            realm_name_ids,
            realm_sequester_keys_bundle_access_ids,
            realm_user_role_ids,
            realm_topic_ids,
        ) = await fetch_references_across_tables(
            conn,
            (
                ("realm_archiving", "realm"),
                ("realm_keys_bundle_access", "realm"),
                ("realm_keys_bundle", "realm"),
                ("realm_name", "realm"),
                ("realm_sequester_keys_bundle_access", "realm"),
                ("realm_user_role", "realm"),
                ("realm_topic", "realm"),
            ),
            realm_ids,
        )
        assert not realm_archiving_ids, "coolorg does not have archived realm"
        assert realm_keys_bundle_access_ids
        assert realm_keys_bundle_ids
        assert realm_name_ids
        assert realm_sequester_keys_bundle_access_ids
        assert realm_user_role_ids
        assert realm_topic_ids

    # 2. Wipe realms
    async with realm.pool.acquire() as conn:
        await conn.fetchmany("DELETE FROM realm WHERE _id = $1", ((id,) for id in realm_ids))

    # 3. Verify that the wipe has cascaded
    async with realm.pool.acquire() as conn:
        for table, id in itertools.chain(
            gen_table_ids("realm", realm_ids),
            gen_table_ids("realm_archiving", realm_archiving_ids),
            gen_table_ids("realm_keys_bundle_access", realm_keys_bundle_access_ids),
            gen_table_ids("realm_keys_bundle", realm_keys_bundle_ids),
            gen_table_ids("realm_name", realm_name_ids),
            gen_table_ids(
                "realm_sequester_keys_bundle_access", realm_sequester_keys_bundle_access_ids
            ),
            gen_table_ids("realm_user_role", realm_user_role_ids),
            gen_table_ids("realm_topic", realm_topic_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted realm"
