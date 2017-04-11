import sys

from logbook import Logger, StreamHandler

from parsec.backend import MockedVlobService, MockedNamedVlobService, MockedBlockService
from parsec.service import BaseService
from parsec.exceptions import ParsecError


LOG_FORMAT = '[{record.time:%Y-%m-%d %H:%M:%S.%f%z}] ({record.thread_name})' \
             ' {record.level_name}: {record.channel}: {record.message}'
log = Logger('Parsec-File-Service')
StreamHandler(sys.stdout, format_string=LOG_FORMAT).push_application()


class BackendAPIError(ParsecError):
    pass


class BackendAPIService(BaseService):

    def __init__(self, backend_host, backend_port):
        super().__init__()
        self.named_vlob_service = MockedNamedVlobService()
        self.vlob_service = MockedVlobService()
        self.block_service = MockedBlockService()

    async def send_cmd(self, msg):
        pass
