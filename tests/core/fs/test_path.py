# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.core.fs import FsPath


@pytest.mark.parametrize(
    "path,is_root",
    [("/", True), ("//", True), ("/foo", False), ("/foo/bar", False), ("/你/😀", False)],
)
def test_root(path, is_root):
    obj = FsPath(path)
    assert obj.is_root() is is_root
    assert "//" not in str(obj)


@pytest.mark.parametrize(
    "path,sanitized_path",
    [
        ("/", "/"),
        ("///", "/"),
        ("/foo", "/foo"),
        ("/foo/bar", "/foo/bar"),
        ("//foo//bar//", "/foo/bar"),
    ],
)
def test_stringify(path, sanitized_path):
    # Don't test '//' because according to POSIX path resolution:
    # http://pubs.opengroup.org/onlinepubs/009695399/basedefs/xbd_chap04.html#tag_04_11
    # "A pathname that begins with two successive slashes may be
    # interpreted in an implementation-defined manner, although more
    # than two leading slashes shall be treated as a single slash".
    obj = FsPath(path)
    assert str(obj) == sanitized_path


@pytest.mark.parametrize("path", ["", "foo", "foo/bar", "C:/foo/bar"])
def test_absolute(path):
    with pytest.raises(ValueError):
        FsPath(path)


@pytest.mark.parametrize(
    "path",
    [
        "/./foo/bar",
        "//foo///bar/",
        "/spam/../foo/bar",
        "/foo/../../../foo/bar",
        "/./foo/./././bar/.",
        "/../foo//./spam////../bar",
    ],
)
def test_dot_collapse(path):
    obj = FsPath(path)
    assert str(obj) == "/foo/bar"
