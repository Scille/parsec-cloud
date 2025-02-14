# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import sqlite3
from dataclasses import dataclass
from pathlib import Path
from shutil import copyfile
from unittest.mock import ANY

import anyio
import pytest

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    RealmRoleCertificate,
    SequesterServiceID,
    UserID,
    VerifyKey,
    VlobID,
)
from parsec._parsec import testbed as tb
from parsec.ballpark import BALLPARK_CLIENT_LATE_OFFSET
from parsec.realm_export import (
    ExportProgressStep,
    RealmExporterInputError,
    RealmExporterOutputDbError,
    export_realm,
)
from tests.common import (
    Backend,
    MinimalorgRpcClients,
    SequesteredOrgRpcClients,
    WorkspaceHistoryOrgRpcClients,
)

# Sample export has been generated by taking the export of test `test_export_ok_sequestered[current_snapshot]`
SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP = DateTime(2025, 2, 14, 22, 42, 31, 419866)
SAMPLE_EXPORT_ORGANIZATION_ID = OrganizationID("Org1")
SAMPLE_EXPORT_REALM_ID = VlobID.from_hex("f0000000000000000000000000000003")


@pytest.fixture
def sample_export_db_path(sequestered_org: SequesteredOrgRpcClients, tmp_path: Path) -> Path:
    output_db_path = (
        tmp_path
        / f"parsec-export-{sequestered_org.organization_id.str}-realm-{SAMPLE_EXPORT_REALM_ID.hex}-{SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP.to_rfc3339()}.sqlite"
    )

    # We copy the file before providing it to the test to ensure the original file is not modified
    copyfile(Path(__file__).parent / "sample_export.sqlite", output_db_path)

    # Also we patch the export so that is has the same organization ID as the one we want to test
    # (since the organization ID changes between tests)
    con = sqlite3.connect(output_db_path)
    con.execute("UPDATE info SET organization_id = ?", (sequestered_org.organization_id.str,))
    con.commit()
    con.close()

    return output_db_path


@dataclass(slots=True)
class ExportExpected:
    realm_id: VlobID
    snapshot_timestamp: DateTime
    root_verify_key: VerifyKey
    common_certificates: list[bytes]
    sequester_certificates: list[bytes]
    realm_certificates: list[bytes]
    # List of (vlob_id, version, key_index, blob, size, author, timestamp)
    vlobs: list[tuple[VlobID, int, int, bytes, int, DeviceID, DateTime]]
    # List of (block_id, author, size, key_index, data)
    blocks: list[tuple[BlockID, DeviceID, int, int, bytes]]
    # List of (key_index, keys_bundle)
    realm_keys_bundle: list[tuple[int, bytes]]
    # List of (user_id, key_index, access)
    realm_keys_bundle_access: list[tuple[UserID, int, bytes]]
    # List of (sequester_service_id, key_index, access)
    realm_sequester_keys_bundle_access: list[tuple[SequesterServiceID, int, bytes]]


