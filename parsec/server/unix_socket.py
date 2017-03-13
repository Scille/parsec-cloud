import os
import asyncio

from parsec.server.base import BaseClientContext, BaseServer


class UnixSocketClientContext(BaseClientContext):
    def __init__(self, reader, writer):
        self.reader = reader
        self.writer = writer

    async def recv(self):
        raw_line = await self.reader.readline()
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

    def start(self, socket_path: str, loop=None):
        loop = loop or asyncio.get_event_loop()
        try:
            connect_coro = asyncio.start_unix_server(
                self.on_connection, path=socket_path, loop=loop)
            loop.run_until_complete(connect_coro)
            loop.run_forever()
        finally:
            loop.close()
            os.remove(socket_path)
