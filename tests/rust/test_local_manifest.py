# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from libparsec.types import DateTime
import re
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

    def _assert_local_file_manifest_eq(py, rs, exclude_base=False, exclude_id=False):
        assert isinstance(py, _PyLocalFileManifest)
        assert isinstance(rs, _RsLocalFileManifest)
        if not exclude_base:
            assert py.base == rs.base
        assert py.need_sync == rs.need_sync
        assert py.updated == rs.updated
        assert py.size == rs.size
        assert py.blocksize == rs.blocksize
        assert len(py.blocks) == len(rs.blocks)
        assert isinstance(rs.blocks, type(py.blocks))
        if len(py.blocks):
            assert isinstance(rs.blocks[0], type(py.blocks[0]))
        if not exclude_id:
            assert py.id == rs.id
        assert py.created == rs.created
        assert py.base_version == rs.base_version
        assert py.is_placeholder == rs.is_placeholder
        assert py.is_reshaped() == rs.is_reshaped()
        for (b1, b2) in zip(sorted(py.blocks), sorted(rs.blocks)):
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

    def _assert_file_manifest_eq(py, rs):
        assert py.author == rs.author
        assert py.parent == rs.parent
        assert py.version == rs.version
        assert py.size == rs.size
        assert py.blocksize == rs.blocksize
        assert py.timestamp == rs.timestamp
        assert py.created == rs.created
        assert py.updated == rs.updated
        assert len(py.blocks) == len(rs.blocks)
        assert isinstance(rs.blocks, type(py.blocks))
        assert all(
            isinstance(b2, BlockAccess)
            and b1.id == b2.id
            and b1.offset == b2.offset
            and b1.size == b2.size
            for (b1, b2) in zip(sorted(py.blocks), sorted(rs.blocks))
        )

    kwargs = {
        "base": FileManifest(
            author=DeviceID("user@device"),
            id=EntryID.new(),
            parent=EntryID.new(),
            version=42,
            size=1337,
            blocksize=85,
            timestamp=DateTime.now(),
            created=DateTime.now(),
            updated=DateTime.now(),
            blocks=(
                BlockAccess(
                    id=BlockID.new(),
                    key=SecretKey.generate(),
                    offset=0,
                    size=1024,
                    digest=HashDigest.from_data(b"a"),
                ),
            ),
        ),
        "need_sync": True,
        "updated": DateTime.now(),
        "size": 42,
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
                        digest=HashDigest.from_data(b"aa"),
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
                "size": 4096,
                "blocksize": 512,
                "timestamp": DateTime.now(),
                "created": DateTime.now(),
                "updated": DateTime.now(),
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
        "updated": DateTime.now(),
        "size": 2048,
        "blocksize": 1024,
        "blocks": (
            (
                Chunk(
                    id=ChunkID.new(),
                    start=0,
                    stop=1024,
                    raw_offset=0,
                    raw_size=1024,
                    access=BlockAccess(
                        id=BlockID.new(),
                        key=SecretKey.generate(),
                        offset=0,
                        size=1024,
                        digest=HashDigest.from_data(b"bb"),
                    ),
                ),
                Chunk(
                    id=ChunkID.new(),
                    start=1024,
                    stop=2048,
                    raw_offset=1024,
                    raw_size=1024,
                    access=None,
                ),
            ),
        ),
    }

    py_lfm = py_lfm.evolve(**kwargs)
    rs_lfm = rs_lfm.evolve(**kwargs)
    _assert_local_file_manifest_eq(py_lfm, rs_lfm)

    sk = SecretKey.generate()

    py_enc = py_lfm.dump_and_encrypt(sk)
    rs_enc = rs_lfm.dump_and_encrypt(sk)

    # Decrypt rust encrypted with Python and vice versa
    lfm1 = _PyLocalFileManifest.decrypt_and_load(rs_enc, sk)
    lfm2 = LocalFileManifest.decrypt_and_load(py_enc, sk)
    assert isinstance(lfm1, LocalFileManifest)
    assert isinstance(lfm2, LocalFileManifest)
    assert lfm1 == lfm2

    py_lfm = py_lfm.evolve(**{"size": 1337})
    rs_lfm = rs_lfm.evolve(**{"size": 1337})
    _assert_local_file_manifest_eq(py_lfm, rs_lfm)

    with pytest.raises(AssertionError):
        py_lfm.assert_integrity()
    with pytest.raises(AssertionError):
        rs_lfm.assert_integrity()

    assert py_lfm.to_stats() == rs_lfm.to_stats()
    assert py_lfm.parent == rs_lfm.parent
    assert py_lfm.get_chunks(0) == rs_lfm.get_chunks(0)
    assert py_lfm.get_chunks(1000) == rs_lfm.get_chunks(1000)
    assert py_lfm.asdict() == rs_lfm.asdict()

    di = DeviceID("a@b")
    ts = DateTime.now()

    kwargs = {
        "size": 1024,
        "blocksize": 1024,
        "blocks": (
            (
                Chunk(
                    id=ChunkID.new(),
                    start=0,
                    stop=1024,
                    raw_offset=0,
                    raw_size=1024,
                    access=BlockAccess(
                        id=BlockID.new(),
                        key=SecretKey.generate(),
                        offset=0,
                        size=1024,
                        digest=HashDigest.from_data(b"bb"),
                    ),
                ),
            ),
        ),
    }

    py_lfm = py_lfm.evolve(**kwargs)
    rs_lfm = rs_lfm.evolve(**kwargs)
    _assert_local_file_manifest_eq(py_lfm, rs_lfm)

    py_rfm = py_lfm.to_remote(author=di, timestamp=ts)
    rs_rfm = rs_lfm.to_remote(author=di, timestamp=ts)
    _assert_file_manifest_eq(py_rfm, rs_rfm)

    py_lfm2 = _PyLocalFileManifest.from_remote(py_rfm)
    rs_lfm2 = LocalFileManifest.from_remote(rs_rfm)
    _assert_local_file_manifest_eq(py_lfm2, rs_lfm2, exclude_base=True, exclude_id=True)

    py_lfm2 = _PyLocalFileManifest.from_remote_with_local_context(
        remote=py_rfm, prevent_sync_pattern=r".+", local_manifest=py_lfm2, timestamp=ts
    )
    rs_lfm2 = LocalFileManifest.from_remote_with_local_context(
        remote=rs_rfm, prevent_sync_pattern=r".+", local_manifest=rs_lfm2, timestamp=ts
    )

    assert py_lfm.match_remote(py_rfm) == rs_lfm.match_remote(rs_rfm)

    py_lfm = py_lfm.evolve_and_mark_updated(timestamp=ts, size=4096)
    rs_lfm = rs_lfm.evolve_and_mark_updated(timestamp=ts, size=4096)

    _assert_local_file_manifest_eq(py_lfm, rs_lfm, exclude_base=True, exclude_id=True)

    with pytest.raises(TypeError) as excinfo:
        py_lfm.evolve_and_mark_updated(timestamp=ts, need_sync=True)
    assert str(excinfo.value) == "Unexpected keyword argument `need_sync`"
    with pytest.raises(TypeError) as excinfo:
        rs_lfm.evolve_and_mark_updated(timestamp=ts, need_sync=True)
    assert str(excinfo.value) == "Unexpected keyword argument `need_sync`"

    ei = EntryID.new()

    # Without blocksize
    py_lfm = _PyLocalFileManifest.new_placeholder(author=di, parent=ei, timestamp=ts)
    rs_lfm = LocalFileManifest.new_placeholder(author=di, parent=ei, timestamp=ts)
    _assert_local_file_manifest_eq(py_lfm, rs_lfm, exclude_base=True, exclude_id=True)
    # With blocksize
    py_lfm = _PyLocalFileManifest.new_placeholder(
        author=di, parent=ei, timestamp=ts, blocksize=1024
    )
    rs_lfm = LocalFileManifest.new_placeholder(author=di, parent=ei, timestamp=ts, blocksize=1024)
    _assert_local_file_manifest_eq(py_lfm, rs_lfm, exclude_base=True, exclude_id=True)


