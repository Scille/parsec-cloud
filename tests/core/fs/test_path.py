# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os

import pytest

from parsec.core.types import FsPath


@pytest.mark.parametrize(
    "path,is_root", [("/", True), ("//", True), ("/foo", False), ("/foo/bar", False)]
)
def test_root(path, is_root):
    obj = FsPath(path)
    assert obj.is_root() is is_root
    assert "//" not in str(obj)
    if os.name == "nt":
        wpath = path.replace("/", "\\")
        obj = FsPath(wpath)
        assert obj.is_root() is is_root


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
    obj = FsPath(path)
    wobj = FsPath(wpath)
    assert obj == wobj
    assert obj.is_root() == wobj.is_root()
    assert str(obj) == str(wobj)


@pytest.mark.parametrize(
    "weird_path",
    [
        "//foo/bar",
        "/foo//bar",
        "/foo\\\\bar",
        "//foo\\\\bar",
        "\\foo\\\\bar",
        "\\\\foo/bar",
        "\\\\foo//bar",
        "\\\\foo\\bar",
        "\\\\foo\\\\bar",
    ],
)
def test_weird_root_and_no_complains(weird_path):
    path = FsPath(weird_path)
    assert str(path) == "/foo/bar"


@pytest.mark.parametrize("path", ["/", "///", "/foo", "/foo/bar"])
def test_stringify(path):
    # Don't test '//' because according to POSIX path resolution:
    # http://pubs.opengroup.org/onlinepubs/009695399/basedefs/xbd_chap04.html#tag_04_11
    # "A pathname that begins with two successive slashes may be
    # interpreted in an implementation-defined manner, although more
    # than two leading slashes shall be treated as a single slash".
    obj = FsPath(path)
    assert str(obj) == path.replace("///", "/")
    if os.name == "nt":
        wpath = path.replace("/", "\\")
        obj = FsPath(wpath)
        assert str(obj) == path.replace("///", "/")


@pytest.mark.parametrize("path", ["", "foo", "foo/bar", "C:\\foo\\bar"])
def test_absolute(path):
    with pytest.raises(ValueError):
        FsPath(path)