def extract_export_expected_from_template(
    testbed_template: tb.TestbedTemplateContent,
    realm_id: VlobID,
    snapshot_timestamp: DateTime,
) -> ExportExpected:
    root_verify_key: VerifyKey | None = None
    common_certificates: list[bytes] = []
    sequester_certificates: list[bytes] = []
    realm_certificates: list[bytes] = []
    vlobs: list[tuple[VlobID, int, int, bytes, int, DeviceID, DateTime]] = []
    blocks: list[tuple[BlockID, DeviceID, int, int, bytes]] = []
    realm_keys_bundle: list[tuple[int, bytes]] = []
    realm_keys_bundle_access: list[tuple[UserID, int, bytes]] = []
    realm_sequester_keys_bundle_access: list[tuple[SequesterServiceID, int, bytes]] = []

    for event in testbed_template.events:
        match event:
            case tb.TestbedEventBootstrapOrganization():
                common_certificates.append(event.first_user_raw_certificate)
                common_certificates.append(event.first_user_first_device_raw_certificate)
                root_verify_key = event.root_signing_key.verify_key
                if (
                    event.sequester_authority_raw_certificate
                    and event.timestamp <= snapshot_timestamp
                ):
                    sequester_certificates.append(event.sequester_authority_raw_certificate)
            case tb.TestbedEventNewSequesterService():
                if event.timestamp <= snapshot_timestamp:
                    sequester_certificates.append(event.raw_certificate)
            case tb.TestbedEventRevokeSequesterService():
                if event.timestamp <= snapshot_timestamp:
                    sequester_certificates.append(event.raw_certificate)
            case tb.TestbedEventNewUser():
                if event.timestamp <= snapshot_timestamp:
                    common_certificates.append(event.user_raw_certificate)
                    common_certificates.append(event.first_device_raw_certificate)
            case tb.TestbedEventNewDevice():
                if event.timestamp <= snapshot_timestamp:
                    common_certificates.append(event.raw_certificate)
            case tb.TestbedEventUpdateUserProfile():
                if event.timestamp <= snapshot_timestamp:
                    common_certificates.append(event.raw_certificate)
            case tb.TestbedEventRevokeUser():
                if event.timestamp <= snapshot_timestamp:
                    common_certificates.append(event.raw_certificate)
            case tb.TestbedEventNewRealm():
                if event.realm_id == realm_id and event.timestamp <= snapshot_timestamp:
                    realm_certificates.append(event.raw_certificate)
            case tb.TestbedEventShareRealm():
                if event.realm == realm_id and event.timestamp <= snapshot_timestamp:
                    realm_certificates.append(event.raw_certificate)
                    if event.recipient_keys_bundle_access is not None:
                        assert event.key_index is not None
                        realm_keys_bundle_access.append(
                            (event.user, event.key_index, event.recipient_keys_bundle_access)
                        )
            case tb.TestbedEventRenameRealm():
                if event.realm == realm_id and event.timestamp <= snapshot_timestamp:
                    realm_certificates.append(event.raw_certificate)
            case tb.TestbedEventRotateKeyRealm():
                if event.realm == realm_id and event.timestamp <= snapshot_timestamp:
                    realm_certificates.append(event.raw_certificate)
                    realm_keys_bundle.append((event.key_index, event.keys_bundle))
                    for user_id, access in event.per_participant_keys_bundle_access.items():
                        realm_keys_bundle_access.append((user_id, event.key_index, access))
                    if event.per_sequester_service_keys_bundle_access:
                        for (
                            sequester_service_id,
                            access,
                        ) in event.per_sequester_service_keys_bundle_access.items():
                            realm_sequester_keys_bundle_access.append(
                                (sequester_service_id, event.key_index, access)
                            )
            case tb.TestbedEventArchiveRealm():
                if event.realm == realm_id and event.timestamp <= snapshot_timestamp:
                    realm_certificates.append(event.raw_certificate)
            case tb.TestbedEventCreateOrUpdateOpaqueVlob():
                if event.realm == realm_id and event.timestamp <= snapshot_timestamp:
                    vlobs.append(
                        (
                            event.vlob_id,
                            event.version,
                            event.key_index,
                            event.encrypted,
                            len(event.encrypted),
                            event.author,
                            event.timestamp,
                        )
                    )
            case tb.TestbedEventCreateBlock():
                if event.realm == realm_id and event.timestamp <= snapshot_timestamp:
                    blocks.append(
                        (
                            event.block_id,
                            event.author,
                            len(event.encrypted),
                            event.key_index,
                            event.encrypted,
                        )
                    )
            case tb.TestbedEventCreateOpaqueBlock():
                if event.realm == realm_id and event.timestamp <= snapshot_timestamp:
                    blocks.append(
                        (
                            event.block_id,
                            event.author,
                            len(event.encrypted),
                            event.key_index,
                            event.encrypted,
                        )
                    )
            # Other events don't produce data that is exported
            case (
                tb.TestbedEventNewShamirRecoveryInvitation()
                | tb.TestbedEventNewUserInvitation()
                | tb.TestbedEventNewDeviceInvitation()
                | tb.TestbedEventNewShamirRecovery()
                | tb.TestbedEventDeleteShamirRecovery()
                | tb.TestbedEventUpdateOrganization()
                | tb.TestbedEventFreezeUser()
            ):
                pass

    assert root_verify_key is not None

    return ExportExpected(
        realm_id=realm_id,
        snapshot_timestamp=snapshot_timestamp,
        root_verify_key=root_verify_key,
        common_certificates=common_certificates,
        sequester_certificates=sequester_certificates,
        realm_certificates=realm_certificates,
        realm_keys_bundle=sorted(realm_keys_bundle),
        realm_keys_bundle_access=sorted(realm_keys_bundle_access),
        realm_sequester_keys_bundle_access=sorted(realm_sequester_keys_bundle_access),
        vlobs=vlobs,
        blocks=blocks,
    )


