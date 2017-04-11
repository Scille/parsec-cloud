import sys

from logbook import Logger, StreamHandler

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError


LOG_FORMAT = '[{record.time:%Y-%m-%d %H:%M:%S.%f%z}] ({record.thread_name})' \
             ' {record.level_name}: {record.channel}: {record.message}'
log = Logger('Parsec-File-Service')
StreamHandler(sys.stdout, format_string=LOG_FORMAT).push_application()


class BackendAPIError(ParsecError):
    pass


class BaseBackendAPIService(BaseService):

    gnupg_pub_keys_service = service('GNUPGPubKeysService')
    named_vlob_service = service('MockedNamedVlobService')
    vlob_service = service('MockedVlobService')
    block_service = service('MockedBlockService')

    @cmd('send_message')
    async def _cmd_SEND_MESSAGE(self, session, msg):
        return await self.send_cmd(**msg)


class BackendAPIService(BaseBackendAPIService):

    def __init__(self, backend_host, backend_port):
        super().__init__()

    async def send_cmd(self, msg):
        pass
