import os
import pytest

from parsec.core.fs.types import Path


@pytest.mark.parametrize(
    "path,is_root", [("/", True), ("//", True), ("/foo", False), ("/foo/bar", False)]
)
def test_root(path, is_root):
    obj = Path(path)
    assert obj.is_root() is is_root
    assert "//" not in str(obj)
    if os.name == "nt":
        wpath = path.replace("/", "\\")
        obj = Path(wpath)
        assert obj.is_root() is is_root


@pytest.mark.skipif(os.name != "nt", reason="No such craziness on Posix")
@pytest.mark.parametrize(
    "path,wpath",
    [
        ("/", "\\"),
        ("//", "\\\\"),
        ("//", "/\\"),
        ("/foo", "\\foo"),
        ("/foo/bar", "\\foo/bar"),
        ("/foo/bar", "/foo\\bar"),
    ],
)
def test_mix_windows_and_posix_slashes(path, wpath):
    obj = Path(path)
    wobj = Path(wpath)
    assert obj == wobj
    assert obj.is_root() == wobj.is_root()
    assert str(obj) == str(wobj)


@pytest.mark.parametrize("path", ["/", "///", "/foo", "/foo/bar"])
def test_stringify(path):
    # Don't test '//' because according to POSIX path resolution:
    # http://pubs.opengroup.org/onlinepubs/009695399/basedefs/xbd_chap04.html#tag_04_11
    # "A pathname that begins with two successive slashes may be
    # interpreted in an implementation-defined manner, although more
    # than two leading slashes shall be treated as a single slash".
    obj = Path(path)
    assert str(obj) == path.replace("///", "/")
    if os.name == "nt":
        wpath = path.replace("/", "\\")
        obj = Path(wpath)
        assert str(obj) == path.replace("///", "/")


@pytest.mark.parametrize("path", ["", "foo", "foo/bar"])
def test_absolute(path):
    with pytest.raises(ValueError):
        Path(path)

    if os.name == "nt":
        wpath = path.replace("/", "\\")
        with pytest.raises(ValueError):
            Path(wpath)