def check_export_content(
    output_db_path: Path,
    export_expected: ExportExpected,
    expected_organization_id: OrganizationID,
    # When checking against a database that has been generated from (typically the sample export),
    # all the data that is signed or encrypted cannot be checked since it changes at every run
    # (even though their content stays the same !).
    ignore_unstable_items: bool = False,
):
    def stabilize(x):
        if ignore_unstable_items:
            return ANY
        else:
            return x

    con = sqlite3.connect(output_db_path)

    # Check export content: info table

    row = con.execute("""
    SELECT
        magic,
        version,
        organization_id,
        realm_id,
        root_verify_key,
        snapshot_timestamp,
        certificates_export_done,
        vlobs_export_done,
        blocks_metadata_export_done,
        blocks_data_export_done
    FROM info
    """).fetchone()
    assert row is not None
    (
        export_magic,
        export_version,
        export_organization_id,
        export_realm_id,
        export_root_verify_key,
        export_snapshot_timestamp,
        export_certificates_export_done,
        export_vlobs_export_done,
        export_blocks_metadata_export_done,
        export_blocks_data_export_done,
    ) = row
    assert export_magic == 87948  # Export format magic number
    assert export_version == 1  # Export format version
    assert export_organization_id == expected_organization_id.str
    assert export_realm_id == export_expected.realm_id.bytes
    assert export_root_verify_key == export_expected.root_verify_key.encode()
    assert export_snapshot_timestamp == export_expected.snapshot_timestamp.as_timestamp_micros()
    assert export_certificates_export_done == 1
    assert export_vlobs_export_done == 1
    assert export_blocks_metadata_export_done == 1
    assert export_blocks_data_export_done == 1

    # Check export content: certificate tables

    rows = con.execute("SELECT certificate FROM common_certificate ORDER BY _id ASC").fetchall()
    assert [stabilize(row[0]) for row in rows] == export_expected.common_certificates

    rows = con.execute("SELECT certificate FROM sequester_certificate ORDER BY _id ASC").fetchall()
    assert [stabilize(row[0]) for row in rows] == export_expected.sequester_certificates

    rows = con.execute("SELECT certificate FROM realm_certificate ORDER BY _id ASC").fetchall()
    assert [stabilize(row[0]) for row in rows] == export_expected.realm_certificates

    rows = con.execute(
        "SELECT key_index, keys_bundle FROM realm_keys_bundle ORDER BY key_index ASC"
    ).fetchall()
    assert [
        (key_index, stabilize(keys_bundle)) for key_index, keys_bundle in rows
    ] == export_expected.realm_keys_bundle

    rows = con.execute(
        "SELECT user_id, key_index, access FROM realm_keys_bundle_access ORDER BY key_index ASC, user_id ASC"
    ).fetchall()
    assert (
        sorted(
            (
                UserID.from_bytes(row[0]),  # user_id
                row[1],  # key_index
                stabilize(row[2]),  # access
            )
            for row in rows
        )
        == export_expected.realm_keys_bundle_access
    )

    rows = con.execute(
        "SELECT sequester_service_id, key_index, access FROM realm_sequester_keys_bundle_access ORDER BY key_index ASC, sequester_service_id ASC"
    ).fetchall()
    assert (
        sorted(
            (
                SequesterServiceID.from_bytes(row[0]),  # sequester_service_id
                row[1],  # key_index
                stabilize(row[2]),  # access
            )
            for row in rows
        )
        == export_expected.realm_sequester_keys_bundle_access
    )

    # Check export content: vlobs table

    vlobs = con.execute("""
    SELECT
        sequential_id,
        vlob_id,
        version,
        key_index,
        blob,
        size,
        author,
        timestamp
    FROM vlob_atom
    ORDER BY sequential_id
    """).fetchall()

    # Ensure `sequential_id` and `timestamp` are consistent, this is not mandatory in
    # production (since two concurrent vlobs can have the same timestamp), however
    # it is the case in our testbed data. So it's an interesting check anyway ;-)

    last_sequential_id = None
    last_timestamp = None
    for sequential_id, _, _, _, _, _, _, timestamp in vlobs:
        if last_sequential_id is not None:
            assert sequential_id > last_sequential_id

        if last_timestamp is not None:
            assert timestamp >= last_timestamp

        last_sequential_id = sequential_id
        last_timestamp = timestamp

    assert [
        (
            # Ignore `sequential_id` since it actual value depends on how the insertion was
            # done (e.g. PostgreSQL autoincrement index skipped due to transaction rollback)
            VlobID.from_bytes(row[1]),  # vlob_id
            row[2],  # version
            row[3],  # key_index
            stabilize(row[4]),  # blob
            row[5],  # size
            DeviceID.from_bytes(row[6]),  # author
            DateTime.from_timestamp_micros(row[7]),  # timestamp
        )
        for row in vlobs
    ] == export_expected.vlobs

    # Check export content: block tables

    blocks = con.execute("""
    SELECT
        block.sequential_id,
        block.block_id,
        block.author,
        block.size,
        block.key_index,
        block_data.data
    FROM block LEFT OUTER JOIN block_data ON block.sequential_id = block_data.block
    ORDER BY block.sequential_id
    """).fetchall()

    assert [
        (
            # Ignore `sequential_id` since it actual value depends on how the insertion was
            # done (e.g. PostgreSQL autoincrement index skipped due to transaction rollback)
            BlockID.from_bytes(row[1]),  # block_id
            DeviceID.from_bytes(row[2]),  # author
            row[3],  # size
            row[4],  # key_index
            stabilize(row[5]),  # data
        )
        for row in blocks
    ] == export_expected.blocks


