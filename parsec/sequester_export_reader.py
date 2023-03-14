# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import enum
import sqlite3
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path, PurePath
from typing import Dict, Iterator, List, Mapping, Tuple

from parsec._parsec import (
    CryptoError,
    DataError,
    DateTime,
    DeviceCertificate,
    DeviceID,
    EntryID,
    EntryName,
    FileManifest,
    FolderManifest,
    RealmID,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SequesterPrivateKeyDer,
    UserCertificate,
    VerifyKey,
    WorkspaceManifest,
    manifest_verify_and_load,
)
from parsec.api.data.manifest import AnyRemoteManifest

REALM_EXPORT_DB_MAGIC_NUMBER = 87947
REALM_EXPORT_DB_VERSION = 1  # Only supported version so far


class RealmExportProgress(enum.Enum):
    GENERIC_ERROR = "Error"
    INCONSISTENT_CERTIFICATE = "Inconsistent certificate"
    INCONSISTENT_MANIFEST = "Inconsistent manifest"
    INCONSISTENT_BLOCK = "Inconsistent block"
    EXTRACT_IN_PROGRESS = "Extract in progress"


class RealmExportReaderError(Exception):
    pass


class InvalidRealmExportDatabaseError(RealmExportReaderError):
    pass


class InconsistentWorkspaceError(RealmExportReaderError):
    pass


@dataclass
class RealmExportDb:
    con: sqlite3.Connection
    realm_id: RealmID
    root_verify_key: VerifyKey

    @classmethod
    @contextmanager
    def open(cls, path: Path) -> Iterator[RealmExportDb]:
        try:
            con = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
        except sqlite3.Error as exc:
            # Database doesn't exists
            raise InvalidRealmExportDatabaseError(f"Invalid realm export database: {exc}") from exc
        # Run sanity checks to make sure the open file is a valid SQLite DB containing a realm export
        try:
            row = con.execute(
                "SELECT version, realm_id, root_verify_key FROM info WHERE magic = ?",
                (REALM_EXPORT_DB_MAGIC_NUMBER,),
            ).fetchone()
        except sqlite3.Error as exc:
            # Multiple reasons we might end up here:
            # - The file exists but is not a valid SQLite DB
            # - The SQLite DB is missing the expected table
            # - The SQLite DB have the expected table but it misses some columns
            raise InvalidRealmExportDatabaseError(f"Invalid realm export database: {exc}") from exc
        if not row:
            # `info` table exists and is valid, but magic number doesn't match
            raise InvalidRealmExportDatabaseError(
                f"Invalid realm export database: invalid magic number"
            )
        db_version, db_realm_id, db_root_verify_key = row
        if db_version != REALM_EXPORT_DB_VERSION:
            raise InvalidRealmExportDatabaseError(
                f"Unsupported realm export database format: got version `{db_version}` but only version `{REALM_EXPORT_DB_VERSION}` is accepted"
            )
        try:
            realm_id = RealmID.from_bytes(db_realm_id)
        except ValueError as exc:
            raise InvalidRealmExportDatabaseError(
                f"Invalid realm export database: cannot parse realm ID"
            ) from exc
        try:
            root_verify_key = VerifyKey(db_root_verify_key)
        except CryptoError as exc:
            raise InvalidRealmExportDatabaseError(
                f"Invalid realm export database: cannot parse root verify key"
            ) from exc

        try:
            yield cls(con=con, realm_id=realm_id, root_verify_key=root_verify_key)

        finally:
            con.close()

    def load_user_certificates(
        self,
        out_certificates: List[Tuple[UserCertificate, RevokedUserCertificate | None]],
    ) -> Iterator[Tuple[PurePath | None, RealmExportProgress, str]]:
        rows = self.con.execute(
            "SELECT _id, user_certificate, revoked_user_certificate FROM user_"
        ).fetchall()
        for row in rows:
            try:
                # TODO: check devices are valid
                user_certif = UserCertificate.unsecure_load(row[1])
            except DataError as exc:
                yield (
                    None,
                    RealmExportProgress.INCONSISTENT_CERTIFICATE,
                    f"Ignoring invalid user certificate { row[0] } ({ exc })",
                )

            try:
                if row[2]:
                    # TODO: check devices are valid
                    revoked_user_certif = RevokedUserCertificate.unsecure_load(row[2])
                else:
                    revoked_user_certif = None
                out_certificates.append((user_certif, revoked_user_certif))
            except DataError as exc:
                yield (
                    None,
                    RealmExportProgress.INCONSISTENT_CERTIFICATE,
                    f"Ignoring invalid user certificate { row[0] } ({ exc })",
                )

    def load_device_certificates(
        self, out_certificates: List[Tuple[int, DeviceCertificate]]
    ) -> Iterator[Tuple[PurePath | None, RealmExportProgress, str]]:
        rows = self.con.execute("SELECT _id, device_certificate FROM device").fetchall()
        for row in rows:
            try:
                # TODO: check devices are valid
                certif = DeviceCertificate.unsecure_load(row[1])
                out_certificates.append((row[0], certif))
            except DataError as exc:
                yield (
                    None,
                    RealmExportProgress.INCONSISTENT_CERTIFICATE,
                    f"Ignoring invalid device certificate { row[0] } ({ exc })",
                )

    def load_role_certificates(
        self, out_certificates: List[RealmRoleCertificate]
    ) -> Iterator[Tuple[PurePath | None, RealmExportProgress, str]]:
        rows = self.con.execute("SELECT _id, role_certificate FROM realm_role").fetchall()
        for row in rows:
            try:
                # TODO: check devices are valid
                certif = RealmRoleCertificate.unsecure_load(row[1])
                out_certificates.append(certif)
            except DataError as exc:
                yield (
                    None,
                    RealmExportProgress.INCONSISTENT_CERTIFICATE,
                    f"Ignoring invalid realm role certificate { row[0] } ({ exc })",
                )


