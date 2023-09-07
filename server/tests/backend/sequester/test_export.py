# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import sqlite3

import pytest

from parsec._parsec import DateTime, HashDigest, SecretKey, SequesterPrivateKeyDer
from parsec.api.data import (
    BlockAccess,
    DeviceCertificate,
    EntryID,
    EntryName,
    FileManifest,
    FolderManifest,
    RealmRoleCertificate,
    RevokedUserCertificate,
    UserCertificate,
    WorkspaceManifest,
)
from parsec.api.protocol import (
    BlockID,
    OrganizationID,
    RealmID,
    RealmRole,
    SequesterServiceID,
    UserProfile,
    VlobID,
)
from parsec.backend.postgresql.sequester_export import (
    OUTPUT_DB_INIT_QUERY,
    RealmExporter,
    RealmExporterInputError,
    RealmExporterOutputDbError,
)
from parsec.backend.realm import RealmGrantedRole
from parsec.sequester_export_reader import extract_workspace
from tests.common import OrganizationFullData, customize_fixtures, sequester_service_factory


@customize_fixtures(coolorg_is_sequestered_organization=True, adam_is_revoked=True)
@pytest.mark.postgresql
@pytest.mark.trio
async def test_sequester_export_full_run(
    tmp_path, coolorg: OrganizationFullData, backend, alice, alice2, bob, adam, other_org
):
    curr_now = DateTime(2000, 1, 1)

    def _next_day() -> DateTime:
        nonlocal curr_now
        curr_now = curr_now.add(days=1)
        return curr_now

    def _sqlite_timestamp(year: int, month: int, day: int) -> int:
        return int(DateTime(year, month, day).timestamp() * 1000000)

    output_db_path = tmp_path / "export.sqlite"

    # Create the sequester service
    s1 = sequester_service_factory(
        authority=coolorg.sequester_authority,
        label="Sequester service 1",
        timestamp=curr_now,  # 2000/1/1
    )
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s1.backend_service
    )

    # Populate: Realm
    realm1 = RealmID.new()
    await backend.realm.create(
        organization_id=coolorg.organization_id,
        self_granted_role=RealmGrantedRole(
            certificate=b"role_cert1",
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
            certificate=b"role_cert2",
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
        created_on=_next_day(),  # 2000/1/7
        block=b"block1",
    )
    await backend.block.create(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        block_id=block2,
        realm_id=realm1,
        created_on=_next_day(),  # 2000/1/8
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
    assert {row[1] for row in rows} == {b"role_cert1", b"role_cert2"}
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
            granted_on=DateTime.now(),
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
            certificate=b"role_cert3",
            realm_id=realm1,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=DateTime.now(),
        ),
    )
    await backend.vlob.update(
        organization_id=coolorg.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=vlob1,
        version=3,
        timestamp=DateTime.now(),
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
        timestamp=DateTime.now(),
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
    # Not bootstrapped organization
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
            **{**default_args, "organization_id": other_org.organization_id}
        ):
            pass


