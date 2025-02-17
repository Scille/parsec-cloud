# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import queue
import sqlite3
import threading
from collections import deque
from contextlib import asynccontextmanager
from pathlib import Path
from typing import Any, AsyncGenerator, Callable, Literal

import anyio

from parsec._parsec import BlockID, DateTime, OrganizationID, VlobID
from parsec.backend import Backend
from parsec.ballpark import BALLPARK_CLIENT_LATE_OFFSET
from parsec.components.blockstore import BlockStoreReadBadOutcome
from parsec.components.realm import (
    RealmExportBlocksMetadataBatch,
    RealmExportCertificates,
    RealmExportDoBaseInfo,
    RealmExportDoBaseInfoBadOutcome,
    RealmExportDoBlocksBatchMetadataBadOutcome,
    RealmExportDoCertificatesBadOutcome,
    RealmExportDoVlobsBatchBadOutcome,
    RealmExportVlobsBatch,
)
from parsec.logging import get_logger

logger = get_logger()


type SequentialID = int


class RealmExporterError(Exception):
    pass


class RealmExporterInputError(RealmExporterError):
    """
    Error raised due to invalid input parameters (i.e. the caller is at fault).

    This typically occurs when trying to export a non-existing realm.
    """

    pass


class RealmExporterOutputDbError(RealmExporterError):
    """
    Error raised due to an issue with the output SQLite database.

    This typically occurs when the database exist but is corrupted, or doesn't
    correspond to the realm being exported.
    """

    pass


class RealmExporterInputDbError(RealmExporterError):
    """
    Error raised due to an issue with the input PostgreSQL database, or the
    blockstore.

    This typically occurs if there is some inconsistency between the block
    metadata (stored in PostgreSQL) and the block data (stored in the blockstore).
    """

    pass


