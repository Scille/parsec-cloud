# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import sqlite3
from pathlib import Path
from typing import Any, Callable

from parsec._parsec import OrganizationID, VlobID
from parsec.backend import Backend
from parsec.components.realm import (
    RealmExportCertificates,
    RealmExportDoBaseInfo,
    RealmExportDoBaseInfoBadOutcome,
    RealmExportDoCertificatesBadOutcome,
)


class RealmExporterError(Exception):
    pass


class RealmExporterInputError(Exception):
    pass


class RealmExporterOutputDbError(RealmExporterError):
    pass


OUTPUT_DB_MAGIC_NUMBER = 87948
OUTPUT_DB_VERSION = 1
OUTPUT_DB_INIT_QUERY = f"""
-- Database info


-- This magic number has two roles:
-- 1) it makes unlikely we mistakenly consider an unrelated database as legit
-- 2) it acts as a constant ID to easily retrieve the single row in the table
CREATE TABLE IF NOT EXISTS info (
    magic INTEGER UNIQUE NOT NULL DEFAULT {OUTPUT_DB_MAGIC_NUMBER},
    version INTEGER NOT NULL DEFAULT {OUTPUT_DB_VERSION},
    organization_id VARCHAR(32) NOT NULL,
    realm_id BLOB NOT NULL,
    root_verify_key BLOB NOT NULL,
    export_up_to TIMESTAMPTZ NOT NULL,
    certificates_export_done BOOL NOT NULL DEFAULT FALSE,
    vlobs_export_done BOOL NOT NULL DEFAULT FALSE,
    blocks_export_done BOOL NOT NULL DEFAULT FALSE
);


-- Metadata & Data export


CREATE TABLE block (
    -- _id is not SERIAL given we will take the one present in the Parsec database
    _id PRIMARY KEY,
    block_id BLOB NOT NULL,
    data BLOB NOT NULL,
    author INTEGER REFERENCES device (_id) NOT NULL,

    UNIQUE(block_id)
);

-- Compared to Parsec's datamodel, we don't store `vlob_encryption_revision` given
-- the vlob is provided with third party encryption key only once at creation time
CREATE TABLE vlob_atom (
    -- We use vlob_update's index as primary key, this is convenient given it makes trivial
    -- keeping track of how far we got when restarting an export
    _id PRIMARY KEY,
    vlob_id BLOB NOT NULL,
    version INTEGER NOT NULL,
    blob BLOB NOT NULL,
    -- author/timestamp are required to validate the consistency of blob
    -- Care must be taken when exporting this field (and the `device` table) to
    -- keep this relationship valid !
    author INTEGER REFERENCES device (_id) NOT NULL,
    -- Note this field is called `created_on` in Parsec datamodel, but it correspond
    -- in fact to the timestamp field in the API ! So we stick with the latter.
    -- On top of that, unlike PostgreSQL, SQLite doesn't have a TIMESTAMPZ type out
    -- of the box so we have to roll our own integer-based format, note this is
    -- different than the 8bytes floating point format used in our msgpack-based
    -- serialization system (it was a bad idea, see Rust implementation for more info)
    timestamp INTEGER NOT NULL,  -- us since UNIX epoch

    UNIQUE(vlob_id, version)
);

-- user/device/realm_role certificates related to the current realm
--
-- There is no need for relationship between user/device given all those data
-- are small enough to have the script just load them once and kept them in memory
--
-- However we cannot just dump all the certificates in a single table given we cannot
-- preserve primary key when merging user/device/role tables together.

CREATE TABLE realm_role (
    _id PRIMARY KEY,
    role_certificate BLOB NOT NULL
);

CREATE TABLE user_ (
    _id PRIMARY KEY,
    user_certificate BLOB NOT NULL,
    revoked_user_certificate BLOB  -- NULL if user not revoked
);

CREATE TABLE user_update (
    _id PRIMARY_KEY,
    user_update_certificate BLOB NOT NULL
);

CREATE TABLE device (
    _id PRIMARY KEY,
    device_certificate BLOB NOT NULL
);
"""


def _sqlite_init_db(
    output_db_path: Path,
    organization_id: OrganizationID,
    realm_id: VlobID,
    root_verify_key: bytes,
) -> tuple[sqlite3.Connection, ExportStatus]:
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
                "INSERT INTO info (organization_id, realm_id, root_verify_key, export_up_to, export_status) VALUES (?, ?, ?, ?, ?)",
                (organization_id.str, realm_id.bytes, root_verify_key, ExportStatus.INITIALIZED.value),
            )
            con.commit()
        except sqlite3.Error as exc:
            raise RealmExporterOutputDbError(f"Cannot create export database: {exc}") from exc

    try:
        # Export database already exists, we should make sure it format is expected
        try:
            row = con.execute(
                "SELECT version, realm_id, root_verify_key, export_up_to, export_status FROM info WHERE magic = ?",
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
                "Existing output target is not a valid export database"
            )
        db_version, db_realm_id, db_root_verify_key, db_export_up_to, db_export_status = row
        if db_version != OUTPUT_DB_VERSION:
            raise RealmExporterOutputDbError(
                f"Existing output export database version format is not supported: got version `{db_version}` but only version `{OUTPUT_DB_VERSION}` is supported"
            )
        if db_realm_id != realm_id.bytes:
            raise RealmExporterOutputDbError(
                f"Existing output export database is for a different realm: got `{db_realm_id}` instead of expected `{realm_id.bytes}`"
            )
        if db_root_verify_key != root_verify_key:
            raise RealmExporterOutputDbError(
                f"Existing output export database is for a different realm: realm ID `{db_realm_id}` is the same but root verify key differs"
            )
        export_status = ExportStatus(db_export_status)

    except:
        con.close()
        raise

    return (con, export_status)


