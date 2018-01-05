from parsec.utils import ParsecError


class VersionError(ParsecError):
    status = 'version_error'
    reason = 'Wrong blob version.'


class NotFoundError(ParsecError):
    status = 'not_found_error'
    reason = 'Element not found.'


class TrustSeedError(ParsecError):
    status = 'trust_seed_error'
    reason = 'Invalid read trust seed.'


class AlreadyExistsError(ParsecError):
    status = 'already_exists_error'
    reason = 'Element already exists.'


class UserClaimError(ParsecError):
    status = 'user_claim_error'
    reason = 'User related error'
