# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import sqlite3
from pathlib import Path
from typing import Any, Callable

from parsec._parsec import DateTime, OrganizationID, VlobID
from parsec.backend import Backend
from parsec.components.realm import (
    RealmExportBlocksMetadataBatch,
    RealmExportCertificates,
    RealmExportDoBaseInfo,
    RealmExportDoBaseInfoBadOutcome,
    RealmExportDoBlocksBatchMetadatBadOutcome,
    RealmExportDoCertificatesBadOutcome,
    RealmExportDoVlobsBatchBadOutcome,
    RealmExportVlobsBatch,
)


class RealmExporterError(Exception):
    pass


class RealmExporterInputError(Exception):
    pass


class RealmExporterOutputDbError(RealmExporterError):
    pass


# Considering vlob to be a couple of Ko in size, 100k items means under 1Go of data per batch
VLOB_EXPORT_BATCH_SIZE = 100_000
# Block metada are really small (< 100 bytes)
BLOCK_METADATA_EXPORT_BATCH_SIZE = 1_000_000


OUTPUT_DB_MAGIC_NUMBER = 87948
OUTPUT_DB_VERSION = 1
OUTPUT_DB_INIT_QUERY = f"""
-------------------------------------------------------------------------
-- Database info
-------------------------------------------------------------------------


-- This magic number has two roles:
-- 1) It makes unlikely we mistakenly consider an unrelated database as legit.
-- 2) It acts as a constant ID to easily retrieve the single row in the table.
CREATE TABLE IF NOT EXISTS info (
    magic INTEGER UNIQUE NOT NULL DEFAULT {OUTPUT_DB_MAGIC_NUMBER},
    version INTEGER NOT NULL DEFAULT {OUTPUT_DB_VERSION},

    organization_id VARCHAR(32) NOT NULL,
    realm_id BLOB NOT NULL,
    root_verify_key BLOB NOT NULL,

    -- The export can be considered to be a snapshot of the realm up to this point in time
    -- (Note this doesn't necessarly correspond to when the export has been done).
    snapshot_timestamp INTEGER NOT NULL, -- us since UNIX epoch

    certificates_export_done INTEGER NOT NULL DEFAULT 0, -- Boolean
    vlobs_export_done INTEGER NOT NULL DEFAULT 0, -- Boolean
    blocks_metadata_export_done INTEGER NOT NULL DEFAULT 0, -- Boolean
    blocks_data_export_done INTEGER NOT NULL DEFAULT 0 -- Boolean
);


-------------------------------------------------------------------------
-- Certificates export
-------------------------------------------------------------------------


-- Common certificates provided in-order
CREATE TABLE common_certificate (
  _id INTEGER PRIMARY KEY,
  certificate BLOB NOT NULL
);

-- Sequester certificates provided in-order
CREATE TABLE sequester_certificate (
  _id INTEGER PRIMARY KEY,
  certificate BLOB NOT NULL
);

-- Realm's certificates provided in-order
CREATE TABLE realm_certificate (
  _id INTEGER PRIMARY KEY,
  certificate BLOB NOT NULL
);

-- Note Shamir recovery and other realms' certificates are not exported
-- since they are unrelated to the realm being exported.


-------------------------------------------------------------------------
-- Vlobs export
-------------------------------------------------------------------------


CREATE TABLE vlob_atom (
    -- We use `realm_vlob_update`'s `index` field as primary key.
    -- This means the vlob atoms are ordered according to how they got added
    -- in the server in the first place.
    realm_vlob_update_index INTEGER PRIMARY KEY,
    vlob_id BLOB NOT NULL,  -- VlobID
    version INTEGER NOT NULL,
    key_index INTEGER NOT NULL,
    blob BLOB NOT NULL,
    size INTEGER NOT NULL,
    author BLOB NOT NULL,  -- DeviceID
    -- Note this field is called `created_on` in Parsec datamodel, but it correspond
    -- in fact to the timestamp field in the API ! So we stick with the latter.
    -- On top of that, unlike PostgreSQL, SQLite doesn't have a TIMESTAMPZ type out
    -- of the box so we have to roll our own integer-based format.
    timestamp INTEGER NOT NULL  -- us since UNIX epoch
);


-------------------------------------------------------------------------
-- Blocks export
-------------------------------------------------------------------------


CREATE TABLE block (
    -- _id is not SERIAL given we will take the one present in the Parsec database
    _id INTEGER PRIMARY KEY,
    block_id BLOB NOT NULL,
    author BLOB NOT NULL,  -- DeviceID
    size INTEGER NOT NULL,
    key_index INTEGER NOT NULL
);

CREATE TABLE block_data (
    block INTEGER PRIMARY KEY REFERENCES block(_id),
    data BLOB NOT NULL
);
"""


