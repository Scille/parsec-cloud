from parsec.utils import ParsecError


class VersionError(ParsecError):
    status = 'version_error'
    reason = 'Wrong blob version.'


class VlobNotFound(VlobError):
    status = 'vlob_not_found'
    reason = 'Vlob not found.'


class TrustSeedError(ParsecError):
    status = 'trust_seed_error'
    reason = 'Invalid read trust seed.'


class BlockAlreadyExistsError(ParsecError):
    status = 'block_already_exist'


class BlockNotFoundError(ParsecError):
    status = 'block_not_found'
    reason = 'Unknown block id.'


class GroupAlreadyExist(ParsecError):
    status = 'group_already_exists'
    reason = 'Group already exist.'


class GroupNotFound(ParsecError):
    status = 'group_not_found'
    reason = 'Group not found.'


class PubKeyAlreadyExists(ParsecError):
    status = 'pubkey_already_exists'


class PubKeyNotFound(ParsecError):
    status = 'pubkey_not_found'
