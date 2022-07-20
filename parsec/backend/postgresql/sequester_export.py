# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from typing import Optional, NewType, Tuple
import trio
import sqlite3
from contextlib import asynccontextmanager

from parsec.api.protocol import OrganizationID, RealmID, BlockID, SequesterServiceID
from parsec.backend.postgresql import PGHandler
from parsec.backend.blockstore import BaseBlockStoreComponent


class RealmExporterOutputDbError(Exception):
    pass


BatchOffsetMarker = NewType("BatchOffsetMarker", int)


OUTPUT_DB_MAGIC_NUMBER = 87947
OUTPUT_DB_VERSION = 1
OUTPUT_DB_INIT_QUERY = f"""
-- Database info


-- This magic number has two roles:
-- - it makes unlikely we mistakenly consider an unrelated database as a legit
-- - it acts as a constant ID to easily retrieve the single row in the table
CREATE TABLE IF NOT EXISTS info(
    magic INTEGER UNIQUE NOT NULL DEFAULT {OUTPUT_DB_MAGIC_NUMBER},
    version INTEGER NOT NULL DEFAULT {OUTPUT_DB_VERSION},  -- should be 1 for now
    realm_id UUID NOT NULL
);


-- Metadata & Data export


CREATE TABLE block (
    -- _id is not SERIAL given we will take the one present in the Parsec database
    _id PRIMARY KEY,
    block_id UUID NOT NULL,
    data BYTEA NOT NULL,
    -- TODO: are author/size/created_on useful ?
    author INTEGER REFERENCES device (_id) NOT NULL,
    size INTEGER NOT NULL,
    -- this field is created by the backend when inserting (unlike vlob's timestamp, see below)
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE(block_id)
);

-- Compared to Parsec's datamodel, we don't store `vlob_encryption_revision` given
-- the vlob is provided with third party encrytion key only once at creation time
CREATE TABLE vlob_atom (
    -- We use vlob_update's index as primary key, this is convenient given it makes trivial
    -- keeping track of how far we got when restarting an export
    _id PRIMARY KEY,
    vlob_id UUID NOT NULL,
    version INTEGER NOT NULL,
    blob BYTEA NOT NULL,
    size INTEGER NOT NULL,
    -- author/timestamp are required to validate the consistency of blob
    -- Care must be taken when exporting this field (and the device_ table) to
    -- keep this relationship valid !
    -- this field is called created_on in Parsec datamodel, but it correspond to the timestamp field in the API
    author INTEGER REFERENCES device (_id) NOT NULL,
    -- (the value is provided by the client when sending request and not created on backend side) so better
    -- give it the better understandable name
    timestamp TIMESTAMPTZ NOT NULL,

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
    role_certificate BYTEA NOT NULL
);

CREATE TABLE user_ (
    _id PRIMARY KEY,
    user_certificate BYTEA NOT NULL,
    revoked_user_certificate BYTEA  -- NULL if user not revoked
);

CREATE TABLE device (
    _id PRIMARY KEY,
    device_certificate BYTEA NOT NULL
);
"""


