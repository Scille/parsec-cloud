from marshmallow import fields

from parsec.service import BaseService, service
from parsec.core2.file_api import FileAPIMixin
from parsec.core2.folder_api import FolderAPIMixin


class CoreService(BaseService, FileAPIMixin, FolderAPIMixin):

    name = 'CoreService'

    identity = service('IdentityService')
    backend = service('BackendAPIService')
    block = service('BlockService')
