import arrow

from parsec.core2.fs_api import BaseFSAPIMixin
from parsec.core2.workspace import Workspace, Reader, Writer


class FSAPIMixin(BaseFSAPIMixin):

    def __init__(self):
        self._fs = Workspace()
        self._reader = Reader()
        self._writer = Writer()

    async def file_create(self, path: str):
        await self._writer.file_create(self._fs, path)

    async def file_write(self, path: str, content: bytes, offset: int=0):
        await self._writer.file_write(self._fs, path, content, offset)

    async def file_read(self, path: str, offset: int=0, size: int=-1):
        return await self._reader.file_read(self._fs, path, offset, size)

    async def stat(self, path: str):
        return await self._reader.stat(self._fs, path)

    async def folder_create(self, path: str):
        await self._writer.folder_create(self._fs, path)

    async def move(self, src: str, dst: str):
        await self._writer.move(self._fs, src, dst)

    async def delete(self, path: str):
        await self._writer.delete(self._fs, path)

    async def file_truncate(self, path: str, length: int):
        await self._writer.file_truncate(self._fs, path, length)
