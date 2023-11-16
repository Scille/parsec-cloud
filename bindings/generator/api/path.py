# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import EntryName, FsPath, Ref


def path_join(parent: Ref[FsPath], child: Ref[EntryName]) -> FsPath:
    raise NotImplementedError


def path_parent(path: Ref[FsPath]) -> FsPath:
    raise NotImplementedError


def path_filename(path: Ref[FsPath]) -> Optional[EntryName]:
    raise NotImplementedError


def path_split(path: Ref[FsPath]) -> list[EntryName]:
    raise NotImplementedError


def path_normalize(path: FsPath) -> FsPath:
    raise NotImplementedError
