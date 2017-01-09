import httplib2
import os
from io import BytesIO
from apiclient import discovery
from apiclient.http import MediaIoBaseUpload
from oauth2client import client, tools
from oauth2client.file import Storage
from json import loads, dumps
from google.protobuf.message import DecodeError
from ..abstract import BaseService
from .volume_pb2 import Request, Response


# If modifying these scopes, delete your previously saved credentials
# at ~/.credentials/drive-python-quickstart.json
SCOPES = 'https://www.googleapis.com/auth/drive'
CLIENT_SECRET_FILE = 'secret.json'
APPLICATION_NAME = 'Parsec'


class CmdError(Exception):

    def __init__(self, error_msg, status_code=Response.BAD_REQUEST):
        self.error_msg = error_msg
        self.status_code = status_code


class GoogleDriverException(CmdError):
    pass


class GoogleDriveVolumeService(BaseService):

    def __init__(self):
        self._initialized = False
        self._credentials = self._get_credentials()
        self._http = self._credentials.authorize(httplib2.Http())
        self._service = discovery.build('drive', 'v3', http=self._http)

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

        store = Storage(credential_path)
        credentials = store.get()
        flags = tools.argparser.parse_args(args=[])
        if not credentials or credentials.invalid:
            flow = client.flow_from_clientsecrets(CLIENT_SECRET_FILE, SCOPES)
            flow.user_agent = APPLICATION_NAME
            credentials = tools.run_flow(flow, store, flags)
        return credentials

    def initialized(self):
        """ Check if the driver is initialized.
            Returns:
                Boolean
            Raises:
                None
            """
        return self._initialized

    def _initialize_driver(self):
        """ Looks up Parsec system files one the cloud and load their ID into
        the Driver class instance. This Function also loads the mapping between
        virtual IDs used by the VFS and the Physical IDs used on the cloud side.
        Sets the initialized attribute to the according boolean value.
        WARNING: You MUST not use the driver if not successfully initialized.
            Returns:
                None
            Raises:
                GoogleDriverException if a system file is not found
        """
        if self._initialized:
            return
        # (name, role, attr_name, function to call on result)
        system_files = (('Parsec', 'root-folder', '_root_folder', None),
                        # ('MANIFEST', 'root-manifest', '_root_manifest', None),
                        ('MAP', 'mapping-file', '_mapping_file', self._load_mapping))

        for (name, role, attr_name, func) in system_files:
            items = self._lookup_app_file(name=name, role=role)
            if len(items) != 1:
                raise GoogleDriverException('Drive Directory is broken, please check.')
            setattr(self, attr_name, items[0].get('id'))
            if getattr(self, attr_name, None) is None:
                raise GoogleDriverException('Cannot locate %s' % role)
            if func:
                func()

        self._initialized = True

    def initialize_driver(self, force=False):
        try:
            self._initialize_driver()
        except GoogleDriverException:
            if force:
                self.create_driver_files()
        finally:
            self._initialize_driver()

    def create_driver_files(self):
        """Initialize Driver Environnement in Google Drive. Creates all required system files
        used by Parsec.
            Returns:
                None
            Raises:
                GoogleDriverException if a system file is not found
        """
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

        # Create the PID/VID mapping file
        results = self._service.files().create(
            body={
                "mimeType": "application/scille.parsec.google.mapping",
                "isAppAuthorized": True,
                "appProperties": {'appName': '%s' % APPLICATION_NAME,
                                  'role': "mapping-file"},
                "parents": (self._root_folder,),
                "name": "MAP",
                'mode': 'file',
            }
        ).execute()
        self._mapping_file = results.get('id')
        self._mapping = {}
        if not self._mapping_file:
            raise GoogleDriverException('Failed to initialise map table')
        self._initialized = True

    def _lookup_app_file(self, name='MANIFEST', role='root-manifest', pageSize=2,
                         fields='nextPageToken, files(id, name)'):
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

    def _load_mapping(self):
        """Opens the content of the MAP system file stored on the drive side and loads it
        into memory. This file contains the mapping between VID used by VFS and PID used
        by Google Drive
            Returns:
                None
            Raises:
                None
        """
        mapping_content = self._service.files().get_media(fileId=self._mapping_file).execute()
        try:
            self._mapping = loads(mapping_content.decode())
        except ValueError:
            self._mapping = {}

    def _save_mapping(self):
        """Saves the content of self._mapping into the MAP system file stored on the
        drive side.
            Returns:
                None
            Raises:
                None
        """
        content_io = BytesIO(dumps(self._mapping).encode())
        media = MediaIoBaseUpload(content_io,
                                  mimetype='application/binary',
                                  resumable=True)
        self._service.files().update(
            fileId=self._mapping_file,
            media_body=media).execute()

    def sync(self):
        self._save_mapping()

    def _read_file(self, msg):
        """ Get the content of a file stored on the Google Drive.
            Returns:
                Content of the file as a str
            Raises:
                GoogleDriverException() if file is not in mapping
        """
        file_id = self._mapping.get(msg.vid)
        if file_id is None:
            return Response(status_code=Response.FILE_NOT_FOUND)
        file_content = self._service.files().get_media(fileId=file_id).execute()
        return Response(status_code=Response.OK, content=file_content)

    def _write_file(self, msg):
        """ Creates or writes a file on the Google Drive.
            self._mapping is updated only in case of a file creation.
            @content may be empty.
            Returns:
                None # Should return a status of the write function...
            Raises:
                GoogleDriverException() if not vid is provided by the VFS
        """
        vid = msg.vid
        content = msg.content
        if vid is None:
            raise GoogleDriverException('A VID is mandatory')

        media = None
        if content:
            content_io = BytesIO(content)
            media = MediaIoBaseUpload(content_io,
                                      mimetype='application/binary',
                                      resumable=True)
        file_id = self._mapping.get(vid)
        if file_id is None:
            infos = self._service.files().create(
                body={
                    'appProperties': {
                        "appName": APPLICATION_NAME,
                        "role": 'parsec-file',
                        "mode": 'file'
                    },
                    'parents': (self._root_folder,),
                },
                media_body=media,
                fields='id').execute()
            self._mapping[vid] = infos['id']
        else:
            self._service.files().update(
                fileId=file_id,
                media_body=media).execute()
        return Response(status_code=Response.OK)

    def _delete_file(self, msg):
        """ Deletes file on the cloud storage. if vis does not exists,
        the function does nothing.
            Returns:
                None # Should return a status of the delete call...
            Raises:
                None
        """
        file_id = self._mapping.get(msg.vid)
        if file_id is not None:
            self._service.files().delete(fileId=file_id).execute()
            del self._mapping[msg.vid]
            return Response(status_code=Response.OK)
        return Response(status_code=Response.FILE_NOT_FOUND)

    def dispatch_msg(self, msg):
        try:
            if msg.type == Request.READ_FILE:
                return self._read_file(msg)
            elif msg.type == Request.WRITE_FILE:
                return self._write_file(msg)
            elif msg.type == Request.DELETE_FILE:
                return self._delete_file(msg)
            else:
                raise CmdError('Unknown msg `%s`' % msg.type)
        except CmdError as exc:
            return Response(status_code=exc.status_code, error_msg=exc.error_msg)

    def dispatch_raw_msg(self, raw_msg):
        try:
            msg = Request()
            msg.ParseFromString(raw_msg)
            ret = self.dispatch_msg(msg)
        except DecodeError as exc:
            ret = Response(status_code=Response.BAD_REQUEST, error_msg='Invalid request format')
        return ret.SerializeToString()
