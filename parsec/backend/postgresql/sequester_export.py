# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Optional, NewType, Tuple
import trio
import sqlite3
from contextlib import asynccontextmanager
from pathlib import Path

from parsec.api.protocol import OrganizationID, RealmID, BlockID, SequesterServiceID
from parsec.backend.postgresql import PGHandler
from parsec.backend.blockstore import BaseBlockStoreComponent


class RealmExporterError(Exception):
    pass


class RealmExporterInputError(Exception):
    pass


class RealmExporterOutputDbError(RealmExporterError):
    pass


BatchOffsetMarker = NewType("BatchOffsetMarker", int)


OUTPUT_DB_MAGIC_NUMBER = 87947
OUTPUT_DB_VERSION = 1
OUTPUT_DB_INIT_QUERY = f"""
-- Database info


-- This magic number has two roles:
-- 1) it makes unlikely we mistakenly consider an unrelated database as legit
-- 2) it acts as a constant ID to easily retrieve the single row in the table
CREATE TABLE IF NOT EXISTS info(
    magic INTEGER UNIQUE NOT NULL DEFAULT {OUTPUT_DB_MAGIC_NUMBER},
    version INTEGER NOT NULL DEFAULT {OUTPUT_DB_VERSION},
    realm_id BLOB NOT NULL,
    root_verify_key BLOB NOT NULL
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
-- the vlob is provided with third party encrytion key only once at creation time
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
    timestamp INTEGER NOT NULL,  -- ms since UNIX epoch

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

CREATE TABLE device (
    _id PRIMARY KEY,
    device_certificate BLOB NOT NULL
);
"""


async def _init_output_db(
    organization_id: OrganizationID,
    realm_id: RealmID,
    service_id: SequesterServiceID,
    output_db_path: Path,
    input_conn,
) -> None:
    # 0) Retreive organization/realm/sequester service from input database

    row = await input_conn.fetchrow(
        "SELECT _id, root_verify_key, sequester_authority_certificate FROM organization WHERE organization_id = $1",
        organization_id.str,
    )
    if not row:
        raise RealmExporterInputError(f"Organization `{organization_id.str}` doesn't exists")
    organization_internal_id = row["_id"]
    root_verify_key = row["root_verify_key"]
    if root_verify_key is None:
        raise RealmExporterInputError(f"Organization `{organization_id.str}` is not boostrapped")
    if not row["sequester_authority_certificate"]:
        raise RealmExporterInputError(
            f"Organization `{organization_id.str}` is not a sequestered organization"
        )

    row = await input_conn.fetchrow(
        "SELECT 1 FROM realm WHERE organization = $1 AND realm_id = $2",
        organization_internal_id,
        realm_id.uuid,
    )
    if not row:
        raise RealmExporterInputError(
            f"Realm `{realm_id.str}` doesn't exist in organization `{organization_id.str}`"
        )

    row = await input_conn.fetchrow(
        "SELECT 1 FROM sequester_service WHERE organization = $1 AND service_id = $2",
        organization_internal_id,
        service_id.uuid,
    )
    if not row:
        raise RealmExporterInputError(
            f"Sequester service `{service_id}` doesn't exist in organization `{organization_id.str}`"
        )

    # 1) Check the output database and create it if needed

    def _sqlite_init_db():
        try:
            con = sqlite3.connect(f"file:{output_db_path}?mode=rw", uri=True)
        except sqlite3.Error:
            # Export database doesn't exists
            try:
                # Create the database...
                con = sqlite3.connect(output_db_path)
                # ...and initialize it
                con.executescript(OUTPUT_DB_INIT_QUERY)
                con.execute(
                    "INSERT INTO info (realm_id, root_verify_key) VALUES (?, ?)",
                    (realm_id.bytes, root_verify_key),
                )
                con.commit()
            except sqlite3.Error as exc:
                raise RealmExporterOutputDbError(f"Cannot create export database: {exc}") from exc

        try:
            # Export database already exists, we should make sure it format is expected
            try:
                row = con.execute(
                    "SELECT version, realm_id, root_verify_key FROM info WHERE magic = ?",
                    (OUTPUT_DB_MAGIC_NUMBER,),
                ).fetchone()
            except sqlite3.Error as exc:
                # If we endup here this is most likely because `info` table doesn't exists (or miss some columns)
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: {exc}"
                ) from exc
            if not row:
                # `info` table exists and is valid, but magic number doesn't match
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database"
                )
            db_version, db_realm_id, db_root_verify_key = row
            if db_version != OUTPUT_DB_VERSION:
                raise RealmExporterOutputDbError(
                    f"Existing output export database version format is not supported: got version `{db_version}` but only version `1` is accepted"
                )
            if db_realm_id != realm_id.bytes:
                raise RealmExporterOutputDbError(
                    f"Existing output export database is for a different realm: got `{db_realm_id}` instead of expected `{realm_id.bytes}`"
                )
            if db_root_verify_key != root_verify_key:
                raise RealmExporterOutputDbError(
                    f"Existing output export database is for a different realm: realm ID `{db_realm_id}` is the same but root verify key differs"
                )
        finally:
            con.close()

    await trio.to_thread.run_sync(_sqlite_init_db)

    # 2) Export the certificates
    # Note in all those exports we keep the `_id` primary key from the input database,
    # this is a trick so we don't have to modify the `author` field in block/vlob_atom

    # User certificates
    rows = await input_conn.fetch(
        """
SELECT _id, user_certificate, revoked_user_certificate
FROM user_
WHERE organization = (SELECT _id FROM organization WHERE organization_id = $1)
""",
        organization_id.str,
    )

    def _sqlite_save_user_certifs():
        output_con = sqlite3.connect(output_db_path)
        try:
            output_con.executemany(
                """
INSERT INTO user_ (_id, user_certificate, revoked_user_certificate)
VALUES (?, ?, ?)
ON CONFLICT DO NOTHING
""",
                rows,
            )
            output_con.commit()
        finally:
            output_con.close()

    await trio.to_thread.run_sync(_sqlite_save_user_certifs)

    # Device certificates
    rows = await input_conn.fetch(
        """
SELECT _id, device_certificate
FROM device
WHERE organization = (
SELECT _id FROM organization WHERE organization_id = $1
)""",
        organization_id.str,
    )

    def _sqlite_save_device_certifs():
        output_con = sqlite3.connect(output_db_path)
        try:
            output_con.executemany(
                "INSERT INTO device (_id, device_certificate) VALUES (?, ?) ON CONFLICT DO NOTHING",
                rows,
            )
            output_con.commit()
        finally:
            output_con.close()

    await trio.to_thread.run_sync(_sqlite_save_device_certifs)

    # Realm role certificates
    rows = await input_conn.fetch(
        """
SELECT _id, certificate
FROM realm_user_role
WHERE realm = (
SELECT _id
FROM realm
WHERE
realm_id = $2
AND organization = (SELECT _id FROM organization WHERE organization_id = $1)
)
""",
        organization_id.str,
        realm_id,
    )

    def _sqlite_save_realm_role_certifs():
        output_con = sqlite3.connect(output_db_path)
        try:
            output_con.executemany(
                "INSERT INTO realm_role (_id, role_certificate) VALUES (?, ?) ON CONFLICT DO NOTHING",
                rows,
            )
            output_con.commit()
        finally:
            output_con.close()

    await trio.to_thread.run_sync(_sqlite_save_realm_role_certifs)


