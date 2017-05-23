import json


class ExceptionsMap:
    def __init__(self):
        self.map = {}

    def register(self, name, bases, nmspc):
        kls = type(name, bases, nmspc)
        already_exists = self.map.get(kls.status)
        if already_exists:
            raise RuntimeError('Exceptions %s already has status %s' % (already_exists, already_exists.status))
        self.map[kls.status] = kls
        return kls

    def retrieve(self, status):
        try:
            return self.map[status]
        except KeyError:
            raise RuntimeError('No parsec exception with status %s' % status)


_parsec_exceptions_map = ExceptionsMap()
exception_from_status = _parsec_exceptions_map.retrieve


class ParsecError(Exception, metaclass=_parsec_exceptions_map.register):
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


class PubKeyError(ParsecError):
    status = 'pubkey_error'


class PubKeyNotFound(PubKeyError):
    status = 'pubkey_not_found'