@pytest.mark.parametrize("kind", ["current_snapshot", "past_snapshot"])
async def test_export_ok_non_sequestered(
    workspace_history_org: WorkspaceHistoryOrgRpcClients,
    backend: Backend,
    tmp_path: Path,
    kind: str,
):
    match kind:
        case "current_snapshot":
            expected_snapshot_timestamp = DateTime.now().subtract(
                seconds=BALLPARK_CLIENT_LATE_OFFSET
            )
        case "past_snapshot":
            expected_snapshot_timestamp = DateTime(2001, 1, 7)
        case unknown:
            assert False, unknown

    output_db_path = tmp_path / "output.sqlite"

    on_progress_events: list[ExportProgressStep] = []

    def _on_progress(event: ExportProgressStep):
        on_progress_events.append(event)

    await export_realm(
        backend=backend,
        organization_id=workspace_history_org.organization_id,
        realm_id=workspace_history_org.wksp1_id,
        output_db_path=output_db_path,
        snapshot_timestamp=expected_snapshot_timestamp,
        on_progress=_on_progress,
    )

    # Check on progress events

    assert on_progress_events[0] == "certificates_start"
    assert on_progress_events[1] == "certificates_done"
    end_of_vlobs_index = next(
        i + 2 for i, (e, *_) in enumerate(on_progress_events[2:]) if e != "vlobs"
    )
    end_of_blocks_metadata_index = next(
        i + end_of_vlobs_index
        for i, (e, *_) in enumerate(on_progress_events[end_of_vlobs_index:])
        if e != "blocks_metadata"
    )
    # Ensure exported bytes is strictly growing
    vlobs_events = on_progress_events[2:end_of_vlobs_index]
    assert sorted(vlobs_events) == vlobs_events
    blocks_metadata_events = on_progress_events[end_of_vlobs_index:end_of_blocks_metadata_index]
    assert sorted(blocks_metadata_events) == blocks_metadata_events
    blocks_data_events = on_progress_events[end_of_blocks_metadata_index:]
    assert sorted(blocks_data_events) == blocks_data_events

    # Check output database

    assert output_db_path.is_file()

    export_expected = extract_export_expected_from_template(
        workspace_history_org.testbed_template,
        workspace_history_org.wksp1_id,
        expected_snapshot_timestamp,
    )
    check_export_content(output_db_path, export_expected, workspace_history_org.organization_id)


