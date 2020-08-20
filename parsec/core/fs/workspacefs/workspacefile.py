# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import re
from typing import Union
from enum import IntEnum
import functools

from parsec.core.fs.exceptions import FSUnsupportedOperation, FSOffsetError
from parsec.core.types import FsPath


class FileState(IntEnum):
    INIT = 0
    OPEN = 1
    CLOSED = 2


def check_state(func):
    @functools.wraps(func)
    def wrapper(self, *args, **kwargs):
        if self.closed:
            raise ValueError("I/O operation on closed file.")
        elif self._state == FileState.INIT:
            raise ValueError("I/O operation on non-initialized file.")

        return func(self, *args, **kwargs)

    return wrapper


class WorkspaceFile:
    """Async file Object.

    Methods of this object should act like trio async file :
    "https://github.com/python-trio/trio/blob/master/trio/_file_io.py" using transactions.

    If you are using this object outside a context manager you should call ainit() method first.

    Keyword arguments:
    fd -- the file descriptor.
    transactions -- object to use for file transactions.
    path -- relative path of the file.
    mode -- the mode where the file have been opened. It needs to have exactly one of "awrx".
    mode("a") -> append file.
    mode("b") -> have to be used with one of "awrx" mode, file will be write/read as bytes instead of string.
    mode("w") -> write file. If the file doesn't exist, create one.
    mode("r") -> read file.
    mode("x") -> If the file doesn't exist, create one, else, raise an Error. File will be opened in write mode.
    mode("+") -> have to be used with one of "awrx" mode, file will be in read/write mode.
    """

    def __init__(self, transactions, path, mode: str = "rb"):
        self._fd = None
        self._offset = 0
        self._state = FileState.INIT
        self._path = FsPath(path)
        self._transactions = transactions
        mode = mode.lower()
        # Preventing to open in write and read in same time or write and append or open with no mode
        if sum(c in mode for c in "rwax") != 1:
            raise ValueError("must have exactly one of create/read/write/append mode")
        # Preventing to open with non-existant mode
        elif re.search("[^arwxb+]", mode) is not None:
            raise ValueError(f"invalid mode: '{mode}'")
        if "b" not in mode:
            raise NotImplementedError("Text mode is not supported at the moment")
        self._mode = mode

    async def __aenter__(self):
        await self.ainit()
        return self

    async def __aexit__(self, *args):
        self._state = FileState.CLOSED
        await self.close()

    async def ainit(self):
        """Initializing the File Object.

            Check the FileState, open or create the file, truncate it and move the offset if needed.
        """
        if self._state == FileState.INIT:
            self._state = FileState.OPEN
            # Opening or creating the file
            if "x" in self._mode:
                # Try create the file
                _, self._fd = await self._transactions.file_create(self._path, open=True)
            else:
                try:
                    mode = "rw" if self.writable() else "r"
                    _, self._fd = await self._transactions.file_open(self._path, mode)
                except FileNotFoundError:
                    if "a" in self._mode or "w" in self._mode:
                        _, self._fd = await self._transactions.file_create(self._path, open=True)
                    else:
                        raise

            # Truncate the file if opened with 'w' mode
            if "w" in self._mode:
                await self.truncate(0)
            # Move the offset to file.len if opened with 'a' mode
            if "a" in self._mode:
                self._offset = await self.get_size()

    async def __anext__(self):
        return await self.readline()

    async def close(self):
        """Close the file"""
        if self._state != FileState.CLOSED:
            self._state = FileState.CLOSED
            await self._transactions.fd_close(self._fd)

    @property
    def closed(self):
        return self._state == FileState.CLOSED

    @check_state
    async def file_stat(self):
        """Getting file stat"""
        stats = await self._transactions.fd_info(self._fd, self._path)
        return stats

    @check_state
    async def get_size(self):
        """Getting file length from file stat"""
        stats = await self.file_stat()
        return stats["size"]

    @property
    def state(self):
        return self._state

    @property
    def mode(self):
        return self._mode

    @property
    def name(self):
        return self._path

    async def read(self, size: int = -1) -> bytes:
        """ Read up to size bytes from the object and return them.

        As a convenience, if size is unspecified or -1, all bytes until EOF are returned.
        If 0 bytes are returned, and size was not 0, this indicates end of file.
        Raises:
            FSUnsupportedOperation
        """
        # Check if readable
        if not self.readable():
            raise FSUnsupportedOperation
        # Correct size :
        if size < -1:
            raise ValueError("read length must be non-negative or -1")
        # Reading until EOF : set offset to end (equal to size)
        if size == -1:
            result = await self._transactions.fd_read(self._fd, size, self._offset)
            self._offset += len(result)
            return result
            # Reading size : add to offset size
        else:
            result = await self._transactions.fd_read(self._fd, size, self._offset)
            self._offset += len(result)
            return result

    @check_state
    def readable(self) -> bool:
        return "r" in self._mode or "+" in self._mode

    async def readline(self, size: int = -1):
        raise NotImplementedError

    async def seek(self, offset: int, whence=os.SEEK_SET) -> int:
        """ Change the stream position to the given offset.
        Behaviour depends on the whence parameter. The default value for whence is SEEK_SET.
        SEEK_SET or 0 -> seek from the start of the stream (the default);
        Offset have to be 0 or bigger.
        SEEK_CUR or 1 -> "seek" to the current position; Offset can be 0, positive or negative but
        it will raise FSOffsetError if the final stream position is negative.
        SEEK_END or 2: seek to the end of the stream; Offset can be 0, positive or negative but
        it will raise FSOffsetError if the final stream position is negative.
        The new stream position is returned.
        Raises:
            FSOffsetError
            FSUnsupportedOperation
        """
        if not self.seekable():
            raise FSUnsupportedOperation
        if whence == os.SEEK_SET:
            if offset < 0:
                raise FSOffsetError
            self._offset = offset
        if whence == os.SEEK_CUR:
            if offset < 0 and self._offset + offset < 0:
                raise FSOffsetError
            self._offset += offset
        if whence == os.SEEK_END:
            # If offset going negative
            if offset < 0 and self._offset + offset < 0:
                raise FSOffsetError
            self._offset = await self.get_size() + offset
        return self._offset

    @check_state
    def seekable(self) -> bool:
        return True

    def tell(self) -> int:
        """Return the current stream position. Unlike trio.open_file files, this method is sync"""
        if not self.seekable():
            raise FSUnsupportedOperation
        return self._offset

    async def truncate(self, size=None) -> int:
        """ Resize the stream to the given size in bytes.
        Resize to the current position if size is not specified.
        The current stream position isn't changed.
        This resizing can extend or reduce the current file size. In case of extension, the
        contents of the new file area depend on the platform (on most systems, additional bytes are
        zero-filled).
        The new file size is returned.
        A negative size will raise a FSOffsetError
        Raises:
            FSOffsetError
            FSUnsupportedOperation
        """
        if not self.seekable():
            raise FSUnsupportedOperation
        if not self.writable():
            raise FSUnsupportedOperation
        if size is None:
            await self._transactions.fd_resize(self._fd, self._offset, truncate_only=False)
            return self._offset
        elif size < 0:
            raise FSOffsetError
        elif size == 0:
            await self._transactions.fd_resize(self._fd, size, truncate_only=True)
            return size
        else:  # Size > 0:
            await self._transactions.fd_resize(self._fd, size, truncate_only=False)
            return size

    @check_state
    def writable(self) -> bool:
        return "w" in self._mode or "a" in self._mode or "x" in self._mode or "+" in self._mode

    async def write(self, data: Union[str, bytes]) -> int:
        """ Check write right and execute write_bytes or write_str depend on the mode
            Raises:
            FSUnsupportedOperation
        """
        if not self.writable():
            raise FSUnsupportedOperation
        # Preparing offset at EOF if open with append mode
        if "a" in self._mode:
            self._offset = await self.get_size()
        if "b" in self._mode:
            if not isinstance(data, (bytes, bytearray)):
                raise TypeError(f"a bytes-like object is required, not '{type(data).__name__}'")
            return await self._write_bytes(data)
        else:
            if not isinstance(data, str):
                raise TypeError(f"write() argument must be str, not {type(data).__name__}")
        return await self._write_str(data)

    async def _write_str(self, data: str) -> int:
        raise NotImplementedError

    async def _write_bytes(self, data: bytes) -> int:
        """ Write the given bytes-like object.
        Return the number of bytes written.
        """

        result = await self._transactions.fd_write(self._fd, data, self._offset)
        self._offset += result
        return result
