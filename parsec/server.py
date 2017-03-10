import os
import sys
import asyncio
import json
from uuid import uuid4
from logbook import Logger, StreamHandler

from parsec.vfs import VFSServiceInMemoryMock
from parsec.file_service import FileService
from parsec.user_manifest_service import UserManifestService
from parsec.base import ParsecError


StreamHandler(sys.stdout).push_application()
log = Logger('Parsec')


class ParsecServer:
    def __init__(self):
        self._cmds = {
            'list_cmds': self.__cmd_LIST_CMDS
        }

    async def __cmd_LIST_CMDS(self, data):
        return {'status': 'ok', 'cmds': list(self._cmds.keys())}

    def register_service(self, service, name=None):
        name = name or type(service).__name__
        for cmdid, cb in service.get_cmds().items():
            print(name, cmdid, cb)
            self.register_cmd('%s:%s' % (name, cmdid), cb)

    def register_cmd(self, cmd, cb):
        if cmd in self._cmds:
            raise RuntimeError('Command `%s` already registered.' % cmd)
        self._cmds[cmd] = cb

    @staticmethod
    def _parse_raw_msg(raw):
        if not raw:
            return None
        try:
            msg = json.loads(raw.decode())
            if isinstance(msg.get('cmd'), str):
                return msg
            else:
                return None
        except json.decoder.JSONDecodeError:
            pass
        # Not a JSON payload, try cmdline mode
        splitted = raw.decode().strip().split(' ')
        cmd = splitted[0]
        raw_msg = '{"cmd": "%s"' % cmd
        for data in splitted[1:]:
            if '=' not in data:
                return None
            raw_msg += ', "%s": %s' % tuple(data.split('=', maxsplit=1))
        raw_msg += '}'
        try:
            return json.loads(raw_msg)
        except json.decoder.JSONDecodeError:
            pass
        # Nothing worked :'-(
        return None

    async def on_connection(self, reader, writer):
        conn_log = Logger('Connection ' + uuid4().hex)
        conn_log.debug('Connection started')
        while True:
            raw_req = await reader.readline()
            if not raw_req:
                conn_log.debug('Connection stopped')
                return
            conn_log.debug('Received: %r' % raw_req)
            msg = self._parse_raw_msg(raw_req[:-1])
            if msg is None:
                resp = {"status": "bad_message", "label": "Message is not a valid JSON."}
            else:
                cmd = self._cmds.get(msg['cmd'])
                if not cmd:
                    resp = {"status": "badcmd", "label": "Unknown command `%s`" % msg['cmd']}
                else:
                    try:
                        resp = await cmd(msg)
                    except ParsecError as exc:
                        resp = exc.to_dict()
            conn_log.debug('Replied: %r' % resp)
            writer.write(json.dumps(resp).encode())
            writer.write(b'\n')


def start_server(socket_path: str):
    loop = asyncio.get_event_loop()
    server = ParsecServer()
    server.register_service(VFSServiceInMemoryMock(), 'vfs')
    server.register_service(UserManifestService(), 'user_manifest')
    server.register_service(FileService(), 'file')
    try:
        connect_coro = asyncio.start_unix_server(
            server.on_connection, path=socket_path, loop=loop)
        loop.run_until_complete(connect_coro)
        loop.run_forever()
    finally:
        loop.close()
        os.remove(socket_path)