@pytest.mark.rust
def test_local_folder_manifest():
    from parsec.core.types.manifest import (
        _RsLocalFolderManifest,
        LocalFolderManifest,
        _PyLocalFolderManifest,
    )

    assert LocalFolderManifest is _RsLocalFolderManifest

    def _assert_local_folder_manifest_eq(py, rs, exclude_base=False, exclude_id=False):
        assert isinstance(py, _PyLocalFolderManifest)
        assert isinstance(rs, _RsLocalFolderManifest)

        if not exclude_base:
            assert py.base == rs.base
        if not exclude_id:
            assert py.id == rs.id
        assert py.need_sync == rs.need_sync
        assert py.updated == rs.updated
        assert len(py.children) == len(rs.children)

        assert all(
            isinstance(name1, EntryName)
            and isinstance(id1, EntryID)
            and name1 == name2
            and id1 == id2
            for ((name1, id1), (name2, id2)) in zip(
                sorted(py.children.items()), sorted(rs.children.items())
            )
        )
        assert len(py.local_confinement_points) == len(rs.local_confinement_points)
        assert rs.local_confinement_points == py.local_confinement_points
        assert len(py.remote_confinement_points) == len(rs.remote_confinement_points)
        assert rs.remote_confinement_points == py.remote_confinement_points

    def _assert_folder_manifest_eq(py, rs):
        assert py.author == rs.author
        assert py.parent == rs.parent
        assert py.version == rs.version
        assert py.timestamp == rs.timestamp
        assert py.created == rs.created
        assert py.updated == rs.updated
        assert py.children == rs.children
        assert isinstance(rs.children, type(py.children))

    kwargs = {
        "base": FolderManifest(
            author=DeviceID("user@device"),
            id=EntryID.new(),
            parent=EntryID.new(),
            version=42,
            timestamp=DateTime.now(),
            created=DateTime.now(),
            updated=DateTime.now(),
            children={EntryName("file1.txt"): EntryID.new()},
        ),
        "need_sync": True,
        "updated": DateTime.now(),
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
                "timestamp": DateTime.now(),
                "created": DateTime.now(),
                "updated": DateTime.now(),
                "children": {EntryName("file2.mp4"): EntryID.new()},
            }
        ),
        "need_sync": False,
        "updated": DateTime.now(),
        "children": {EntryName("wksp1"): EntryID.new()},
        "local_confinement_points": frozenset({EntryID.new()}),
        "remote_confinement_points": frozenset({EntryID.new()}),
    }

    py_lfm = py_lfm.evolve(**kwargs)
    rs_lfm = rs_lfm.evolve(**kwargs)
    _assert_local_folder_manifest_eq(py_lfm, rs_lfm)

    sk = SecretKey.generate()

    py_enc = py_lfm.dump_and_encrypt(sk)
    rs_enc = py_lfm.dump_and_encrypt(sk)

    # Decrypt rust encrypted with Python and vice versa
    # _PyLocalFolderManifest.decrypt_and_load returns a LocalFolderManifest, not a _PyFolderManifest
    lfm1 = _PyLocalFolderManifest.decrypt_and_load(rs_enc, sk)
    lfm2 = LocalFolderManifest.decrypt_and_load(py_enc, sk)
    assert isinstance(lfm1, LocalFolderManifest)
    assert isinstance(lfm2, LocalFolderManifest)
    assert lfm1 == lfm2

    assert py_lfm.to_stats() == rs_lfm.to_stats()
    assert py_lfm.parent == rs_lfm.parent
    assert py_lfm.asdict() == rs_lfm.asdict()

    ts = DateTime.now()
    ei = EntryID.new()
    di = DeviceID("a@b")

    py_lfm = _PyLocalFolderManifest.new_placeholder(author=di, parent=ei, timestamp=ts)
    rs_lfm = LocalFolderManifest.new_placeholder(author=di, parent=ei, timestamp=ts)
    _assert_local_folder_manifest_eq(py_lfm, rs_lfm, exclude_base=True, exclude_id=True)

    py_rfm = py_lfm.to_remote(author=di, timestamp=ts)
    rs_rfm = rs_lfm.to_remote(author=di, timestamp=ts)
    _assert_folder_manifest_eq(py_rfm, rs_rfm)

    py_lfm2 = _PyLocalFolderManifest.from_remote(py_rfm, r".+")
    rs_lfm2 = LocalFolderManifest.from_remote(rs_rfm, r".+")
    _assert_local_folder_manifest_eq(py_lfm2, rs_lfm2, exclude_base=True, exclude_id=True)

    py_lfm2 = _PyLocalFolderManifest.from_remote_with_local_context(
        remote=py_rfm, prevent_sync_pattern=r".+", local_manifest=py_lfm2, timestamp=ts
    )
    rs_lfm2 = LocalFolderManifest.from_remote_with_local_context(
        remote=rs_rfm, prevent_sync_pattern=r".+", local_manifest=rs_lfm2, timestamp=ts
    )

    assert py_lfm.match_remote(py_rfm) == rs_lfm.match_remote(rs_rfm)

    children = {EntryName("wksp1"): EntryID.new()}

    py_lfm = py_lfm.evolve_and_mark_updated(timestamp=ts, children=children)
    rs_lfm = rs_lfm.evolve_and_mark_updated(timestamp=ts, children=children)

    _assert_local_folder_manifest_eq(py_lfm, rs_lfm, exclude_base=True, exclude_id=True)

    with pytest.raises(TypeError) as excinfo:
        py_lfm.evolve_and_mark_updated(timestamp=ts, need_sync=True)
    assert str(excinfo.value) == "Unexpected keyword argument `need_sync`"
    with pytest.raises(TypeError) as excinfo:
        rs_lfm.evolve_and_mark_updated(timestamp=ts, need_sync=True)
    assert str(excinfo.value) == "Unexpected keyword argument `need_sync`"

    ei = EntryID.new()
    py_lfm = py_lfm.evolve_children_and_mark_updated(
        data={EntryName("file1.txt"): ei}, prevent_sync_pattern=re.compile(r".+"), timestamp=ts
    )
    rs_lfm = rs_lfm.evolve_children_and_mark_updated(
        data={EntryName("file1.txt"): ei}, prevent_sync_pattern=re.compile(r".+"), timestamp=ts
    )
    _assert_local_folder_manifest_eq(py_lfm, rs_lfm, exclude_base=True, exclude_id=True)

    py_lfm = py_lfm.apply_prevent_sync_pattern(re.compile(".+"), timestamp=ts)
    rs_lfm = rs_lfm.apply_prevent_sync_pattern(re.compile(".+"), timestamp=ts)
    _assert_local_folder_manifest_eq(py_lfm, rs_lfm, exclude_base=True, exclude_id=True)


