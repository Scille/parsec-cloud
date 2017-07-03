import attr
import random
import string
import asyncio
from logbook import Logger
from blinker import signal
from websockets import ConnectionClosed
from effect2 import TypeDispatcher, ComposedDispatcher, asyncio_perform

from parsec.tools import ejson_dumps
from parsec.core.base import base_dispatcher


@attr.s
class EPushClientMsg:
    payload = attr.ib()


@attr.s
class EClientSubscribeEvent:
    event = attr.ib()
    sender = attr.ib()


def _unique_enough_id():
    # Colision risk is high, but this is pretty fine (and much more readable
    # than a uuid4) for giving id to connections
    return ''.join([random.choice(string.ascii_letters + string.digits) for ch in range(4)])


@attr.s
class ClientConnectionContext:
    reader = attr.ib()
    writer = attr.ib()
    queued_pushed_events = attr.ib(default=attr.Factory(asyncio.Queue))
    logger = attr.ib(default=attr.Factory(
        lambda: Logger('Connection ' + _unique_enough_id())))
    _buffer = attr.ib(default=b'', init=False)
    subscribed_events = attr.ib(default=attr.Factory(dict))

    async def recv(self):
        while True:
            if b'\n' in self._buffer:
                buf, self._buffer = self._buffer.split(b'\n', 1)
                return buf
            buf = await self.reader.read(65536)
            if not buf and not self._buffer:
                # Socket has reach EOF
                return buf
            self._buffer += buf

    async def send(self, body):
        if isinstance(body, str):
            body = body.encode()
        try:
            self.writer.write(body)
            self.writer.write(b'\n')
        except BrokenPipeError:
            raise ConnectionClosed()


def client_dispatcher_factory(client_context):
    def perform_push_client_msg(intent):
        client_context.queued_pushed_events.put_nowait(intent.payload)

    def perform_client_subscribe_event(intent):
        key = (intent.event, intent.sender)

        def on_event(sender):
            payload = ejson_dumps({'event': intent.event, 'sender': sender})
            client_context.queued_pushed_events.put_nowait(payload)

        # Attach the callbacks to the client context to make them have the same
        # lifetime given event registration expires when callback is destroyed
        # TODO: allow a subset of the possible events
        client_context.subscribed_events[key] = on_event
        signal(intent.event).connect(on_event, sender=intent.sender)

    return TypeDispatcher({
        EPushClientMsg: perform_push_client_msg,
        EClientSubscribeEvent: perform_client_subscribe_event
    })


def on_connection_factory(execute_cmd, base_dispatcher=base_dispatcher):

    async def on_connection(reader, writer):
        context = ClientConnectionContext(reader, writer)
        client_dispatcher = client_dispatcher_factory(context)
        dispatcher = ComposedDispatcher([base_dispatcher, client_dispatcher])
        context.logger.info('Connection started')
        # Wait for two things:
        # - User's command (incomming request)
        # - Event subscribed by user (pushed to client requests)
        # Note user's command should have been replied before sending an event notification
        get_event = asyncio.ensure_future(context.queued_pushed_events.get())
        get_cmd = asyncio.ensure_future(context.recv())
        try:
            while True:
                done, pending = await asyncio.wait((get_event, get_cmd),
                                                   return_when='FIRST_COMPLETED')
                if get_event in done:
                    payload = get_event.result()
                    context.logger.debug('Got event: %s' % payload)
                    await context.send(payload)
                    # Restart watch on incoming notifications
                    get_event = asyncio.ensure_future(context.queued_pushed_events.get())
                else:
                    raw_cmd = get_cmd.result()
                    if not raw_cmd:
                        context.logger.debug('Connection stopped')
                        return
                    context.logger.debug('Received: %r' % raw_cmd)
                    intent = execute_cmd(raw_cmd)
                    raw_resp = await asyncio_perform(dispatcher, intent)
                    context.logger.debug('Replied: %r' % raw_resp)
                    await context.send(raw_resp)
                    # Restart watch on incoming messages
                    get_cmd = asyncio.ensure_future(context.recv())
        except ConnectionClosed:
            context.logger.info('Connection closed')
        finally:
            get_event.cancel()
            get_cmd.cancel()

    return on_connection
