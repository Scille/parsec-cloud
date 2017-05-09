from datetime import datetime
from uuid import uuid4
from dateutil import parser
from pydrive.auth import GoogleAuth
from pydrive.drive import GoogleDrive

from parsec.core.block_service import BaseBlockService, BlockError


class GoogleDriveBlockService(BaseBlockService):
    def __init__(self, directory='parsec-storage'):
        super().__init__()
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
            raise BlockError('Multiple base directories found')
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
            raise BlockError(message)
        return self.drive.CreateFile({'id': file_list[0]['id']})

    async def create(self, content, id=None):
        id = id if id else uuid4().hex  # TODO uuid4 or trust seed?
        file = self.drive.CreateFile({'title': id,
                                      'parents': [{'id': self.base_directory}]})
        file.SetContentString(content)
        file.Upload()
        return id

    async def read(self, id):
        file = await self._get_file(id)
        file['lastViewedByMeDate'] = datetime.utcnow().isoformat()
        file.Upload()
        return {'content': file.GetContentString(), 'creation_timestamp': file['createdDate']}

    async def stat(self, id):
        file = await self._get_file(id)
        return {'creation_timestamp': parser.parse(file['createdDate']).timestamp()}
