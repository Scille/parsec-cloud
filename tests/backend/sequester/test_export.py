# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import sqlite3
from pendulum import now as pendulum_now, datetime, DateTime

from parsec.api.protocol import VlobID, RealmID, RealmRole, BlockID, OrganizationID
from parsec.api.protocol.sequester import SequesterServiceID
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.postgresql.sequester_export import (
    RealmExporter,
    RealmExporterOutputDbError,
    RealmExporterInputError,
)

from tests.common import OrganizationFullData, customize_fixtures, sequester_service_factory


@customize_fixtures(
    real_data_storage=True, coolorg_is_sequestered_organization=True, adam_is_revoked=True
)
@pytest.mark.postgresql
@pytest.mark.trio
async def test_sequester_export_full_run(
    tmp_path, coolorg: OrganizationFullData, backend, alice, alice2, bob, adam, otherorg
):
    curr_now = datetime(2000, 1, 1)

    def _next_day() -> DateTime:
        nonlocal curr_now
        curr_now = curr_now.add(days=1)
        return curr_now

    def _sqlite_timestamp(year: int, month: int, day: int) -> int:
        return int(datetime(year, month, day).timestamp() * 1000)

    output_db_path = tmp_path / "export.sqlite"

    # Create the sequester service
    s1 = sequester_service_factory(
        authority=coolorg.sequester_authority,
        label="Sequester service 1",
        timestamp=curr_now,  # 2000/1/1
    )
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s1.backend_service, now=curr_now
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
            granted_on=_next_day(),  # 2000/1/2
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
            granted_on=_next_day(),  # 2000/1/3
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
        timestamp=_next_day(),  # 2000/1/4
        blob=b"vlob1v1",
        # Note sequester blob can have a different size than the regular blob !
        sequester_blob={s1.service_id: b"s1:vlob1v1"},
    )
    await backend.vlob.update(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=vlob1,
        version=2,
        timestamp=_next_day(),  # 2000/1/5
        blob=b"vlob1v2",
        sequester_blob={s1.service_id: b"s1:vlob1v2"},
    )
    await backend.vlob.create(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        realm_id=realm1,
        encryption_revision=1,
        vlob_id=vlob2,
        timestamp=_next_day(),  # 2000/1/6
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

    # 1) info table
    row = con.execute("SELECT magic, version, realm_id from info").fetchone()
    assert row == (87947, 1, realm1.bytes)

    # 2) realm_role table
    rows = con.execute("SELECT _id, role_certificate from realm_role").fetchall()
    # SQLite does dynamic typing, so better be careful
    for row in rows:
        assert isinstance(row[0], int)  # _id
        assert isinstance(row[1], bytes)  # role_certificate
    assert len(rows) == 2  # Contains alice's OWNER role and bob MANAGER roles on realm1
    assert {row[1] for row in rows} == {b"rolecert1", b"rolecert2"}
    assert len({row[0] for row in rows}) == 2  # Make sure all ids are unique

    # 3) user table
    rows = con.execute(
        "SELECT _id, user_certificate, revoked_user_certificate FROM user_"
    ).fetchall()
    # SQLite does dynamic typing, so better be careful
    for row in rows:
        assert isinstance(row[0], int)  # _id
        assert isinstance(row[1], bytes)  # user_certificate
        assert row[2] is None or isinstance(row[2], bytes)  # revoked_user_certificate
    assert len(rows) == 3  # Contains alice, bob and adam
    user_ids = set()
    for device in (alice, bob, adam):
        b_user = await backend.user.get_user(
            organization_id=device.organization_id, user_id=device.user_id
        )
        row = next(row for row in rows if row[1] == b_user.user_certificate)
        user_ids.add(row[0])
        assert row[2] == b_user.revoked_user_certificate
    assert len(user_ids) == 3  # Make sure all ids are unique

    # 4) device table
    rows = con.execute("SELECT _id, device_certificate FROM device").fetchall()
    # SQLite does dynamic typing, so better be careful
    for row in rows:
        assert isinstance(row[0], int)  # _id
        assert isinstance(row[1], bytes)  # device_certificate
    assert len(rows) == 4  # Contains alice@dev1, alice@dev2, bob@dev1 and adam@dev1
    device_ids = set()
    alice_internal_id = None
    for device in (alice, alice2, bob, adam):
        _, b_device = await backend.user.get_user_with_device(
            organization_id=device.organization_id, device_id=device.device_id
        )
        row = next(row for row in rows if row[1] == b_device.device_certificate)
        device_ids.add(row[0])
        if device is alice:
            alice_internal_id = row[0]
    assert len(device_ids) == 4  # Make sure all ids are unique
    assert isinstance(alice_internal_id, int)  # Sanity check

    # 5) block table
    rows = con.execute("SELECT _id, block_id, data, author from block").fetchall()
    assert rows == [
        (1, block1.bytes, b"block1", alice_internal_id),
        (2, block2.bytes, b"block2", alice_internal_id),
    ]

    # 5) vlob table
    rows = con.execute(
        "SELECT _id, vlob_id, version, blob, author, timestamp from vlob_atom"
    ).fetchall()
    assert rows == [
        (1, vlob1.bytes, 1, b"s1:vlob1v1", alice_internal_id, _sqlite_timestamp(2000, 1, 4)),
        (2, vlob1.bytes, 2, b"s1:vlob1v2", alice_internal_id, _sqlite_timestamp(2000, 1, 5)),
        (3, vlob2.bytes, 1, b"s1:vlob2v1", alice_internal_id, _sqlite_timestamp(2000, 1, 6)),
    ]

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

    # Bonus points: check errors handling on invalid input

    default_args = {
        "organization_id": coolorg.organization_id,
        "realm_id": realm1,
        "service_id": s1.service_id,
        "output_db_path": output_db_path,
        "input_dbh": backend.sequester.dbh,
        "input_blockstore": backend.blockstore,
    }
    # Unknown organization
    dummy_org = OrganizationID("Dummy")
    with pytest.raises(RealmExporterInputError):
        async with RealmExporter.run(**{**default_args, "organization_id": dummy_org}):
            pass
    # Not boostrapped organization
    await backend.organization.create(id=dummy_org, bootstrap_token="")
    with pytest.raises(RealmExporterInputError):
        async with RealmExporter.run(**{**default_args, "organization_id": dummy_org}):
            pass
    # Unknown realm
    with pytest.raises(RealmExporterInputError):
        async with RealmExporter.run(**{**default_args, "realm_id": RealmID.new()}):
            pass
    # Unknown sequester service
    with pytest.raises(RealmExporterInputError):
        async with RealmExporter.run(**{**default_args, "service_id": SequesterServiceID.new()}):
            pass
    # Non sequestered organization
    with pytest.raises(RealmExporterInputError):
        async with RealmExporter.run(
            **{**default_args, "organization_id": otherorg.organization_id}
        ):
            pass
