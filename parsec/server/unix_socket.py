import os
import asyncio

from parsec.server.base import BaseClientContext, BaseServer


class UnixSocketClientContext(BaseClientContext):
    def __init__(self, reader, writer):
        super().__init__()
        self.reader = reader
        self.writer = writer

    async def recv(self):
        raw_line = b''
        while True:
            raw_line += await self.reader.read(65536)
            if not raw_line or raw_line[-1] == ord(b'\n'):
                break
        if not raw_line:
            return
        else:
            # raw_line is raw cmd and a trailing '\n'
            return raw_line[:-1]

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