# Considering vlob to be a couple of Ko in size, 100k items means under 1Go of data per batch
VLOB_EXPORT_BATCH_SIZE = 100_000
# Block metadata are really small (< 100 bytes)
BLOCK_METADATA_EXPORT_BATCH_SIZE = 1_000_000
BLOCK_DATA_EXPORT_PARALLELISM = 10
# Among of RAM we are willing to use to store block data in memory
# before flushing it to the SQLite database.
BLOCK_DATA_EXPORT_RAM_LIMIT = 2**30  # 1Go
MAX_CONSECUTIVE_STORE_UNAVAILABLE_ERRORS = 5
MAX_STORE_UNAVAILABLE_BACKOFF_SLEEP = 60  # seconds

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
    -- (Note this doesn't necessarily correspond to when the export has been done).
    snapshot_timestamp INTEGER NOT NULL, -- us since UNIX epoch

    -- Total amount of data exported
    vlobs_total_bytes INTEGER NOT NULL,
    blocks_total_bytes INTEGER NOT NULL,

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

CREATE TABLE realm_keys_bundle (
    key_index INTEGER PRIMARY KEY,
    keys_bundle BLOB NOT NULL
);

-- Note there can be multiple entries with the same (user_id, key_index) pair
-- as a new access is provided by every new sharing.
CREATE TABLE realm_keys_bundle_access (
    _id INTEGER PRIMARY KEY,
    user_id BLOB NOT NULL,
    key_index INTEGER NOT NULL,
    access BLOB NOT NULL
);

CREATE TABLE realm_sequester_keys_bundle_access (
    _id INTEGER PRIMARY KEY,
    sequester_service_id BLOB NOT NULL,
    key_index INTEGER NOT NULL,
    access BLOB NOT NULL
);

-------------------------------------------------------------------------
-- Vlobs export
-------------------------------------------------------------------------


CREATE TABLE vlob_atom (
    -- We use `realm_vlob_update`'s `index` field as primary key.
    -- This means the vlob atoms are ordered according to how they got added
    -- in the server in the first place.
    sequential_id INTEGER PRIMARY KEY,
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
    timestamp INTEGER NOT NULL,  -- us since UNIX epoch

    UNIQUE(vlob_id, timestamp)
);


-------------------------------------------------------------------------
-- Blocks export
-------------------------------------------------------------------------


CREATE TABLE block (
    -- Primary key is not SERIAL given we will take the one present in the Parsec database.
    -- This means the blocks are ordered according to how they got added in the server
    -- in the first place.
    sequential_id INTEGER PRIMARY KEY,
    block_id BLOB NOT NULL UNIQUE,
    author BLOB NOT NULL,  -- DeviceID
    size INTEGER NOT NULL,
    key_index INTEGER NOT NULL
);

CREATE TABLE block_data (
    block INTEGER PRIMARY KEY REFERENCES block(sequential_id),
    data BLOB NOT NULL
);
"""


def _sqlite_init_db(
    output_db_path: Path,
    organization_id: OrganizationID,
    realm_id: VlobID,
    snapshot_timestamp: DateTime,
    root_verify_key: bytes,
    vlobs_total_bytes: int,
    blocks_total_bytes: int,
) -> sqlite3.Connection:
    try:
        con = sqlite3.connect(
            f"file:{output_db_path}?mode=rw", uri=True, autocommit=False, check_same_thread=True
        )
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
                    snapshot_timestamp,\
                    root_verify_key,\
                    vlobs_total_bytes,\
                    blocks_total_bytes\
                ) VALUES (?, ?, ?, ?, ?, ?)\
                """,
                (
                    organization_id.str,
                    realm_id.bytes,
                    snapshot_timestamp.as_timestamp_micros(),
                    root_verify_key,
                    vlobs_total_bytes,
                    blocks_total_bytes,
                ),
            )
            con.commit()
        except sqlite3.Error as exc:
            raise RealmExporterOutputDbError(f"Cannot create export database: {exc}") from exc

    try:
        # Export database already exists, we should make sure it format is expected
        try:
            row = con.execute(
                """SELECT\
                    version,\
                    organization_id,\
                    realm_id,\
                    snapshot_timestamp,\
                    root_verify_key,\
                    vlobs_total_bytes,\
                    blocks_total_bytes\
                FROM info WHERE magic = ?\
                """,
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
        (
            db_version,
            db_organization_id,
            db_realm_id,
            db_snapshot_timestamp,
            db_root_verify_key,
            db_vlobs_total_bytes,
            db_blocks_total_bytes,
        ) = row

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

        match db_root_verify_key:
            case bytes():
                if db_root_verify_key != root_verify_key:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database is for a different realm: realm ID `0x{realm_id.hex}` is the same but root verify key differs"
                    )
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: invalid `info.root_verify_key` value `{unknown!r}` (expected bytes)"
                )

        match db_vlobs_total_bytes:
            case int():
                if db_vlobs_total_bytes != vlobs_total_bytes:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database doesn't match: realm ID `0x{realm_id.hex}` and snapshot timestamp `{snapshot_timestamp}` are the same, but vlobs total bytes differs"
                    )
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: invalid `info.vlobs_total_bytes` value `{unknown!r}` (expected int)"
                )

        match db_blocks_total_bytes:
            case int():
                if db_blocks_total_bytes != blocks_total_bytes:
                    raise RealmExporterOutputDbError(
                        f"Existing output export database doesn't match: realm ID `0x{realm_id.hex}` and snapshot timestamp `{snapshot_timestamp}` are the same, but blocks total bytes differs"
                    )
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Existing output target is not a valid export database: invalid `info.blocks_total_bytes` value `{unknown!r}` (expected int)"
                )

    except:
        con.close()
        raise

    return con


# We send callback function from asyncio to access the SQLite connection.
# This is to have a single thread managing the SQLite connection and ensuring
# thread safety on it.
type OutputDBSqliteJobCb[R] = tuple[Callable[[sqlite3.Connection], R], asyncio.Queue[R | Exception]]


