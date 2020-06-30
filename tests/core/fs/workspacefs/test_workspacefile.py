# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from enum import IntEnum
from typing import Union
import io
from parsec.core.fs.exceptions import FSUnsupportedOperation, FSOffsetError
from parsec.core.fs.workspacefs.workspacefile import WorkspaceFile
from parsec.core.types import FsPath

AnyPath = Union[FsPath, str]


class FileState(IntEnum):
    INIT = 0
    OPEN = 1
    CLOSED = 2


@pytest.fixture
@pytest.mark.trio
async def trio_file(tmp_path):
    d = tmp_path / "triofoo"
    d.mkdir()
    p = d / "triobar"
    p.write_text("Hello World !")
    return p


@pytest.fixture
@pytest.mark.trio
def random_text():
    text = """Mem dem ima nequeam vos gratiam junctas aliunde maximam. Tot denuo terea tur
    ritas justa talem vel. At possumus ac privatio superare in excitari tractant.
    Advertisse incrementi quaerendum conservant denegassem ei in facillimum.
    Ii mo utilius quamvis rationi ut fuerunt. Mea dignum vos ibidemtantae quidam cau quaeri cap.
    Adipisci co de rationis ut originem competit sessione sequatur ad. Artificium frequenter agi
    excoluisse mortalibus sum describere cau accidentia.
    """
    return text


async def compare_read(triof=None, f=None, size: int = -1, alice_workspace=None, trio_file=None):
    """ Comparing the full read of boths files with both methods"""
    if (f is None and alice_workspace is None) or (triof is None and trio_file is None):
        raise ValueError
    if f is None:
        f = await alice_workspace.open_file("/foo/bar", "rb")
    if triof is None:
        triof = await trio.open_file(trio_file, "rb")
    output = await f.read(size)
    trio_output = await triof.read(size)
    assert trio_output == output
    assert f.tell() == await triof.tell()


async def write_in_both_files(triof, f, text):
    """ writting in both files"""
    # Writting text in both files
    output = await f.write(text)
    trio_output = await triof.write(text)

    # Assert the number of bytes writed
    assert trio_output == output


@pytest.mark.trio
async def test_open(alice_workspace, trio_file):

    # Testing open multiples times same file
    f = await alice_workspace.open_file("/foo/bar", "r")
    f2 = await alice_workspace.open_file("/foo/bar", "r")
    triof = await trio.open_file(trio_file, "r")
    triof2 = await trio.open_file(trio_file, "r")
    assert (triof != triof2) == (f != f2)
    assert f._fd == 1 and f2._fd == 2
    # Testing opening in write and read mode in same time. should raise ValueError
    with pytest.raises(ValueError):
        await alice_workspace.open_file("/foo/bar", "rw")
    with pytest.raises(ValueError):
        await alice_workspace.open_file("/foo/bar", "rab")
    with pytest.raises(ValueError):
        await alice_workspace.open_file("/foo/bar", "wab")
    with pytest.raises(ValueError):
        await trio.open_file(trio_file, "rw")
    with pytest.raises(ValueError):
        await trio.open_file(trio_file, "rab")
    with pytest.raises(ValueError):
        await trio.open_file(trio_file, "wab")
    # Testing opening with wrong mode
    with pytest.raises(ValueError):
        await alice_workspace.open_file("/foo/bar", "wz")
    with pytest.raises(ValueError):
        await trio.open_file(trio_file, "wz")
    # Testing opening with no mode
    with pytest.raises(ValueError):
        await alice_workspace.open_file("/foo/bar", "")
    with pytest.raises(ValueError):
        await trio.open_file(trio_file, "")


@pytest.mark.trio
async def test_open_right(alice_workspace, trio_file, random_text):
    # The text tested in both methods
    random_text = random_text.encode("utf-8")

    # Open with r
    f = await alice_workspace.open_file("/foo/bar", "rb")
    triof = await trio.open_file(trio_file, "rb")
    assert f.writable() == triof.writable()
    assert f.readable() == triof.readable()
    # Trying to write anyway
    with pytest.raises(FSUnsupportedOperation):
        await f.write(random_text)
    with pytest.raises(io.UnsupportedOperation):
        await triof.write(random_text)
    await f.close()
    await triof.aclose()

    # Open with w
    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")
    assert f.writable() == triof.writable()
    assert f.readable() == triof.readable()
    # Trying to read anyway
    with pytest.raises(FSUnsupportedOperation):
        await f.read()
    with pytest.raises(io.UnsupportedOperation):
        await triof.read()
    await f.close()
    await triof.aclose()
    # Open with a
    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")
    assert f.writable() == triof.writable()
    assert f.readable() == triof.readable()
    # Trying to read anyway
    with pytest.raises(FSUnsupportedOperation):
        await f.read()
    with pytest.raises(io.UnsupportedOperation):
        await triof.read()
    await f.close()