type ProgressReportCallback = Callable


async def export_realm(
        backend: Backend,
        organization_id: OrganizationID,
        realm_id: VlobID,
        output_db_path: Path,
        on_progress: ProgressReportCallback
    ):
        # 0) Load basic info about the export

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

        output_db_con, current_export_status = await asyncio.to_thread(
            _sqlite_init_db,
            output_db_path=output_db_path,
            organization_id=organization_id,
            realm_id=realm_id,
            root_verify_key=base_info.root_verify_key.encode(),
        )

        while True:

            match current_export_status:
                case ExportStatus.INITIALIZED:
                    # 2) Export certificates
                    current_export_status = await _do_export_certificates(backend, output_db_con, on_progress, organization_id, realm_id)

                case ExportStatus.CERTIFICATES_EXPORTED:
                    # 3) Export vlobs
                    current_export_status = await _do_export_vlobs(backend, output_db_con, on_progress)

                    # TODO

                    current_export_status = ExportStatus.VLOBS_EXPORTED
                    await asyncio.to_thread(_sqlite_update_export_status, output_db_con, current_export_status)

                case ExportStatus.VLOBS_EXPORTED:
                    # 3) Export blocks
                    current_export_status = await _do_export_blocks(backend, output_db_con, on_progress)

                    # TODO

                    current_export_status = ExportStatus.DONE
                    await asyncio.to_thread(_sqlite_update_export_status, output_db_con, current_export_status)

                case ExportStatus.DONE:
                    # 4) All done \o/
                    break


async def _do_export_certificates(backend: Backend, output_db_con: sqlite3.Connection, organization_id: OrganizationID, realm_id: VlobID) -> None:
    # Skip the operation if the export database already contains it

    def _get_certificates_export_done() -> Any:
        return output_db_con.execute(
            "SELECT certificates_export_done FROM info",
        ).fetchone()
    certificates_export_done = await asyncio.to_thread(_get_certificates_export_done)
    match certificates_export_done:
        case True:
            # Nothing to do
            return
        case False:
            # Export needed
            pass
        case unknown:
            raise RealmExporterOutputDbError(f"Output export database appears to be corrupted: `certificates_export_done` contains unexpected value {unknown:r}")

    # Fetch certificates

    # TODO: limit certificates to a certain date ?
    outcome = await backend.realm.export_do_certificates(organization_id=organization_id, realm_id=realm_id)
    match outcome:
        case RealmExportCertificates() as batch:
            pass
        case (RealmExportDoCertificatesBadOutcome.ORGANIZATION_NOT_FOUND | RealmExportDoCertificatesBadOutcome.REALM_NOT_FOUND) as error:
            # Organization&realm existence has already been checked, so this shouldn't occur
            raise RealmExporterInputError(f"Unexpect outcome when exporting certificates: {error}")

    # Write certificates to export database

    def _write_sqlite_db():
        output_db_con.executemany(
            "INSERT INTO realm_role (_id, role_certificate) VALUES (?, ?) ON CONFLICT(_id) DO NOTHING",
            batch.realm_role_certificates
        )
        output_db_con.executemany(
            "INSERT INTO user_ (_id, user_certificate, revoked_user_certificate) VALUES (?, ?, ?) ON CONFLICT(_id) DO NOTHING",
            batch.user_certificates
        )
        output_db_con.executemany(
            "INSERT INTO user_update (_id, user_update_certificate) VALUES (?, ?) ON CONFLICT(_id) DO NOTHING",
            batch.user_update_certificates
        )
        output_db_con.executemany(
            "INSERT INTO device (_id, device_certificate) VALUES (?, ?) ON CONFLICT(_id) DO NOTHING",
            batch.device_certificates
        )
        output_db_con.execute(
            "UPDATE info SET (certificates_export_done = TRUE)",
        )
        output_db_con.commit()

    await asyncio.to_thread(_write_sqlite_db)


async def _do_export_vlobs(backend: Backend, output_db_con: sqlite3.Connection, on_progress: ProgressReportCallback) -> ExportStatus:
    # TODO: retreive marker from SQLite

    # TODO: run multiple batches in parallel (but save in SQLite must be in order !)

    # TODO: while loop to run multiple batches missing


    # TODO: progress report

    # TODO: save batch to sqlite
    raise NotImplementedError


async def _do_export_blocks(backend: Backend, output_db_con: sqlite3.Connection, on_progress: ProgressReportCallback) -> ExportStatus:
    raise NotImplementedError
