# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from typing import NewType

from parsec.core.types import EntryID


FileDescriptor = NewType("FileDescriptor", int)


@attr.s(slots=True, auto_attribs=True)
class FileCursor:
    entry_id: EntryID
    offset: int = 0
