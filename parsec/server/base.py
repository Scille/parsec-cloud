import re
import json
import asyncio
import random
import string
from marshmallow import fields
from marshmallow.validate import OneOf
from logbook import Logger
from effect2 import base_dispatcher, ComposedDispatcher

from parsec.exceptions import ParsecError
from parsec.session import anonymous_handshake, ConnectionClosed
from parsec.tools import BaseCmdSchema, json_dumps


class BaseClientContext:

    async def recv(self):
        raise NotImplementedError()

    async def send(self, body):
        raise NotImplementedError()


def _unique_enough_id():
    # Colision risk is high, but this is pretty fine (and much more readable
    # than a uuid4) for giving id to connections
    return ''.join([random.choice(string.ascii_letters + string.digits) for ch in range(4)])


class BaseServer:
    def __init__(self, handshake=anonymous_handshake):
        self._cmds = {
            'list_cmds': self.__cmd_LIST_CMDS,
            'list_events': self.__cmd_LIST_EVENTS,
            'subscribe': self.__cmd_SUBSCRIBE,
            'unsubscribe': self.__cmd_UNSUBSCRIBE
        }
        self._post_bootstrap_cbs = []
        self._events = {}
        self._services = {}
        self._handshake = handshake

    async def __cmd_LIST_CMDS(self, session, msg):
        return {'status': 'ok', 'cmds': sorted(self._cmds.keys())}

    async def __cmd_LIST_EVENTS(self, session, msg):
        return {'status': 'ok', 'events': sorted(self._events.keys())}

    async def __cmd_UNSUBSCRIBE(self, session, msg):

        class cmd_UNSUBSCRIBE_Schema(BaseCmdSchema):
            event = fields.String(required=True, validate=OneOf(self._events.keys()))
            sender = fields.String(
                required=True, validate=lambda x: re.match(r'[A-Za-z0-9]{0,32}', x))

        msg = cmd_UNSUBSCRIBE_Schema().load(msg)
        try:
            delattr(session, '_cb_{event}_{sender}'.format(**msg))
        except AttributeError:
            # TODO send error if event/sender wasn't subscribed ?
            pass
        return {'status': 'ok'}

    async def __cmd_SUBSCRIBE(self, session, msg):

        class cmd_SUBSCRIBE_Schema(BaseCmdSchema):
            event = fields.String(required=True, validate=OneOf(self._events.keys()))
            sender = fields.String(
                required=True, validate=lambda x: re.match(r'[A-Za-z0-9]{0,32}', x))

        msg = cmd_SUBSCRIBE_Schema().load(msg)

        event = msg['event']

        def on_event(sender):
            session.received_events.put_nowait((event, sender))

        # Attach the callback to the session to make them have the same
        # lifetime given event registration expires when callback is destroyed
        setattr(session, '_cb_%s_%s' % (event, msg['sender']), on_event)
        self._events[event].connect(on_event, sender=msg['sender'])
        return {'status': 'ok'}

    def register_service(self, service):
        self._services[service.name] = service
        for cmd_name, cb in service.cmds.items():
            self.register_cmd(cmd_name, cb)
        for event_name, event in service.events.items():
            self.register_event(event_name, event)

    def register_cmd(self, name, cb):
        if name in self._cmds:
            raise RuntimeError('Command `%s` already registered.' % name)
        self._cmds[name] = cb

    def register_event(self, name, event):
        if name in self._events:
            raise RuntimeError('Event `%s` already registered.' % name)
        self._events[name] = event

    @staticmethod
    def _load_raw_cmd(raw):
        if not raw:
            return None
        try:
            if isinstance(raw, bytes):
                raw = raw.decode()
            msg = json.loads(raw)
            if isinstance(msg.get('cmd'), str):
                return msg
            else:
                return None
        except json.decoder.JSONDecodeError:
            pass
        # Not a JSON payload, try cmdline mode
        splitted = raw.strip().split(' ')
        cmd = splitted[0]
        raw_msg = '{"cmd": "%s"' % cmd
        for data in splitted[1:]:
            if '=' not in data:
                return None
            raw_msg += ', "%s": %s' % tuple(data.split('=', maxsplit=1))
        raw_msg += '}'
        try:
            return json.loads(raw_msg)
        except json.decoder.JSONDecodeError:
            pass
        # Nothing worked :'-(
        return None

    async def on_connection(self, context: BaseClientContext):
        conn_log = Logger('Connection ' + _unique_enough_id())
        conn_log.info('Connection started')
        # Handle handshake if auth is required
        session = await self._handshake(context)
        if not session:
            conn_log.info('Connection closed due to bad handshake')
            return
        get_event = asyncio.ensure_future(session.received_events.get())
        get_cmd = asyncio.ensure_future(context.recv())
        try:
            while True:
                # Wait for two things:
                # - User's command
                # - Event subscribed by user
                # Note user's command should have been replied before sending an event notification
                done, pending = await asyncio.wait((get_event, get_cmd),
                                                   return_when='FIRST_COMPLETED')
                if get_event in done:
                    event, sender = get_event.result()
                    conn_log.debug('Got event: %s@%s' % (event, sender))
                    resp = {'event': event, 'sender': sender}
                    await context.send(json_dumps(resp).encode())
                    # Restart watch on incoming notifications
                    get_event = asyncio.ensure_future(session.received_events.get())
                else:
                    raw_cmd = get_cmd.result()
                    if not raw_cmd:
                        get_event.cancel()
                        conn_log.debug('Connection stopped')
                        return
                    conn_log.debug('Received: %r' % raw_cmd)
                    msg = self._load_raw_cmd(raw_cmd)
                    request_id = None
                    if msg is None:
                        resp = {'status': 'bad_message', 'label': 'Message is not a valid JSON.'}
                    else:
                        if 'request_id' in msg:
                            request_id = msg['request_id']
                            del msg['request_id']
                        cmd = self._cmds.get(msg['cmd'])
                        if not cmd:
                            resp = {'status': 'badcmd',
                                    'label': 'Unknown command `%s`' % msg['cmd']}
                        else:
                            try:
                                resp = await cmd(session, msg)
                            except ParsecError as exc:
                                resp = exc.to_dict()
                        if request_id:
                            resp['request_id'] = request_id
                    conn_log.debug('Replied: %r' % resp)
                    await context.send(json_dumps(resp).encode())
                    # Restart watch on incoming messages
                    get_cmd = asyncio.ensure_future(context.recv())
        except ConnectionClosed:
            conn_log.info('Connection closed')
        finally:
            get_event.cancel()
            get_cmd.cancel()

    def post_bootstrap(self, callback):
        """"
        Decorator registering an async callback to run after boostrap but
        before server is started.
        """
        self._post_bootstrap_cbs.append(callback)
        return callback

    async def bootstrap_services(self):
        # TODO: clean this hack
        global_dispatchers = ComposedDispatcher([base_dispatcher])
        errors = []
        for service in self._services.values():
            if service.dispatcher:
                global_dispatchers.dispatchers.append(service.dispatcher)
                service.dispatcher = global_dispatchers
            try:
                boot = service.inject_services()
                dep = next(boot)
                while True:
                    if dep not in self._services:
                        errors.append(
                            'Service `%s` required unknown service `%s`' % (service.name, dep))
                        break
                    dep = boot.send(self._services[dep])
            except StopIteration:
                pass
        if errors:
            raise RuntimeError(errors)
        await asyncio.wait([s.bootstrap() for s in self._services.values()])
        if self._post_bootstrap_cbs:
            await asyncio.wait([cb() for cb in self._post_bootstrap_cbs])

    async def teardown_services(self):
        for service in self._services.values():
            await service.teardown()

    def start(self):
        raise NotImplementedError()