@pytest.mark.parametrize("kind", ["current_snapshot", "past_snapshot"])
async def test_export_ok_sequestered(
    sequestered_org: SequesteredOrgRpcClients,
    backend: Backend,
    tmp_path: Path,
    kind: str,
):
    match kind:
        case "current_snapshot":
            expected_snapshot_timestamp = DateTime.now().subtract(
                seconds=BALLPARK_CLIENT_LATE_OFFSET
            )
        case "past_snapshot":
            # We want our snapshot to export up to when `sequester_service_1` is
            # revoked, so the certificate about `sequester_service_2` creation should
            # be ignored (see documentation of the `sequestered` testbed template).
            events = iter(sequestered_org.testbed_template.events)
            expected_snapshot_timestamp = next(
                (e for e in events if isinstance(e, tb.TestbedEventRevokeSequesterService))
            ).timestamp
            # Sanity check to ensure our snapshot will skip some data
            assert (
                len([e for e in events if isinstance(e, tb.TestbedEventNewSequesterService)]) == 1
            )
        case unknown:
            assert False, unknown

    output_db_path = tmp_path / "output.sqlite"

    await export_realm(
        backend=backend,
        organization_id=sequestered_org.organization_id,
        realm_id=sequestered_org.wksp1_id,
        output_db_path=output_db_path,
        snapshot_timestamp=expected_snapshot_timestamp,
        on_progress=lambda x: None,
    )

    assert output_db_path.is_file()

    export_expected = extract_export_expected_from_template(
        sequestered_org.testbed_template,
        sequestered_org.wksp1_id,
        expected_snapshot_timestamp,
    )
    check_export_content(output_db_path, export_expected, sequestered_org.organization_id)


@pytest.mark.xfail(reason="Flaky test, see issue #9576")
async def test_restart_partially_exported(
    workspace_history_org: WorkspaceHistoryOrgRpcClients,
    backend: Backend,
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
):
    expected_snapshot_timestamp = DateTime.now().subtract(seconds=BALLPARK_CLIENT_LATE_OFFSET)

    output_db_path = tmp_path / "output.sqlite"

    monkeypatch.setattr("parsec.realm_export.VLOB_EXPORT_BATCH_SIZE", 3)
    monkeypatch.setattr("parsec.realm_export.BLOCK_METADATA_EXPORT_BATCH_SIZE", 3)

    for cancel_on_event in (
        "certificates_start",
        "certificates_done",
        ("vlobs", 0, ANY),
        lambda e: e[0] == "vlobs" and e[1] > 0,
        ("blocks_metadata", 0, ANY),
        lambda e: e[0] == "blocks_metadata" and e[1] > 0,
        ("blocks_data", 0, ANY),
        lambda e: e[0] == "blocks_data" and e[1] > 0,
        # Finally we finished without being cancelled
        None,
    ):
        with anyio.CancelScope() as cancel_scope:
            # Cancel the export at multiple different steps
            def _on_progress(event: ExportProgressStep):
                if cancel_on_event is None:
                    return
                elif callable(cancel_on_event) and cancel_on_event(event):
                    cancel_scope.cancel()
                elif cancel_on_event == event:
                    cancel_scope.cancel()

            await export_realm(
                backend=backend,
                organization_id=workspace_history_org.organization_id,
                realm_id=workspace_history_org.wksp1_id,
                output_db_path=output_db_path,
                snapshot_timestamp=expected_snapshot_timestamp,
                on_progress=_on_progress,
            )

            if cancel_on_event is None:
                assert not cancel_scope.cancel_called
            else:
                assert cancel_scope.cancel_called

    assert output_db_path.is_file()

    export_expected = extract_export_expected_from_template(
        workspace_history_org.testbed_template,
        workspace_history_org.wksp1_id,
        expected_snapshot_timestamp,
    )
    check_export_content(output_db_path, export_expected, workspace_history_org.organization_id)


async def test_sample_export_is_valid(
    sequestered_org: SequesteredOrgRpcClients,
    sample_export_db_path: Path,
):
    export_expected = extract_export_expected_from_template(
        sequestered_org.testbed_template,
        sequestered_org.wksp1_id,
        SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP,
    )
    check_export_content(
        sample_export_db_path,
        export_expected,
        SAMPLE_EXPORT_ORGANIZATION_ID,
        ignore_unstable_items=True,
    )