class RealmExporter:
    def __init__(
        self,
        organization_id: OrganizationID,
        realm_id: RealmID,
        service_id: SequesterServiceID,
        output_db_path: sqlite3.Connection,
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
        output_db_path: sqlite3.Connection,
        input_dbh: PGHandler,
        input_blockstore: BaseBlockStoreComponent,
    ):
        exporter = cls(
            organization_id=organization_id,
            realm_id=realm_id,
            service_id=service_id,
            output_db_path=output_db_path,
            input_dbh=input_dbh,
            input_blockstore=input_blockstore,
        )
        await exporter._init_output_db()
        yield exporter

    async def _init_output_db(self):
        # 1) Check the output database and create it if needed

        def _init_output_db():
            try:
                con = sqlite3.connect(f"file:{self.output_db_path}?mode=rw", uri=True)
            except sqlite3.Error:
                # Export database doesn't exists
                try:
                    # Create the database...
                    con = sqlite3.connect(self.output_db_path)
                    # ...and initialize it
                    con.executescript(OUTPUT_DB_INIT_QUERY)
                    con.execute("INSERT INTO info (realm_id) VALUES (?)", (self.realm_id.bytes,))
                    con.commit()
                except sqlite3.Error as exc:
                    raise RealmExporterOutputDbError(
                        f"Cannot create export database: {exc}"
                    ) from exc

            try:
                # Export database already exists, we should make sure it format is expected
                try:
                    row = con.execute(
                        "SELECT version, realm_id FROM info WHERE magic = ?",
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
                db_version, db_realm_id = row
                if db_version != OUTPUT_DB_VERSION:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database version format is not supported: got version `{db_version}` but only version `1` is accepted"
                    )
                if db_realm_id != self.realm_id.bytes:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database is for a different realm: got `{db_realm_id}` instead of expected `{self.realm_id.bytes}`"
                    )
            finally:
                con.close()

        await trio.to_thread.run_sync(_init_output_db)

        # 2) Export the certificates
        # Note in all those exports we keep the `_id` primary key from the input database,
        # this is a trick so we don't have to modify the `author` field in block/vlob_atom
        async with self.input_dbh.pool.acquire() as input_conn:
            # User certificates
            rows = await input_conn.fetch(
                """
SELECT _id, user_certificate, revoked_user_certificate
FROM user_
WHERE organization = (SELECT _id FROM organization WHERE organization_id = $1)
""",
                self.organization_id.str,
            )

            def _save_in_output_db():
                output_con = sqlite3.connect(self.output_db_path)
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

            await trio.to_thread.run_sync(_save_in_output_db)

            # Device certificates
            rows = await input_conn.fetch(
                """
SELECT _id, device_certificate
FROM device
WHERE organization = (
    SELECT _id FROM organization WHERE organization_id = $1
)""",
                self.organization_id.str,
            )

            def _save_in_output_db():
                output_con = sqlite3.connect(self.output_db_path)
                try:
                    output_con.executemany(
                        "INSERT INTO device (_id, device_certificate) VALUES (?, ?) ON CONFLICT DO NOTHING",
                        rows,
                    )
                    output_con.commit()
                finally:
                    output_con.close()

            await trio.to_thread.run_sync(_save_in_output_db)

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
                self.organization_id.str,
                self.realm_id,
            )

            def _save_in_output_db():
                output_con = sqlite3.connect(self.output_db_path)
                try:
                    output_con.executemany(
                        "INSERT INTO realm_role (_id, role_certificate) VALUES (?, ?) ON CONFLICT DO NOTHING",
                        rows,
                    )
                    output_con.commit()
                finally:
                    output_con.close()

            await trio.to_thread.run_sync(_save_in_output_db)

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
    sequester_service_vlob.blob,
    vlob_atom.size,
    vlob_atom.author,
    vlob_atom.created_on AS timestamp
FROM
    realm_vlob_update
    LEFT JOIN vlob_atom
        ON realm_vlob_update.vlob_atom = vlob_atom._id
    LEFT JOIN sequester_service_vlob
        ON sequester_service_vlob.vlob = vlob_atom._id
WHERE
    realm_vlob_update.realm = (
        SELECT _id
        FROM realm
        WHERE
            realm_id = $1
            AND organization = (SELECT _id FROM organization WHERE organization_id = $2)
    )
    AND realm_vlob_update.index >= $3
    AND sequester_service_vlob.service = (SELECT _id FROM sequester_service WHERE service_id = $4)
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
                cooked_rows = [(r[0], r[1].bytes, r[2], r[3], r[4], r[5], r[6]) for r in rows]
                con = sqlite3.connect(self.output_db_path)
                try:
                    con.executemany(
                        """
INSERT INTO vlob_atom (
    _id,
    vlob_id,
    version,
    blob,
    size,
    author,
    timestamp
)
VALUES (?, ?, ?, ?, ?, ?, ?)
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
    author,
    size,
    created_on
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
                    organization_id=self.organization_id, id=BlockID(row["block_id"])
                )
                cooked_rows.append(
                    (
                        row["_id"],
                        # Must convert `block_id`` fields from UUID to bytes given SQLite doesn't handle the former
                        row["block_id"].bytes,
                        block,
                        row["author"],
                        row["size"],
                        row["created_on"],
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
    author,
    size,
    created_on
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
