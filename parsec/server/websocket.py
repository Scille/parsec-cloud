import asyncio
import websockets

from parsec.server.base import BaseServer


class WebSocketServer(BaseServer):

    async def on_connection(self, websocket, path):
        await super().on_connection(websocket)

    def start(self, domain: str='localhost', port: int=677, loop=None):
        self.bootstrap_services()
        loop = loop or asyncio.get_event_loop()
        try:
            start_server = websockets.serve(self.on_connection, domain, port)
            loop.run_until_complete(start_server)
            loop.run_forever()
        except KeyboardInterrupt:
            self.teardown_services(loop)
        finally:
            loop.close()
