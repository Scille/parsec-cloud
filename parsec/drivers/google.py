import httplib2
import os
from io import BytesIO
from os.path import split
from base64 import decodebytes, encodebytes

from apiclient import discovery
from apiclient.http import MediaIoBaseUpload
import oauth2client
from oauth2client import client
from oauth2client import tools
from time import mktime
from dateutil.parser import parse
import zmq


# If modifying these scopes, delete your previously saved credentials
# at ~/.credentials/drive-python-quickstart.json
SCOPES = 'https://www.googleapis.com/auth/drive'
CLIENT_SECRET_FILE = 'secret.json'
APPLICATION_NAME = 'Parsec'
PREFIX = 'PARSEC'


def _content_wrap(content):
    return encodebytes(content).decode()


def _content_unwrap(wrapped_content):
    return decodebytes(wrapped_content.encode())


def _clean_path(path):
    return '/' + '/'.join([e for e in path.split('/') if e])


class GoogleDriverException(Exception):
    pass


class GoogleDriver:

    def __init__(self):
        self._initialized = False
        self._credentials = self._get_credentials()
        self._http = self._credentials.authorize(httplib2.Http())
        self._service = discovery.build('drive', 'v3', http=self._http)

    def initialized(self):
        return self._initialized

    def initialize_driver(self):
        # check for specific files
        if self._initialized:
            return
        # Locate the root folder
        items = self._lookup_app_file(name="Parsec", role='root-folder')
        if len(items) != 1:
            raise GoogleDriverException('Drive Directory is broken, please check.')
        self._root_folder = items[0].get('id')
        if not self._root_folder:
            raise GoogleDriverException('Cannot locate root folder')

        # Locate the root manifest
        items = self._lookup_app_file(name="MANIFEST", role='root-manifest')
        if len(items) != 1:
            raise GoogleDriverException('Drive Directory is broken, please check.')
        self._root_manifest = items[0].get('id')
        if not self._root_manifest:
            raise GoogleDriverException('Cannot locate root manifest')
        self._initialized = True

        # TODO : load all other manifests

    def create_driver_files(self):
        """Initialize Driver Environnement in Google Drive"""

        if self._initialized:
            raise GoogleDriverException('Driver is already initialized, so does folder.')

        # Create Folder if does not exists
        results = self._service.files().create(
            body={
                "mimeType": "application/vnd.google-apps.folder",
                "isAppAuthorized": True,
                "appProperties": {'appName': '%s' % APPLICATION_NAME,
                                  'role': "root-folder",
                                  'path': '/',
                                  'mode': 'directory'},
                "name": APPLICATION_NAME,
            }
        ).execute()
        self._root_folder = results.get('id')
        if not self._root_folder:
            raise GoogleDriverException('Failed to initialise root folder')

        results = self._service.files().create(
            body={
                "mimeType": "application/scille.parsec.manifest",
                "isAppAuthorized": True,
                "appProperties": {'appName': '%s' % APPLICATION_NAME,
                                  'role': "root-manifest"},
                "parents": (self._root_folder,),
                "name": "MANIFEST",
                'mode': 'file',
            }
        ).execute()
        self._root_manifest = results.get('id')
        if not self._root_manifest:
            raise GoogleDriverException('Failed to initialise root manifest')
        self.initialized = True

    def _get_credentials(self):
        """Gets valid user credentials from storage.

        If nothing has been stored, or if the stored credentials are invalid,
        the OAuth2 flow is completed to obtain the new credentials.

        Returns:
            Credentials, the obtained credential.
        """
        home_dir = os.path.expanduser('~')
        credential_dir = os.path.join(home_dir, '.credentials')
        if not os.path.exists(credential_dir):
            os.makedirs(credential_dir)
        credential_path = os.path.join(credential_dir,
                                       'drive-python-quickstart.json')

        store = oauth2client.file.Storage(credential_path)
        credentials = store.get()
        if not credentials or credentials.invalid:
            flow = client.flow_from_clientsecrets(CLIENT_SECRET_FILE, SCOPES)
            flow.user_agent = APPLICATION_NAME
            credentials = tools.run(flow, store)
        return credentials

    def _lookup_app_file(self, name='MANIFEST', role='root-manifest', pageSize=2, fields='nextPageToken, files(id, name)'):
        lookup = self._service.files().list(
            pageSize=pageSize,
            spaces='drive',
            q=("name='{}'"
               " and appProperties has {{ key='appName' and value='{}' }}"
               " and appProperties has {{ key='role' and value='{}' }}"
               ).format(name, APPLICATION_NAME, role),
            fields=fields
        ).execute()
        return lookup.get('files', [])

    def _lookup_file(self, path, pageSize=2, fields='nextPageToken, files(id, name)'):
        lookup = self._service.files().list(
            pageSize=pageSize,
            spaces='drive',
            q=("appProperties has {{ key='appName' and value='{}' }}"
               " and appProperties has {{ key='role' and value='parsec-file' }}"
               " and appProperties has {{ key='path' and value='{}' }}"
               ).format(APPLICATION_NAME, path),
            fields=fields
        ).execute()
        return lookup.get('files', [])

    def _get_parent(self, path, isFolder=False):
        # Lookup for parent folder:
        if not isFolder:
            path, _ = path.rsplit('/', 1)
        if path in (None, '/'):
            parent = self._root_folder
        else:
            parent = self._lookup_file(path=path)
            if len(parent) != 1:
                raise GoogleDriverException('File not found')
            parent = parent[0].get('id')
        return parent

    def cmd_READ_FILE(self, path):
        items = self._lookup_file(path)
        if (len(items) != 1):
            raise GoogleDriverException('File not found')
        file_content = self._service.files().get_media(fileId=items[0].get('id')).execute()

        return {'content': _content_wrap(file_content)}

    def cmd_CREATE_FILE(self, path, content=None):
        return self.cmd_WRITE_FILE(path, content=content)

    def cmd_WRITE_FILE(self, path, content=None):
        if content:
            content_io = BytesIO(content.encode())
            media = MediaIoBaseUpload(content_io,
                                      mimetype='application/binary',
                                      resumable=True)
        else:
            media = None
        items = self._lookup_file(path)
        if len(items) == 1:
            infos = self._service.files().update(
                fileId=items[0].get('id'),
                media_body=media).execute()
        else:
            parent = self._get_parent(path=path, isFolder=False)
            infos = self._service.files().create(
                body={
                    'appProperties': {
                        "appName": APPLICATION_NAME,
                        "path": path,
                        "role": 'parsec-file',
                        "mode": 'file'
                    },
                    'parents': (parent,),
                    'name': split(path)[1]
                }, media_body=media).execute()

    def cmd_DELETE_FILE(self, path):
        items = self._lookup_file(path)
        if len(items) != 1:
            raise GoogleDriverException('File not found')
        file = self._service.files().delete(fileId=items[0].get('id')).execute()

    def cmd_STAT(self, path):
        from stat import S_ISDIR, S_IFREG, S_IFDIR

        if path == '/':
            items = self._lookup_app_file(
                name=APPLICATION_NAME, role='root-folder', fields='files', pageSize=1)
        else:
            items = self._lookup_file(path=path, fields='files')
        if len(items) != 1:
            raise GoogleDriverException('File not found')
        file_info = items[0]
        data = {
            'st_size': int(file_info.get('size', '0')),
            'st_ctime': mktime(parse(file_info.get('createdTime', '')).timetuple()),
            'st_mtime': mktime(parse(file_info.get('modifiedTime', '')).timetuple()),
        }
        if file_info.get('appProperties', {}).get('mode') == 'file':
            data['st_mode'] = S_IFREG
        else:
            data['st_mode'] = S_IFDIR
            data['st_size'] = 4096

        return data

    def cmd_LIST_DIR(self, path):
        # TODO implement for more than 1000 files in dir
        path = _clean_path(path)
        parent = self._get_parent(path, isFolder=True)
        lookup = self._service.files().list(
            pageSize=1000,
            spaces='drive',
            q=("appProperties has {{ key='appName' and value='{}' }}"
               " and appProperties has {{ key='role' and value='parsec-file' }}"
               " and '{}' in parents").format(APPLICATION_NAME, parent),
            fields='files(appProperties, name)'
        ).execute()
        ret = [item.get('name') for item in lookup.get('files', [])] or []
        return {'_items': ret}

    def cmd_MAKE_DIR(self, path):
        # TODO implement for more than 1000 files in dir
        path = _clean_path(path)
        check = self._lookup_file(path=path, pageSize=1)
        if len(check):
            raise GoogleDriverException('File Already exists')
        # Lookup for parent folder:
        parent_path, folder_name = path.rsplit('/', 1)
        if not parent_path:
            parent = {'id': self._root_folder}
        else:
            parent = self._lookup_file(path=parent_path)
            if len(parent) != 1:
                raise GoogleDriverException('File not found')
            parent = parent[0]
        infos = self._service.files().create(
            body={
                "mimeType": "application/vnd.google-apps.folder",
                'appProperties': {
                    "appName": APPLICATION_NAME,
                    "path": path,
                    "role": 'parsec-file',
                    "mode": 'folder'
                },
                'parents': (parent.get('id'),),
                'name': folder_name
            }).execute()

    def cmd_dispach(self, cmd, params):

        attr_name = 'cmd_%s' % cmd.upper()
        if hasattr(self, attr_name):
            try:
                ret = getattr(self, attr_name)(**params)
            except TypeError:
                raise GoogleDriverException('Bad params for cmd `%s`' % cmd)
        else:
            raise GoogleDriverException('Unknown cmd `%s`' % cmd)

        return ret


def main(addr='tcp://127.0.0.1:5000', mock_path='/tmp'):
    context = zmq.Context()
    socket = context.socket(zmq.REP)
    socket.bind(addr)
    driver = GoogleDriver()
    try:
        driver.initialize_driver()
    except GoogleDriverException:
        driver.create_driver_files()
    finally:
        driver.initialize_driver()
    while True:
        msg = socket.recv_json()
        cmd = msg.get('cmd')
        params = msg.get('params')
        try:
            print('==>', cmd, params)
            data = driver.cmd_dispach(cmd, params)
        except GoogleDriverException as exc:
            ret = {'ok': False, 'reason': str(exc)}
        else:
            ret = {'ok': True}
            if data:
                ret['data'] = data
        print('<==', ret)
        socket.send_json(ret)

if __name__ == '__main__':
    main()