class OutputDBConnection:
    def __init__(self, queries_queue: queue.Queue[OutputDBSqliteJobCb[Any]]):
        # Queue contains: (SQL query, parameters, oneshot queue to return result)
        self._queries_queue = queries_queue

    @classmethod
    @asynccontextmanager
    async def connect_or_create(
        cls,
        output_db_path: Path,
        organization_id: OrganizationID,
        realm_id: VlobID,
        snapshot_timestamp: DateTime,
        root_verify_key: bytes,
        vlobs_total_bytes: int,
        blocks_total_bytes: int,
    ) -> AsyncGenerator[OutputDBConnection]:
        # Make this queue infinite to ensure we won't block the asyncio event loop
        queries_queue: queue.Queue[OutputDBSqliteJobCb[Any]] = queue.Queue(maxsize=0)
        sqlite3_worker_ready = asyncio.Event()
        stop_requested = False

        asyncio_loop = asyncio.get_event_loop()

        def _sqlite3_worker():
            nonlocal sqlite3_worker_ready
            nonlocal stop_requested

            con = _sqlite_init_db(
                output_db_path,
                organization_id,
                realm_id,
                snapshot_timestamp,
                root_verify_key,
                vlobs_total_bytes,
                blocks_total_bytes,
            )
            try:
                asyncio_loop.call_soon_threadsafe(sqlite3_worker_ready.set)
                del sqlite3_worker_ready

                while not stop_requested:
                    cb, result_queue = queries_queue.get()
                    if stop_requested:
                        break
                    try:
                        result = cb(con)
                    except Exception as exc:
                        asyncio_loop.call_soon_threadsafe(result_queue.put_nowait, exc)
                    else:
                        asyncio_loop.call_soon_threadsafe(result_queue.put_nowait, result)

            finally:
                con.close()

        try:
            async with asyncio.TaskGroup() as tg:
                sqlite3_worker_task = tg.create_task(asyncio.to_thread(_sqlite3_worker))
                await sqlite3_worker_ready.wait()

                try:
                    yield cls(queries_queue)
                finally:
                    stop_requested = True
                    # Also push a dummy job to wake up the worker if no job is in the queue
                    queries_queue.put_nowait((lambda _: None, asyncio.Queue(maxsize=1)))
                    # Ensure the SQLite3 worker has been properly stopped before leaving,
                    # this is especially important during test to ensure the SQLite3 database
                    # has been properly closed before retrying to use it (to avoid
                    # `sqlite3.OperationalError: database is locked` error).
                    with anyio.CancelScope(shield=True):
                        await sqlite3_worker_task

        # `RealmExporterError` are our own error type, so if it occurs there most
        # likely won't be any other concurrent errors (and if there is, those errors
        # are less relevant than our own error in any case).
        except* RealmExporterError as exc:
            raise exc.exceptions[0]

    async def execute[R](self, cb: Callable[[sqlite3.Connection], R]) -> R:
        ret_queue: asyncio.Queue[R | Exception] = asyncio.Queue(maxsize=1)
        self._queries_queue.put_nowait((cb, ret_queue))
        match await ret_queue.get():
            case Exception() as exc:
                raise exc
            case ret:
                return ret


type ToExportBytes = int  # Total amount of data in bytes to be exported
type ExportedBytes = int  # Amount of data in bytes that have been exported so far
type ExportProgressStep = (
    Literal["certificates_start"]
    | Literal["certificates_done"]
    | tuple[Literal["vlobs"], ExportedBytes, ToExportBytes]
    | tuple[Literal["blocks_metadata"], ExportedBytes, ToExportBytes]
    | tuple[Literal["blocks_data"], ExportedBytes, ToExportBytes]
)
type ProgressReportCallback = Callable[[ExportProgressStep], None]


def default_realm_export_db_name(
    organization_id: OrganizationID, realm_id: VlobID, snapshot_timestamp: DateTime
) -> str:
    # The obvious choice would be to put the timestamp in the filename as RFC3339, however:
    # - The format contains `:` characters that don't play nice with Windows file system.
    # - The format is pretty verbose, and the file name is already long enough.
    # So instead we just use a custom `yyyymmddThhmmssZ` format.
    display_date = snapshot_timestamp.to_rfc3339().replace("-", "").replace(":", "")
    if "." in display_date:
        display_date = display_date.split(".", 1)[0] + "Z"
    return f"parsec-export-{organization_id.str}-realm-{realm_id.hex}-{display_date}.sqlite"


def get_earliest_allowed_snapshot_timestamp() -> DateTime:
    """
    The very first step we do when exporting a realm is to determine what should be
    exported according to the snapshot timestamp parameter.

    Obviously snapshot timestamp is not allowed to be beyond present time since only
    Bardock can see in the future.

    On top of that, it is also bad to have the snapshot timestamp in the past but
    too close to the present time as the server is still allowed accept data timed
    before our snapshot timestamp.
    """
    return DateTime.now().subtract(seconds=BALLPARK_CLIENT_LATE_OFFSET)


