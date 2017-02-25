import os
import sys
import asyncio
from uuid import uuid4
from base64 import b64decode, b64encode
from logbook import Logger, StreamHandler


StreamHandler(sys.stdout).push_application()
log = Logger('Parsec')


class Response:
    @classmethod
    def from_raw(cls, data: bytes):
        assert isinstance(data, bytes)
        args = data.split(b' ', maxsplit=1)
        if len(args) == 2:
            return cls(status=args[0].decode(), raw_body=args[1])
        else:
            return cls(status=args[0].decode())

    def __init__(self, status: str='ok', body: bytes=None, raw_body: bytes=None):
        assert raw_body is None or isinstance(raw_body, bytes)
        assert isinstance(status, str)
        if isinstance(body, str):
            body = body.encode()
        self.status = status
        self._body = body
        self._raw_body = raw_body or b''

    @property
    def body(self):
        if self._body is None:
            self._body = b64decode(self._raw_body)
        return self._body

    @property
    def is_ok(self):
        return self.status == 'ok'

    def pack(self) -> bytes:
        return self.status.encode() + b' ' + b64encode(self.body)

    def __repr__(self):
        return '<Response(status=%s, body=%s)>' % (self.status, self.body)


class Request:
    @classmethod
    def from_raw(cls, data: bytes):
        assert isinstance(data, bytes)
        args = data.split(b' ', maxsplit=1)
        if len(args) == 2:
            return cls(args[0].decode(), raw_body=args[1])
        else:
            return cls(args[0].decode())

    def __init__(self, cmdid: str, body: bytes=None, raw_body: bytes=None):
        assert raw_body is None or isinstance(raw_body, bytes)
        assert isinstance(cmdid, str)
        if isinstance(body, str):
            body = body.encode()
        self.cmdid = cmdid
        self._body = body
        self._raw_body = raw_body or b''

    @property
    def body(self):
        if self._body is None:
            self._body = b64decode(self._raw_body)
        return self._body

    def pack(self) -> bytes:
        return self.cmdid.encode() + b' ' + b64encode(self.body)

    def __repr__(self):
        return '<Request(cmdid=%s, body=%s)>' % (self.cmdid, self.body)


class ParsecServer:
    def __init__(self):
        self._cmds = {}

    def register_cmd(self, cmd, cb):
        if cmd in self._cmds:
            raise RuntimeError('Command `%s` already registered.' % cmd)
        self._cmds[cmd] = cb

    @staticmethod
    def _parse_raw_msg(msg):
        return msg.split(maxsplit=1)

    async def on_connection(self, reader, writer):
        conn_log = Logger('Connection ' + uuid4().hex)
        conn_log.debug('Connection started')
        while True:
            raw_req = await reader.readline()
            if not raw_req:
                conn_log.debug('Connection stopped')
                return
            req = Request.from_raw(raw_req)
            conn_log.debug('Received: %r' % req)
            cmd = self._cmds.get(req.cmdid)
            if not cmd:
                resp = Response('badcmd', 'Unknown command `%s`' % req.cmdid)
            else:
                resp = await cmd(req)
            conn_log.debug('Replied: %r' % resp)
            writer.write(resp.pack())
            writer.write(b'\n')


def start_server(socket_path):
    loop = asyncio.get_event_loop()
    server = ParsecServer()
    try:
        connect_coro = asyncio.start_unix_server(
            server.on_connection, path=socket_path, loop=loop)
        loop.create_task(connect_coro)
        loop.run_forever()
    finally:
        loop.close()
        os.remove(socket_path)


if __name__ == '__main__':
    start_server()