@dataclass
class WorkspaceExport:
    db: RealmExportDb
    decryption_key: SequesterPrivateKeyDer
    devices_form_internal_id: Dict[int, Tuple[DeviceID, VerifyKey]]
    filter_on_date: DateTime

    def load_manifest(self, manifest_id: EntryID) -> AnyRemoteManifest:
        # Convert datetime to integer timestamp with us precision (format used in sqlite dump).
        filter_timestamp = int(self.filter_on_date.timestamp() * 1000000)
        row = self.db.con.execute(
            "SELECT version, blob, author, timestamp FROM vlob_atom WHERE vlob_id = ? and timestamp <= ? ORDER BY version DESC LIMIT 1",
            (manifest_id.bytes, filter_timestamp),
        ).fetchone()
        if not row:
            raise InconsistentWorkspaceError(
                f"Cannot retrieve workspace manifest: vlob {manifest_id.hex} doesn't exist"
            )

        try:
            version: int | None = row[0]
            blob: bytes = row[1]
            author_internal_id: int = row[2]
            raw_timestamp: int = row[3]

            try:
                author, author_verify_key = self.devices_form_internal_id[author_internal_id]
            except KeyError:
                raise InconsistentWorkspaceError(f"Missing device certificate for `{author}`")
            timestamp = DateTime.from_timestamp(raw_timestamp / 1000000)

            decrypted_blob = self.decryption_key.decrypt(blob)

            manifest = manifest_verify_and_load(
                signed=decrypted_blob,
                author_verify_key=author_verify_key,
                expected_author=author,
                expected_timestamp=timestamp,
                expected_id=manifest_id,
                expected_version=version,
            )
            return manifest

        except Exception as exc:
            # TODO: better exceptions handling
            raise InconsistentWorkspaceError(
                f"Invalid manifest data from vlob {manifest_id.hex}: {exc}"
            ) from exc

    def load_workspace_manifest(self) -> WorkspaceManifest:
        manifest = self.load_manifest(self.db.realm_id.to_entry_id())
        if not isinstance(manifest, WorkspaceManifest):
            raise InconsistentWorkspaceError(
                f"Vlob with realm id is expected to contain a Workspace manifest, but actually contained {manifest}"
            )
        return manifest

    def extract_children(
        self, output: Path, fs_path: PurePath, children: Mapping[EntryName, EntryID]
    ) -> Iterator[Tuple[PurePath | None, RealmExportProgress, str]]:
        """
        Raises nothing (errors are passed through `on_progress` callback)
        """
        yield (fs_path, RealmExportProgress.EXTRACT_IN_PROGRESS, "Extracting folder...")

        try:
            output.mkdir(exist_ok=True)
        except OSError as exc:
            yield (
                fs_path,
                RealmExportProgress.GENERIC_ERROR,
                f"Failed to create folder {output}: {exc}",
            )

        for child_name, child_id in children.items():
            child_fs_path = fs_path / child_name.str
            # TODO: this may cause issue on Windows (e.g. `AUX`, `COM1`, `<!>`)
            child_output = output / child_name.str
            try:
                child_manifest = self.load_manifest(child_id)
                if isinstance(child_manifest, FileManifest):
                    yield from self.extract_file(
                        output=child_output, fs_path=child_fs_path, manifest=child_manifest
                    )
                elif isinstance(child_manifest, FolderManifest):
                    yield from self.extract_children(
                        output=child_output, fs_path=child_fs_path, children=child_manifest.children
                    )
                else:
                    yield (
                        child_fs_path,
                        RealmExportProgress.INCONSISTENT_MANIFEST,
                        f"Vlob {child_id.hex} version {child_manifest.version}: Expected file or folder manifest, got instead {child_manifest}",
                    )

            except Exception as exc:
                yield (
                    child_fs_path,
                    RealmExportProgress.INCONSISTENT_MANIFEST,
                    f"Vlob {child_id.hex} version {child_manifest.version}: {exc}",
                )

    def extract_file(
        self, output: Path, fs_path: PurePath, manifest: FileManifest
    ) -> Iterator[Tuple[PurePath | None, RealmExportProgress, str]]:
        """
        Raises nothing (errors are passed through `on_progress` callback)
        """
        yield (fs_path, RealmExportProgress.EXTRACT_IN_PROGRESS, "Extracting file...")

        try:
            fd = open(output, "bw")
        except OSError as exc:
            yield (
                fs_path,
                RealmExportProgress.GENERIC_ERROR,
                f"Failed to create file {output}: {exc}",
            )

        fd.truncate(manifest.size)
        for i, block in enumerate(manifest.blocks):
            yield (
                fs_path,
                RealmExportProgress.EXTRACT_IN_PROGRESS,
                f"Extracting blocks {i+1}/{len(manifest.blocks)}",
            )

            row = self.db.con.execute(
                "SELECT data FROM block WHERE block_id = ?", (block.id.bytes,)
            ).fetchone()
            if not row:
                yield (
                    fs_path,
                    RealmExportProgress.INCONSISTENT_BLOCK,
                    f"Block {block.id.hex} is missing",
                )
                continue

            try:
                clear_data = block.key.decrypt(row[0])

            except CryptoError as exc:
                yield (
                    fs_path,
                    RealmExportProgress.INCONSISTENT_BLOCK,
                    f"Block {block.id.hex}: {exc}",
                )
                continue

            try:
                if fd.tell() != block.offset:
                    fd.seek(block.offset)
                if block.size < len(clear_data):
                    # Shouldn't happen, block.size should be equal to len(clear_data)
                    fd.write(clear_data[: block.size])
                else:
                    fd.write(clear_data)
            except OSError as exc:
                yield (
                    fs_path,
                    RealmExportProgress.GENERIC_ERROR,
                    f"Failed to write block {block.id.hex} at offset {block.offset}: {exc}",
                )
                continue

        try:
            fd.close()
        except OSError as exc:
            yield (
                fs_path,
                RealmExportProgress.GENERIC_ERROR,
                f"Failed to close file {output}: {exc}",
            )

    def extract_workspace(
        self, output: Path
    ) -> Iterator[Tuple[PurePath | None, RealmExportProgress, str]]:
        """
        Raises nothing (errors are passed through `on_progress` callback)
        """
        # In theory we should use `parsec.core.fs.path.FsPath` instead of `pathlib.PurePath`.
        # However we cannot import stuff from `parsec.core` if only backend dependencies are
        # installed. In our case `PurePath` is good enough given we need anything fancy anyway.
        fs_path = PurePath("/")
        try:
            workspace_manifest = self.load_workspace_manifest()
        except InconsistentWorkspaceError as exc:
            yield (
                fs_path,
                RealmExportProgress.INCONSISTENT_MANIFEST,
                f"Inconsistent workspace manifest: {exc}",
            )
            return

        yield (fs_path, RealmExportProgress.EXTRACT_IN_PROGRESS, "Workspace manifest loaded")
        yield from self.extract_children(
            output=output, fs_path=fs_path, children=workspace_manifest.children
        )


def extract_workspace(
    output: Path, export_db: Path, decryption_key: SequesterPrivateKeyDer, filter_on_date: DateTime
) -> Iterator[Tuple[PurePath | None, RealmExportProgress, str]]:
    with RealmExportDb.open(export_db) as db:
        out_certificates: list[Tuple[int, DeviceCertificate]] = []
        yield from db.load_device_certificates(out_certificates=out_certificates)
        devices_form_internal_id = {
            id: (certif.device_id, certif.verify_key) for id, certif in out_certificates
        }
        wksp = WorkspaceExport(
            db=db,
            decryption_key=decryption_key,
            devices_form_internal_id=devices_form_internal_id,
            filter_on_date=filter_on_date,
        )
        yield from wksp.extract_workspace(output=output)