async def export_realm(
    backend: Backend,
    organization_id: OrganizationID,
    realm_id: VlobID,
    snapshot_timestamp: DateTime,
    output_db_path: Path,
    on_progress: ProgressReportCallback,
):
    earlier_allowed_timestamp = get_earliest_allowed_snapshot_timestamp()
    if snapshot_timestamp > earlier_allowed_timestamp:
        raise RealmExporterInputError(
            f"Snapshot timestamp cannot be more recent than present time - {BALLPARK_CLIENT_LATE_OFFSET}s (i.e. `{earlier_allowed_timestamp}`)"
        )

    # 0) Ensure the export is possible since organization & realm exist on the server

    outcome = await backend.realm.export_do_base_info(
        organization_id=organization_id, realm_id=realm_id, snapshot_timestamp=snapshot_timestamp
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
        case RealmExportDoBaseInfoBadOutcome.REALM_DIDNT_EXIST_AT_SNAPSHOT_TIMESTAMP:
            raise RealmExporterInputError(
                f"Requested snapshot timestamp `{snapshot_timestamp}` is older than realm creation"
            )

    # 1) Create the output SQLite database (or re-open if it already exists)
    #
    #    Note that the database stores everything needed to determine if the export
    #    is complete, and if not, everything to be able to continue it ;-)
    #
    #    So the following commands lead to the same foo.sqlite (even if subsequent
    #    export occurs after additional changes are added to the export):
    #
    #    $ realm_export ./foo.sqlite
    #    $ realm_export ./foo.sqlite &; sleep 1; kill %1; realm_export ./foo.sqlite

    async with OutputDBConnection.connect_or_create(
        output_db_path=output_db_path,
        organization_id=organization_id,
        realm_id=realm_id,
        snapshot_timestamp=snapshot_timestamp,
        root_verify_key=base_info.root_verify_key.encode(),
        vlobs_total_bytes=base_info.vlobs_total_bytes,
        blocks_total_bytes=base_info.blocks_total_bytes,
    ) as output_db_con:
        # 2) Export certificates

        await _do_export_certificates(
            on_progress,
            backend,
            output_db_con,
            organization_id,
            realm_id,
            common_certificate_timestamp_upper_bound=base_info.common_certificate_timestamp_upper_bound,
            realm_certificate_timestamp_upper_bound=base_info.realm_certificate_timestamp_upper_bound,
            sequester_certificate_timestamp_upper_bound=base_info.sequester_certificate_timestamp_upper_bound,
        )

        # 3) Export vlobs

        await _do_export_vlobs(
            on_progress,
            backend,
            output_db_con,
            organization_id,
            realm_id,
            base_info.vlob_offset_marker_upper_bound,
        )

        # 4) Export blocks metadata

        await _do_export_blocks_metadata(
            on_progress,
            backend,
            output_db_con,
            organization_id,
            realm_id,
            base_info.block_offset_marker_upper_bound,
        )

        # 4) Export blocks data

        await _do_export_blocks_data(on_progress, backend, output_db_con, organization_id)

        # 5) All done \o/


async def _do_export_certificates(
    on_progress: ProgressReportCallback,
    backend: Backend,
    output_db_con: OutputDBConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    common_certificate_timestamp_upper_bound: DateTime,
    realm_certificate_timestamp_upper_bound: DateTime,
    sequester_certificate_timestamp_upper_bound: DateTime | None,
) -> None:
    # 0) Skip the operation if the export database already contains it

    def _get_certificates_export_done(con: sqlite3.Connection) -> bool:
        row = con.execute(
            "SELECT certificates_export_done FROM info",
        ).fetchone()
        match row[0]:
            case 1:
                return True
            case 0:
                return False
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Output export database appears to be corrupted: `certificates_export_done` contains unexpected value `{unknown!r}`"
                )

    certificates_export_done = await output_db_con.execute(_get_certificates_export_done)
    if certificates_export_done:
        return

    # Export needed

    on_progress("certificates_start")

    # 1) Fetch all certificates

    outcome = await backend.realm.export_do_certificates(
        organization_id=organization_id,
        realm_id=realm_id,
        common_certificate_timestamp_upper_bound=common_certificate_timestamp_upper_bound,
        realm_certificate_timestamp_upper_bound=realm_certificate_timestamp_upper_bound,
        sequester_certificate_timestamp_upper_bound=sequester_certificate_timestamp_upper_bound,
    )
    match outcome:
        case RealmExportCertificates() as certificates:
            pass
        case (
            (
                RealmExportDoCertificatesBadOutcome.ORGANIZATION_NOT_FOUND
                | RealmExportDoCertificatesBadOutcome.REALM_NOT_FOUND
            ) as error
        ):
            # Organization&realm existence has already been checked, so this shouldn't occur
            raise RealmExporterInputError(f"Unexpect outcome when exporting certificates: {error}")

    # 2) Write certificates to export database (and mark this export step as done)

    def _write_sqlite_db(con: sqlite3.Connection):
        con.executemany(
            "INSERT INTO common_certificate (_id, certificate) VALUES (?, ?)",
            enumerate(certificates.common_certificates),
        )
        con.executemany(
            "INSERT INTO realm_certificate (_id, certificate) VALUES (?, ?)",
            enumerate(certificates.realm_certificates),
        )
        con.executemany(
            "INSERT INTO sequester_certificate (_id, certificate) VALUES (?, ?)",
            enumerate(certificates.sequester_certificates),
        )
        con.executemany(
            "INSERT INTO realm_keys_bundle (key_index, keys_bundle) VALUES (?, ?)",
            certificates.realm_keys_bundles,
        )
        con.executemany(
            "INSERT INTO realm_keys_bundle_access (_id, user_id, key_index, access) VALUES (?, ?, ?, ?)",
            (
                (i, user_id.bytes, key_index, access)
                for i, (user_id, key_index, access) in enumerate(
                    certificates.realm_keys_bundle_user_accesses
                )
            ),
        )
        con.executemany(
            "INSERT INTO realm_sequester_keys_bundle_access (_id, sequester_service_id, key_index, access) VALUES (?, ?, ?, ?)",
            (
                (i, sequester_service_id.bytes, key_index, access)
                for i, (sequester_service_id, key_index, access) in enumerate(
                    certificates.realm_keys_bundle_sequester_accesses
                )
            ),
        )
        con.execute(
            "UPDATE info SET certificates_export_done = 1",
        )
        con.commit()

    await output_db_con.execute(_write_sqlite_db)

    on_progress("certificates_done")


