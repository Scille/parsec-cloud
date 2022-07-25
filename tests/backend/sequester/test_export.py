# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import sqlite3
from pendulum import now as pendulum_now

from parsec.api.protocol import VlobID, RealmID, RealmRole
from parsec.api.protocol.block import BlockID
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.postgresql.sequester_export import RealmExporter, RealmExporterOutputDbError

from tests.common import OrganizationFullData, customize_fixtures, sequester_service_factory


@customize_fixtures(real_data_storage=True, coolorg_is_sequestered_organization=True)
@pytest.mark.postgresql
@pytest.mark.trio
async def test_sequester_export_full_run(
    tmp_path, coolorg: OrganizationFullData, realm, backend, alice, bob
):
    output_db_path = tmp_path / "export.sqlite"

    # Create the sequester service
    s1 = sequester_service_factory(
        authority=coolorg.sequester_authority, label="Sequester service 1"
    )
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s1.backend_service
    )

    # Populate: Realm
    realm1 = RealmID.new()
    await backend.realm.create(
        organization_id=coolorg.organization_id,
        self_granted_role=RealmGrantedRole(
            certificate=b"rolecert1",
            realm_id=realm1,
            user_id=alice.user_id,
            role=RealmRole.OWNER,
            granted_by=alice.device_id,
            granted_on=pendulum_now(),
        ),
    )

    await backend.realm.update_roles(
        organization_id=coolorg.organization_id,
        new_role=RealmGrantedRole(
            certificate=b"rolecert2",
            realm_id=realm1,
            user_id=bob.user_id,
            role=RealmRole.MANAGER,
            granted_by=alice.device_id,
            granted_on=pendulum_now(),
        ),
    )

    # Populate: Vlobs
    vlob1 = VlobID.new()
    vlob2 = VlobID.new()
    await backend.vlob.create(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        realm_id=realm1,
        encryption_revision=1,
        vlob_id=vlob1,
        timestamp=pendulum_now(),
        blob=b"vlob1v1",
        sequester_blob={s1.service_id: b"s1:vlob1v1"},
    )
    await backend.vlob.update(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=vlob1,
        version=2,
        timestamp=pendulum_now(),
        blob=b"vlob1v2",
        sequester_blob={s1.service_id: b"s1:vlob1v2"},
    )
    await backend.vlob.create(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        realm_id=realm1,
        encryption_revision=1,
        vlob_id=vlob2,
        timestamp=pendulum_now(),
        blob=b"vlob2v1",
        sequester_blob={s1.service_id: b"s1:vlob2v1"},
    )

    # Populate: blocks
    block1 = BlockID.new()
    block2 = BlockID.new()
    await backend.block.create(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        block_id=block1,
        realm_id=realm1,
        block=b"block1",
    )
    await backend.block.create(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        block_id=block2,
        realm_id=realm1,
        block=b"block2",
    )

    # Now we can do the actual export !
    async with RealmExporter.run(
        organization_id=coolorg.organization_id,
        realm_id=realm1,
        service_id=s1.service_id,
        output_db_path=output_db_path,
        input_dbh=backend.sequester.dbh,
        input_blockstore=backend.blockstore,
    ) as exporter:

        # Export vlobs
        to_export_count, vlob_batch_offset_marker0 = await exporter.compute_vlobs_export_status()
        assert to_export_count == 3
        assert vlob_batch_offset_marker0 == 0

        vlob_batch_offset_marker1 = await exporter.export_vlobs(batch_size=1)
        assert vlob_batch_offset_marker1 == 1
        vlob_batch_offset_marker2 = await exporter.export_vlobs(
            batch_offset_marker=vlob_batch_offset_marker1
        )
        assert vlob_batch_offset_marker2 == 3

        # Export blocks
        to_export_count, block_batch_offset_marker0 = await exporter.compute_blocks_export_status()
        assert to_export_count == 2
        assert block_batch_offset_marker0 == 0
        block_batch_offset_marker1 = await exporter.export_blocks(batch_size=1)
        assert block_batch_offset_marker1 == 1
        block_batch_offset_marker2 = await exporter.export_blocks(
            batch_offset_marker=block_batch_offset_marker1
        )
        assert block_batch_offset_marker2 == 2

        # Export done, check idempotency for vlobs...
        vlob_batch_offset_marker3 = await exporter.export_vlobs(
            batch_offset_marker=vlob_batch_offset_marker1
        )
        assert vlob_batch_offset_marker3 == 3
        # ...and blocks
        block_batch_offset_marker3 = await exporter.export_blocks(
            batch_offset_marker=block_batch_offset_marker1
        )
        assert block_batch_offset_marker3 == 2

    # Check exported database
    con = sqlite3.connect(f"file:{output_db_path}?mode=ro", uri=True)
    row = con.execute("SELECT magic, version, realm_id from info").fetchone()
    assert row == (87947, 1, realm1.bytes)
    rows = con.execute("SELECT block_id, data from block").fetchall()
    assert rows == [(block1.bytes, b"block1"), (block2.bytes, b"block2")]
    rows = con.execute("SELECT _id, vlob_id, version, blob from vlob_atom").fetchall()
    assert rows == [
        (1, vlob1.bytes, 1, b"s1:vlob1v1"),
        (2, vlob1.bytes, 2, b"s1:vlob1v2"),
        (3, vlob2.bytes, 1, b"s1:vlob2v1"),
    ]

    row = con.execute("SELECT count(*) from realm_role").fetchone()
    assert row[0] == 2  # Contains alice's OWNER role and bob MANAGER roles on realm1
    row = con.execute("SELECT count(*) from user_").fetchone()
    assert row[0] == 3  # Contains alice, bob and adam
    row = con.execute("SELECT count(*) from device").fetchone()
    assert row[0] == 4  # Contains alice@dev1, alice@dev2, bob@dev1 and adam@dev1

    # Also check for idempotency with a different realm exporter
    async with RealmExporter.run(
        organization_id=coolorg.organization_id,
        realm_id=realm1,
        service_id=s1.service_id,
        output_db_path=output_db_path,
        input_dbh=backend.sequester.dbh,
        input_blockstore=backend.blockstore,
    ) as exporter:
        await exporter.export_vlobs()
        await exporter.export_blocks()

    # Exporting a different realm on the same database export should fail
    realm2 = RealmID.new()
    await backend.realm.create(
        organization_id=coolorg.organization_id,
        self_granted_role=RealmGrantedRole(
            certificate=b"cert2",
            realm_id=realm2,
            user_id=alice.user_id,
            role=RealmRole.OWNER,
            granted_by=alice.device_id,
            granted_on=pendulum_now(),
        ),
    )
    with pytest.raises(RealmExporterOutputDbError):
        async with RealmExporter.run(
            organization_id=coolorg.organization_id,
            realm_id=realm2,
            service_id=s1.service_id,
            output_db_path=output_db_path,
            input_dbh=backend.sequester.dbh,
            input_blockstore=backend.blockstore,
        ) as exporter:
            pass

    # The export script can detect missing items and update the export
    await backend.realm.update_roles(
        organization_id=coolorg.organization_id,
        new_role=RealmGrantedRole(
            certificate=b"rolecert3",
            realm_id=realm1,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=pendulum_now(),
        ),
    )
    await backend.vlob.update(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=vlob1,
        version=3,
        timestamp=pendulum_now(),
        blob=b"vlob1v3",
        sequester_blob={s1.service_id: b"s1:vlob1v3"},
    )
    vlob3 = VlobID.new()
    await backend.vlob.create(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        realm_id=realm1,
        encryption_revision=1,
        vlob_id=vlob3,
        timestamp=pendulum_now(),
        blob=b"vlob3v1",
        sequester_blob={s1.service_id: b"s1:vlob3v1"},
    )
    block3 = BlockID.new()
    await backend.block.create(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        block_id=block3,
        realm_id=realm1,
        block=b"block3",
    )
    async with RealmExporter.run(
        organization_id=coolorg.organization_id,
        realm_id=realm1,
        service_id=s1.service_id,
        output_db_path=output_db_path,
        input_dbh=backend.sequester.dbh,
        input_blockstore=backend.blockstore,
    ) as exporter:

        # Export vlobs
        to_export_count, vlob_batch_offset_marker0 = await exporter.compute_vlobs_export_status()
        assert to_export_count == 5
        assert vlob_batch_offset_marker0 == 3
        vlob_batch_offset_marker1 = await exporter.export_vlobs(
            batch_offset_marker=vlob_batch_offset_marker0
        )
        assert vlob_batch_offset_marker1 == 5

        # Export blocks
        to_export_count, block_batch_offset_marker0 = await exporter.compute_blocks_export_status()
        assert to_export_count == 3
        assert block_batch_offset_marker0 == 2
        block_batch_offset_marker1 = await exporter.export_blocks(
            batch_offset_marker=block_batch_offset_marker1
        )
        assert block_batch_offset_marker1 == 3

    # Again, check exported database
    con = sqlite3.connect(f"file:{output_db_path}?mode=ro", uri=True)
    row = con.execute("SELECT magic, version, realm_id from info").fetchone()
    assert row == (87947, 1, realm1.bytes)
    rows = con.execute("SELECT block_id, data from block").fetchall()
    assert rows == [(block1.bytes, b"block1"), (block2.bytes, b"block2"), (block3.bytes, b"block3")]
    rows = con.execute("SELECT _id, vlob_id, version, blob from vlob_atom").fetchall()
    assert rows == [
        (1, vlob1.bytes, 1, b"s1:vlob1v1"),
        (2, vlob1.bytes, 2, b"s1:vlob1v2"),
        (3, vlob2.bytes, 1, b"s1:vlob2v1"),
        (4, vlob1.bytes, 3, b"s1:vlob1v3"),
        (5, vlob3.bytes, 1, b"s1:vlob3v1"),
    ]
    row = con.execute("SELECT count(*) from realm_role").fetchone()
    assert row[0] == 3  # Contains alice's OWNER role and bob MANAGER&None roles on realm1
    row = con.execute("SELECT count(*) from user_").fetchone()
    assert row[0] == 3  # Contains alice, bob and adam
    row = con.execute("SELECT count(*) from device").fetchone()
    assert row[0] == 4  # Contains alice@dev1, alice@dev2, bob@dev1 and adam@dev1
