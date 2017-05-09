from uuid import uuid4
from dateutil import parser
import dropbox

from parsec.core.block_service import BaseBlockService


class DropboxBlockService(BaseBlockService):
    def __init__(self, directory='parsec-storage'):
        super().__init__()
        token = 'SECRET'  # TODO load token
        self.dbx = dropbox.client.DropboxClient(token)

    async def create(self, content, id=None):
        id = id if id else uuid4().hex  # TODO uuid4 or trust seed?
        self.dbx.put_file(id, content)
        return id

    async def read(self, id):
        file, metadata = self.dbx.get_file_and_metadata(id)
        modified_date = parser.parse(metadata['modified']).timestamp()
        return {'content': file.read().decode(), 'creation_timestamp': modified_date}

    async def stat(self, id):
        _, metadata = self.dbx.get_file_and_metadata(id)
        modified_date = parser.parse(metadata['modified']).timestamp()
        return {'creation_timestamp': modified_date}