@pytest.mark.trio
async def test_read_write(alice_workspace, trio_file, random_text):
    # The text tested in both methods
    random_text = random_text.encode("utf-8")

    # Opening both files
    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")

    # Writting text in both files
    output = await f.write(random_text)
    trio_output = await triof.write(random_text)
    await f.close()
    await triof.aclose()

    # Assert the number of bytes writed
    assert trio_output == output == 491

    # Assert workspacefile read and trio read are same
    triof = await trio.open_file(trio_file, "rb")
    f = await alice_workspace.open_file("/foo/bar", "rb")
    await compare_read(triof, f)

    # Closing both files directly and opening again with w to test the truncate
    await f.close()
    await triof.aclose()
    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")
    await f.close()
    await triof.aclose()
    f = await alice_workspace.open_file("/foo/bar", "rb")
    triof = await trio.open_file(trio_file, "rb")
    await compare_read(triof, f)
    await f.close()
    await triof.aclose()

    # Opening both files
    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")

    # Writting text in both files
    output = await f.write(random_text)
    trio_output = await triof.write(random_text)
    await f.close()
    await triof.aclose()

    # Opening both files again to see if w erased the file content
    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")
    new_text = "My Best Hello World !".encode("utf-8")
    # Writting new text in both files 2 times to check if erasing again or not
    await write_in_both_files(triof, f, new_text)
    await write_in_both_files(triof, f, new_text)
    await f.close()
    await triof.aclose()

    # Assert workspacefile read and trio read are same
    triof = await trio.open_file(trio_file, "rb")
    f = await alice_workspace.open_file("/foo/bar", "rb")
    await compare_read(triof, f)

    # Comparing the read again to compare offset management
    await compare_read(triof, f)
    # Setting offset to 0 and comparing the read again to compare offset management
    await f.seek(0, io.SEEK_SET)
    await triof.seek(0, io.SEEK_SET)
    await compare_read(triof, f)
    await f.close()
    await triof.aclose()

    # Opening both files again to test if 'append' work correctly
    f = await alice_workspace.open_file("/foo/bar", "ab")
    triof = await trio.open_file(trio_file, "ab")
    # Writting new text in both files 2 times to check if erasing again or not
    # We should have 4 times "My Best Hello World !"
    await write_in_both_files(triof, f, new_text)
    await write_in_both_files(triof, f, new_text)

    # Setting offset to 0 and appending again to compare offset management + append management
    await f.seek(0, io.SEEK_SET)
    await triof.seek(0, io.SEEK_SET)
    new_text = " I append after seek".encode("utf-8")
    await write_in_both_files(triof, f, new_text)
    await f.close()
    await triof.aclose()

    # Assert workspacefile read and trio read are same
    triof = await trio.open_file(trio_file, "rb")
    f = await alice_workspace.open_file("/foo/bar", "rb")
    await compare_read(triof, f)

    # Read a positive size smaller than file_size, setting offset to 0
    await f.seek(0, io.SEEK_SET)
    await triof.seek(0, io.SEEK_SET)
    await compare_read(triof, f, 10)

    # Read a positive size bigger than file_size, setting offset to 0
    await f.seek(0, io.SEEK_SET)
    await triof.seek(0, io.SEEK_SET)
    await compare_read(triof, f, 1000)

    # Read a negative size, setting offset to 0
    await f.seek(0, io.SEEK_SET)
    await triof.seek(0, io.SEEK_SET)
    with pytest.raises(ValueError):
        trio_output = await triof.read(-1000)
        trio_output = trio_output.decode("utf-8")
    with pytest.raises(ValueError):
        output = await f.read(-1000)
        output = f.decode("utf-8")