class RealmExporter:
    def __init__(
        self,
        organization_id: OrganizationID,
        realm_id: RealmID,
        service_id: SequesterServiceID,
        output_db_path: Path,
        input_dbh: PGHandler,
        input_blockstore: BaseBlockStoreComponent,
    ):
        self.organization_id = organization_id
        self.realm_id = realm_id
        self.service_id = service_id
        self.output_db_path = output_db_path
        self.input_dbh = input_dbh
        self.input_blockstore = input_blockstore

    @classmethod
    @asynccontextmanager
    async def run(
        cls,
        organization_id: OrganizationID,
        realm_id: RealmID,
        service_id: SequesterServiceID,
        output_db_path: Path,
        input_dbh: PGHandler,
        input_blockstore: BaseBlockStoreComponent,
    ):
        async with input_dbh.pool.acquire() as input_conn:
            await _init_output_db(
                organization_id=organization_id,
                realm_id=realm_id,
                service_id=service_id,
                output_db_path=output_db_path,
                input_conn=input_conn,
            )

        yield cls(
            organization_id=organization_id,
            realm_id=realm_id,
            service_id=service_id,
            output_db_path=output_db_path,
            input_dbh=input_dbh,
            input_blockstore=input_blockstore,
        )

    # Vlobs export

    async def compute_vlobs_export_status(self) -> Tuple[int, BatchOffsetMarker]:
        def _retreive_vlobs_export_status():
            con = sqlite3.connect(self.output_db_path)
            try:
                row = con.execute("SELECT max(_id) FROM vlob_atom").fetchone()
            finally:
                con.close()
            return row[0] or 0

        last_exported_index = await trio.to_thread.run_sync(_retreive_vlobs_export_status)

        async with self.input_dbh.pool.acquire() as conn:
            rows = await conn.fetch(
                """
SELECT
    count(*)
FROM
    realm_vlob_update
WHERE
    realm = (
        SELECT _id
        FROM realm
        WHERE
            realm_id = $1
            AND organization = (SELECT _id FROM organization WHERE organization_id = $2)
    )
""",
                self.realm_id,
                self.organization_id.str,
            )
            to_export_count = rows[0][0]

        return (to_export_count, last_exported_index)

    async def export_vlobs(
        self, batch_size: int = 1000, batch_offset_marker: Optional[BatchOffsetMarker] = None
    ) -> BatchOffsetMarker:
        batch_offset_marker = batch_offset_marker or 0

        async with self.input_dbh.pool.acquire() as conn:
            rows = await conn.fetch(
                """
SELECT
    -- Return index instead of `vlob_atom.id` given it is more readable and we export only a single realm
    realm_vlob_update.index AS _id,
    vlob_atom.vlob_id,
    vlob_atom.version,
    sequester_service_vlob_atom.blob,
    vlob_atom.author,
    vlob_atom.created_on AS timestamp
FROM
    realm_vlob_update
    LEFT JOIN vlob_atom
        ON realm_vlob_update.vlob_atom = vlob_atom._id
    LEFT JOIN sequester_service_vlob_atom
        ON sequester_service_vlob_atom.vlob_atom = vlob_atom._id
WHERE
    realm_vlob_update.realm = (
        SELECT _id
        FROM realm
        WHERE
            realm_id = $1
            AND organization = (SELECT _id FROM organization WHERE organization_id = $2)
    )
    AND realm_vlob_update.index >= $3
    AND sequester_service_vlob_atom.service = (SELECT _id FROM sequester_service WHERE service_id = $4)
LIMIT $5
""",
                self.realm_id,
                self.organization_id.str,
                batch_offset_marker,
                self.service_id,
                batch_size,
            )

            def _save_in_output_db():
                # Must convert `vlob_id`` fields from UUID to bytes given SQLite doesn't handle the former
                # Must also convert datetime to a number of ms since UNIX epoch
                cooked_rows = [
                    (r[0], r[1].bytes, r[2], r[3], r[4], int(r[5].timestamp() * 1000)) for r in rows
                ]
                con = sqlite3.connect(self.output_db_path)
                try:
                    con.executemany(
                        """
INSERT INTO vlob_atom (
    _id,
    vlob_id,
    version,
    blob,
    author,
    timestamp
)
VALUES (?, ?, ?, ?, ?, ?)
ON CONFLICT DO NOTHING
""",
                        cooked_rows,
                    )
                    con.commit()
                finally:
                    con.close()

            await trio.to_thread.run_sync(_save_in_output_db)

        return max(r["_id"] for r in rows)

    # Blocks export

    async def compute_blocks_export_status(self) -> Tuple[int, BatchOffsetMarker]:
        def _retreive_vlobs_export_status():
            con = sqlite3.connect(self.output_db_path)
            try:
                row = con.execute("SELECT max(_id) FROM block").fetchone()
            finally:
                con.close()
            return row[0] or 0

        last_exported_index = await trio.to_thread.run_sync(_retreive_vlobs_export_status)

        async with self.input_dbh.pool.acquire() as conn:
            rows = await conn.fetch(
                """
SELECT
    count(*)
FROM
    block
WHERE
    realm = (
        SELECT _id
        FROM realm
        WHERE
            realm_id = $1
            AND organization = (SELECT _id FROM organization WHERE organization_id = $2)
    )
""",
                self.realm_id,
                self.organization_id.str,
            )
            to_export_count = rows[0][0]

        return (to_export_count, last_exported_index)

    async def export_blocks(
        self, batch_size: int = 100, batch_offset_marker: Optional[BatchOffsetMarker] = None
    ) -> BatchOffsetMarker:
        batch_offset_marker = batch_offset_marker or 0

        async with self.input_dbh.pool.acquire() as conn:
            rows = await conn.fetch(
                """
SELECT
    _id,
    block_id,
    author
FROM
    block
WHERE
    realm = (
        SELECT _id
        FROM realm
        WHERE
            realm_id = $1
            AND organization = (SELECT _id FROM organization WHERE organization_id = $2)
    )
    AND _id >= $3
LIMIT $4
""",
                self.realm_id,
                self.organization_id.str,
                batch_offset_marker,
                batch_size,
            )
            cooked_rows = []
            for row in rows:
                block = await self.input_blockstore.read(
                    organization_id=self.organization_id, block_id=BlockID(row["block_id"])
                )
                cooked_rows.append(
                    (
                        row["_id"],
                        # Must convert `block_id`` fields from UUID to bytes given SQLite doesn't handle the former
                        row["block_id"].bytes,
                        block,
                        row["author"],
                    )
                )

            def _save_in_output_db():
                con = sqlite3.connect(self.output_db_path)
                try:
                    # Must convert `vlob_id`` fields from UUID to bytes given SQLite doesn't handle the former
                    con.executemany(
                        """
INSERT INTO block (
    _id,
    block_id,
    data,
    author
)
VALUES (?, ?, ?, ?)
ON CONFLICT DO NOTHING
""",
                        cooked_rows,
                    )
                    con.commit()
                finally:
                    con.close()

            await trio.to_thread.run_sync(_save_in_output_db)

        return max(r["_id"] for r in rows)
