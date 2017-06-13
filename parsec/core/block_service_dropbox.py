from dateutil import parser
from uuid import uuid4

import dropbox

from parsec.core.cache import cached_block
from parsec.core.block_service import BaseBlockService


class DropboxBlockService(BaseBlockService):

    def __init__(self, directory='parsec-storage'):
        super().__init__()
        token = 'SECRET'  # TODO load token
        self.dbx = dropbox.client.DropboxClient(token)

    @cached_block
    async def create(self, content, id=None):
        id = id if id else uuid4().hex  # TODO uuid4 or trust seed?
        self.dbx.put_file(id, content)
        return id

    @cached_block
    async def read(self, id):
        file, metadata = self.dbx.get_file_and_metadata(id)
        modified_date = parser.parse(metadata['modified']).isoformat()
        return {'content': file.read().decode(), 'creation_date': modified_date}

    @cached_block
    async def stat(self, id):
        _, metadata = self.dbx.get_file_and_metadata(id)
        modified_date = parser.parse(metadata['modified']).isoformat()
        return {'creation_date': modified_date}
