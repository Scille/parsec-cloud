import json


class ParsecError(Exception):
    status = 'error'

    def __init__(self, *args):
        args_count = len(args)
        if args_count == 1:
            self.label = args[0]
        elif args_count == 2:
            self.status, self.label = args

    def to_dict(self):
        return {'status': self.status, 'label': self.label}

    def to_raw(self):
        return json.dumps(self.to_dict).encode()


class ServiceNotReadyError(ParsecError):
    status = 'service_not_ready'


class BadMessageError(ParsecError):
    status = 'bad_msg'


class HandshakeError(ParsecError):
    status = 'bad_handshake'
    label = 'Session handshake failed.'