async def test_re_export_is_noop(
    sequestered_org: SequesteredOrgRpcClients,
    backend: Backend,
    sample_export_db_path: Path,
):
    output_db_stat = sample_export_db_path.stat()

    await export_realm(
        backend=backend,
        organization_id=sequestered_org.organization_id,
        realm_id=sequestered_org.wksp1_id,
        output_db_path=sample_export_db_path,
        snapshot_timestamp=SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP,
        on_progress=lambda x: None,
    )

    new_output_db_stat = sample_export_db_path.stat()
    assert new_output_db_stat.st_mtime_ns == output_db_stat.st_mtime_ns
    assert new_output_db_stat.st_size == output_db_stat.st_size


@pytest.mark.parametrize(
    "kind",
    [
        "organization_id",
        "realm_id",
        "snapshot_timestamp",
        "root_verify_key",
        "vlobs_total_bytes",
        "blocks_total_bytes",
    ],
)
async def test_re_export_with_different_params(
    sequestered_org: SequesteredOrgRpcClients,
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    sample_export_db_path: Path,
    kind: str,
):
    snapshot_timestamp = SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP
    organization_id = sequestered_org.organization_id
    realm_id = SAMPLE_EXPORT_REALM_ID
    match kind:
        case "organization_id":
            organization_id = minimalorg.organization_id
            expected_error_msg = f"Existing output export database is for a different realm: got `{sequestered_org.organization_id}` instead of expected `{organization_id}`"
            # Export operation checks the organization & realm exist before reading
            # the existing export database, so we need to create a realm with the
            # corresponding realm ID in the different organization.
            now = SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP.subtract(seconds=1)
            await backend.realm.create(
                now=now,
                organization_id=minimalorg.organization_id,
                author=minimalorg.alice.device_id,
                author_verify_key=minimalorg.alice.signing_key.verify_key,
                realm_role_certificate=RealmRoleCertificate(
                    author=minimalorg.alice.device_id,
                    timestamp=now,
                    realm_id=realm_id,
                    user_id=minimalorg.alice.user_id,
                    role=RealmRole.OWNER,
                ).dump_and_sign(minimalorg.alice.signing_key),
            )

        case "realm_id":
            other_realm_id = VlobID.new()
            now = SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP.subtract(seconds=1)
            await backend.realm.create(
                now=now,
                organization_id=sequestered_org.organization_id,
                author=sequestered_org.alice.device_id,
                author_verify_key=sequestered_org.alice.signing_key.verify_key,
                realm_role_certificate=RealmRoleCertificate(
                    author=sequestered_org.alice.device_id,
                    timestamp=now,
                    realm_id=other_realm_id,
                    user_id=sequestered_org.alice.user_id,
                    role=RealmRole.OWNER,
                ).dump_and_sign(sequestered_org.alice.signing_key),
            )
            realm_id = other_realm_id
            expected_error_msg = f"Existing output export database is for a different realm: got `0x{SAMPLE_EXPORT_REALM_ID.hex}` instead of expected `0x{other_realm_id.hex}`"

        case "snapshot_timestamp":
            snapshot_timestamp = DateTime(2020, 1, 1)
            expected_error_msg = f"Existing output export database is for a different timestamp: got `{SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP}` instead of expected `2020-01-01T00:00:00Z`"

        case "root_verify_key":
            con = sqlite3.connect(sample_export_db_path)
            con.execute("UPDATE info SET root_verify_key = ?", (b"\x01" * 32,))
            con.commit()
            con.close()
            expected_error_msg = f"Existing output export database is for a different realm: realm ID `0x{SAMPLE_EXPORT_REALM_ID.hex}` is the same but root verify key differs"

        case "vlobs_total_bytes":
            con = sqlite3.connect(sample_export_db_path)
            con.execute("UPDATE info SET vlobs_total_bytes = vlobs_total_bytes + 1")
            con.commit()
            con.close()
            expected_error_msg = f"Existing output export database doesn't match: realm ID `0x{SAMPLE_EXPORT_REALM_ID.hex}` and snapshot timestamp `{SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP}` are the same, but vlobs total bytes differs"

        case "blocks_total_bytes":
            con = sqlite3.connect(sample_export_db_path)
            con.execute("UPDATE info SET blocks_total_bytes = blocks_total_bytes + 1")
            con.commit()
            con.close()
            expected_error_msg = f"Existing output export database doesn't match: realm ID `0x{SAMPLE_EXPORT_REALM_ID.hex}` and snapshot timestamp `{SAMPLE_EXPORT_SNAPSHOT_TIMESTAMP}` are the same, but blocks total bytes differs"

        case unknown:
            assert False, unknown

    with pytest.raises(RealmExporterOutputDbError) as exc:
        await export_realm(
            backend=backend,
            organization_id=organization_id,
            realm_id=realm_id,
            output_db_path=sample_export_db_path,
            snapshot_timestamp=snapshot_timestamp,
            on_progress=lambda x: None,
        )

    assert str(exc.value) == expected_error_msg