async def _do_export_vlobs(
    on_progress: ProgressReportCallback,
    backend: Backend,
    output_db_con: OutputDBConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    vlob_offset_marker_upper_bound: SequentialID,
) -> None:
    # 0) Skip the operation if the export database already contains it

    def _get_vlobs_export_done(
        con: sqlite3.Connection,
    ) -> tuple[bool, SequentialID, ExportedBytes, ToExportBytes]:
        row = con.execute("SELECT vlobs_export_done, vlobs_total_bytes FROM info").fetchone()
        match row[0]:
            case 1:
                done = True
            case 0:
                done = False
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Output export database appears to be corrupted: `info.vlobs_export_done` contains unexpected value `{unknown!r}` (expected bool)"
                )
        match row[1]:
            case int() as vlobs_total_bytes:
                pass
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Output export database appears to be corrupted: `info.vlobs_total_bytes` contains unexpected value `{unknown!r}` (expected int)"
                )

        row = con.execute("SELECT MAX(sequential_id), SUM(size) FROM vlob_atom").fetchone()
        match row[0]:
            case None:
                # Sequential ID starts at 1 so using 0 here is safe
                last_vlob_sequential_id = 0
            case int() as last_vlob_sequential_id:
                pass
            case unknown:
                assert False, f"Unexpected value for `last_vlob_sequential_id`: {unknown!r}"
        match row[1]:
            case None:
                exported_bytes = 0
            case int() as exported_bytes:
                pass
            case unknown:
                assert False, f"Unexpected value for `exported_bytes`: {unknown!r}"

        return done, last_vlob_sequential_id, exported_bytes, vlobs_total_bytes

    (
        vlobs_export_done,
        current_batch_offset_marker,
        exported_bytes,
        vlobs_total_bytes,
    ) = await output_db_con.execute(_get_vlobs_export_done)
    if vlobs_export_done:
        return

    # Export needed

    on_progress(("vlobs", exported_bytes, vlobs_total_bytes))

    while current_batch_offset_marker < vlob_offset_marker_upper_bound:
        # 1) Download a batch of data

        outcome = await backend.realm.export_do_vlobs_batch(
            organization_id=organization_id,
            realm_id=realm_id,
            batch_offset_marker=current_batch_offset_marker,
            batch_size=VLOB_EXPORT_BATCH_SIZE,
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
                raise RealmExporterInputError(f"Unexpect outcome when exporting vlobs: {error}")

        if not batch.items:
            raise RealmExporterInputError(
                "Unexpect outcome when exporting vlobs: all vlob has been exported without finding the upper bound marker"
            )
        elif batch.items[-1].sequential_id > vlob_offset_marker_upper_bound:
            # This batch is the last one as it contains the upper bound,
            # hence we have to filter out the items that are above it.
            batch.items = [
                item for item in batch.items if item.sequential_id <= vlob_offset_marker_upper_bound
            ]

        # 2) Write the batch to export database

        def _write_sqlite_db(con: sqlite3.Connection):
            con.executemany(
                "INSERT INTO vlob_atom (\
                    sequential_id,\
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
                        item.sequential_id,
                        item.vlob_id.bytes,
                        item.version,
                        item.key_index,
                        item.blob,
                        item.size,
                        item.author.bytes,
                        item.timestamp.as_timestamp_micros(),
                    )
                    for item in batch.items
                ),
            )
            con.commit()

        await output_db_con.execute(_write_sqlite_db)

        current_batch_offset_marker = batch.batch_offset_marker

        exported_bytes += sum(item.size for item in batch.items)
        on_progress(("vlobs", exported_bytes, vlobs_total_bytes))

    # 3) Mark this export step as done

    def _write_sqlite_db(con: sqlite3.Connection):
        con.execute(
            "UPDATE info SET vlobs_export_done = TRUE",
        )
        con.commit()

    await output_db_con.execute(_write_sqlite_db)


