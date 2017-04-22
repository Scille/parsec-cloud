from datetime import datetime
from uuid import uuid4
from marshmallow import fields

from dateutil import parser
import dropbox
from pydrive.auth import GoogleAuth
from pydrive.drive import GoogleDrive

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class BlockError(ParsecError):
    pass


class BlockNotFound(BlockError):
    status = 'not_found'


class cmd_CREATE_Schema(BaseCmdSchema):
    content = fields.String(required=True)


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class BaseBlockService(BaseService):

    name = 'BlockService'

    @cmd('block_create')
    async def _cmd_CREATE(self, session, msg):
        msg = cmd_CREATE_Schema().load(msg)
        id = await self.create(msg['content'])
        return {'status': 'ok', 'id': id}

    @cmd('block_read')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        block = await self.read(msg['id'])
        block.update({'status': 'ok'})
        return block

    @cmd('block_stat')
    async def _cmd_STAT(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        stat = await self.stat(msg['id'])
        stat.update({'status': 'ok'})
        return stat

    async def create(self, content):
        raise NotImplementedError()

    async def read(self, id):
        raise NotImplementedError()

    async def stat(self, id):
        raise NotImplementedError()


class MockedBlockService(BaseBlockService):
    def __init__(self):
        super().__init__()
        self._blocks = {}

    async def create(self, content):
        id = uuid4().hex  # TODO uuid4 or trust seed?
        timestamp = datetime.utcnow().timestamp()
        self._blocks[id] = {'content': content,
                            'access_timestamp': timestamp,
                            'creation_timestamp': timestamp}
        return id

    async def read(self, id):
        try:
            self._blocks[id]['access_timestamp'] = datetime.utcnow().timestamp()
            return self._blocks[id]
        except KeyError:
            raise BlockNotFound('Block not found.')

    async def stat(self, id):
        try:
            return {'access_timestamp': self._blocks[id]['access_timestamp'],
                    'creation_timestamp': self._blocks[id]['creation_timestamp']}
        except KeyError:
            raise BlockNotFound('Block not found.')


class DropboxBlockService(BaseBlockService):
    def __init__(self, directory='parsec-storage'):
        token = 'SECRET'  # TODO load token
        self.dbx = dropbox.client.DropboxClient(token)

    async def create(self, content):
        id = uuid4().hex  # TODO uuid4 or trust seed?
        self.dbx.put_file(id, content)
        return id

    async def read(self, id):
        file, metadata = self.dbx.get_file_and_metadata(id)
        modified_date = parser.parse(metadata['modified']).timestamp()
        # TODO impossible to set access_timestamp in Dropbox?
        return {'content': file.read().decode(),
                'access_timestamp': datetime.utcnow().timestamp(),
                'creation_timestamp': modified_date}

    async def stat(self, id):
        _, metadata = self.dbx.get_file_and_metadata(id)
        modified_date = parser.parse(metadata['modified']).timestamp()
        return {'access_timestamp': datetime.utcnow().timestamp(),
                'creation_timestamp': modified_date}


class GoogleDriveBlockService(BaseBlockService):
    def __init__(self, directory='parsec-storage'):
        credentials_path = '/tmp/parsec-googledrive-credentials.txt'
        gauth = GoogleAuth()
        gauth.LoadCredentialsFile(credentials_path)  # TODO other path?
        if gauth.credentials is None:
            gauth.LocalWebserverAuth()
        elif gauth.access_token_expired:
            gauth.Refresh()
        else:
            gauth.Authorize()
        gauth.SaveCredentialsFile(credentials_path)
        self.drive = GoogleDrive(gauth)
        # TODO call _get_file method instead
        query = "mimeType='application/vnd.google-apps.folder' and "
        query += 'trashed=false and '
        query += "title='" + directory + "'"
        file_list = self.drive.ListFile({'q': query}).GetList()
        if len(file_list) > 1:
            raise(BlockError('Multiple base directories found'))
        elif len(file_list) == 1:
            self.base_directory = file_list[0]['id']
        else:
            file = self.drive.CreateFile({'title': directory,
                                          'mimeType': 'application/vnd.google-apps.folder'})
            file.Upload()
            self.base_directory = file['id']

    async def _get_file(self, id):
        query = "'" + self.base_directory + "' in parents and "
        query += 'trashed=false and '
        query += "title='" + id + "'"
        file_list = self.drive.ListFile({'q': query}).GetList()
        if len(file_list) != 1:
            message = 'Multiple blocks found.' if file_list else 'Block not found.'
            raise(BlockError(message))
        return self.drive.CreateFile({'id': file_list[0]['id']})

    async def create(self, content):
        id = uuid4().hex  # TODO uuid4 or trust seed?
        file = self.drive.CreateFile({'title': id,
                                      'parents': [{'id': self.base_directory}]})
        file.SetContentString(content)
        file.Upload()
        return id

    async def read(self, id):
        file = await self._get_file(id)
        file['lastViewedByMeDate'] = datetime.utcnow().isoformat()
        file.Upload()
        return {'content': file.GetContentString(),
                'access_timestamp': file['lastViewedByMeDate'],
                'creation_timestamp': file['createdDate']}

    async def stat(self, id):
        file = await self._get_file(id)
        return {'access_timestamp': parser.parse(file['lastViewedByMeDate']).timestamp(),
                'creation_timestamp': parser.parse(file['createdDate']).timestamp()}
