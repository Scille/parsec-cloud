import pytest
from pendulum import datetime

from tests.common import freeze_time
from parsec.core.fs import BaseRootEntry, BaseFolderEntry, BaseFileEntry, FSInvalidPath


@pytest.fixture
async def alice_core(core, alice):
    await core.login(alice)
    return core


@pytest.fixture
async def root(alice_core):
    return alice_core.fs.root


@pytest.fixture
async def foo(alice_core):
    with freeze_time("2018-01-02"):
        return await alice_core.fs.root.create_folder("foo")


@pytest.fixture
async def bar_txt(alice_core):
    with freeze_time("2018-01-02"):
        return await alice_core.fs.root.create_file("bar.txt")


@pytest.fixture
async def animals(root):
    with freeze_time("2018-01-02"):
        animals = await root.create_folder("animals")
    with freeze_time("2018-01-03"):
        mammals = await animals.create_folder("mammals")
        insects = await animals.create_folder("insects")
    with freeze_time("2018-01-04"):
        await mammals.create_file("cat.txt")
        await mammals.create_file("dog.txt")
        await insects.create_file("ant.txt")
        await insects.create_file("cockroach.txt")


@pytest.mark.trio
async def test_read_only_attributes(root, foo, bar_txt):
    # Too slow to use parametrize for this
    for attr in ("parent", "name", "path", "is_placeholder", "created", "updated", "base_version"):
        with pytest.raises(AttributeError):
            setattr(root, attr, None)

        with pytest.raises(AttributeError):
            setattr(foo, attr, None)

        with pytest.raises(AttributeError):
            setattr(bar_txt, attr, None)


@pytest.mark.trio
async def test_get_root(core, alice):
    await core.login(alice)
    root = core.fs.root
    assert isinstance(root, BaseRootEntry)
    assert root.path == ""  # hoooo big hack here !
    assert root.parent is None
    assert root.name == ""


@pytest.mark.trio
async def test_create_folder(root):
    with freeze_time("2018-01-05"):
        foo = await root.create_folder("foo")

    assert isinstance(foo, BaseFolderEntry)
    assert root.keys() == {"foo"}
    assert foo.path == "/foo"
    assert foo.parent is root
    assert foo.name == "foo"
    assert foo.need_flush
    assert foo.need_sync
    assert foo.base_version == 0
    assert foo.created == datetime(2018, 1, 5)
    assert root.updated == datetime(2018, 1, 5)


@pytest.mark.trio
async def test_create_recursive_folder(foo):
    with freeze_time("2018-01-05"):
        bar = await foo.create_folder("bar")

    assert isinstance(bar, BaseFolderEntry)
    assert foo.keys() == {"bar"}
    assert bar.path == "/foo/bar"
    assert bar.name == "bar"
    assert bar.parent is foo
    assert bar.need_flush
    assert bar.need_sync
    assert bar.base_version == 0
    assert bar.created == datetime(2018, 1, 5)
    assert foo.updated == datetime(2018, 1, 5)


@pytest.mark.trio
async def test_delete_child(root, foo):
    with freeze_time("2018-01-05"):
        foo2 = await root.delete_child("foo")

    assert root.keys() == set()
    assert foo is foo2
    assert foo.name == "foo"
    assert foo.parent is None
    assert root.need_sync
    assert root.need_flush
    assert root.updated == datetime(2018, 1, 5)

    with pytest.raises(FSInvalidPath):
        await root.delete_child("dummy")


@pytest.mark.trio
async def test_reinsert_child(root, foo):
    with freeze_time("2018-01-05"):
        bar = await root.delete_child("foo")
    with freeze_time("2018-01-06"):
        await root.insert_child("bar", bar)

    assert root.keys() == {"bar"}
    assert bar.path == "/bar"
    assert bar.parent is root
    assert bar.name == "bar"


@pytest.mark.trio
async def test_fetch_child(core, root, animals):
    animals = await root.fetch_child("animals")
    assert isinstance(animals, BaseFolderEntry)

    ant_txt = await core.fs.fetch_path("/animals/insects/ant.txt")
    assert isinstance(ant_txt, BaseFileEntry)
    insects = await animals.fetch_child("insects")
    ant_txt2 = await insects.fetch_child("ant.txt")
    assert ant_txt is ant_txt2

    with pytest.raises(FSInvalidPath):
        await animals.fetch_child("dummy")
