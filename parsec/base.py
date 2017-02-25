
class ParsecError(Exception):
    status = 'error'

    def __init__(self, *args):
        if len(args) == 1:
            self.label = args[0]
        else:
            self.status, self.label = args

    def to_dict(self):
        return {'status': self.status, 'label': self.label}


def cmd(param):
    # TODO: do that with metaclasses
    def patcher(name, callback):
        if not hasattr(callback, '_cmds'):
            callback._cmds = []
        callback._cmds.append(name)
        return callback

    if callable(param):
        patcher(param.__name__, param)
        return param
    else:
        return lambda fn: patcher(param, fn)


class BaseService:
    def get_cmds(self):
        cmds = {}
        for key in dir(self):
            value = getattr(self, key)
            if hasattr(value, '_cmds'):
                for name in value._cmds:
                    cmds[name] = value
        return cmds

    async def dispatch_msg(self, msg):
        try:
            return await self.get_cmds()[msg['cmd']](msg)
        except ParsecError as exc:
            return exc.to_dict()