@pytest.mark.rust
def test_local_workspace_manifest():
    from parsec.core.types.manifest import (
        _RsLocalWorkspaceManifest,
        LocalWorkspaceManifest,
        _PyLocalWorkspaceManifest,
    )

    assert LocalWorkspaceManifest is _RsLocalWorkspaceManifest

    def _assert_local_workspace_manifest_eq(py, rs, exclude_base=False, exclude_id=False):
        assert isinstance(py, _PyLocalWorkspaceManifest)
        assert isinstance(rs, _RsLocalWorkspaceManifest)

        if not exclude_base:
            assert py.base == rs.base
        if not exclude_id:
            assert py.id == rs.id
        assert py.need_sync == rs.need_sync
        assert py.updated == rs.updated
        assert py.speculative == rs.speculative
        assert len(py.children) == len(rs.children)
        assert isinstance(rs.children, type(py.children)), "Rust type is {}, should be {}".format(
            type(rs.children), type(py.children)
        )
        assert all(
            isinstance(name1, EntryName)
            and isinstance(id1, EntryID)
            and name1 == name2
            and id1 == id2
            for ((name1, id1), (name2, id2)) in zip(
                sorted(py.children.items()), sorted(rs.children.items())
            )
        )
        assert len(py.local_confinement_points) == len(rs.local_confinement_points)
        assert py.local_confinement_points == rs.local_confinement_points
        assert len(py.remote_confinement_points) == len(rs.remote_confinement_points)
        assert py.remote_confinement_points == rs.remote_confinement_points

    def _assert_workspace_manifest_eq(py, rs):
        assert py.author == rs.author
        assert py.version == rs.version
        assert py.timestamp == rs.timestamp
        assert py.created == rs.created
        assert py.updated == rs.updated
        assert py.children == rs.children

    kwargs = {
        "base": WorkspaceManifest(
            author=DeviceID("user@device"),
            id=EntryID.new(),
            version=42,
            timestamp=DateTime.now(),
            created=DateTime.now(),
            updated=DateTime.now(),
            children={EntryName("file1.txt"): EntryID.new()},
        ),
        "need_sync": True,
        "updated": DateTime.now(),
        "children": {EntryName("wksp2"): EntryID.new()},
        "local_confinement_points": frozenset({EntryID.new()}),
        "remote_confinement_points": frozenset({EntryID.new()}),
        "speculative": True,
    }

    py_lwm = _PyLocalWorkspaceManifest(**kwargs)
    rs_lwm = LocalWorkspaceManifest(**kwargs)
    _assert_local_workspace_manifest_eq(py_lwm, rs_lwm)

    kwargs = {
        "base": kwargs["base"].evolve(
            **{
                "author": DeviceID("a@b"),
                "id": EntryID.new(),
                "version": 1337,
                "timestamp": DateTime.now(),
                "created": DateTime.now(),
                "updated": DateTime.now(),
                "children": {EntryName("file2.mp4"): EntryID.new()},
            }
        ),
        "need_sync": False,
        "updated": DateTime.now(),
        "children": {EntryName("wksp1"): EntryID.new()},
        "local_confinement_points": frozenset({EntryID.new()}),
        "remote_confinement_points": frozenset({EntryID.new()}),
        "speculative": False,
    }

    py_lwm = py_lwm.evolve(**kwargs)
    rs_lwm = rs_lwm.evolve(**kwargs)
    _assert_local_workspace_manifest_eq(py_lwm, rs_lwm)

    sk = SecretKey.generate()

    py_enc = py_lwm.dump_and_encrypt(sk)
    rs_enc = py_lwm.dump_and_encrypt(sk)

    # Decrypt rust encrypted with Python and vice versa
    lwm1 = _PyLocalWorkspaceManifest.decrypt_and_load(rs_enc, sk)
    lwm2 = LocalWorkspaceManifest.decrypt_and_load(py_enc, sk)
    assert isinstance(lwm1, LocalWorkspaceManifest)
    assert isinstance(lwm2, LocalWorkspaceManifest)
    assert lwm1 == lwm2

    assert py_lwm.to_stats() == rs_lwm.to_stats()
    assert py_lwm.asdict() == rs_lwm.asdict()

    ts = DateTime.now()
    ei = EntryID.new()
    di = DeviceID("a@b")

    # With optional parameters
    py_lwm = _PyLocalWorkspaceManifest.new_placeholder(
        author=di, id=ei, timestamp=ts, speculative=True
    )
    rs_lwm = LocalWorkspaceManifest.new_placeholder(
        author=di, id=ei, timestamp=ts, speculative=True
    )
    _assert_local_workspace_manifest_eq(py_lwm, rs_lwm, exclude_base=True, exclude_id=True)

    # Without optional parameters
    py_lwm = _PyLocalWorkspaceManifest.new_placeholder(author=di, timestamp=ts)
    rs_lwm = LocalWorkspaceManifest.new_placeholder(author=di, timestamp=ts)
    _assert_local_workspace_manifest_eq(py_lwm, rs_lwm, exclude_base=True, exclude_id=True)

    py_rwm = py_lwm.to_remote(author=di, timestamp=ts)
    rs_rwm = rs_lwm.to_remote(author=di, timestamp=ts)
    _assert_workspace_manifest_eq(py_rwm, rs_rwm)

    children = {EntryName("wksp1"): EntryID.new()}
    py_lwm = py_lwm.evolve_and_mark_updated(timestamp=ts, children=children)
    rs_lwm = rs_lwm.evolve_and_mark_updated(timestamp=ts, children=children)

    _assert_local_workspace_manifest_eq(py_lwm, rs_lwm, exclude_base=True, exclude_id=True)

    with pytest.raises(TypeError) as excinfo:
        py_lwm.evolve_and_mark_updated(timestamp=ts, need_sync=True)
    assert str(excinfo.value) == "Unexpected keyword argument `need_sync`"
    with pytest.raises(TypeError) as excinfo:
        rs_lwm.evolve_and_mark_updated(timestamp=ts, need_sync=True)
    assert str(excinfo.value) == "Unexpected keyword argument `need_sync`"

    py_lwm2 = _PyLocalWorkspaceManifest.from_remote(py_rwm, r".+")
    rs_lwm2 = LocalWorkspaceManifest.from_remote(rs_rwm, r".+")
    _assert_local_workspace_manifest_eq(py_lwm2, rs_lwm2, exclude_base=True, exclude_id=True)

    py_lwm2 = _PyLocalWorkspaceManifest.from_remote_with_local_context(
        remote=py_rwm, prevent_sync_pattern=r".+", local_manifest=py_lwm2, timestamp=ts
    )
    rs_lwm2 = LocalWorkspaceManifest.from_remote_with_local_context(
        remote=rs_rwm, prevent_sync_pattern=r".+", local_manifest=rs_lwm2, timestamp=ts
    )

    assert py_lwm.match_remote(py_rwm) == rs_lwm.match_remote(rs_rwm)


