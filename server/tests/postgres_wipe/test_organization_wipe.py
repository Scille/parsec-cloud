# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools

import pytest

from parsec.backend import Backend
from parsec.components.postgresql.organization import PGOrganizationComponent
from tests.common.client import CoolorgRpcClients, SequesteredOrgRpcClients
from tests.postgres_wipe.utils import (
    fetch_org_id_from_org_name,
    fetch_references_across_tables,
    gen_table_ids,
)


@pytest.mark.postgresql
async def test_wipe_cascade_wipe(
    coolorg: CoolorgRpcClients, sequestered_org: SequesteredOrgRpcClients, backend: Backend
):
    organization = backend.organization
    assert isinstance(organization, PGOrganizationComponent)

    # 1. Acquire related info about organization
    async with organization.pool.acquire() as conn:
        pg_org_ids = (
            await fetch_org_id_from_org_name(conn, coolorg.organization_id),
            await fetch_org_id_from_org_name(conn, sequestered_org.organization_id),
        )

        (
            sequester_service_ids,
            human_ids,
            user_ids,
            device_ids,
            shamir_recovery_setup_ids,
            shamir_recovery_share_ids,
            invitation_ids,
            greeting_attempt_ids,
            async_enrollment_ids,
            realm_ids,
            common_topic_ids,
            sequester_topic_ids,
            shamir_recovery_topic_ids,
            realm_topic_ids,
            cryptpad_session_ids,
        ) = await fetch_references_across_tables(
            conn,
            (
                ("sequester_service", "organization"),
                ("human", "organization"),
                ("user_", "organization"),
                ("device", "organization"),
                ("shamir_recovery_setup", "organization"),
                ("shamir_recovery_share", "organization"),
                ("invitation", "organization"),
                ("greeting_attempt", "organization"),
                ("async_enrollment", "organization"),
                ("realm", "organization"),
                ("common_topic", "organization"),
                ("sequester_topic", "organization"),
                ("shamir_recovery_topic", "organization"),
                ("realm_topic", "organization"),
                ("cryptpad_session", "organization"),
            ),
            pg_org_ids,
        )
        assert sequester_service_ids
        assert human_ids
        assert user_ids
        assert device_ids
        assert not shamir_recovery_setup_ids, "No shamir configured"
        assert not shamir_recovery_share_ids, "No shamir configured"
        assert invitation_ids
        assert not greeting_attempt_ids, "No invitation in progress"
        assert not async_enrollment_ids, "No invitation in progress"
        assert realm_ids
        assert common_topic_ids
        assert sequester_topic_ids
        assert not shamir_recovery_topic_ids, "No shamir configured"
        assert realm_topic_ids
        assert not cryptpad_session_ids, "No cryptpad session"

    # 2. Wipe organization
    async with organization.pool.acquire() as conn:
        await conn.fetchmany(
            "DELETE FROM organization WHERE _id = $1", ((id,) for id in pg_org_ids)
        )

    # 3. Verify that the wipe has cascaded
    async with organization.pool.acquire() as conn:
        for table, id in itertools.chain(
            gen_table_ids("organization", pg_org_ids),
            gen_table_ids("sequester_service", sequester_service_ids),
            gen_table_ids("human", human_ids),
            gen_table_ids("user_", user_ids),
            gen_table_ids("device", device_ids),
            gen_table_ids("shamir_recovery_setup", shamir_recovery_setup_ids),
            gen_table_ids("shamir_recovery_share", shamir_recovery_share_ids),
            gen_table_ids("invitation", invitation_ids),
            gen_table_ids("greeting_attempt", greeting_attempt_ids),
            gen_table_ids("async_enrollment", async_enrollment_ids),
            gen_table_ids("realm", realm_ids),
            gen_table_ids("common_topic", common_topic_ids),
            gen_table_ids("sequester_topic", sequester_topic_ids),
            gen_table_ids("shamir_recovery_topic", shamir_recovery_topic_ids),
            gen_table_ids("realm_topic", realm_topic_ids),
            gen_table_ids("cryptpad_session", cryptpad_session_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted realm"