@pytest.mark.trio
async def test_seek(alice_workspace, trio_file):

    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")
    await write_in_both_files(triof, f, "thisis10ch".encode("utf-8"))
    assert f.tell() == await triof.tell()
    # Test seek end
    await f.seek(0, io.SEEK_END)
    await triof.seek(0, io.SEEK_END)
    assert f.tell() == await triof.tell() == 10
    # Test seek set with negative value
    with pytest.raises(FSOffsetError):
        await f.seek(-12, io.SEEK_SET)
    with pytest.raises(OSError):
        await triof.seek(-10, io.SEEK_SET)
    assert f.tell() == await triof.tell() == 10
    # Test seek set with positive value
    await f.seek(0, io.SEEK_SET)
    await triof.seek(0, io.SEEK_SET)
    assert f.tell() == await triof.tell() == 0
    await f.seek(1000, io.SEEK_SET)
    await triof.seek(1000, io.SEEK_SET)
    assert f.tell() == await triof.tell() == 1000
    # Test seek cur with negative value
    await triof.seek(-100, io.SEEK_CUR)
    await f.seek(-100, io.SEEK_CUR)
    assert f.tell() == await triof.tell() == 900
    # Test seek cur with negative value going in negativ offset
    with pytest.raises(OSError):
        await triof.seek(-1000, io.SEEK_CUR)
    with pytest.raises(FSOffsetError):
        await f.seek(-1000, io.SEEK_CUR)
    assert f.tell() == await triof.tell() == 900
    # Test seek cur with null value going in negativ offset
    await triof.seek(0, io.SEEK_CUR)
    await f.seek(0, io.SEEK_CUR)
    assert f.tell() == await triof.tell() == 900
    # Test seek cur with positive value going in negativ offset
    await triof.seek(1100, io.SEEK_CUR)
    await f.seek(1100, io.SEEK_CUR)
    assert f.tell() == await triof.tell() == 2000
    # Test seek end with null value
    await triof.seek(0, io.SEEK_END)
    await f.seek(0, io.SEEK_END)
    assert f.tell() == await triof.tell() == 10
    # Test seek end with negative value
    await triof.seek(-5, io.SEEK_END)
    await f.seek(-5, io.SEEK_END)
    assert f.tell() == await triof.tell() == 5
    # Test seek end with negative value going in negativ offset
    with pytest.raises(OSError):
        await triof.seek(-100, io.SEEK_END)
    with pytest.raises(FSOffsetError):
        await f.seek(-100, io.SEEK_END)
    assert f.tell() == await triof.tell() == 5
    # Test seek end with positive value
    await triof.seek(500, io.SEEK_END)
    await f.seek(500, io.SEEK_END)
    assert f.tell() == await triof.tell() == 510

    # closing files
    await f.close()
    await triof.aclose()


async def open_file_no_init(workspace, path: AnyPath, mode="r"):
    path = FsPath(path)
    _, fd = await workspace.transactions.file_open(path, mode)
    f = WorkspaceFile(fd, workspace.transactions, mode=mode, path=path)
    return f


@pytest.mark.trio
async def test_file_state(alice_workspace, trio_file, random_text):
    # testing that file state is in INIT mode.
    f = await open_file_no_init(alice_workspace, "/foo/bar", "wb")
    assert int(f.state) == int(FileState.INIT)

    # trying some methods with INIT file state
    with pytest.raises(ValueError) as e:
        f.writable()
    assert str(e.value) == "I/O operation on non-initialized file."
    with pytest.raises(ValueError) as e:
        f.readable()
    assert str(e.value) == "I/O operation on non-initialized file."
    with pytest.raises(ValueError) as e:
        f.seekable()
    assert str(e.value) == "I/O operation on non-initialized file."

    # init the file
    await f.ainit()
    assert int(f.state) == int(FileState.OPEN)

    # trying some methods with OPEN file state
    f.writable()
    f.readable()
    f.seekable()

    # closing the file
    await f.close()
    assert int(f.state) == int(FileState.CLOSED)

    # trying some methods with CLOSED file state
    with pytest.raises(ValueError) as e:
        f.writable()
    assert str(e.value) == "I/O operation on closed file."
    with pytest.raises(ValueError) as e:
        f.readable()
    assert str(e.value) == "I/O operation on closed file."
    with pytest.raises(ValueError) as e:
        f.seekable()
    assert str(e.value) == "I/O operation on closed file."

    # test opening with context manager
    async with await open_file_no_init(alice_workspace, "/foo/bar", "wb") as f2:
        # testing that file state is in OPEN mode.
        assert int(f2.state) == int(FileState.OPEN)
    # testing that file is closed after context manager
    assert int(f2.state) == int(FileState.CLOSED)