def _sqlite_init_db(
    output_db_path: Path,
    organization_id: OrganizationID,
    realm_id: VlobID,
    root_verify_key: bytes,
    snapshot_timestamp: DateTime,
) -> sqlite3.Connection:
    try:
        con = sqlite3.connect(f"file:{output_db_path}?mode=rw", uri=True, autocommit=False)
    except sqlite3.Error:
        # Export database doesn't exists
        try:
            # Create the database...
            con = sqlite3.connect(output_db_path)
            # ...and initialize it
            con.executescript(OUTPUT_DB_INIT_QUERY)
            con.execute(
                """\
                INSERT INTO info (
                    organization_id,\
                    realm_id,\
                    root_verify_key,\
                    snapshot_timestamp\
                ) VALUES (?, ?, ?, ?)\
                """,
                (
                    organization_id.str,
                    realm_id.bytes,
                    root_verify_key,
                    snapshot_timestamp.as_timestamp_micros(),
                ),
            )
            con.commit()
        except sqlite3.Error as exc:
            raise RealmExporterOutputDbError(f"Cannot create export database: {exc}") from exc

    try:
        # Export database already exists, we should make sure it format is expected
        try:
            row = con.execute(
                "SELECT version, organization_id, realm_id, root_verify_key, snapshot_timestamp FROM info WHERE magic = ?",
                (OUTPUT_DB_MAGIC_NUMBER,),
            ).fetchone()
        except sqlite3.Error as exc:
            # If we end up here this is most likely because `info` table doesn't exists (or miss some columns)
            raise RealmExporterOutputDbError(
                f"Existing output target is not a valid export database: {exc}"
            ) from exc
        if not row:
            # `info` table exists and is valid, but magic number doesn't match
            raise RealmExporterOutputDbError(
                "Existing output target is not a valid export database: no info row"
            )
        db_version, db_organization_id, db_realm_id, db_root_verify_key, db_snapshot_timestamp = row

        match db_version:
            case int():
                if db_version != OUTPUT_DB_VERSION:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database version format is not supported: got version `{db_version}` but only version `{OUTPUT_DB_VERSION}` is supported"
                    )
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: invalid `info.version` value `{unknown!r}` (expected int)"
                )

        match db_organization_id:
            case str():
                if db_organization_id != organization_id.str:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database is for a different realm: got `{db_organization_id}` instead of expected `{organization_id.str}`"
                    )
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: invalid `info.organization_id` value `{unknown!r}` (expected str)"
                )

        match db_realm_id:
            case bytes():
                if db_realm_id != realm_id.bytes:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database is for a different realm: got `0x{db_realm_id.hex()}` instead of expected `0x{realm_id.hex}`"
                    )
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: invalid `info.realm_id` value `{unknown!r}` (expected bytes)"
                )

        match db_root_verify_key:
            case bytes():
                if db_root_verify_key != root_verify_key:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database is for a different realm: realm ID `{db_realm_id}` is the same but root verify key differs"
                    )
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: invalid `info.root_verify_key` value `{unknown!r}` (expected bytes)"
                )

        match db_snapshot_timestamp:
            case int():
                if db_snapshot_timestamp != snapshot_timestamp.as_timestamp_micros():
                    try:
                        display_db_snapshot_timestamp = DateTime.from_timestamp_micros(
                            db_snapshot_timestamp
                        )
                    except ValueError:
                        display_db_snapshot_timestamp = (
                            f"<invalid timestamp: {db_snapshot_timestamp}>"
                        )
                    raise RealmExporterOutputDbError(
                        f"Existing output export database is for a different timestamp: got `{display_db_snapshot_timestamp}` instead of expected `{snapshot_timestamp}`"
                    )
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: invalid `info.snapshot_timestamp` value `{unknown!r}` (expected int)"
                )

    except:
        con.close()
        raise

    return con


type ProgressReportCallback = Callable


