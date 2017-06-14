from marshmallow import fields

from parsec.service import BaseService, service
from parsec.core2.fs_api import MockedFSAPIMixin
from parsec.core2.fs_api_impl import FSAPIMixin


def core_service_factory(mixins):
    nmspc = {
        'name': 'CoreService',
        'identity': service('IdentityService'),
        'backend': service('BackendAPIService'),
        'block': service('BlockService'),
    }
    return type('CoreService', (BaseService, ) + tuple(mixins), nmspc)


CoreService = core_service_factory([FSAPIMixin])
MockedCoreService = core_service_factory([MockedFSAPIMixin])