@pytest.mark.trio
async def test_close(alice_workspace, trio_file, random_text):
    # The text tested in both methods
    random_text = random_text.encode("utf-8")

    # Opening both files
    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")
    assert f.closed == triof.closed is False

    # closing files
    await f.close()
    await triof.aclose()
    assert f.closed == triof.closed is True

    # closing a 2nd time
    await f.close()
    await triof.aclose()
    assert f.closed == triof.closed is True

    # Test truncate with closed file
    with pytest.raises(ValueError):
        await f.truncate(8)
    with pytest.raises(ValueError):
        await triof.truncate(8)
    # Test seek with closed file
    with pytest.raises(ValueError):
        await triof.seek(500, io.SEEK_END)
    with pytest.raises(ValueError):
        await f.seek(500, io.SEEK_END)

    # Test write with closed file
    with pytest.raises(ValueError):
        await f.write(random_text)
    with pytest.raises(ValueError):
        await triof.write(random_text)

    # Test read with closed file
    with pytest.raises(ValueError):
        await f.read()
    with pytest.raises(ValueError):
        await triof.read()

    # Test tell with closed file
    with pytest.raises(ValueError):
        f.tell()
    with pytest.raises(ValueError):
        await triof.tell()


@pytest.mark.trio
async def test_truncate(alice_workspace, trio_file, random_text):
    f = await alice_workspace.open_file("/foo/bar", "wb")
    triof = await trio.open_file(trio_file, "wb")
    await write_in_both_files(triof, f, "thisis10ch".encode("utf-8"))
    # Test truncate with no value
    trio_output = await triof.truncate()
    output = await f.truncate()
    assert output == trio_output
    await compare_read(alice_workspace=alice_workspace, trio_file=trio_file)

    # Test truncate with negative value
    with pytest.raises(OSError):
        trio_output = await triof.truncate(-1)
    with pytest.raises(FSOffsetError):
        output = await f.truncate(-1)
    assert output == trio_output
    await compare_read(alice_workspace=alice_workspace, trio_file=trio_file)

    # Test truncate with positive value under file_size
    trio_output = await triof.truncate(8)
    output = await f.truncate(8)
    assert output == trio_output
    await compare_read(alice_workspace=alice_workspace, trio_file=trio_file)

    # Test truncate with positive value bigger than file_size
    trio_output = await triof.truncate(100)
    output = await f.truncate(100)
    assert output == trio_output
    await compare_read(alice_workspace=alice_workspace, trio_file=trio_file)

    # Test truncate with 0 value
    trio_output = await triof.truncate(0)
    output = await f.truncate(0)
    assert output == trio_output
    await compare_read(alice_workspace=alice_workspace, trio_file=trio_file)
    await f.close()
    await triof.aclose()

    # Test truncate with no write right
    f = await alice_workspace.open_file("/foo/bar", "rb")
    triof = await trio.open_file(trio_file, "rb")
    with pytest.raises(FSUnsupportedOperation):
        output = await f.truncate(100)
    with pytest.raises(io.UnsupportedOperation):
        trio_output = await triof.truncate(100)
    assert output == trio_output
    await compare_read(alice_workspace=alice_workspace, trio_file=trio_file)


@pytest.mark.trio
async def test_mode(alice_workspace, trio_file):
    f = await alice_workspace.open_file("/foo/bar", "wb")
    assert f.mode == "wb"
    await f.close()


@pytest.mark.trio
async def test_name(alice_workspace, trio_file):
    f = await alice_workspace.open_file("/foo/bar", "wb")
    assert f.name == FsPath("/foo/bar")
    await f.close()


# TODO
# @pytest.mark.trio
# async def test_move_file_from_workspace_to_another_workspace(
#     alice_workspace, bob_workspace, trio_file
# ):
#     f = await alice_workspace.open_file("/foo/bar", "wb")
#     assert f.name == FsPath("/foo/bar")
#     await f.close()