async def export_realm(
    backend: Backend,
    organization_id: OrganizationID,
    realm_id: VlobID,
    snapshot_timestamp: DateTime,
    output_db_path: Path,
    on_progress: ProgressReportCallback,
):
    # 0) Ensure the export is possible since organization & realm exist on the server

    outcome = await backend.realm.export_do_base_info(
        organization_id=organization_id, realm_id=realm_id
    )
    match outcome:
        case RealmExportDoBaseInfo() as base_info:
            pass
        case RealmExportDoBaseInfoBadOutcome.ORGANIZATION_NOT_FOUND:
            raise RealmExporterInputError(f"Organization `{organization_id.str}` doesn't exists")
        case RealmExportDoBaseInfoBadOutcome.REALM_NOT_FOUND:
            raise RealmExporterInputError(
                f"Realm `{realm_id.hex}` doesn't exist in organization `{organization_id.str}`"
            )

    # 1) Create the output SQLite database (or re-open if it already exists)

    output_db_con = await asyncio.to_thread(
        _sqlite_init_db,
        output_db_path=output_db_path,
        organization_id=organization_id,
        realm_id=realm_id,
        root_verify_key=base_info.root_verify_key.encode(),
        snapshot_timestamp=snapshot_timestamp,
    )

    # 2) Export certificates

    await _do_export_certificates(
        backend, output_db_con, organization_id, realm_id, snapshot_timestamp
    )

    # 3) Export vlobs

    await _do_export_vlobs(backend, output_db_con, organization_id, realm_id, snapshot_timestamp)

    # # 4) Export blocks metadata

    # await _do_export_blocks_metadata(backend, output_db_con, organization_id, realm_id)

    # # 4) Export blocks data

    # await _do_export_blocks_data(backend, output_db_con, organization_id, realm_id)

    # 5) All done \o/


async def _do_export_certificates(
    backend: Backend,
    output_db_con: sqlite3.Connection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    snapshot_timestamp: DateTime,
) -> None:
    # 0) Skip the operation if the export database already contains it

    def _get_certificates_export_done() -> Any:
        row = output_db_con.execute(
            "SELECT certificates_export_done FROM info",
        ).fetchone()
        return bool(row[0])

    certificates_export_done = await asyncio.to_thread(_get_certificates_export_done)
    match certificates_export_done:
        case True:
            # Nothing to do
            return
        case False:
            # Export needed
            pass
        case unknown:
            raise RealmExporterOutputDbError(
                f"Output export database appears to be corrupted: `certificates_export_done` contains unexpected value `{unknown!r}`"
            )

    # 1) Fetch all certificates

    outcome = await backend.realm.export_do_certificates(
        organization_id, realm_id, snapshot_timestamp
    )
    match outcome:
        case RealmExportCertificates() as batch:
            pass
        case (
            (
                RealmExportDoCertificatesBadOutcome.ORGANIZATION_NOT_FOUND
                | RealmExportDoCertificatesBadOutcome.REALM_NOT_FOUND
            ) as error
        ):
            # Organization&realm existence has already been checked, so this shouldn't occur
            raise RealmExporterInputError(f"Unexpect outcome when exporting certificates: {error}")

    # 2) Write certificates to export database

    def _write_sqlite_db(batch: RealmExportCertificates):
        output_db_con.executemany(
            "INSERT INTO common_certificate (_id, certificate) VALUES (?, ?)",
            enumerate(batch.common_certificates),
        )
        output_db_con.executemany(
            "INSERT INTO realm_certificate (_id, certificate) VALUES (?, ?)",
            enumerate(batch.realm_certificates),
        )
        output_db_con.executemany(
            "INSERT INTO sequester_certificate (_id, certificate) VALUES (?, ?)",
            enumerate(batch.sequester_certificates),
        )
        output_db_con.execute(
            "UPDATE info SET certificates_export_done = 1",
        )
        output_db_con.commit()

    await asyncio.to_thread(_write_sqlite_db, batch)


async def _do_export_vlobs(
    backend: Backend,
    output_db_con: sqlite3.Connection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    snapshot_timestamp: DateTime,
) -> None:
    # 0) Skip the operation if the export database already contains it

    def _get_vlobs_export_done() -> Any:
        row = output_db_con.execute(
            "SELECT vlobs_export_done FROM info",
        ).fetchone()
        return bool(row[0])

    vlobs_export_done = await asyncio.to_thread(_get_vlobs_export_done)
    match vlobs_export_done:
        case True:
            # Nothing to do
            return
        case False:
            # Export needed
            pass
        case unknown:
            raise RealmExporterOutputDbError(
                f"Output export database appears to be corrupted: `vlobs_export_done` contains unexpected value `{unknown!r}`"
            )

    current_batch_offset_marker = 0
    while True:
        # 1) Download a batch of data

        outcome = await backend.realm.export_do_vlobs_batch(
            organization_id, realm_id, batch_offset_marker=current_batch_offset_marker
        )
        match outcome:
            case RealmExportVlobsBatch() as batch:
                pass
            case (
                (
                    RealmExportDoVlobsBatchBadOutcome.ORGANIZATION_NOT_FOUND
                    | RealmExportDoVlobsBatchBadOutcome.REALM_NOT_FOUND
                ) as error
            ):
                # Organization&realm existence has already been checked, so this shouldn't occur
                raise RealmExporterInputError(
                    f"Unexpect outcome when exporting certificates: {error}"
                )

        if not batch.items:
            break

        # 2) Write the batch to export database

        def _write_sqlite_db():
            output_db_con.executemany(
                "INSERT INTO vlob_atom (\
                    realm_vlob_update_index,\
                    vlob_id,\
                    version,\
                    key_index,\
                    blob,\
                    size,\
                    author,\
                    timestamp\
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                (
                    (
                        item.realm_vlob_update_index,
                        item.vlob_id.bytes,
                        item.version,
                        item.key_index,
                        item.blob,
                        item.size,
                        item.author,
                        item.timestamp.as_timestamp_micros(),
                    )
                    for item in batch.items
                ),
            )
            output_db_con.commit()

        await asyncio.to_thread(_write_sqlite_db)

        current_batch_offset_marker = batch.batch_offset_marker

        # TODO: progress report

    def _write_sqlite_db():
        output_db_con.execute(
            "UPDATE info SET vlobs_export_done = TRUE",
        )
        output_db_con.commit()

    await asyncio.to_thread(_write_sqlite_db)


