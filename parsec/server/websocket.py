import asyncio
import websockets

from parsec.server.base import BaseServer


class WebSocketServer(BaseServer):

    async def on_connection(self, websocket, path):
        await super().on_connection(websocket)

    def start(self, domain: str='localhost', port: int=6777, loop=None, block=True):
        loop = loop or asyncio.get_event_loop()
        start_server = websockets.serve(self.on_connection, domain, port, loop=loop)

        if not block:
            async def boostrap():
                await self.bootstrap_services()
                return await start_server

            return asyncio.ensure_future(boostrap(), loop=loop)
        else:
            loop.run_until_complete(self.bootstrap_services())
            try:
                server = loop.run_until_complete(start_server)
                loop.run_forever()
            except KeyboardInterrupt:
                loop.run_until_complete(self.teardown_services())
                server.close()
                loop.run_until_complete(server.wait_closed())
            finally:
                loop.close()