async def test_export_organization_not_found(backend: Backend, tmp_path: Path):
    output_db_path = tmp_path / "output.sqlite"

    with pytest.raises(RealmExporterInputError) as exc:
        await export_realm(
            backend=backend,
            organization_id=OrganizationID("Dummy"),
            realm_id=VlobID.new(),
            output_db_path=output_db_path,
            snapshot_timestamp=DateTime.now().subtract(seconds=BALLPARK_CLIENT_LATE_OFFSET),
            on_progress=lambda x: None,
        )
    assert str(exc.value) == "Organization `Dummy` doesn't exists"


async def test_export_realm_not_found(
    workspace_history_org: WorkspaceHistoryOrgRpcClients,
    backend: Backend,
    tmp_path: Path,
):
    output_db_path = tmp_path / "output.sqlite"

    with pytest.raises(RealmExporterInputError) as exc:
        await export_realm(
            backend=backend,
            organization_id=workspace_history_org.organization_id,
            realm_id=VlobID.from_hex("aec70837083b48c98d6b305f608975b3"),
            output_db_path=output_db_path,
            snapshot_timestamp=DateTime.now().subtract(seconds=BALLPARK_CLIENT_LATE_OFFSET),
            on_progress=lambda x: None,
        )
    assert (
        str(exc.value)
        == f"Realm `aec70837083b48c98d6b305f608975b3` doesn't exist in organization `{workspace_history_org.organization_id}`"
    )


@pytest.mark.parametrize("kind", ["in_the_future", "too_close_to_present"])
async def test_export_snapshot_timestamp_in_the_future(
    workspace_history_org: WorkspaceHistoryOrgRpcClients,
    backend: Backend,
    tmp_path: Path,
    kind: str,
):
    match kind:
        case "in_the_future":
            bad_snapshot_timestamp = DateTime.now().add(seconds=1)
        case "too_close_to_present":
            bad_snapshot_timestamp = (
                DateTime.now().subtract(seconds=BALLPARK_CLIENT_LATE_OFFSET).add(seconds=1)
            )
        case unknown:
            assert False, unknown

    output_db_path = tmp_path / "output.sqlite"

    with pytest.raises(RealmExporterInputError) as exc:
        await export_realm(
            backend=backend,
            organization_id=workspace_history_org.organization_id,
            realm_id=workspace_history_org.wksp1_id,
            output_db_path=output_db_path,
            snapshot_timestamp=bad_snapshot_timestamp,
            on_progress=lambda x: None,
        )
    assert str(exc.value).startswith(
        "Snapshot timestamp cannot be more recent than present time - 320s (i.e. `"
    )


async def test_export_snapshot_timestamp_older_than_realm_creation(
    workspace_history_org: WorkspaceHistoryOrgRpcClients,
    backend: Backend,
    tmp_path: Path,
):
    wksp1_created_on = next(
        (
            e
            for e in workspace_history_org.testbed_template.events
            if isinstance(e, tb.TestbedEventNewRealm)
            and e.realm_id == workspace_history_org.wksp1_id
        )
    ).timestamp
    bad_snapshot_timestamp = wksp1_created_on.subtract(seconds=1)

    output_db_path = tmp_path / "output.sqlite"

    with pytest.raises(RealmExporterInputError) as exc:
        await export_realm(
            backend=backend,
            organization_id=workspace_history_org.organization_id,
            realm_id=workspace_history_org.wksp1_id,
            output_db_path=output_db_path,
            snapshot_timestamp=bad_snapshot_timestamp,
            on_progress=lambda x: None,
        )
    assert (
        str(exc.value)
        == "Requested snapshot timestamp `2000-12-31T23:59:59Z` is older than realm creation"
    )
