import json
from uuid import uuid4
from logbook import Logger

from parsec.exceptions import ParsecError


class BaseClientContext:

    async def recv(self):
        raise NotImplementedError()

    async def send(self, body):
        raise NotImplementedError()


class BaseServer:
    def __init__(self):
        self._cmds = {
            'list_cmds': self.__cmd_LIST_CMDS
        }
        self._services = {}

    async def __cmd_LIST_CMDS(self, data):
        return {'status': 'ok', 'cmds': list(self._cmds.keys())}

    def register_service(self, service):
        self._services[service.name] = service
        for cmdid, cb in service.cmds.items():
            self.register_cmd('%s:%s' % (service.name, cmdid), cb)

    def register_cmd(self, cmd, cb):
        if cmd in self._cmds:
            raise RuntimeError('Command `%s` already registered.' % cmd)
        self._cmds[cmd] = cb

    @staticmethod
    def _load_raw_cmd(raw):
        if not raw:
            return None
        try:
            msg = json.loads(raw.decode())
            if isinstance(msg.get('cmd'), str):
                return msg
            else:
                return None
        except json.decoder.JSONDecodeError:
            pass
        # Not a JSON payload, try cmdline mode
        splitted = raw.decode().strip().split(' ')
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
        conn_log = Logger('Connection ' + uuid4().hex)
        conn_log.debug('Connection started')
        while True:
            raw_cmd = await context.recv()
            if not raw_cmd:
                conn_log.debug('Connection stopped')
                return
            conn_log.debug('Received: %r' % raw_cmd)
            msg = self._load_raw_cmd(raw_cmd)
            if msg is None:
                resp = {"status": "bad_message", "label": "Message is not a valid JSON."}
            else:
                cmd = self._cmds.get(msg['cmd'])
                if not cmd:
                    resp = {"status": "badcmd", "label": "Unknown command `%s`" % msg['cmd']}
                else:
                    try:
                        resp = await cmd(msg)
                    except ParsecError as exc:
                        resp = exc.to_dict()
            conn_log.debug('Replied: %r' % resp)
            await context.send(json.dumps(resp).encode())

    def bootstrap_services(self):
        errors = []
        for service in self._services.values():
            try:
                boot = service.bootstrap()
                dep = next(boot)
                while True:
                    if dep not in self._services:
                        errors.append("Service `%s` required unknown service `%s`" % (service.name, dep))
                        break
                    dep = boot.send(self._services[dep])
            except StopIteration:
                pass
        if errors:
            raise RuntimeError(errors)

    def start(self):
        raise NotImplementedError()
