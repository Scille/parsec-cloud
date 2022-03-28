# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

import pendulum

from parsec.api.data import EntryID, BlockID, EntryName
from parsec.api.protocol import RealmRole, DeviceID
from parsec.crypto import SecretKey, HashDigest, SigningKey


@pytest.mark.rust
def test_entry_name():
    from parsec.api.data.entry import _RsEntryName, EntryName, _PyEntryName

    assert EntryName is _RsEntryName

    py_en = _PyEntryName("entry_name")
    rs_en = EntryName("entry_name")

    assert str(py_en) == str(rs_en)
    assert repr(py_en) == repr(rs_en)
    assert hash(py_en) == hash(rs_en)
    assert EntryName("a") == EntryName("a")
    assert EntryName("a") != EntryName("b")


@pytest.mark.rust
@pytest.mark.parametrize("invalid_en", ["a" * 256, "", ".", "..", "/", "a\x00b"])
def test_invalid_entry_names(invalid_en):
    from parsec.api.data.entry import _RsEntryName, EntryName, _PyEntryName, EntryNameTooLongError

    assert EntryName is _RsEntryName

    if len(invalid_en) > 255:
        with pytest.raises(EntryNameTooLongError) as excinfo:
            _PyEntryName(invalid_en)
        assert str(excinfo.value) == "Invalid data"
        with pytest.raises(EntryNameTooLongError) as excinfo:
            EntryName(invalid_en)
        assert str(excinfo.value) == "Invalid data"
    else:
        with pytest.raises(ValueError) as excinfo:
            _PyEntryName(invalid_en)
        assert str(excinfo.value) == "Invalid data"
        with pytest.raises(ValueError) as excinfo:
            EntryName(invalid_en)
        assert str(excinfo.value) == "Invalid data"


@pytest.mark.rust
def test_workspace_entry():
    from parsec.api.data.manifest import _RsWorkspaceEntry, WorkspaceEntry, _PyWorkspaceEntry
    from parsec.api.data import EntryName

    assert WorkspaceEntry is _RsWorkspaceEntry

    def _assert_workspace_entry_eq(py, rs):
        assert py.is_revoked() == rs.is_revoked()
        assert py.name == rs.name
        assert py.id == rs.id
        assert py.key == rs.key
        assert py.encryption_revision == rs.encryption_revision
        assert py.encrypted_on == rs.encrypted_on
        assert py.role_cached_on == rs.role_cached_on
        assert py.role == rs.role

    kwargs = {
        "name": EntryName("name"),
        "id": EntryID.new(),
        "key": SecretKey.generate(),
        "encryption_revision": 1,
        "encrypted_on": pendulum.now(),
        "role_cached_on": pendulum.now(),
        "role": RealmRole.OWNER,
    }

    py_we = _PyWorkspaceEntry(**kwargs)
    rs_we = WorkspaceEntry(**kwargs)
    _assert_workspace_entry_eq(py_we, rs_we)

    kwargs = {
        "name": EntryName("new_name"),
        "id": EntryID.new(),
        "key": SecretKey.generate(),
        "encryption_revision": 42,
        "encrypted_on": pendulum.now(),
        "role_cached_on": pendulum.now(),
        "role": None,
    }
    py_we = py_we.evolve(**kwargs)
    rs_we = rs_we.evolve(**kwargs)
    _assert_workspace_entry_eq(py_we, rs_we)


@pytest.mark.rust
def test_block_access():
    from parsec.api.data.manifest import _RsBlockAccess, BlockAccess, _PyBlockAccess

    assert BlockAccess is _RsBlockAccess

    def _assert_block_access_eq(py, rs):
        assert py.id == rs.id
        assert py.key == rs.key
        assert py.offset == rs.offset
        assert py.size == rs.size
        assert py.digest == rs.digest

    kwargs = {
        "id": BlockID.new(),
        "key": SecretKey.generate(),
        "offset": 0,
        "size": 1024,
        "digest": HashDigest.from_data(b"a"),
    }

    py_ba = _PyBlockAccess(**kwargs)
    rs_ba = BlockAccess(**kwargs)
    _assert_block_access_eq(py_ba, rs_ba)

    kwargs = {
        "id": BlockID.new(),
        "key": SecretKey.generate(),
        "offset": 64,
        "size": 2048,
        "digest": HashDigest.from_data(b"b"),
    }
    py_ba = py_ba.evolve(**kwargs)
    rs_ba = rs_ba.evolve(**kwargs)
    _assert_block_access_eq(py_ba, rs_ba)


@pytest.mark.rust
def test_file_manifest():
    from parsec.api.data.manifest import _RsFileManifest, FileManifest, _PyFileManifest, BlockAccess

    assert FileManifest is _RsFileManifest

    def _assert_file_manifest_eq(py, rs):
        assert py.author == rs.author
        assert py.id == rs.id
        assert py.parent == rs.parent
        assert py.version == rs.version
        assert py.size == rs.size
        assert py.blocksize == rs.blocksize
        assert py.timestamp == rs.timestamp
        assert py.created == rs.created
        assert py.updated == rs.updated
        assert len(py.blocks) == len(rs.blocks)
        assert all(isinstance(b, BlockAccess) for b in rs.blocks)
        assert all(
            b1.id == b2.id and b1.offset == b2.offset and b1.size == b2.size
            for (b1, b2) in zip(py.blocks, rs.blocks)
        )

    kwargs = {
        "author": DeviceID("user@device"),
        "id": EntryID.new(),
        "parent": EntryID.new(),
        "version": 42,
        "size": 1337,
        "blocksize": 64,
        "timestamp": pendulum.now(),
        "created": pendulum.now(),
        "updated": pendulum.now(),
        "blocks": (
            BlockAccess(
                id=BlockID.new(),
                key=SecretKey.generate(),
                offset=0,
                size=1024,
                digest=HashDigest.from_data(b"a"),
            ),
        ),
    }

    py_fm = _PyFileManifest(**kwargs)
    rs_fm = FileManifest(**kwargs)
    _assert_file_manifest_eq(py_fm, rs_fm)

    kwargs = {
        "author": DeviceID("a@b"),
        "id": EntryID.new(),
        "parent": EntryID.new(),
        "version": 1337,
        "timestamp": pendulum.now(),
        "created": pendulum.now(),
        "updated": pendulum.now(),
        "blocks": (
            BlockAccess(
                id=BlockID.new(),
                key=SecretKey.generate(),
                offset=64,
                size=2048,
                digest=HashDigest.from_data(b"b"),
            ),
        ),
    }

    py_fm = py_fm.evolve(**kwargs)
    rs_fm = rs_fm.evolve(**kwargs)
    _assert_file_manifest_eq(py_fm, rs_fm)


