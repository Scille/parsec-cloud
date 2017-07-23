import asyncio
import websockets
import attr


@attr.s
class WebSocketServer:
    host = attr.ib()
    port = attr.ib()
    on_connection = attr.ib()
    server_task = attr.ib()

    async def stop(self):
        self.server_task.close()
        await self.server_task.wait_closed()


def run_websocket_server(on_connection, host: str='localhost', port: int=6777, loop=None):
    loop = loop or asyncio.get_event_loop()

    class WebSocketServerContextManager:
        async def __aenter__(self):
            server_task = await websockets.serve(on_connection, host, port, loop=loop)
            self.context = WebSocketServer(host, port, on_connection, server_task)
            return self.context

        async def __aexit__(self, *args):
            await self.context.stop()

        async def __await__(self):
            return await self.__aenter__()

    return WebSocketServerContextManager()
