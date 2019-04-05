import attr
from typing import NewType

from parsec.core.types import Access


FileDescriptor = NewType("FileDescriptor", int)


@attr.s(slots=True)
class FileCursor:
    access = attr.ib(type=Access)
    offset = attr.ib(default=0, type=int)