@pytest.mark.rust
def test_folder_manifest():
    from parsec.api.data.manifest import _RsFolderManifest, FolderManifest, _PyFolderManifest

    assert FolderManifest is _RsFolderManifest

    def _assert_folder_manifest_eq(py, rs):
        assert py.author == rs.author
        assert py.parent == rs.parent
        assert py.id == rs.id
        assert py.version == rs.version
        assert py.timestamp == rs.timestamp
        assert py.created == rs.created
        assert py.updated == rs.updated
        assert py.children == rs.children

    kwargs = {
        "author": DeviceID("user@device"),
        "id": EntryID.new(),
        "parent": EntryID.new(),
        "version": 42,
        "timestamp": pendulum.now(),
        "created": pendulum.now(),
        "updated": pendulum.now(),
        "children": {EntryName("file1.txt"): EntryID.new()},
    }
    py_wm = _PyFolderManifest(**kwargs)
    rs_wm = FolderManifest(**kwargs)
    _assert_folder_manifest_eq(py_wm, rs_wm)

    kwargs = {
        "author": DeviceID("a@b"),
        "id": EntryID.new(),
        "parent": EntryID.new(),
        "version": 1337,
        "timestamp": pendulum.now(),
        "created": pendulum.now(),
        "updated": pendulum.now(),
        "children": {EntryName("file2.mp4"): EntryID.new()},
    }

    py_wm = py_wm.evolve(**kwargs)
    rs_wm = rs_wm.evolve(**kwargs)
    _assert_folder_manifest_eq(py_wm, rs_wm)


@pytest.mark.xfail(reason="Waiting for a fix on datetimes serialization")
@pytest.mark.rust
def test_workspace_manifest():
    from parsec.api.data.manifest import (
        _RsWorkspaceManifest,
        WorkspaceManifest,
        _PyWorkspaceManifest,
    )

    assert WorkspaceManifest is _RsWorkspaceManifest

    def _assert_workspace_manifest_eq(py, rs):
        assert py.author == rs.author
        assert py.id == rs.id
        assert py.version == rs.version
        assert py.timestamp == rs.timestamp
        assert py.created == rs.created
        assert py.updated == rs.updated
        assert py.children == rs.children

    kwargs = {
        "author": DeviceID("user@device"),
        "id": EntryID.new(),
        "version": 42,
        "timestamp": pendulum.now(),
        "created": pendulum.now(),
        "updated": pendulum.now(),
        "children": {EntryName("file1.txt"): EntryID.new()},
    }
    py_wm = _PyWorkspaceManifest(**kwargs)
    rs_wm = WorkspaceManifest(**kwargs)
    _assert_workspace_manifest_eq(py_wm, rs_wm)

    kwargs = {
        "author": DeviceID("a@b"),
        "id": EntryID.new(),
        "version": 1337,
        "timestamp": pendulum.now(),
        "created": pendulum.now(),
        "updated": pendulum.now(),
        "children": {EntryName("file2.mp4"): EntryID.new()},
    }

    py_wm = py_wm.evolve(**kwargs)
    rs_wm = rs_wm.evolve(**kwargs)
    _assert_workspace_manifest_eq(py_wm, rs_wm)

    signing_key = SigningKey(b"a" * 32)
    secret_key = SecretKey.generate()

    py_signed_and_encrypted = py_wm.dump_sign_and_encrypt(signing_key, secret_key)
    rs_signed_and_encrypted = rs_wm.dump_sign_and_encrypt(signing_key, secret_key)

    py_decrypted_and_verified = _PyWorkspaceManifest.decrypt_verify_and_load(
        py_signed_and_encrypted, secret_key, signing_key.verify_key, py_wm.author, py_wm.timestamp
    )
    rs_decrypted_and_verified = WorkspaceManifest.decrypt_verify_and_load(
        rs_signed_and_encrypted, secret_key, signing_key.verify_key, rs_wm.author, rs_wm.timestamp
    )
    _assert_workspace_manifest_eq(py_decrypted_and_verified, rs_decrypted_and_verified)

    py_decrypted_and_verified = _PyWorkspaceManifest.decrypt_verify_and_load(
        rs_signed_and_encrypted, secret_key, signing_key.verify_key, rs_wm.author, rs_wm.timestamp
    )
    rs_decrypted_and_verified = WorkspaceManifest.decrypt_verify_and_load(
        py_signed_and_encrypted, secret_key, signing_key.verify_key, py_wm.author, py_wm.timestamp
    )
    _assert_workspace_manifest_eq(py_decrypted_and_verified, rs_decrypted_and_verified)
