# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

import pendulum

from parsec.api.data import EntryID, BlockID, EntryName
from parsec.api.data.manifest import (
    FileManifest,
    FolderManifest,
    WorkspaceEntry,
    WorkspaceManifest,
    UserManifest,
)
from parsec.api.protocol import DeviceID
from parsec.crypto import SecretKey, HashDigest
from parsec.core.types import ChunkID


@pytest.mark.rust
def test_local_file_manifest():
    from parsec.api.data.manifest import BlockAccess
    from parsec.core.types.manifest import (
        _RsLocalFileManifest,
        LocalFileManifest,
        _PyLocalFileManifest,
        Chunk,
    )

    assert LocalFileManifest is _RsLocalFileManifest

    def _assert_local_file_manifest_eq(py, rs):
        assert py.base == rs.base
        assert py.need_sync == rs.need_sync
        assert py.updated == rs.updated
        assert py.size == rs.size
        assert py.blocksize == rs.blocksize
        assert len(py.blocks) == len(rs.blocks)
        for (b1, b2) in zip(py.blocks, rs.blocks):
            assert len(b1) == len(b2)
            assert all(
                isinstance(c2, Chunk)
                and c1.id == c2.id
                and c1.start == c2.start
                and c1.stop == c2.stop
                and c1.raw_offset == c2.raw_offset
                and c1.raw_size == c2.raw_size
                and c1.access == c2.access
                for (c1, c2) in zip(b1, b2)
            )

    kwargs = {
        "base": FileManifest(
            **{
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
        ),
        "need_sync": True,
        "updated": pendulum.now(),
        "size": 1337,
        "blocksize": 64,
        "blocks": (
            (
                Chunk(
                    id=ChunkID.new(),
                    start=0,
                    stop=250,
                    raw_offset=0,
                    raw_size=512,
                    access=BlockAccess(
                        id=BlockID.new(),
                        key=SecretKey.generate(),
                        offset=0,
                        size=512,
                        digest=HashDigest.from_data(b"a"),
                    ),
                ),
                Chunk(
                    id=ChunkID.new(), start=0, stop=250, raw_offset=250, raw_size=250, access=None
                ),
            ),
        ),
    }

    py_lfm = _PyLocalFileManifest(**kwargs)
    rs_lfm = LocalFileManifest(**kwargs)
    _assert_local_file_manifest_eq(py_lfm, rs_lfm)

    kwargs = {
        "base": kwargs["base"].evolve(
            **{
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
        ),
        "need_sync": False,
        "updated": pendulum.now(),
        "size": 2048,
        "blocksize": 32,
        "blocks": (
            (
                Chunk(
                    id=ChunkID.new(),
                    start=250,
                    stop=500,
                    raw_offset=32,
                    raw_size=1024,
                    access=BlockAccess(
                        id=BlockID.new(),
                        key=SecretKey.generate(),
                        offset=32,
                        size=1024,
                        digest=HashDigest.from_data(b"b"),
                    ),
                ),
                Chunk(
                    id=ChunkID.new(), start=250, stop=500, raw_offset=500, raw_size=750, access=None
                ),
            ),
        ),
    }

    py_lfm = py_lfm.evolve(**kwargs)
    rs_lfm = rs_lfm.evolve(**kwargs)
    _assert_local_file_manifest_eq(py_lfm, rs_lfm)


@pytest.mark.rust
def test_local_folder_manifest():
    from parsec.core.types.manifest import (
        _RsLocalFolderManifest,
        LocalFolderManifest,
        _PyLocalFolderManifest,
    )

    assert LocalFolderManifest is _RsLocalFolderManifest

    def _assert_local_folder_manifest_eq(py, rs):
        assert py.base == rs.base
        assert py.need_sync == rs.need_sync
        assert py.updated == rs.updated
        assert len(py.children) == len(rs.children)
        assert all(
            isinstance(name1, EntryName)
            and isinstance(id1, EntryID)
            and name1 == name2
            and id1 == id2
            for ((name1, id1), (name2, id2)) in zip(py.children.items(), rs.children.items())
        )
        assert len(py.local_confinement_points) == len(rs.local_confinement_points)
        assert all(
            isinstance(lcp1, EntryID) and lcp1 == lcp2
            for (lcp1, lcp2) in zip(py.local_confinement_points, rs.local_confinement_points)
        )
        assert len(py.remote_confinement_points) == len(rs.remote_confinement_points)
        assert all(
            isinstance(rcp1, EntryID) and rcp1 == rcp2
            for (rcp1, rcp2) in zip(py.remote_confinement_points, rs.remote_confinement_points)
        )

    kwargs = {
        "base": FolderManifest(
            **{
                "author": DeviceID("user@device"),
                "id": EntryID.new(),
                "parent": EntryID.new(),
                "version": 42,
                "timestamp": pendulum.now(),
                "created": pendulum.now(),
                "updated": pendulum.now(),
                "children": {EntryName("file1.txt"): EntryID.new()},
            }
        ),
        "need_sync": True,
        "updated": pendulum.now(),
        "children": {EntryName("wksp2"): EntryID.new()},
        "local_confinement_points": frozenset({EntryID.new()}),
        "remote_confinement_points": frozenset({EntryID.new()}),
    }

    py_lfm = _PyLocalFolderManifest(**kwargs)
    rs_lfm = LocalFolderManifest(**kwargs)
    _assert_local_folder_manifest_eq(py_lfm, rs_lfm)

    kwargs = {
        "base": kwargs["base"].evolve(
            **{
                "author": DeviceID("a@b"),
                "id": EntryID.new(),
                "parent": EntryID.new(),
                "version": 1337,
                "timestamp": pendulum.now(),
                "created": pendulum.now(),
                "updated": pendulum.now(),
                "children": {EntryName("file2.mp4"): EntryID.new()},
            }
        ),
        "need_sync": False,
        "updated": pendulum.now(),
        "children": {EntryName("wksp1"): EntryID.new()},
        "local_confinement_points": {EntryID.new()},
        "remote_confinement_points": {EntryID.new()},
    }

    py_lfm = py_lfm.evolve(**kwargs)
    rs_lfm = rs_lfm.evolve(**kwargs)
    _assert_local_folder_manifest_eq(py_lfm, rs_lfm)


@pytest.mark.rust
def test_local_workspace_manifest():
    from parsec.core.types.manifest import (
        _RsLocalWorkspaceManifest,
        LocalWorkspaceManifest,
        _PyLocalWorkspaceManifest,
    )

    assert LocalWorkspaceManifest is _RsLocalWorkspaceManifest

    def _assert_local_workspace_manifest_eq(py, rs):
        assert py.base == rs.base
        assert py.need_sync == rs.need_sync
        assert py.updated == rs.updated
        assert py.speculative == rs.speculative
        assert len(py.children) == len(rs.children)
        assert all(
            isinstance(name1, EntryName)
            and isinstance(id1, EntryID)
            and name1 == name2
            and id1 == id2
            for ((name1, id1), (name2, id2)) in zip(py.children.items(), rs.children.items())
        )
        assert len(py.local_confinement_points) == len(rs.local_confinement_points)
        assert all(
            isinstance(lcp1, EntryID) and lcp1 == lcp2
            for (lcp1, lcp2) in zip(py.local_confinement_points, rs.local_confinement_points)
        )
        assert len(py.remote_confinement_points) == len(rs.remote_confinement_points)
        assert all(
            isinstance(rcp1, EntryID) and rcp1 == rcp2
            for (rcp1, rcp2) in zip(py.remote_confinement_points, rs.remote_confinement_points)
        )

    kwargs = {
        "base": WorkspaceManifest(
            **{
                "author": DeviceID("user@device"),
                "id": EntryID.new(),
                "parent": EntryID.new(),
                "version": 42,
                "timestamp": pendulum.now(),
                "created": pendulum.now(),
                "updated": pendulum.now(),
                "children": {EntryName("file1.txt"): EntryID.new()},
            }
        ),
        "need_sync": True,
        "updated": pendulum.now(),
        "children": {EntryName("wksp2"): EntryID.new()},
        "local_confinement_points": frozenset({EntryID.new()}),
        "remote_confinement_points": frozenset({EntryID.new()}),
        "speculative": False,
    }

    py_lwm = _PyLocalWorkspaceManifest(**kwargs)
    rs_lwm = LocalWorkspaceManifest(**kwargs)
    _assert_local_workspace_manifest_eq(py_lwm, rs_lwm)

    kwargs = {
        "base": kwargs["base"].evolve(
            **{
                "author": DeviceID("a@b"),
                "id": EntryID.new(),
                "parent": EntryID.new(),
                "version": 1337,
                "timestamp": pendulum.now(),
                "created": pendulum.now(),
                "updated": pendulum.now(),
                "children": {EntryName("file2.mp4"): EntryID.new()},
            }
        ),
        "need_sync": False,
        "updated": pendulum.now(),
        "children": {EntryName("wksp1"): EntryID.new()},
        "local_confinement_points": {EntryID.new()},
        "remote_confinement_points": {EntryID.new()},
        "speculative": True,
    }

    py_lwm = py_lwm.evolve(**kwargs)
    rs_lwm = rs_lwm.evolve(**kwargs)
    _assert_local_workspace_manifest_eq(py_lwm, rs_lwm)


@pytest.mark.rust
def test_local_user_manifest():
    from parsec.core.types.manifest import (
        _RsLocalUserManifest,
        LocalUserManifest,
        _PyLocalUserManifest,
    )

    assert LocalUserManifest is _RsLocalUserManifest

    def _assert_local_user_manifest_eq(py, rs):
        assert py.base == rs.base
        assert py.need_sync == rs.need_sync
        assert py.updated == rs.updated
        assert py.speculative == rs.speculative

    kwargs = {
        "base": UserManifest(
            **{
                "author": DeviceID("user@device"),
                "id": EntryID.new(),
                "parent": EntryID.new(),
                "version": 42,
                "timestamp": pendulum.now(),
                "created": pendulum.now(),
                "updated": pendulum.now(),
                "last_processed_message": 0,
                "workspaces": (WorkspaceEntry.new(EntryName("user"), pendulum.now()),),
            }
        ),
        "need_sync": True,
        "updated": pendulum.now(),
        "last_processed_message": 0,
        "workspaces": (WorkspaceEntry.new(EntryName("user"), pendulum.now()),),
        "speculative": False,
    }

    py_lwm = _PyLocalUserManifest(**kwargs)
    rs_lwm = LocalUserManifest(**kwargs)
    _assert_local_user_manifest_eq(py_lwm, rs_lwm)

    kwargs = {
        "base": kwargs["base"].evolve(
            **{
                "author": DeviceID("a@b"),
                "id": EntryID.new(),
                "parent": EntryID.new(),
                "version": 1337,
                "timestamp": pendulum.now(),
                "created": pendulum.now(),
                "updated": pendulum.now(),
                "last_processed_message": 1,
                "workspaces": (),
            }
        ),
        "need_sync": False,
        "updated": pendulum.now(),
        "last_processed_message": 1,
        "workspaces": (),
        "speculative": True,
    }

    py_lwm = py_lwm.evolve(**kwargs)
    rs_lwm = rs_lwm.evolve(**kwargs)
    _assert_local_user_manifest_eq(py_lwm, rs_lwm)
