from pathlib import PurePosixPath
from typing import NewType, Union

from parsec.crypto import SymetricKey, HashDigest
from parsec.core.types.base import TrustSeed, AccessID, TrustSeedField


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ManifestAccess:
    id: AccessID
    rts: TrustSeed
    wts: TrustSeed
    key: SymetricKey


class ManifestAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.SymetricKey(required=True)
    rts = TrustSeedField(required=True, validate=validate.Length(min=1, max=32))
    wts = TrustSeedField(required=True, validate=validate.Length(min=1, max=32))


manifest_access_schema = ManifestAccessSchema(strict=True)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BlockAccess:
    id: Id
    key: SymetricKey
    offset: int
    size: int
    digest: HashDigest


class BlockAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    # TODO: provide digest as hexa string
    digest = fields.String(required=True, validate=validate.Length(min=1, max=64))


block_access_schema = BlockAccessSchema(strict=True)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DirtyBlockAccess:
    id: Id
    key: SymetricKey
    offset: int
    size: int


class DirtyBlockAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))


dirty_block_access_schema = DirtyBlockAccessSchema(strict=True)
