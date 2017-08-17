import os
import attr
import asyncio
from tempfile import mktemp


@attr.s
class UnixSocketServer:
    socket_path = attr.ib()
    on_connection = attr.ib()
    server_task = attr.ib()
    connections_made = attr.ib()

    async def stop(self):
        # Closing a task in asyncio: simple and elegant !
        for conn in list(self.connections_made):
            conn.cancel()
            try:
                await conn
            except asyncio.CancelledError:
                pass
        self.server_task.close()
        await self.server_task.wait_closed()
        os.remove(self.socket_path)


def run_unix_socket_server(on_connection, socket_path=None, loop=None):
    loop = loop or asyncio.get_event_loop()
    socket_path = socket_path or mktemp()

    class UnixSocketServerContextManager:
        async def __aenter__(self):
            # Enjoy asyncio in it full glory: we must register connection
            # tasks to be able to cancel them when the server is closed.
            # Yes, there is like a rotten taste of C in my Python :'-(
            connections_made = set()

            def on_connection_register(reader, writer):
                coro = on_connection(reader, writer)
                fut = asyncio.ensure_future(coro)
                connections_made.add(fut)
                fut.add_done_callback(lambda res: connections_made.remove(fut))
                return fut

            server_task = await asyncio.start_unix_server(
                on_connection_register, path=socket_path, loop=loop)
            self.context = UnixSocketServer(
                socket_path, on_connection, server_task, connections_made)
            return self.context

        async def __aexit__(self, *args):
            await self.context.stop()

        async def __await__(self):
            return await self.__aenter__()

    return UnixSocketServerContextManager()


class UnixSocketApplication:
    """
    Mimic ``aiohttp.web.Application`` system, but for raw unix socket.
    """
    def __init__(self):
        self.on_connection = None
        self._on_startup = []
        self._on_shutdown = []

    @property
    def on_startup(self):
        return self._on_startup

    @property
    def on_shutdown(self):
        return self._on_shutdown

    async def startup(self):
        for callback in self._on_startup:
            await callback(self)

    async def shutdown(self):
        for callback in self._on_shutdown:
            await callback(self)


def run_app(app, path, loop=None):
    if not app.on_connection:
        raise RuntimeError('`app.on_connection` must be configured before running the app')
    loop = loop or asyncio.get_event_loop()
    server = loop.run_until_complete(
        run_unix_socket_server(app.on_connection, path))
    try:
        loop.run_until_complete(app.startup())
        print('======== Running on unix socket %s ========' % path)
        print('(Press CTRL+C to quit)')
        loop.run_forever()
    except KeyboardInterrupt:
        pass
    finally:
        loop.run_until_complete(app.shutdown())
        loop.run_until_complete(server.stop())
        loop.close()