async def _do_export_blocks_metadata(
    backend: Backend,
    output_db_con: sqlite3.Connection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    batch_size: int,
) -> None:
    # 0) Skip the operation if the export database already contains it

    def _get_blocks_metadata_export_done() -> Any:
        row = output_db_con.execute(
            "SELECT blocks_metadata_export_done FROM info",
        ).fetchone()
        return bool(row[0])

    blocks_metadata_export_done = await asyncio.to_thread(_get_blocks_metadata_export_done)
    match blocks_metadata_export_done:
        case True:
            # Nothing to do
            return
        case False:
            # Export needed
            pass
        case unknown:
            raise RealmExporterOutputDbError(
                f"Output export database appears to be corrupted: `blocks_metadata_export_done` contains unexpected value `{unknown!r}`"
            )

    current_batch_offset_marker = 0
    while True:
        # 1) Download a batch of data

        outcome = await backend.realm.export_do_blocks_metadata_batch(
            organization_id=organization_id,
            realm_id=realm_id,
            batch_offset_marker=current_batch_offset_marker,
            batch_size=batch_size,
        )
        match outcome:
            case RealmExportBlocksMetadataBatch() as batch:
                pass
            case (
                (
                    RealmExportDoBlocksBatchMetadatBadOutcome.ORGANIZATION_NOT_FOUND
                    | RealmExportDoBlocksBatchMetadatBadOutcome.REALM_NOT_FOUND
                ) as error
            ):
                # Organization&realm existence has already been checked, so this shouldn't occur
                raise RealmExporterInputError(
                    f"Unexpect outcome when exporting certificates: {error}"
                )

        if not batch.items:
            break

        # 2) Write the batch to export database

        def _write_sqlite_db():
            output_db_con.executemany(
                "INSERT INTO block (\
                    _id,\
                    block_id,\
                    author,\
                    size,\
                    key_index\
                ) VALUES (?, ?, ?, ?, ?)",
                (
                    (
                        item.id,
                        item.block_id.bytes,
                        item.author,
                        item.key_index,
                        item.size,
                    )
                    for item in batch.items
                ),
            )
            output_db_con.commit()

        await asyncio.to_thread(_write_sqlite_db)

        current_batch_offset_marker = batch.batch_offset_marker

        # TODO: progress report

    def _write_sqlite_db():
        output_db_con.execute(
            "UPDATE info SET blocks_metadata_export_done = TRUE",
        )
        output_db_con.commit()

    await asyncio.to_thread(_write_sqlite_db)


async def _do_export_blocks_data(
    backend: Backend,
    output_db_con: sqlite3.Connection,
    organization_id: OrganizationID,
    realm_id: VlobID,
) -> None:
    # 0) Skip the operation if the export database already contains it

    def _get_blocks_export_done() -> Any:
        row = output_db_con.execute(
            "SELECT blocks_export_done FROM info",
        ).fetchone()
        return bool(row[0])

    blocks_export_done = await asyncio.to_thread(_get_blocks_export_done)
    match blocks_export_done:
        case True:
            # Nothing to do
            return
        case False:
            # Export needed
            pass
        case unknown:
            raise RealmExporterOutputDbError(
                f"Output export database appears to be corrupted: `blocks_export_done` contains unexpected value `{unknown!r}`"
            )

    # TODO: retreive marker from SQLite

    # TODO: run multiple batches in parallel (but save in SQLite must be in order !)

    # TODO: while loop to run multiple batches missing

    # TODO: progress report

    def _write_sqlite_db():
        output_db_con.execute(
            "UPDATE info SET blocks_data_export_done = TRUE",
        )
        output_db_con.commit()

    await asyncio.to_thread(_write_sqlite_db)

    # TODO: save batch to sqlite
    raise NotImplementedError
