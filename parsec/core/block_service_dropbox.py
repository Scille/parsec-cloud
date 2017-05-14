from uuid import uuid4
from dateutil import parser
import dropbox

from parsec.service import service
from parsec.core.block_service import BaseBlockService
from parsec.core.cache_service import CacheNotFound


class DropboxBlockService(BaseBlockService):

    cache_service = service('CacheService')

    def __init__(self, directory='parsec-storage'):
        super().__init__()
        token = 'SECRET'  # TODO load token
        self.dbx = dropbox.client.DropboxClient(token)

    async def create(self, content, id=None):
        id = id if id else uuid4().hex  # TODO uuid4 or trust seed?
        self.dbx.put_file(id, content)
        return id

    async def read(self, id):
        try:
            response = await self.cache_service.get(('read', id))
        except CacheNotFound:
            file, metadata = self.dbx.get_file_and_metadata(id)
            modified_date = parser.parse(metadata['modified']).timestamp()
            response = {'content': file.read().decode(), 'creation_timestamp': modified_date}
            await self.cache_service.set(('read', id), response)
        return response

    async def stat(self, id):
        try:
            response = await self.cache_service.get(('stat', id))
        except CacheNotFound:

            _, metadata = self.dbx.get_file_and_metadata(id)
            modified_date = parser.parse(metadata['modified']).timestamp()
            response = {'creation_timestamp': modified_date}
            await self.cache_service.set(('stat', id), response)
        return response