@pytest.mark.trio
async def test_export_reader_full_run(tmp_path, coolorg: OrganizationFullData, alice, bob, adam):
    output_db_path = tmp_path / "export.sqlite"
    realm1 = RealmID.new()
    # Don't use such a small key size in real world, this is only for test !
    # (RSA key generation gets ~10x slower between 1024 and 4096)
    service_decryption_key, service_encryption_key = SequesterPrivateKeyDer.generate_pair(1024)

    # Generate the export db by hand here
    con = sqlite3.connect(output_db_path)
    con.executescript(OUTPUT_DB_INIT_QUERY)

    con.execute(
        "INSERT INTO info (realm_id, root_verify_key) VALUES (?, ?)",
        (realm1.bytes, coolorg.root_verify_key.encode()),
    )

    # Timelapse:
    #
    # 2000-01-01: Orga bootstrapped by Alice
    # 2000-01-02: Bob created by Alice
    # 2000-01-03: Adam created by Alice
    #
    # 2000-02-01: Realm created by Alice
    # 2000-02-02: Bob got READER Realm access from Alice
    # 2000-02-03: Bob got MANAGER Realm access from Alice
    # 2000-02-04: Adam got CONTRIBUTOR Realm access from Bob
    #
    # 2000-03-01: Alice upload workspace manifest v1 (no children)
    # 2000-03-10: Bob upload /file1's manifest v1 (empty file)
    # 2000-03-11: Bob upload /folder1's manifest v1 (empty file)
    # 2000-03-12: Bob upload /folder2's manifest v1 (empty file)
    # 2000-03-13: Bob upload workspace manifest v2 (containing file1 and folder1&2)
    # 2000-03-20: Adam upload /folder2/file2's manifest v1 (empty file)
    # 2000-03-21: Adam upload /folder2/folder3's manifest v1 (no children)
    # 2000-03-22: Adam upload /folder2's manifest v2 (containing file2 and folder3)
    # 2000-03-23: Adam upload file2block1
    # 2000-03-24: Adam upload file2block2
    # 2000-03-25: Adam upload /folder2/file2's manifest v2 (containing file2block1&2)
    #
    # 2000-04-01: Adam removed from Realm access by Alice
    # 2000-05-01: Adam revoked by Alice

    # Populate `user_` table
    alice_user_certif = UserCertificate(
        user_id=alice.user_id,
        profile=UserProfile.ADMIN,
        human_handle=alice.human_handle,
        public_key=alice.public_key,
        author=None,
        timestamp=DateTime(2000, 1, 1),
    ).dump_and_sign(coolorg.root_signing_key)
    bob_user_certif = UserCertificate(
        user_id=bob.user_id,
        profile=UserProfile.STANDARD,
        human_handle=bob.human_handle,
        public_key=bob.public_key,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 2),
    ).dump_and_sign(alice.signing_key)
    adam_user_certif = UserCertificate(
        user_id=adam.user_id,
        profile=UserProfile.STANDARD,
        human_handle=adam.human_handle,
        public_key=adam.public_key,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 3),
    ).dump_and_sign(alice.signing_key)
    adam_revoked_user_certif = RevokedUserCertificate(
        user_id=adam.user_id, author=alice.device_id, timestamp=DateTime(2000, 5, 1)
    ).dump_and_sign(alice.signing_key)
    con.executemany(
        "INSERT INTO user_(_id, user_certificate, revoked_user_certificate) VALUES (?, ?, ?)",
        [
            (1, alice_user_certif, None),
            (2, bob_user_certif, None),
            (3, adam_user_certif, adam_revoked_user_certif),
        ],
    )

    # Populate `device` table
    alice_device_certif = DeviceCertificate(
        device_id=alice.device_id,
        device_label=alice.device_label,
        verify_key=alice.verify_key,
        author=None,
        timestamp=DateTime(2000, 1, 1),
    ).dump_and_sign(coolorg.root_signing_key)
    bob_device_certif = DeviceCertificate(
        device_id=bob.device_id,
        device_label=bob.device_label,
        verify_key=bob.verify_key,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 2),
    ).dump_and_sign(alice.signing_key)
    adam_device_certif = DeviceCertificate(
        device_id=adam.device_id,
        device_label=adam.device_label,
        verify_key=adam.verify_key,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 3),
    ).dump_and_sign(alice.signing_key)
    alice_device_internal_id = 1
    bob_device_internal_id = 2
    adam_device_internal_id = 3
    con.executemany(
        "INSERT INTO device(_id, device_certificate) VALUES (?, ?)",
        [
            (alice_device_internal_id, alice_device_certif),
            (bob_device_internal_id, bob_device_certif),
            (adam_device_internal_id, adam_device_certif),
        ],
    )

    # Populate `realm_role` table
    realm_roles = [
        RealmRoleCertificate(
            realm_id=realm1,
            user_id=alice.user_id,
            role=RealmRole.OWNER,
            timestamp=DateTime(2000, 2, 1),
            author=None,
        ).dump_and_sign(coolorg.root_signing_key),
        RealmRoleCertificate(
            realm_id=realm1,
            user_id=bob.user_id,
            role=RealmRole.READER,
            timestamp=DateTime(2000, 2, 2),
            author=alice.device_id,
        ).dump_and_sign(alice.signing_key),
        RealmRoleCertificate(
            realm_id=realm1,
            user_id=bob.user_id,
            role=RealmRole.MANAGER,
            timestamp=DateTime(2000, 2, 3),
            author=alice.device_id,
        ).dump_and_sign(alice.signing_key),
        RealmRoleCertificate(
            realm_id=realm1,
            user_id=adam.user_id,
            role=RealmRole.CONTRIBUTOR,
            timestamp=DateTime(2000, 2, 4),
            author=bob.device_id,
        ).dump_and_sign(bob.signing_key),
        RealmRoleCertificate(
            realm_id=realm1,
            user_id=adam.user_id,
            role=None,
            timestamp=DateTime(2000, 4, 1),
            author=alice.device_id,
        ).dump_and_sign(alice.signing_key),
    ]
    con.executemany(
        "INSERT INTO realm_role(_id, role_certificate) VALUES (?, ?)", enumerate(realm_roles)
    )

    # Populate `block` table
    file2block1 = BlockID.new()
    file2block1_key = SecretKey.generate()
    file2block1_data = file2block1_key.encrypt(b"a" * 10)
    file2block1_digest = HashDigest.from_data(b"a" * 10)
    file2block2 = BlockID.new()
    file2block2_key = SecretKey.generate()
    file2block2_data = file2block2_key.encrypt(b"b" * 10)
    file2block2_digest = HashDigest.from_data(b"b" * 10)

    blocks = [
        (1, file2block1.bytes, file2block1_data, alice_device_internal_id),
        (2, file2block2.bytes, file2block2_data, alice_device_internal_id),
    ]
    con.executemany("INSERT INTO block(_id, block_id, data, author) VALUES (?, ?, ?, ?)", blocks)

    # Populate `vlob` table
    workspace_id = realm1.to_entry_id()
    file1 = EntryID.new()
    file2 = EntryID.new()
    folder1 = EntryID.new()
    folder2 = EntryID.new()
    folder3 = EntryID.new()
    workspace_manifest_v1 = WorkspaceManifest(
        author=alice.device_id,
        timestamp=DateTime(2000, 3, 1),
        version=1,
        id=workspace_id,
        created=DateTime(2000, 3, 1),
        updated=DateTime(2000, 3, 1),
        children={},
    ).dump_and_sign(author_signkey=alice.signing_key)
    file1_manifest_v1 = FileManifest(
        author=bob.device_id,
        timestamp=DateTime(2000, 3, 10),
        version=1,
        id=file1,
        parent=workspace_id,
        created=DateTime(2000, 3, 10),
        updated=DateTime(2000, 3, 10),
        size=0,
        blocksize=10,
        blocks=[],
    ).dump_and_sign(author_signkey=bob.signing_key)
    folder1_manifest_v1 = FolderManifest(
        author=bob.device_id,
        timestamp=DateTime(2000, 3, 11),
        version=1,
        id=folder1,
        parent=workspace_id,
        created=DateTime(2000, 3, 11),
        updated=DateTime(2000, 3, 11),
        children={},
    ).dump_and_sign(author_signkey=bob.signing_key)
    folder2_manifest_v1 = FolderManifest(
        author=bob.device_id,
        timestamp=DateTime(2000, 3, 12),
        version=1,
        id=folder2,
        parent=workspace_id,
        created=DateTime(2000, 3, 12),
        updated=DateTime(2000, 3, 12),
        children={},
    ).dump_and_sign(author_signkey=bob.signing_key)
    workspace_manifest_v2 = WorkspaceManifest(
        author=bob.device_id,
        timestamp=DateTime(2000, 3, 13),
        version=2,
        id=workspace_id,
        created=DateTime(2000, 3, 13),
        updated=DateTime(2000, 3, 13),
        children={
            EntryName("file1"): file1,
            EntryName("folder1"): folder1,
            EntryName("folder2"): folder2,
        },
    ).dump_and_sign(author_signkey=bob.signing_key)
    file2_manifest_v1 = FileManifest(
        author=adam.device_id,
        timestamp=DateTime(2000, 3, 20),
        version=1,
        id=file2,
        parent=folder2,
        created=DateTime(2000, 3, 20),
        updated=DateTime(2000, 3, 20),
        size=0,
        blocksize=10,
        blocks=[],
    ).dump_and_sign(author_signkey=adam.signing_key)
    folder3_manifest_v1 = FolderManifest(
        author=adam.device_id,
        timestamp=DateTime(2000, 3, 21),
        version=1,
        id=folder3,
        parent=workspace_id,
        created=DateTime(2000, 3, 21),
        updated=DateTime(2000, 3, 21),
        children={},
    ).dump_and_sign(author_signkey=adam.signing_key)
    folder2_manifest_v2 = FolderManifest(
        author=adam.device_id,
        timestamp=DateTime(2000, 3, 22),
        version=2,
        id=folder2,
        parent=workspace_id,
        created=DateTime(2000, 3, 22),
        updated=DateTime(2000, 3, 22),
        children={EntryName("file2"): file2, EntryName("folder3"): folder3},
    ).dump_and_sign(author_signkey=adam.signing_key)
    file2_manifest_v2 = FileManifest(
        author=adam.device_id,
        timestamp=DateTime(2000, 3, 25),
        version=2,
        id=file2,
        parent=folder2,
        created=DateTime(2000, 3, 25),
        updated=DateTime(2000, 3, 25),
        size=20,
        blocksize=10,
        blocks=[
            BlockAccess(
                id=file2block1, key=file2block1_key, offset=0, size=10, digest=file2block1_digest
            ),
            BlockAccess(
                id=file2block2, key=file2block2_key, offset=10, size=10, digest=file2block2_digest
            ),
        ],
    ).dump_and_sign(author_signkey=adam.signing_key)

    def _sqlite_ts(year, month, day):
        return int(DateTime(year, month, day).timestamp() * 1000000)

    vlob_atoms = [
        # / v1
        (
            1,
            workspace_id.bytes,
            1,
            service_encryption_key.encrypt(workspace_manifest_v1),
            alice_device_internal_id,
            _sqlite_ts(2000, 3, 1),
        ),
        # /file1 v1
        (
            2,
            file1.bytes,
            1,
            service_encryption_key.encrypt(file1_manifest_v1),
            bob_device_internal_id,
            _sqlite_ts(2000, 3, 10),
        ),
        # /folder1 v1
        (
            3,
            folder1.bytes,
            1,
            service_encryption_key.encrypt(folder1_manifest_v1),
            bob_device_internal_id,
            _sqlite_ts(2000, 3, 11),
        ),
        # /folder2 v1
        (
            4,
            folder2.bytes,
            1,
            service_encryption_key.encrypt(folder2_manifest_v1),
            bob_device_internal_id,
            _sqlite_ts(2000, 3, 12),
        ),
        # / v2
        (
            5,
            workspace_id.bytes,
            2,
            service_encryption_key.encrypt(workspace_manifest_v2),
            bob_device_internal_id,
            _sqlite_ts(2000, 3, 13),
        ),
        # /folder2/file2 v1
        (
            6,
            file2.bytes,
            1,
            service_encryption_key.encrypt(file2_manifest_v1),
            adam_device_internal_id,
            _sqlite_ts(2000, 3, 20),
        ),
        # /folder2/folder3 v1
        (
            7,
            folder3.bytes,
            1,
            service_encryption_key.encrypt(folder3_manifest_v1),
            adam_device_internal_id,
            _sqlite_ts(2000, 3, 21),
        ),
        # /folder2 v2
        (
            8,
            folder2.bytes,
            2,
            service_encryption_key.encrypt(folder2_manifest_v2),
            adam_device_internal_id,
            _sqlite_ts(2000, 3, 22),
        ),
        # /folder2/file2 v2
        (
            9,
            file2.bytes,
            2,
            service_encryption_key.encrypt(file2_manifest_v2),
            adam_device_internal_id,
            _sqlite_ts(2000, 3, 25),
        ),
    ]
    con.executemany(
        "INSERT INTO vlob_atom(_id, vlob_id, version, blob, author, timestamp) VALUES (?, ?, ?, ?, ?, ?)",
        vlob_atoms,
    )

    con.commit()
    con.close()

    # Finally do the actual export \o/

    dump_path = tmp_path / "extract_dump"
    list(
        extract_workspace(
            output=dump_path,
            export_db=output_db_path,
            decryption_key=service_decryption_key,
            filter_on_date=DateTime.now(),
        )
    )

    # Check the result
    assert {x.name for x in dump_path.iterdir()} == {"file1", "folder1", "folder2"}
    assert (dump_path / "file1").read_bytes() == b""
    assert {x.name for x in (dump_path / "folder1").iterdir()} == set()
    assert {x.name for x in (dump_path / "folder2").iterdir()} == {"file2", "folder3"}
    assert (dump_path / "folder2/file2").read_bytes() == b"a" * 10 + b"b" * 10
    assert {x.name for x in (dump_path / "folder2/folder3").iterdir()} == set()

    # Extract dump at 2000-03-14, where folder2 was empty
    dump_path_ts = tmp_path / "extract_dump_ts"
    list(
        extract_workspace(
            output=dump_path_ts,
            export_db=output_db_path,
            decryption_key=service_decryption_key,
            filter_on_date=DateTime(2000, 3, 14),
        )
    )
    # Check the result
    assert {x.name for x in dump_path_ts.iterdir()} == {"file1", "folder1", "folder2"}
    assert (dump_path_ts / "file1").read_bytes() == b""
    assert {x.name for x in (dump_path_ts / "folder1").iterdir()} == set()
    assert {x.name for x in (dump_path_ts / "folder2").iterdir()} == set()