async def _do_export_blocks_metadata(
    on_progress: ProgressReportCallback,
    backend: Backend,
    output_db_con: OutputDBConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    block_offset_marker_upper_bound: SequentialID,
) -> None:
    # 0) Skip the operation if the export database already contains it

    def _get_blocks_metadata_export_done(
        con: sqlite3.Connection,
    ) -> tuple[bool, SequentialID, ExportedBytes, ToExportBytes]:
        row = con.execute(
            "SELECT blocks_metadata_export_done, blocks_total_bytes FROM info"
        ).fetchone()
        match row[0]:
            case 1:
                done = True
            case 0:
                done = False
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Output export database appears to be corrupted: `blocks_metadata_export_done` contains unexpected value `{unknown!r}`"
                )
        match row[1]:
            case int() as blocks_total_bytes:
                pass
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Output export database appears to be corrupted: `info.blocks_total_bytes` contains unexpected value `{unknown!r}` (expected int)"
                )

        row = con.execute("SELECT MAX(sequential_id), SUM(size) FROM block").fetchone()
        match row[0]:
            case None:
                # Sequential ID starts at 1 so using 0 here is safe
                last_block_metadata_sequential_id = 0
            case int() as last_block_metadata_sequential_id:
                pass
            case unknown:
                assert False, (
                    f"Unexpected value for `last_block_metadata_sequential_id`: {unknown!r}"
                )
        match row[1]:
            case None:
                exported_bytes = 0
            case int() as exported_bytes:
                pass
            case unknown:
                assert False, f"Unexpected value for `exported_bytes`: {unknown!r}"

        return done, last_block_metadata_sequential_id, exported_bytes, blocks_total_bytes

    (
        blocks_metadata_export_done,
        current_batch_offset_marker,
        exported_bytes,
        blocks_total_bytes,
    ) = await output_db_con.execute(_get_blocks_metadata_export_done)
    if blocks_metadata_export_done:
        return

    # Export needed

    on_progress(("blocks_metadata", exported_bytes, blocks_total_bytes))

    while current_batch_offset_marker < block_offset_marker_upper_bound:
        # 1) Download a batch of data

        outcome = await backend.realm.export_do_blocks_metadata_batch(
            organization_id=organization_id,
            realm_id=realm_id,
            batch_offset_marker=current_batch_offset_marker,
            batch_size=BLOCK_METADATA_EXPORT_BATCH_SIZE,
        )
        match outcome:
            case RealmExportBlocksMetadataBatch() as batch:
                pass
            case (
                (
                    RealmExportDoBlocksBatchMetadataBadOutcome.ORGANIZATION_NOT_FOUND
                    | RealmExportDoBlocksBatchMetadataBadOutcome.REALM_NOT_FOUND
                ) as error
            ):
                # Organization&realm existence has already been checked, so this shouldn't occur
                raise RealmExporterInputError(
                    f"Unexpect outcome when exporting certificates: {error}"
                )

        if not batch.items:
            raise RealmExporterInputError(
                "Unexpect outcome when exporting blocks metadata: all blocks has been exported without finding the upper bound marker"
            )
        elif batch.items[-1].sequential_id > block_offset_marker_upper_bound:
            # This batch is the last one as it contains the upper bound,
            # hence we have to filter out the items that are above it.
            batch.items = [
                item
                for item in batch.items
                if item.sequential_id <= block_offset_marker_upper_bound
            ]

        # 2) Write the batch to export database

        def _write_sqlite_db(con: sqlite3.Connection):
            con.executemany(
                "INSERT INTO block (\
                    sequential_id,\
                    block_id,\
                    author,\
                    size,\
                    key_index\
                ) VALUES (?, ?, ?, ?, ?)",
                (
                    (
                        item.sequential_id,
                        item.block_id.bytes,
                        item.author.bytes,
                        item.size,
                        item.key_index,
                    )
                    for item in batch.items
                ),
            )
            con.commit()

        await output_db_con.execute(_write_sqlite_db)

        current_batch_offset_marker = batch.batch_offset_marker

        exported_bytes += sum(item.size for item in batch.items)
        on_progress(("blocks_metadata", exported_bytes, blocks_total_bytes))

    def _write_sqlite_db(con: sqlite3.Connection):
        con.execute(
            "UPDATE info SET blocks_metadata_export_done = TRUE",
        )
        con.commit()

    await output_db_con.execute(_write_sqlite_db)


