from parsec import tools


class ExceptionsMap:
    def __init__(self):
        self.map = {}

    def register(self, kls):
        already_exists = self.map.get(kls.status)
        if already_exists:
            raise RuntimeError('Exceptions %s conflicts with %s (both has status %s)' %
                (kls, already_exists, already_exists.status))
        self.map[kls.status] = kls

    def retrieve(self, status):
        try:
            return self.map[status]
        except KeyError:
            raise RuntimeError('No parsec exception with status %s' % status)


class MetaParsecError(type):

    def __new__(cls, name, bases, nmspc):
        kls = type.__new__(cls, name, bases, nmspc)
        _parsec_exceptions_map.register(kls)
        return kls


_parsec_exceptions_map = ExceptionsMap()
exception_from_status = _parsec_exceptions_map.retrieve


class ParsecError(Exception, metaclass=MetaParsecError):
    status = 'error'
    label = ''

    def __init__(self, *args):
        args_count = len(args)
        if args_count == 1:
            self.label = args[0]
        elif args_count == 2:
            self.status, self.label = args

    def to_dict(self):
        return {'status': self.status, 'label': self.label}

    def to_raw(self):
        return tools.ejson_dumps(self.to_dict())


class ServiceNotReadyError(ParsecError):
    status = 'service_not_ready'


class BadMessageError(ParsecError):
    status = 'bad_msg'


class HandshakeError(ParsecError):
    status = 'bad_handshake'
    label = 'Session handshake failed.'


# Backend errors

class PrivKeyError(ParsecError):
    status = 'privkey_error'


class PrivKeyHashCollision(PrivKeyError):
    status = 'privkey_hash_collision'


class PrivKeyNotFound(PrivKeyError):
    status = 'privkey_not_found'


class PubKeyError(ParsecError):
    status = 'pubkey_error'


class PubKeyNotFound(PubKeyError):
    status = 'pubkey_not_found'


class VlobError(ParsecError):
    status = 'vlob_error'


class VlobNotFound(VlobError):
    status = 'vlob_not_found'


class TrustSeedError(ParsecError):
    status = 'trust_seed_error'


class UserVlobError(ParsecError):
    status = 'user_vlob_error'


class UserVlobNotFound(ParsecError):
    status = 'user_vlob_not_found'


class GroupError(ParsecError):
    status = 'group_error'


class GroupAlreadyExist(GroupError):
    status = 'group_already_exists'


class GroupNotFound(GroupError):
    status = 'group_not_found'


class BlockError(ParsecError):
    status = 'block_error'


class BlockConnectionError(ParsecError):
    status = 'block_connection_error'


class BlockNotFound(BlockError):
    status = 'block_not_found'


# Core errors


class FileError(ParsecError):
    status = 'file_error'


class FileNotFound(FileError):
    status = 'file_not_found'


class IdentityError(ParsecError):
    status = 'identity_error'


class IdentityNotLoadedError(IdentityError):
    status = 'identity_not_loaded'


class InvalidPath(ParsecError):
    status = 'invalid_path'


class ManifestError(ParsecError):
    status = 'manifest_error'


class ManifestNotFound(ParsecError):
    status = 'manifest_not_found'


class BackendConnectionError(ParsecError):
    status = 'backend_connection_error'


class BackendIdentityRegisterError(ParsecError):
    status = 'backend_identity_register_error'