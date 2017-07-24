import asyncio
import attr
from effect2 import ComposedDispatcher, asyncio_perform, Effect

from parsec.base import base_dispatcher, EEvent
from parsec.backend.server import run_websocket_server
from parsec.backend.client_connection import on_connection_factory
from parsec.backend.backend_api import execute_raw_cmd


def app_factory(*additional_dispatchers):
    app = App()
    dispatcher = ComposedDispatcher([base_dispatcher] + list(additional_dispatchers))
    on_connection = on_connection_factory(execute_raw_cmd, dispatcher)
    app.on_connection = on_connection
    app.dispatcher = dispatcher
    return app


def run_app(host, port, app=None, loop=None):
    app = app or app_factory()
    loop = loop or asyncio.get_event_loop()
    server = loop.run_until_complete(run_websocket_server(app.on_connection, host, port, loop=loop))
    try:
        loop.run_until_complete(app.async_perform(Effect(EEvent('app_start', app))))
        loop.run_forever()
    except KeyboardInterrupt:
        pass
    finally:
        loop.run_until_complete(app.async_perform(Effect(EEvent('app_stop', app))))
        loop.run_until_complete(server.stop())
        loop.close()


@attr.s
class App:
    async def async_perform(self, intent):
        return await asyncio_perform(self.dispatcher, intent)
