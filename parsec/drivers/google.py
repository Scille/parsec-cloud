import httplib2
import os
from io import StringIO, BytesIO
from os.path import split
from base64 import decodebytes, encodebytes

from apiclient import discovery
from apiclient.http import MediaIoBaseUpload
import oauth2client
from oauth2client import client
from oauth2client import tools
from datetime import datetime
from time import mktime
from dateutil.parser import parse
import json
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
                                  'path': '/'},
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
            if flags:
                credentials = tools.run_flow(flow, store, flags)
            else:  # Needed only for compatibility with Python 2.6
                credentials = tools.run(flow, store)
            print('Storing credentials to ' + credential_path)
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

    def _lookup_file(self, path, pageSize=2, parent=None, fields='nextPageToken, files(id, name)'):
        if not parent:
            parent = self._root_folder
        lookup = self._service.files().list(
            pageSize=pageSize,
            spaces='drive',
            q=("appProperties has {{ key='appName' and value='{}' }}"
               " and appProperties has {{ key='role' and value='parsec-file' }}"
               " and appProperties has {{ key='path' and value='{}' }}"
               " and '{}' in parents").format(APPLICATION_NAME, path, parent),
            fields=fields
        ).execute()
        return lookup.get('files', [])

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
            infos = self._service.files().create(
                body={
                    'appProperties': {
                        "appName": APPLICATION_NAME,
                        "path": path,
                        "role": 'parsec-file'
                    },
                    'parents': (self._root_folder,),
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
            'st_size': int(file_info.get('size')),
            'st_ctime': mktime(parse(file_info.get('createdTime')).timetuple()),
            'st_mtime': mktime(parse(file_info.get('modifiedTime')).timetuple()),
        }
        if path != '/':  # ugly fix
            data['st_mode'] = S_IFREG
        else:
            data['st_mode'] = S_IFDIR
            data['st_size'] = 4096

        return data

    def cmd_LIST_DIR(self, path):
        # TODO implement for more than 1000 files in dir
        path = _clean_path(path)
        lookup = self._service.files().list(
            pageSize=1000,
            spaces='drive',
            q=("appProperties has {{ key='appName' and value='{}' }}"
               " and appProperties has {{ key='role' and value='parsec-file' }}"
               " and '{}' in parents").format(APPLICATION_NAME, self._root_folder),
            fields='files(appProperties, name)'
        ).execute()
        items = lookup.get('files', [])
        ret = []
        for item in items:
            item_path, item_name = item.get('appProperties', {}).get('path').rsplit('/', 1)
            if path == item_path:
                ret.append(item_name)
        return {'_items': ret}

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


# def main():
#     """Shows basic usage of the Google Drive API.

#     Creates a Google Drive API service object and outputs the names and IDs
#     for up to 10 files.
#     """

#     driver = GoogleDriver()
#     try:
#         driver.initialize_driver()
#     except GoogleDriverException:
#         driver.create_driver_files()
#     finally:
#         driver.initialize_driver()
#     driver.cmd_WRITE_FILE(path="/test/toto.txt", content="BITE")
#     driver.cmd_WRITE_FILE(path="/test/test2/toto2.txt", content="BITE")
#     driver.cmd_WRITE_FILE(path="/test2/toto2.txt", content="BITE")
#     driver.cmd_WRITE_FILE(path="/test2/toto3.txt", content="BITE")
#     driver.cmd_LIST_DIR(path="/test/")
#     # driver.cmd_READ_FILE(path="test/toto.txt")
#     # driver.cmd_DELETE_FILE(path="test/toto.txt")
#     # import pdb
#     # pdb.set_trace()


if __name__ == '__main__':
    main()
