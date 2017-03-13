
class ParsecError(Exception):
    status = 'error'

    def __init__(self, *args):
        if len(args) == 1:
            self.label = args[0]
        else:
            self.status, self.label = args

    def to_dict(self):
        return {'status': self.status, 'label': self.label}


class ServiceNotReadyError(ParsecError):
    status = 'service_not_ready'


class BadMessageError(ParsecError):
    status = 'bad_msg'