@pytest.mark.rust
def test_local_user_manifest():
    from parsec.core.types.manifest import (
        _RsLocalUserManifest,
        LocalUserManifest,
        _PyLocalUserManifest,
    )

    assert LocalUserManifest is _RsLocalUserManifest

    def _assert_local_user_manifest_eq(py, rs, exclude_base=False, exclude_id=False):
        assert isinstance(py, _PyLocalUserManifest)
        assert isinstance(rs, _RsLocalUserManifest)

        if not exclude_base:
            assert py.base == rs.base
        if not exclude_id:
            assert py.id == rs.id
        assert py.need_sync == rs.need_sync
        assert py.updated == rs.updated
        assert py.last_processed_message == rs.last_processed_message
        assert len(py.workspaces) == len(rs.workspaces)
        assert isinstance(rs.workspaces, type(py.workspaces))
        assert py.workspaces == rs.workspaces
        assert py.speculative == rs.speculative

    def _assert_user_manifest_eq(py, rs):
        assert py.author == rs.author
        assert py.version == rs.version
        assert py.timestamp == rs.timestamp
        assert py.created == rs.created
        assert py.updated == rs.updated
        assert py.last_processed_message == rs.last_processed_message
        assert py.workspaces == rs.workspaces
        assert isinstance(rs.workspaces, type(py.workspaces))

    kwargs = {
        "base": UserManifest(
            author=DeviceID("user@device"),
            id=EntryID.new(),
            version=42,
            timestamp=DateTime.now(),
            created=DateTime.now(),
            updated=DateTime.now(),
            last_processed_message=0,
            workspaces=(WorkspaceEntry.new(EntryName("user"), DateTime.now()),),
        ),
        "need_sync": True,
        "updated": DateTime.now(),
        "last_processed_message": 0,
        "workspaces": (),
        "speculative": True,
    }

    py_lum = _PyLocalUserManifest(**kwargs)
    rs_lum = LocalUserManifest(**kwargs)
    _assert_local_user_manifest_eq(py_lum, rs_lum)

    kwargs = {
        "base": kwargs["base"].evolve(
            **{
                "author": DeviceID("a@b"),
                "id": EntryID.new(),
                "version": 1337,
                "timestamp": DateTime.now(),
                "created": DateTime.now(),
                "updated": DateTime.now(),
                "last_processed_message": 1,
                "workspaces": (WorkspaceEntry.new(EntryName("user"), DateTime.now()),),
            }
        ),
        "need_sync": False,
        "updated": DateTime.now(),
        "last_processed_message": 1,
        "workspaces": (WorkspaceEntry.new(EntryName("wk"), DateTime.now()),),
        "speculative": False,
    }

    py_lum = py_lum.evolve(**kwargs)
    rs_lum = rs_lum.evolve(**kwargs)
    _assert_local_user_manifest_eq(py_lum, rs_lum)

    sk = SecretKey.generate()

    py_enc = py_lum.dump_and_encrypt(sk)
    rs_enc = py_lum.dump_and_encrypt(sk)

    # Decrypt rust encrypted with Python and vice versa
    lum1 = _PyLocalUserManifest.decrypt_and_load(rs_enc, sk)
    lum2 = LocalUserManifest.decrypt_and_load(py_enc, sk)
    assert isinstance(lum1, LocalUserManifest)
    assert isinstance(lum2, LocalUserManifest)
    assert lum1 == lum2

    assert py_lum.to_stats() == rs_lum.to_stats()
    assert py_lum.asdict() == rs_lum.asdict()

    ts = DateTime.now()
    ei = EntryID.new()
    di = DeviceID("a@b")

    # With optional parameters
    py_lum = _PyLocalUserManifest.new_placeholder(author=di, id=ei, timestamp=ts, speculative=True)
    rs_lum = LocalUserManifest.new_placeholder(author=di, id=ei, timestamp=ts, speculative=True)
    _assert_local_user_manifest_eq(py_lum, rs_lum, exclude_base=True, exclude_id=True)

    # Without optional parameters
    py_lum = _PyLocalUserManifest.new_placeholder(author=di, timestamp=ts)
    rs_lum = LocalUserManifest.new_placeholder(author=di, timestamp=ts)
    _assert_local_user_manifest_eq(py_lum, rs_lum, exclude_base=True, exclude_id=True)

    py_rum = py_lum.to_remote(author=di, timestamp=ts)
    rs_rum = rs_lum.to_remote(author=di, timestamp=ts)
    _assert_user_manifest_eq(py_rum, rs_rum)

    assert py_lum.match_remote(py_rum) == rs_lum.match_remote(rs_rum)

    py_lum2 = _PyLocalUserManifest.from_remote(py_rum)
    rs_lum2 = LocalUserManifest.from_remote(rs_rum)
    _assert_local_user_manifest_eq(py_lum2, rs_lum2, exclude_base=True, exclude_id=True)

    _PyLocalUserManifest.from_remote_with_local_context(
        remote=py_rum, prevent_sync_pattern=r".+", local_manifest=py_lum2, timestamp=ts
    )
    LocalUserManifest.from_remote_with_local_context(
        remote=rs_rum, prevent_sync_pattern=r".+", local_manifest=rs_lum2, timestamp=ts
    )
