import asyncio
from blinker import signal

from parsec.exceptions import ParsecError
from parsec.session import AnonymousSession


class CmdWrap:
    def __init__(self, name, callback):
        self.name = name
        self.callback = callback

    def __call__(self, *args, **kwargs):
        return self.callback(*args, **kwargs)


class EventWrap:
    def __init__(self, name):
        self.name = name
        self.event = signal(name)


class ServiceWrap:
    def __init__(self, name):
        self.name = name


def event(name):
    return EventWrap(name)


def service(name):
    return ServiceWrap(name)


def cmd(param):
    if callable(param):
        return CmdWrap(param.__name__, param)
    else:
        return lambda fn: CmdWrap(param, fn)


class MetaBaseService(type):

    def __new__(cls, name, bases, nmspc):
        cmd_keys = {}
        events = {}
        service_keys = {}
        cooked_nmspc = {'_cmd_keys': cmd_keys, '_events': events, '_service_keys': service_keys}

        def scan_nmspc(nmspc):
            for k, v in nmspc.items():
                if isinstance(v, CmdWrap):
                    cmd_keys[k] = v.name
                    cooked_nmspc[k] = v.callback
                elif isinstance(v, EventWrap):
                    events[v.name] = v.event
                    cooked_nmspc[k] = v.event
                elif isinstance(v, ServiceWrap):
                    service_keys[k] = v.name
                    cooked_nmspc[k] = None
                else:
                    cooked_nmspc[k] = v

        # Retrieve parents' cmds&events
        for b in bases:
            if issubclass(b, (BaseService, ServiceMixin)):
                cmd_keys.update(getattr(b, '_cmd_keys', {}))
                events.update(getattr(b, '_events', {}))
                service_keys.update(getattr(b, '_service_keys', {}))
        # Retrieve new cmd and event here
        # TODO: check for overwritten cmd/events ?

        scan_nmspc(nmspc)
        # Build the actual type and register the list of events&cmds
        return type.__new__(cls, name, bases, cooked_nmspc)


class ServiceMixin(metaclass=MetaBaseService):
    pass


class BaseService(metaclass=MetaBaseService):
    def __init__(self, name=None):
        super().__init__()
        self._cmds = None
        self._bootstrapped = asyncio.Future()
        self.name = name or getattr(self, 'name', None)
        assert self.name, 'Unnamed service is not allowed.'

    async def teardown(self):
        pass

    async def bootstrap(self):
        self._bootstrapped.set_result(None)

    async def wait_bootstrapped(self):
        await self._bootstrapped

    def inject_services(self):
        for key, service_name in self._service_keys.items():
            service = yield service_name
            setattr(self, key, service)

    @property
    def cmds(self):
        # Lazy load commands to get them binded to the self object
        if not self._cmds:
            self._cmds = {name: getattr(self, field) for field, name in self._cmd_keys.items()}
        return self._cmds

    @property
    def events(self):
        return self._events

    async def dispatch_msg(self, msg, session=None):
        session = session or AnonymousSession()
        try:
            cmd_name = msg.get('cmd')
            if not cmd_name:
                return {'status': 'bad_msg', 'label': 'Missing cmd field.'}
            return await self.cmds[msg['cmd']](session, msg)
        except ParsecError as exc:
            return exc.to_dict()