async def _do_export_blocks_data(
    on_progress: ProgressReportCallback,
    backend: Backend,
    output_db_con: OutputDBConnection,
    organization_id: OrganizationID,
) -> None:
    # 0) Skip the operation if the export database already contains it

    def _get_blocks_data_export_done(
        con: sqlite3.Connection,
    ) -> tuple[bool, ExportedBytes, ToExportBytes]:
        row = con.execute(
            "SELECT blocks_data_export_done, blocks_total_bytes FROM info",
        ).fetchone()
        match row[0]:
            case 1:
                done = True
            case 0:
                done = False
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Output export database appears to be corrupted: `blocks_data_export_done` contains unexpected value `{unknown!r}`"
                )
        match row[1]:
            case int() as blocks_total_bytes:
                pass
            case unknown:
                raise RealmExporterOutputDbError(
                    f"Output export database appears to be corrupted: `info.blocks_total_bytes` contains unexpected value `{unknown!r}` (expected int)"
                )

        row = con.execute(
            "SELECT SUM(block.size) FROM block_data LEFT JOIN block ON block_data.block = block.sequential_id"
        ).fetchone()
        match row[0]:
            case None:
                exported_bytes = 0
            case int() as exported_bytes:
                pass
            case unknown:
                assert False, f"Unexpected value for `exported_bytes`: {unknown!r}"

        return done, exported_bytes, blocks_total_bytes

    blocks_data_export_done, exported_bytes, blocks_total_bytes = await output_db_con.execute(
        _get_blocks_data_export_done
    )
    if blocks_data_export_done:
        return

    # Export needed

    on_progress(("blocks_data", exported_bytes, blocks_total_bytes))

    # /!\ One important point to note is that, unlike vlob and block metadata export
    # steps, we don't export items in a strictly growing pattern according to their
    # sequential ID.
    #
    # This is for two reasons:
    # - We can determine what remains to export simply by looking into the output
    #   database for block metadata without corresponding data.
    #   This ensure the output database is always consistent once export is finished.
    # - We fetch the block data in parallel, and flush it to the output database
    #   whenever a memory threshold is reached.
    #   Also, connection to the blockstore can be unreliable and we try to carry on
    #   nevertheless.
    #
    # In particular, the last point means two things:
    # - The data we flush on the output database has no ordering guarantee.
    # - Any given batch can end up partially exported before we move to the next one.

    def _get_next_batch_of_blocks(con: sqlite3.Connection) -> deque[tuple[SequentialID, BlockID]]:
        rows = con.execute(
            "SELECT block.sequential_id, block.block_id\
            FROM block LEFT JOIN block_data\
            ON block_data.block = block.sequential_id\
            WHERE block_data.data IS NULL\
            LIMIT ?",
            # Use the metadata batch size here since we *are* retrieving metadata
            # from the export database.
            (BLOCK_METADATA_EXPORT_BATCH_SIZE,),
        ).fetchall()

        batch: deque[tuple[SequentialID, BlockID]] = deque()
        for row in rows:
            match row[0]:
                case int() as sequential_id:
                    pass
                case unknown:
                    raise RealmExporterOutputDbError(
                        f"Output export database appears to be corrupted: `block` table contains unexpected `sequential_id` value `{unknown!r}` (expected int)"
                    )

            match row[1]:
                case bytes() as raw:
                    try:
                        block_id = BlockID.from_bytes(raw)
                    except ValueError as exc:
                        raise RealmExporterOutputDbError(
                            f"Output export database appears to be corrupted: `block` table contains unexpected `block_id` value `{raw!r}` (not a valid BlockID)"
                        ) from exc

                case unknown:
                    raise RealmExporterOutputDbError(
                        f"Output export database appears to be corrupted: `block` table contains unexpected `block_id` value `{unknown!r}` (expected bytes)"
                    )

            batch.append((sequential_id, block_id))

        return batch

    # We need a lock here to ensure the SQLite flush and updates of `to_flush_data` &
    # `to_flush_data_total_size` are done atomically.
    # This is important since `to_flush_data` & `to_flush_data_total_size` are accessed
    # from multiple threads (the main asyncio thread + the `asyncio.to_thread` used
    # to do SQLite operations).
    to_flush_lock = threading.Lock()
    to_flush_data: list[tuple[SequentialID, bytes]] = []
    to_flush_data_total_size = 0

    def _flush_data_to_sqlite(con: sqlite3.Connection) -> None:
        nonlocal to_flush_data_total_size, to_flush_data

        # It could be tempting here to take the lock for the whole duration of the
        # SQLite operation here, but this would be a bad idea since it would block
        # the asyncio event loop whenever `_add_block_data_and_maybe_flush_to_sqlite`
        # is called.
        # So instead we only take the lock to steal the list of data to flush and
        # reset it.
        with to_flush_lock:
            to_insert = to_flush_data
            to_flush_data = []
            to_flush_data_total_size = 0

        con.executemany(
            "INSERT INTO block_data (block, data) VALUES (?, ?)",
            to_insert,
        )
        con.commit()

    async def _add_block_data_and_maybe_flush_to_sqlite(
        block_sequential_id: SequentialID, data: bytes
    ) -> None:
        nonlocal to_flush_data_total_size
        nonlocal exported_bytes

        with to_flush_lock:
            to_flush_data.append((block_sequential_id, data))
            to_flush_data_total_size += len(data)

        if to_flush_data_total_size >= BLOCK_DATA_EXPORT_RAM_LIMIT:
            await output_db_con.execute(_flush_data_to_sqlite)

        # Note we report the progress before the flush is actually done on the output
        # database.
        # This is because the slow part (justifying the need for this progress report)
        # is fetching data from the blockstore.
        exported_bytes += len(data)
        on_progress(("blocks_data", exported_bytes, blocks_total_bytes))

    while True:
        batch = await output_db_con.execute(_get_next_batch_of_blocks)
        if not batch:
            break

        # Now process our batch in parallel

        consecutive_store_unavailable_errors = 0

        class StoreUnavailable(Exception):
            pass

        async def _fetch_data_in_batch() -> None:
            while True:
                try:
                    (block_sequential_id, block_id) = batch.popleft()
                except IndexError:
                    # Nothing more to fetch !
                    break

                outcome = await backend.blockstore.read(organization_id, block_id)
                match outcome:
                    case bytes() as data:
                        nonlocal consecutive_store_unavailable_errors
                        consecutive_store_unavailable_errors = 0
                        await _add_block_data_and_maybe_flush_to_sqlite(block_sequential_id, data)

                    case BlockStoreReadBadOutcome.BLOCK_NOT_FOUND:
                        # TODO: We currently never remove any block data from a realm (there
                        #       is a `deleted_on` field in the `block` table but it is unused
                        #       for now).
                        #       This code should be updated if we ever decide to do so.
                        raise RealmExporterInputDbError(
                            f"Block `{block_id}` is missing from the blockstore database"
                        )

                    case BlockStoreReadBadOutcome.STORE_UNAVAILABLE:
                        # By raising this error, we stop all parallel tasks (leaving the
                        # current batch un-achieved but this is fine by design) to wait a
                        # bit before retrying.
                        # It would be a shame to abandon all those precious downloaded bytes,
                        # so flush them before leaving !
                        await output_db_con.execute(_flush_data_to_sqlite)
                        raise StoreUnavailable

        while True:
            try:
                async with asyncio.TaskGroup() as tg:
                    for _ in range(BLOCK_DATA_EXPORT_PARALLELISM):
                        tg.create_task(_fetch_data_in_batch())

                # The current batch is done, we must ensure all the corresponding data
                # are flushed to the output database before fetching a new one (otherwise
                # the next batch will contain the blocks that have been fetched but not
                # flushed yet !).
                await output_db_con.execute(_flush_data_to_sqlite)
                break

            except StoreUnavailable:
                consecutive_store_unavailable_errors += 1
                if consecutive_store_unavailable_errors > MAX_CONSECUTIVE_STORE_UNAVAILABLE_ERRORS:
                    raise RealmExporterInputDbError(
                        "Blockstore database is unavailable after too many retries"
                    )
                backoff = min(
                    2**consecutive_store_unavailable_errors, MAX_STORE_UNAVAILABLE_BACKOFF_SLEEP
                )
                logger.warning(f"Blockstore database is unavailable, retrying in {backoff}s")

    def _write_sqlite_db(con: sqlite3.Connection):
        # Sanity check to ensure each `block` and `block_data` tables are
        # consistent with each other.
        row = con.execute(
            "SELECT 1 FROM block LEFT JOIN block_data ON block.sequential_id = block_data.block WHERE block_data.data IS NULL LIMIT 1"
        ).fetchone()
        assert row is None, "Some blocks have been exported but their data is missing"

        con.execute(
            "UPDATE info SET blocks_data_export_done = TRUE",
        )
        con.commit()

    await output_db_con.execute(_write_sqlite_db)
