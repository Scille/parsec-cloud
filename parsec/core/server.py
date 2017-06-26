import os
import asyncio
import attr
from tempfile import mktemp


@attr.s
class UnixSocketServer:
    socket_path = attr.ib()
    on_connection = attr.ib()
    server_task = attr.ib()

    async def stop(self):
        self.server_task.close()
        await self.server_task.wait_closed()
        os.remove(self.socket_path)


def run_unix_socket_server(on_connection, socket_path=None):
    socket_path = socket_path or mktemp()

    class UnixSocketServerContextManager:
        async def __aenter__(self):
            server_task = await asyncio.start_unix_server(
                on_connection, path=socket_path)
            self.context = UnixSocketServer(socket_path, on_connection, server_task)
            return self.context

        async def __aexit__(self, *args):
            await self.context.stop()

        async def __await__(self):
            return await self.__aenter__()

    return UnixSocketServerContextManager()
