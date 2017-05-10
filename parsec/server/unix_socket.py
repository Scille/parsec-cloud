import os
import asyncio

from parsec.server.base import BaseClientContext, BaseServer


class UnixSocketClientContext(BaseClientContext):
    def __init__(self, reader, writer):
        super().__init__()
        self.reader = reader
        self.writer = writer
        self._buffer = b''

    async def recv(self):
        while True:
            buf = await self.reader.read(65536)
            if not buf:
                buf = self._buffer
                self._buffer = b''
                return buf
            self._buffer += buf
            # raw_line is raw cmd and a trailing '\n'
            msgs = self._buffer.split(b'\n', 1)
            if len(msgs) == 2:
                # If message is followed by more data, keep it for the next processing
                self._buffer = msgs[1]
                return msgs[0]

    async def send(self, body):
        self.writer.write(body)
        self.writer.write(b'\n')


class UnixSocketServer(BaseServer):

    async def on_connection(self, reader, writer):
        context = UnixSocketClientContext(reader, writer)
        await super().on_connection(context)

    def start(self, socket_path: str, loop=None, block=True):
        loop = loop or asyncio.get_event_loop()
        start_server = asyncio.start_unix_server(
            self.on_connection, path=socket_path, loop=loop)

        if not block:
            async def boostrap():
                await self.bootstrap_services()
                return await start_server

            return asyncio.ensure_future(boostrap(), loop=loop)
        else:
            try:
                loop.run_until_complete(self.bootstrap_services())
                server = loop.run_until_complete(start_server)
                loop.run_forever()
            except KeyboardInterrupt:
                loop.run_until_complete(self.teardown_services())
                server.close()
                loop.run_until_complete(server.wait_closed())
            finally:
                loop.close()
                os.remove(socket_path)
